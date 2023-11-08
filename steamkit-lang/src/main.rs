mod generator;
mod parser;

use once_cell::sync::Lazy;
use petgraph::prelude::*;
use regex::Regex;
use std::fmt::{Display, Formatter};
use std::fs;
use std::str::FromStr;

fn main() {
    let modules = vec!["emsg", "enums", "eresult", "steammsg"];
    let mut graph = Graph::<Node, NodeEdge>::new();

    for module_name in modules {
        let path = format!("assets/SteamKit/Resources/SteamLanguage/{module_name}.steamd");
        let content = fs::read_to_string(&path).unwrap();

        if let Ok((_, document)) = parser::document(&content) {
            if document.entries.is_empty() {
                continue;
            }

            let module = graph.add_node(Node::Module(Module {
                name: module_name.to_owned(),
            }));

            // TODO: first step is to build a graph of the document without any imports
            for entry in document.entries {
                match entry {
                    parser::DocumentEntry::Enum(enum_) => {
                        let node = graph.add_node(Node::Enum(Enum {
                            name: enum_.name.to_owned(),
                            flags: enum_.flags,
                            type_: DataType::from(enum_.generic.unwrap_or("int".into()).as_ref()),
                        }));

                        graph.add_edge(module, node, NodeEdge::Enum);

                        for variant in enum_.variants {
                            let variant_node = graph.add_node(Node::EnumVariant(EnumVariant {
                                name: variant.name.to_owned(),
                                removed: variant.removed,
                                obsolete: variant.obsolete,
                                reason: variant.reason,
                                comment: variant.comment,
                            }));

                            graph.add_edge(
                                node,
                                variant_node,
                                NodeEdge::EnumVariant(variant.name.to_owned()),
                            );
                        }
                    }
                    parser::DocumentEntry::Class(class) => {
                        let node = graph.add_node(Node::Class(Class {
                            name: class.name.to_owned(),
                            generic: class.generic,
                            removed: class.removed,
                        }));

                        graph.add_edge(module, node, NodeEdge::Class);

                        for member in class.members {
                            let member_node = graph.add_node(Node::ClassMember(ClassMember {
                                name: member.name.to_owned(),
                                modifier: member.modifier,
                                constant: member.constant,
                                type_: DataType::from(member.type_.as_str()),
                            }));

                            graph.add_edge(
                                node,
                                member_node,
                                NodeEdge::ClassMember(member.name.to_owned()),
                            );
                        }
                    }
                    _ => {}
                }
            }

            // TODO: second step is to go through the graph and add imports to module for any external types (that exist in the list of imports)

            let mut writer = String::new();
            generator::generate(&graph, module, &mut writer);
            fs::write(format!("generated/{module_name}.rs.txt"), writer).unwrap();
        }
    }
}

#[derive(Debug, Clone)]
pub enum NodeEdge {
    EnumVariant(String),
    ClassMember(String),
    Import(String),
    Class,
    Enum,
    Generic,
    Module,
}

impl Display for NodeEdge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub flags: bool,
    pub type_: DataType,
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub removed: bool,
    pub obsolete: bool,
    pub reason: Option<String>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    pub generic: Option<String>,
    pub removed: bool,
}

#[derive(Debug, Clone)]
pub struct ClassMember {
    pub name: String,
    pub modifier: Option<String>,
    pub constant: bool,
    pub type_: DataType,
}

#[derive(Debug, Clone)]
pub enum Node {
    Module(Module),
    Import(NodeIndex),
    Enum(Enum),
    EnumVariant(EnumVariant),
    // EnumVariantValue {},
    Class(Class),
    ClassMember(ClassMember),
    // ClassMemberValue {},
    // Typing {
    //     // TODO: Figure out how to represent typings?
    // },
    // Value {
    //     // TODO: Figure out how to represent values?
    //     // Maybe use a boxed trait?
    // },
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Reference(String),
    FixedLengthArray {
        type_: Box<Self>,
        length: usize,
    },
}

impl From<&str> for DataType {
    fn from(s: &str) -> Self {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<type>[^<>]+)<(?<length>\d+)>").unwrap());

        if let Some(captures) = RE.captures(s) {
            let type_ = captures.name("type").unwrap().as_str();
            let length = captures.name("length").unwrap().as_str();

            return Self::FixedLengthArray {
                type_: Box::new(Self::from(type_)),
                length: usize::from_str(length).unwrap(),
            };
        }

        match s {
            "byte" => Self::U8,
            "ushort" => Self::U16,
            "uint" => Self::U32,
            "ulong" => Self::U64,
            "char" => Self::I8,
            "short" => Self::I16,
            "int" => Self::I32,
            "long" => Self::I64,
            "float" => Self::F32,
            "double" => Self::F64,
            _ => Self::Reference(s.to_owned()),
        }
    }
}
