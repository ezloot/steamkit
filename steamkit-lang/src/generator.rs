// use petgraph::prelude::*;

// fn generate_module(graph: &Graph<Node, NodeEdge>, node_idx: NodeIndex, writer: &mut String) {
//     let Node::Module(module) = &graph[node_idx] else {
//         panic!("node is not a module")
//     };

//     let children = graph
//         .neighbors_directed(node_idx, Direction::Outgoing)
//         .filter(|child_idx| {
//             let child = &graph[*child_idx];
//             match child {
//                 Node::Class(_) | Node::Enum(_) => true,
//                 _ => false,
//             }
//         })
//         .collect::<Vec<_>>();

//     let mut iter = children.into_iter().peekable();
//     let mut structures = vec![];

//     while let Some(node_idx) = iter.next() {
//         let node = &graph[node_idx];
//         if node.is_class() || node.is_enum() {
//             structures.push(node_idx);
//         }
//     }

//     for (i, node_idx) in structures.iter().enumerate() {
//         generate(graph, *node_idx, writer);

//         if i < structures.len() - 1 {
//             writer.push_str("\n");
//         }
//     }
// }

// fn generate_enum(graph: &Graph<Node, NodeEdge>, node_idx: NodeIndex, writer: &mut String) {
//     let Node::Enum(enum_) = &graph[node_idx] else {
//         panic!("node is not an enum")
//     };

//     let mut variants = graph
//         .neighbors_directed(node_idx, Direction::Outgoing)
//         .collect::<Vec<_>>();

//     // sort by index
//     variants.sort_by(|a, b| {
//         let Node::EnumVariant(a) = &graph[*a] else {
//             panic!("node is not an enum variant")
//         };

//         let Node::EnumVariant(b) = &graph[*b] else {
//             panic!("node is not an enum variant")
//         };

//         a.index.cmp(&b.index)
//     });

//     let name = &enum_.name;
//     let type_ = "i32";

//     // if enum_.flags {
//     //     writer.push_str("bitflags! {\n");
//     //     writer.push_str("    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]\n");
//     //     writer.push_str(&format!("    pub struct {name}: i32 {{\n"));
//     // } else {
//     //     writer.push_str(
//     //         "#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]\n",
//     //     );
//     //     writer.push_str("#[derive(FromPrimitive, ToPrimitive)]\n");
//     //     writer.push_str(&format!("#[repr(i32)]\n"));
//     //     writer.push_str(&format!("pub enum {name} {{\n"));
//     // }

//     writer.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]\n");
//     writer.push_str(&format!("pub struct {name}(pub {type_});\n\n"));
//     writer.push_str(&format!("impl {name} {{\n"));

//     let first_variant = variants.first().cloned();
//     for variant in variants {
//         generate_enum_variant(graph, variant, writer);
//     }

//     writer.push_str("}\n\n");
//     writer.push_str("");

//     writer.push_str(&format!("impl PartialEq<{type_}> for {name} {{\n"));
//     writer.push_str(&format!("    fn eq(&self, other: &{type_}) -> bool {{\n"));
//     writer.push_str("        self.0 == *other\n");
//     writer.push_str("    }\n");
//     writer.push_str("}\n\n");

//     writer.push_str(&format!("impl PartialEq<{name}> for {type_} {{\n"));
//     writer.push_str(&format!("    fn eq(&self, other: &{name}) -> bool {{\n"));
//     writer.push_str("        *self == other.0\n");
//     writer.push_str("    }\n");
//     writer.push_str("}\n");

//     if enum_.flags {
//         writer.push_str(&format!("\nimpl ops::BitOr for {name} {{\n"));
//         writer.push_str("    type Output = Self;\n");
//         writer.push_str("    fn bitor(self, rhs: Self) -> Self::Output {\n");
//         writer.push_str("        Self(self.0 | rhs.0)\n");
//         writer.push_str("    }\n");
//         writer.push_str("}\n");
//     } else if let Some(variant) = first_variant {
//         let Node::EnumVariant(variant) = &graph[variant] else {
//             panic!("node is not an enum variant")
//         };

//         let variant_name = &variant.name;

//         writer.push_str(&format!("\nimpl Default for {name} {{\n"));
//         writer.push_str(&"    fn default() -> Self {\n");
//         writer.push_str(&format!("        Self::{variant_name}\n"));
//         writer.push_str(&"    }\n");
//         writer.push_str(&"}\n");
//     }
// }

// fn generate_enum_variant(graph: &Graph<Node, NodeEdge>, node_idx: NodeIndex, writer: &mut String) {
//     let Node::EnumVariant(variant) = &graph[node_idx] else {
//         panic!("node is not an enum variant")
//     };

//     let name = variant.name.to_shouty_snake_case();
//     let value = match &variant.value {
//         EnumVariantValue::Number(num) => format!("Self({num})"),
//         EnumVariantValue::Hex(hex) => format!("Self(0x{hex})"),
//         EnumVariantValue::Union(list) => list
//             .iter()
//             .map(|name| format!("Self::{name}"))
//             .collect::<Vec<_>>()
//             .join(" | "),
//     };

//     let mut s = format!("pub const {name}: Self = {value};\n");

//     if variant.removed {
//         if let Some(reason) = &variant.reason {
//             writer.push_str(&format!("    // {reason}\n"));
//         }
//         // add comment to line
//         s = format!("// {s}");
//     } else if variant.obsolete {
//         match &variant.reason {
//             Some(reason) => writer.push_str(&format!("    #[deprecated = \"{reason}\"]\n")),
//             None => writer.push_str("    #[deprecated]\n"),
//         }
//     }

//     writer.push_str(&format!("    {s}"));

//     // if flags {

//     //     writer.push_str("        ");
//     //
//     //     if variant.removed {
//     //         if let Some(reason) = &variant.reason {
//     //             writer.push_str(&format!("// {reason}\n        "));
//     //         }
//     //         writer.push_str("// ");
//     //     } else if variant.obsolete {
//     //         match &variant.reason {
//     //             Some(reason) => writer.push_str(&format!("#[deprecated = \"{reason}\"]\n        ")),
//     //             None => writer.push_str("#[deprecated]\n        "),
//     //         }
//     //     }
//     //
//     //     writer.push_str(&format!("const {name} = {value};\n"))
//     // } else {
//     //     let name = &variant.name;
//     //     let value = match &variant.value {
//     //         EnumVariantValue::Number(num) => num.to_string(),
//     //         EnumVariantValue::Hex(hex) => format!("0x{hex}"),
//     //         EnumVariantValue::Union(_) => panic!(),
//     //     };
//     //
//     //     writer.push_str("    ");
//     //
//     //     if variant.removed {
//     //         if let Some(reason) = &variant.reason {
//     //             writer.push_str(&format!("// {reason}\n    "));
//     //         }
//     //         writer.push_str("// ");
//     //     } else if variant.obsolete {
//     //         match &variant.reason {
//     //             Some(reason) => writer.push_str(&format!("#[deprecated = \"{reason}\"]\n    ")),
//     //             None => writer.push_str("#[deprecated]\n    "),
//     //         }
//     //     } else if first {
//     //         writer.push_str("#[default]\n    ");
//     //     }
//     //
//     //     writer.push_str(&format!("{name} = {value},"));
//     //
//     //     if let Some(comment) = &variant.comment {
//     //         writer.push_str(&format!(" // {comment}\n"));
//     //     } else {
//     //         writer.push_str("\n");
//     //     }
//     // }
// }

// pub fn generate(graph: &Graph<Node, NodeEdge>, node_idx: NodeIndex, writer: &mut String) {
//     let node = &graph[node_idx];
//     match node {
//         Node::Module(_) => generate_module(graph, node_idx, writer),
//         Node::Enum(_) => generate_enum(graph, node_idx, writer),
//         e => println!("unexpected: {e:?}"),
//     }
// }

use std::collections::HashMap;

use heck::ToShoutySnakeCase;

use crate::parser::{DataType, Document, DocumentEntry, Enum, EnumVariantValue};

pub struct Context {}

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
            DataType::Reference(_) => Err(anyhow::anyhow!("todo")),
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
                DocumentEntry::Class(class) => {}
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

pub fn generate(modules: HashMap<String, Document>) -> anyhow::Result<HashMap<String, String>> {
    let mut ctx = Context {};
    let mut outputs = HashMap::new();

    for (name, module) in modules {
        module.generate(&mut ctx)?;
        outputs.insert(name, module.generate(&mut ctx)?);
    }

    Ok(outputs)
}
