use std::collections::{HashMap, HashSet};

use anyhow::Context as AnyhowContext;
use heck::{ToShoutySnakeCase, ToSnakeCase};

use crate::parser::{
    Class, ClassMemberValue, DataType, Document, DocumentEntry, Enum, EnumVariantValue,
};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Mapping(Vec<String>);

#[derive(Debug, Clone)]
struct Mappings {
    global: HashMap<Mapping, Mapping>,
    local: HashMap<Mapping, Mapping>,
}

impl Mappings {
    fn new(modules: &HashMap<String, Document>) -> anyhow::Result<Self> {
        let mut removed = HashSet::new();
        let mut mappings = HashMap::new();
        let mut key = vec![];
        let mut value = vec![];

        static RESERVED: &[&str] = &["type"];

        for (name, module) in modules {
            key.push(name.clone());
            value.push(name.clone());

            for entry in &module.entries {
                match entry {
                    DocumentEntry::Class(class) => {
                        key.push(class.name.clone());
                        value.push(class.name.clone());

                        mappings.insert(Mapping(key.clone()), Mapping(value.clone()));
                        // class.

                        for member in &class.members {
                            let name = if member.constant {
                                member.name.to_shouty_snake_case()
                            } else {
                                let mut name = member.name.to_snake_case();
                                // add workaround for reserved keywords
                                if RESERVED.contains(&name.as_str()) {
                                    name.push('_');
                                }
                                name
                            };

                            key.push(member.name.clone());
                            value.push(name);

                            mappings.insert(Mapping(key.clone()), Mapping(value.clone()));

                            key.pop();
                            value.pop();
                        }

                        key.pop();
                        value.pop();
                    }
                    DocumentEntry::Enum(enum_) => {
                        key.push(enum_.name.clone());
                        value.push(enum_.name.clone());

                        mappings.insert(Mapping(key.clone()), Mapping(value.clone()));

                        for variant in &enum_.variants {
                            let name = variant.name.to_shouty_snake_case();

                            key.push(variant.name.clone());
                            value.push(name);

                            if variant.removed {
                                removed.insert(Mapping(key.clone()));
                            }

                            mappings.insert(Mapping(key.clone()), Mapping(value.clone()));

                            key.pop();
                            value.pop();
                        }

                        key.pop();
                        value.pop();
                    }
                    DocumentEntry::Import(_) => {}
                }
            }

            key.pop();
            value.pop();
        }

        Ok(Self {
            local: HashMap::with_capacity(mappings.len()),
            global: mappings,
        })
    }

    fn get(&self, key: &[&str]) -> Option<&str> {
        let key = key.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let key = Mapping(key);

        self.global
            .get(&key)
            .map(|value| value.0.last().unwrap().as_str())
    }

    fn get_local(&self, key: &[&str]) -> Option<&str> {
        let key = key.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let key = Mapping(key);

        self.local
            .get(&key)
            .map(|value| value.0.last().unwrap().as_str())
    }

    fn update_local(&mut self, modules: &HashSet<String>) -> anyhow::Result<()> {
        self.local.clear();
        // for module in modules {
        //     let key = Mapping(vec![module.clone()]);
        //     for
        // }

        for (key, value) in &self.global {
            if modules.contains(&key.0[0]) {
                self.local
                    .insert(Mapping(key.0[1..].to_vec()), value.clone());
            }
        }

        Ok(())
    }
}

struct Context {
    imports: HashSet<String>,
    mappings: Mappings,
    module: Option<String>,
}

trait Generate {
    fn generate(&self, ctx: &Context) -> anyhow::Result<String>;
}

impl Generate for DataType {
    fn generate(&self, ctx: &Context) -> anyhow::Result<String> {
        match self {
            DataType::I8 => Ok("i8".to_string()),
            DataType::I16 => Ok("i16".to_string()),
            DataType::I32 => Ok("i32".to_string()),
            DataType::I64 => Ok("i64".to_string()),
            DataType::U8 => Ok("u8".to_string()),
            DataType::U16 => Ok("u16".to_string()),
            DataType::U32 => Ok("u32".to_string()),
            DataType::U64 => Ok("u64".to_string()),
            DataType::F32 => Ok("f32".to_string()),
            DataType::F64 => Ok("f64".to_string()),
            DataType::Reference(value) => {
                if value == "SteamKit2.GC.Internal.CMsgProtoBufHeader"
                    || value == "SteamKit2.Internal.CMsgProtoBufHeader"
                {
                    return Ok("Vec<u8>".to_string());
                }

                let value = ctx.mappings.get_local(&[&value]).context(format!(
                    "missing mapping {value} for {}",
                    ctx.module.as_ref().unwrap()
                ))?;

                Ok(value.into())
            }
            DataType::FixedLengthArray { type_, length } => {
                Ok(format!("[{}; {}]", type_.generate(ctx)?, length))
            }
        }
    }
}

impl Generate for ClassMemberValue {
    fn generate(&self, ctx: &Context) -> anyhow::Result<String> {
        match self {
            ClassMemberValue::Number(value) => Ok(value.into()),
            ClassMemberValue::Hex(value) => Ok(format!("0x{value}")),
            ClassMemberValue::Reference(value) => {
                if value == "ulong.MaxValue" {
                    return Ok("u64::MAX".to_string());
                }

                let key = Mapping(value.split("::").map(|s| s.to_owned()).collect::<Vec<_>>());
                let value = ctx.mappings.local.get(&key).context(format!(
                    "missing mapping {value} for {}",
                    ctx.module.as_ref().unwrap()
                ))?;

                Ok(value.0[1..].join("::"))
            }
        }
    }
}

impl Generate for Document {
    fn generate(&self, ctx: &Context) -> anyhow::Result<String> {
        let mut writer = String::from("#![allow(unused_imports)]\n#![allow(deprecated)]\n\n");

        // add imports
        for import in &ctx.imports {
            writer.push_str(&format!("use {import};\n"));
        }

        // add space between imports and definitions
        if !ctx.imports.is_empty() {
            writer.push_str("\n");
        }

        for entry in &self.entries {
            match entry {
                DocumentEntry::Class(class) => {
                    writer.push_str(&class.generate(ctx)?);
                }
                DocumentEntry::Enum(enum_) => {
                    writer.push_str(&enum_.generate(ctx)?);
                }
                DocumentEntry::Import(_) => {}
            }
        }

        Ok(writer)
    }
}

impl Generate for Enum {
    fn generate(&self, ctx: &Context) -> anyhow::Result<String> {
        let mut writer = String::new();
        let name = &self.name;
        let type_ = match &self.generic {
            Some(generic) => generic.generate(ctx)?,
            None => "i32".to_string(),
        };

        writer.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]\n");
        writer.push_str(&format!("pub struct {name}(pub {type_});\n\n"));
        writer.push_str(&format!("impl {name} {{\n"));

        let first_variant = self.variants.iter().find(|variant| !variant.removed);
        for variant in &self.variants {
            let name = ctx
                .mappings
                .get(&[
                    &ctx.module.clone().context("missing module")?,
                    &self.name,
                    &variant.name,
                ])
                .context(format!(
                    "missing mapping {} for {}",
                    variant.name, self.name
                ))?;

            let value = match &variant.value {
                EnumVariantValue::Number(num) => format!("Self({num})"),
                EnumVariantValue::Hex(hex) => format!("Self(0x{hex})"),
                EnumVariantValue::Union(list) => format!(
                    "Self({})",
                    list.iter()
                        .map(|name| format!("Self::{}.0", name.to_shouty_snake_case())) // TODO: actually verify mappings exist locally
                        .collect::<Vec<_>>()
                        .join(" | ")
                ),
            };

            let mut s = format!("pub const {name}: Self = {value};\n");

            if variant.removed {
                if let Some(reason) = &variant.reason {
                    writer.push_str(&format!("    // {reason}\n"));
                }
                // add comment to line
                s = format!("// {s}");
            } else if variant.obsolete {
                match &variant.reason {
                    Some(reason) => writer.push_str(&format!("    #[deprecated = \"{reason}\"]\n")),
                    None => writer.push_str("    #[deprecated]\n"),
                }
            }

            writer.push_str(&format!("    {s}"));
        }

        writer.push_str("}\n\n");
        writer.push_str("");

        writer.push_str(&format!("impl PartialEq<{type_}> for {name} {{\n"));
        writer.push_str(&format!("    fn eq(&self, other: &{type_}) -> bool {{\n"));
        writer.push_str("        self.0 == *other\n");
        writer.push_str("    }\n");
        writer.push_str("}\n\n");

        writer.push_str(&format!("impl PartialEq<{name}> for {type_} {{\n"));
        writer.push_str(&format!("    fn eq(&self, other: &{name}) -> bool {{\n"));
        writer.push_str("        *self == other.0\n");
        writer.push_str("    }\n");
        writer.push_str("}\n");

        if self.flags {
            writer.push_str(&format!("\nimpl std::ops::BitOr for {name} {{\n"));
            writer.push_str("    type Output = Self;\n");
            writer.push_str("    fn bitor(self, rhs: Self) -> Self::Output {\n");
            writer.push_str("        Self(self.0 | rhs.0)\n");
            writer.push_str("    }\n");
            writer.push_str("}\n");
        } else if let Some(variant) = first_variant {
            let variant_name = ctx
                .mappings
                .get(&[
                    &ctx.module.clone().context("missing module")?,
                    &self.name,
                    &variant.name,
                ])
                .context("missing mapping")?;

            writer.push_str(&format!("\nimpl Default for {name} {{\n"));
            writer.push_str(&"    fn default() -> Self {\n");
            writer.push_str(&format!("        Self::{variant_name}\n"));
            writer.push_str(&"    }\n");
            writer.push_str(&"}\n");

            // TODO: add a way to verify validity of value
        }

        Ok(writer)
    }
}

impl Generate for Class {
    fn generate(&self, ctx: &Context) -> anyhow::Result<String> {
        let mut writer = String::new();

        let name = &self.name;
        let prefix = if self.removed {
            "// "
        } else {
            ""
        };

        let members = self
            .members
            .iter()
            .filter(|member| !member.constant)
            .collect::<Vec<_>>();
        let constants = self
            .members
            .iter()
            .filter(|member| member.constant)
            .collect::<Vec<_>>();

        if !members.is_empty() {
            writer.push_str(&format!("{prefix}#[derive(Debug, Clone, derive_new::new)]\n"));
            writer.push_str(&format!("{prefix}pub struct {name} {{\n"));

            for member in members {
                // skip constants
                if member.constant {
                    continue;
                }

                let name = ctx
                    .mappings
                    .get(&[
                        &ctx.module.clone().context("missing module")?,
                        &self.name,
                        &member.name,
                    ])
                    .context("missing mapping")?;

                // TODO: handle complex types like steamidmarshal etc
                let type_ = member.type_.generate(ctx)?;

                if let Some(_value) = &member.value {
                    let value = member
                        .value
                        .as_ref()
                        .context("missing value")?
                        .generate(ctx)?;

                    writer.push_str(&format!("{prefix}    #[new(value = \"{value}\")]\n"));
                }

                writer.push_str(&format!("{prefix}    pub {name}: {type_},\n"));
            }

            writer.push_str(&format!("{prefix}}}\n\n"));
        } else {
            writer.push_str(&format!("{prefix}#[derive(Debug, Clone, Default)]\n"));
            writer.push_str(&format!("{prefix}pub struct {name};\n\n"));
        }

        if !constants.is_empty() {
            writer.push_str(&format!("{prefix}impl {name} {{\n"));

            for member in &self.members {
                // skip constants
                if !member.constant {
                    continue;
                }

                let name = ctx
                    .mappings
                    .get(&[
                        &ctx.module.clone().context("missing module")?,
                        &self.name,
                        &member.name,
                    ])
                    .context("missing mapping")?;

                let type_ = member.type_.generate(ctx)?;
                let value = member
                    .value
                    .as_ref()
                    .context("missing value")?
                    .generate(ctx)?;

                // TODO: register definition

                writer.push_str(&format!("{prefix}    pub const {name}: {type_} = {value};\n"));
            }

            writer.push_str(&format!("{prefix}}}\n\n"));
        }

        if let Some(DataType::Reference(value)) = &self.generic {
            if value.starts_with("EMsg::") {
                let key = Mapping(vec!["emsg".into(), "EMsg".into(), value[6..].into()]);

                if let Some(value) = ctx.mappings.global.get(&key) {
                    let value = value.0.join("::");
                    writer.push_str(&format!("{prefix}impl crate::HasEMsg for {name} {{\n"));
                    writer.push_str(&format!("{prefix}    fn emsg() -> EMsg {{\n"));
                    writer.push_str(&format!("{prefix}        crate::{value}\n"));
                    writer.push_str(&format!("{prefix}    }}\n"));
                    writer.push_str(&format!("{prefix}}}\n\n"));
                }
            }
        }

        Ok(writer)
    }
}

fn flatten_imports(
    modules: &HashMap<String, Document>,
    name: &str,
    output: &mut HashSet<String>,
) -> anyhow::Result<()> {
    if output.contains(name) {
        return Ok(());
    }

    let raw_imports = modules
        .get(name)
        .context("missing module")?
        .entries
        .iter()
        .filter_map(|entry| match &entry {
            DocumentEntry::Import(import) => Some(import),
            _ => None,
        })
        .collect::<Vec<_>>();

    for raw_import in raw_imports {
        if !raw_import.ends_with(".steamd") {
            anyhow::bail!("invalid import: {raw_import}");
        }

        // remove .steamd
        let import = &raw_import[0..raw_import.len() - 7];
        flatten_imports(modules, import, output)?;
        output.insert(import.into());
    }

    Ok(())
}

pub fn generate(modules: HashMap<String, Document>) -> anyhow::Result<HashMap<String, String>> {
    let mut outputs = HashMap::new();
    let mut ctx = Context {
        imports: HashSet::new(),
        mappings: Mappings::new(&modules)?,
        module: None,
    };

    for (name, module) in &modules {
        let mut imports = HashSet::with_capacity(modules.len());
        flatten_imports(&modules, name, &mut imports)?;

        // generate imports for this module
        ctx.imports.clear();
        for import in &imports {
            ctx.imports.insert(format!("super::{import}::*"));
        }

        imports.insert(name.clone());
        // load local mappings
        ctx.mappings.update_local(&imports)?;
        ctx.module = Some(name.clone());

        outputs.insert(name.clone(), module.generate(&mut ctx)?);

        ctx.module = None;
        ctx.imports.clear();
    }

    Ok(outputs)
}
