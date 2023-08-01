use std::collections::HashSet;

use heck::ToShoutySnakeCase;

use crate::parser::{Class, Document, DocumentEntry, Enum, EnumValue, EnumVariant};

#[derive(Default)]
pub struct Generator {
    imports: HashSet<String>,
    body: String,
}

impl ToString for Generator {
    fn to_string(&self) -> String {
        let body = &self.body;
        let imports = self
            .imports
            .iter()
            .map(|import| format!("use {import};\n"))
            .collect::<String>();

        if imports.is_empty() {
            body.clone()
        } else {
            format!("#![allow(non_camel_case_types)]\n#![allow(deprecated)]\n#![allow(unused_imports)]\n\n{imports}\n{body}")
        }
    }
}

pub trait Generate {
    fn generate(&self, gen: &mut Generator);

    fn generate_stream(&self) -> Generator {
        let mut gen = Generator::default();
        self.generate(&mut gen);
        gen
    }
}

impl Generate for Document {
    fn generate(&self, gen: &mut Generator) {
        for entry in self.entries.iter() {
            match entry {
                DocumentEntry::Enum(enum_) => enum_.generate(gen),
                DocumentEntry::Class(class) => class.generate(gen),
                DocumentEntry::Import(import) => {
                    let module = import.replace(".steamd", "");
                    gen.imports.insert(format!("super::{module}::*"));
                }
            }
        }
    }
}

fn unique_variants<'a>(variants: &'a Vec<EnumVariant>) -> Vec<EnumVariant> {
    let mut variants = variants.clone();
    let mut discriminants = HashSet::new();

    for variant in variants.iter_mut().rev() {
        if variant.removed {
            continue;
        }

        let value = match &variant.value {
            EnumValue::Hex(hex) => i64::from_str_radix(hex, 16).unwrap(),
            EnumValue::Number(num) => num.parse::<i64>().unwrap(),
            _ => panic!(),
        };

        if discriminants.contains(&value) {
            variant.removed = true;
        } else {
            discriminants.insert(value);
        }
    }

    variants
}

fn convert_type(type_: &str) -> &str {
    match type_ {
        "char" => "i8",
        "short" => "i16",
        "int" => "i32",
        "long" => "i64",
        "byte" => "u8",
        "ushort" => "u16",
        "uint" => "u32",
        "ulong" => "u64",
        _ => panic!(),
    }
}

impl Generate for Enum {
    fn generate(&self, gen: &mut Generator) {
        let name = &self.name;
        let repr = convert_type(self.generic.as_ref().map(String::as_str).unwrap_or("int"));

        if self.flags {
            gen.imports.insert("bitflags::bitflags".to_owned());

            gen.body.push_str("bitflags! {\n");
            gen.body.push_str(
                "    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]\n",
            );
            gen.body
                .push_str(&format!("    pub struct {name}: {repr} {{\n"));

            for variant in self.variants.iter() {
                let name = variant.name.to_shouty_snake_case();
                let value = match &variant.value {
                    crate::parser::EnumValue::Number(num) => num.to_string(),
                    crate::parser::EnumValue::Hex(hex) => format!("0x{hex}"),
                    crate::parser::EnumValue::Or(list) => list
                        .iter()
                        .map(|name| {
                            let name = name.to_shouty_snake_case();
                            format!("Self::{name}.bits()")
                        })
                        .collect::<Vec<_>>()
                        .join(" | "),
                };

                // add indentation
                gen.body.push_str("        ");

                if variant.removed {
                    if let Some(reason) = &variant.reason {
                        gen.body.push_str(&format!("// {reason}\n        "));
                    }
                    gen.body.push_str("// ");
                } else if variant.obsolete {
                    match &variant.reason {
                        Some(reason) => gen
                            .body
                            .push_str(&format!("#[deprecated = \"{reason}\"]\n        ")),
                        None => gen.body.push_str("#[deprecated]\n        "),
                    }
                }

                gen.body.push_str(&format!("const {name} = {value};\n"))
            }

            gen.body.push_str("    }\n");
            gen.body.push_str("}\n\n");
        } else {
            gen.imports.insert("num_derive::FromPrimitive".to_owned());
            gen.imports.insert("num_derive::ToPrimitive".to_owned());

            gen.body.push_str(
                "#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]\n",
            );
            gen.body.push_str("#[derive(FromPrimitive, ToPrimitive)]\n");
            gen.body.push_str(&format!("#[repr({repr})]\n"));
            gen.body.push_str(&format!("pub enum {name} {{\n"));

            let mut first = true;

            for variant in unique_variants(&self.variants) {
                let name = &variant.name;
                let value = match &variant.value {
                    crate::parser::EnumValue::Number(num) => num.to_string(),
                    crate::parser::EnumValue::Hex(hex) => format!("0x{hex}"),
                    crate::parser::EnumValue::Or(_) => panic!(),
                };

                // add indentation
                gen.body.push_str("    ");

                if variant.removed {
                    if let Some(reason) = &variant.reason {
                        gen.body.push_str(&format!("// {reason}\n    "));
                    }
                    gen.body.push_str("// ");
                } else if variant.obsolete {
                    match &variant.reason {
                        Some(reason) => gen
                            .body
                            .push_str(&format!("#[deprecated = \"{reason}\"]\n    ")),
                        None => gen.body.push_str("#[deprecated]\n    "),
                    }
                } else if first {
                    gen.body.push_str("#[default]\n    ");
                    first = false;
                }

                gen.body.push_str(&format!("{name} = {value},"));

                if let Some(comment) = &variant.comment {
                    gen.body.push_str(&format!(" // {comment}\n"));
                } else {
                    gen.body.push_str("\n");
                }
            }

            gen.body.push_str("}\n\n");
        }
    }
}

impl Generate for Class {
    fn generate(&self, _gen: &mut Generator) {}
}
