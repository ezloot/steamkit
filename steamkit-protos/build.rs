use std::{env, fs, ops::Not, path::PathBuf};

use heck::{ToShoutySnakeCase, ToUpperCamelCase};
use itertools::Itertools;
use lazy_static::lazy_static;
use parse_int::parse;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use regex::Regex;

fn protobufs() {
    let paths = fs::read_dir("src/protos")
        .expect("failed to get protobuf files")
        .filter_map(|path| {
            let path = path.unwrap().path();
            if let Some(ext) = path.extension() {
                if ext == "proto" {
                    return Some(path);
                }
            }
            None
        });

    protobuf_codegen::Codegen::new()
        .protoc()
        .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
        .includes(["src/protos"])
        .inputs(paths)
        .cargo_out_dir("protos")
        .run_from_script();
}

#[derive(Debug, Clone)]
struct Enum {
    name: String,
    variants: Vec<EnumVariant>,
}

impl ToTokens for Enum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = syn::parse_str::<Ident>(&self.name).unwrap();
        let variants = &self.variants;

        tokens.append_all(quote! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct #name(pub u32);

            impl #name {
                #(#variants)*
            }

            impl From<#name> for u32 {
                fn from(value: #name) -> Self {
                    value.0
                }
            }

            impl From<u32> for #name {
                fn from(value: u32) -> Self {
                    #name(value)
                }
            }

            impl ::std::ops::BitOr for #name {
                type Output = Self;

                fn bitor(self, rhs: Self) -> Self::Output {
                    Self(self.0 | rhs.0)
                }
            }

            impl ::std::ops::BitOrAssign for #name {
                fn bitor_assign(&mut self, rhs: Self) {
                    self.0 |= rhs.0;
                }
            }

            impl ::std::ops::BitAnd for #name {
                type Output = Self;

                fn bitand(self, rhs: Self) -> Self::Output {
                    Self(self.0 & rhs.0)
                }
            }

            impl ::std::ops::BitAndAssign for #name {
                fn bitand_assign(&mut self, rhs: Self) {
                    self.0 &= rhs.0;
                }
            }

            impl ::std::ops::BitXor for #name {
                type Output = Self;

                fn bitxor(self, rhs: Self) -> Self::Output {
                    Self(self.0 ^ rhs.0)
                }
            }

            impl ::std::ops::BitXorAssign for #name {
                fn bitxor_assign(&mut self, rhs: Self) {
                    self.0 ^= rhs.0;
                }
            }

            impl ::std::ops::Not for #name {
                type Output = Self;

                fn not(self) -> Self::Output{
                    Self(!self.0)
                }
            }

            impl ::std::ops::Deref for #name {
                type Target = u32;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl ::std::ops::DerefMut for #name {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }
        });
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EnumVariant {
    key: String,
    value: EnumVariantValue,
    comment: Option<String>,
    deprecated: bool,
}

impl ToTokens for EnumVariant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let key = syn::parse_str::<Ident>(&self.key).unwrap();
        let value = &self.value;

        let deprecated = self.deprecated.then(|| quote!(#[deprecated]));
        let comment = self
            .comment
            .as_ref()
            .map(|comment| quote!(#[doc = #comment]));

        tokens.append_all(quote! {
            #comment
            #deprecated
            pub const #key: Self = #value;
        });
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum EnumVariantValue {
    Int(u32),
    Reference(String),
    Union(Vec<Self>),
}

impl ToTokens for EnumVariantValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            EnumVariantValue::Int(int) => tokens.append_all(quote!(Self(#int))),
            EnumVariantValue::Reference(key) => {
                let key = syn::parse_str::<Ident>(key).unwrap();
                tokens.append_all(quote!(Self::#key.0));
            }
            EnumVariantValue::Union(vec) => {
                let inner = Itertools::intersperse(
                    vec.iter().map(|value| value.into_token_stream()),
                    quote!(|),
                )
                .collect::<TokenStream>();
                tokens.append_all(quote!(Self(#inner)));
            }
        }
    }
}

lazy_static! {
    static ref ENUM_REGEX: Regex =
        Regex::new(r"(?i)enum\s+(?P<name>[a-z]+)[\s\n][^{]*\{(?P<inner>[^}]+)\}").unwrap();
    static ref VARIANT_REGEX: Regex =
        Regex::new(r"(?i)(?P<key>[a-z0-9_]+)\s*=\s*\s*(?P<value>[a-z0-9_|\s]+)\s*;(?P<comment>.*)")
            .unwrap();
}

fn parse_enums(data: &str) -> Vec<Enum> {
    let mut enums = vec![];
    for capture in ENUM_REGEX.captures_iter(data) {
        let name = capture["name"].to_upper_camel_case();
        let mut variants = vec![];

        for capture in VARIANT_REGEX.captures_iter(&capture["inner"]) {
            let key = capture["key"].to_shouty_snake_case();

            let value = capture["value"].trim();
            let value = match parse::<u32>(value) {
                Ok(int) => EnumVariantValue::Int(int),
                Err(_) => EnumVariantValue::Union(
                    value
                        .split('|')
                        .map(|key| EnumVariantValue::Reference(key.trim().to_shouty_snake_case()))
                        .collect(),
                ),
            };

            let comment = capture["comment"].trim();
            let comment = comment.is_empty().not().then(|| comment.to_owned());

            variants.push(EnumVariant {
                key,
                value,
                deprecated: false,
                comment,
            });
        }

        let mut used_ids = vec![];
        for variant in variants.iter_mut().rev() {
            if let EnumVariantValue::Int(int) = variant.value {
                if used_ids.contains(&int) {
                    variant.deprecated = true;
                }
                used_ids.push(int);
            }

            if variant
                .comment
                .as_ref()
                .filter(|comment| comment.contains("removed") || comment.contains("obsolete"))
                .is_some()
            {
                variant.deprecated = true;
            }
        }

        let mut used_keys = vec![];
        for variant in variants.iter_mut().rev() {
            if used_keys.contains(&variant.key) {
                variant.key = format!("{}_LEGACY", variant.key);
            }
            used_keys.push(variant.key.clone());
        }

        enums.push(Enum { name, variants });
    }
    enums
}

fn resources() {
    // get resources from source files
    let paths = fs::read_dir("src/resources")
        .expect("failed to get resources")
        .filter_map(|path| {
            let path = path.unwrap().path();
            if let Some(ext) = path.extension() {
                if ext == "steamd" {
                    return Some(path);
                }
            }
            None
        });

    let mut tokens = TokenStream::new();

    for path in paths {
        let data = fs::read_to_string(&path).expect("failed to read resource");
        let enums = parse_enums(&data);
        tokens.append_all(enums.iter());
    }

    // get resources output path
    let cargo_out_dir = env::var("OUT_DIR").expect("OUT_DIR env var not set");
    let mut res_path = PathBuf::from(cargo_out_dir);
    res_path.push("resources.rs");

    let code = prettyplease::unparse(&syn::parse2(tokens).expect("bad code generation"));

    // write generated file
    fs::write(&res_path, code).unwrap();
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/protos");
    println!("cargo:rerun-if-changed=src/resources");

    resources();
    protobufs();
}
