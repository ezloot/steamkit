use std::collections::HashMap;

use anyhow::Context as AnyhowContext;
use heck::{ToShoutySnakeCase, ToSnakeCase};

use crate::parser::{Class, DataType, Document, DocumentEntry, Enum, EnumVariantValue};

pub struct Context {
    imports: Vec<(String, String)>,
    module: Option<String>,
}

pub trait Generate {
    fn generate(&self, ctx: &mut Context) -> anyhow::Result<String>;
}

impl Generate for DataType {
    fn generate(&self, ctx: &mut Context) -> anyhow::Result<String> {
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
            // DataType::Reference(_) => Err(anyhow::anyhow!("todo")),
            DataType::Reference(_) => Ok("TODO".to_string()),
            DataType::FixedLengthArray { type_, length } => {
                Ok(format!("[{}; {}]", type_.generate(ctx)?, length))
            }
        }
    }
}

impl Generate for Document {
    fn generate(&self, ctx: &mut Context) -> anyhow::Result<String> {
        let mut writer = String::new();

        for entry in &self.entries {
            match entry {
                DocumentEntry::Class(class) => {
                    writer.push_str(&class.generate(ctx)?);
                }
                DocumentEntry::Enum(enum_) => {
                    writer.push_str(&enum_.generate(ctx)?);
                }
                DocumentEntry::Import(import) => {}
            }
        }

        Ok(writer)
    }
}

impl Generate for Enum {
    fn generate(&self, ctx: &mut Context) -> anyhow::Result<String> {
        let mut writer = String::new();

        let name = &self.name;
        let type_ = match &self.generic {
            Some(generic) => generic.generate(ctx)?,
            None => "i32".to_string(),
        };

        writer.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]\n");
        writer.push_str(&format!("pub struct {name}(pub {type_});\n\n"));
        writer.push_str(&format!("impl {name} {{\n"));

        let first_variant = self.variants.first();
        for variant in &self.variants {
            let name = variant.name.to_shouty_snake_case();
            let value = match &variant.value {
                EnumVariantValue::Number(num) => format!("Self({num})"),
                EnumVariantValue::Hex(hex) => format!("Self(0x{hex})"),
                EnumVariantValue::Union(list) => list
                    .iter()
                    .map(|name| format!("Self::{name}"))
                    .collect::<Vec<_>>()
                    .join(" | "),
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
            writer.push_str(&format!("\nimpl ops::BitOr for {name} {{\n"));
            writer.push_str("    type Output = Self;\n");
            writer.push_str("    fn bitor(self, rhs: Self) -> Self::Output {\n");
            writer.push_str("        Self(self.0 | rhs.0)\n");
            writer.push_str("    }\n");
            writer.push_str("}\n");
        } else if let Some(variant) = first_variant {
            let variant_name = &variant.name.to_shouty_snake_case();

            writer.push_str(&format!("\nimpl Default for {name} {{\n"));
            writer.push_str(&"    fn default() -> Self {\n");
            writer.push_str(&format!("        Self::{variant_name}\n"));
            writer.push_str(&"    }\n");
            writer.push_str(&"}\n");
        }

        Ok(writer)
    }
}

impl Generate for Class {
    fn generate(&self, ctx: &mut Context) -> anyhow::Result<String> {
        let mut writer = String::new();

        let name = &self.name;

        if self.removed {
            writer.push_str("#[deprecated]\n");
        }

        writer.push_str("#[derive(Debug, Clone, new)]\n");
        writer.push_str(&format!("pub struct {name} {{\n"));

        for member in &self.members {
            // skip constants
            if member.constant {
                continue;
            }

            let mut name = member.name.to_snake_case();
            // add workaround for reserved keywords
            if name == "type" {
                name.push('_');
            }

            // TODO: handle complex types like steamidmarshal etc
            let type_ = member.type_.generate(ctx)?;

            if let Some(_value) = &member.value {
                // TODO: member default values
                writer.push_str(&format!("    #[new(default = \"todo!()\")]\n"));
            }

            writer.push_str(&format!("    pub {name}: {type_},\n"));
        }

        writer.push_str("}\n\n");
        writer.push_str(&format!("impl {name} {{\n"));

        for member in &self.members {
            // skip constants
            if !member.constant {
                continue;
            }

            let name = member.name.to_shouty_snake_case();
            let type_ = member.type_.generate(ctx)?;

            // TODO: register definition

            writer.push_str(&format!("    pub const {name}: {type_} = todo!();\n"));
        }

        writer.push_str("}\n\n");

        Ok(writer)
    }
}

fn get_imports(
    name: &str,
    modules: &HashMap<String, Document>,
    skip: Vec<String>,
) -> Vec<(String, String)> {
    let mut v = vec![];

    if let Some(module) = modules.get(name) {
        for entry in &module.entries {
            let DocumentEntry::Import(import) = entry else {
                continue;
            };

            println!("{name} {import}");
        }
    }

    v
}

pub fn generate(modules: HashMap<String, Document>) -> anyhow::Result<HashMap<String, String>> {
    let mut outputs = HashMap::new();
    let mut ctx = Context {
        imports: vec![],
        module: None,
    };

    for (name, module) in &modules {
        ctx.module = Some(name.clone());
        ctx.imports = get_imports(&name, &modules, vec![]);

        outputs.insert(name.clone(), module.generate(&mut ctx)?);

        ctx.module = None;
        ctx.imports.clear();
    }

    Ok(outputs)
}
