mod generator;
mod parser;

use std::collections::HashMap;
use once_cell::sync::Lazy;
use petgraph::prelude::*;
use regex::Regex;
use std::fmt::{Display, Formatter};
use std::fs;
use std::str::FromStr;
use petgraph::data::DataMap;
use petgraph::visit::{Data, IntoEdges};

fn main() {
    let modules = vec!["emsg", "enums", "eresult", "steammsg"];
    let mut graph = Graph::<Node, NodeEdge>::new();
    let root = graph.add_node(Node::Root);

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

            graph.add_edge(root, module, NodeEdge::Module);

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
                                NodeEdge::EnumVariant,
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
                                NodeEdge::ClassMember,
                            );
                        }
                    }
                    _ => {}
                }
            }

            // let mut ref_map = HashMap::new();
            //
            // let classes_and_enums = graph.edge_references()
            //     .filter(|edge_ref| *edge_ref.weight() == NodeEdge::Class || *edge_ref.weight() == NodeEdge::Enum)
            //     .map(|edge_ref| edge_ref.target());
            //
            // for node_idx in classes_and_enums {
            //     match &graph[node_idx] {
            //         Node::Class(class) => { ref_map.insert(class.name.to_owned(), node_idx); }
            //         Node::Enum(enum_) => { ref_map.insert(enum_.name.to_owned(), node_idx); }
            //         _ => {}
            //     }
            // }

            // let class_members = graph.edge_references()
            //     .filter(|edge_ref| *edge_ref.weight() == NodeEdge::ClassMember)
            //     .map(|edge_ref| edge_ref.target())
            //     .collect::<Vec<_>>();

            // map of all modules and all classes/enums (including imports)
            // let module_data_structures = HashMap::<NodeIndex, HashMap<String, NodeIndex>>::new();

            // for node_idx in class_members {
            //     let Node::ClassMember(member) = &graph[node_idx] else { continue; };
            //     let DataType::Reference(ref_str) = &member.type_ else { continue; };


                //
                // if let Some(target_node_idx) = ref_map.get(ref_str) {
                //     graph.add_edge(node_idx, node_idx, NodeEdge::DataTypeReference);
                // } else {
                //     panic!("unknown reference data type: {ref_str}");
                // }
            // }

            // let mut writer = String::new();
            // generator::generate/(&graph, module, &mut writer);
            // fs::write(format!("generated/{module_name}.rs.txt"), writer).unwrap();
        }
    }

    generate_context_map(&graph);
}

fn generate_context_map(graph: &Graph<Node, NodeEdge>) {
    let module_nodes = graph.edge_references()
        .filter(|edge_ref| *edge_ref.weight() == NodeEdge::Module)
        .map(|edge_ref| edge_ref.target())
        .collect::<Vec<_>>();

    for module_node in module_nodes {

    }

    println!("{:?}", module_nodes);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeEdge {
    EnumVariant,
    ClassMember,
    Import,
    Class,
    Enum,
    Generic,
    Module,
    DataTypeReference,
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
    Root,
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


/*

when generating the typing imports, run after first initial pass so we know the entire hierarchy

make a map of all local types

type TypeName = String;
enum TypeImport {
    Local,
    External(NodeRef), // <-- this will point to the actual typing node (since we don't know the new names yet)
    // it's the job of the generator to have a name map that will map the old names to the new names
    //
}

Map<Name, TypeImport>

rules:
enum types can not be reference types

pseudo code when generating code for struct member

context:

mod 1:
Mod1Test1 {
    local_test2: Mod1Test2,
    external_test2: Mod2Test2,
},
Mod1Test2,

mod 2:
Mod2Test1,
Mod2Test2,

we're generating code for Mod1Test1 -> local_test2

Mod1Test2 is local so it is a local reference/import

Mod2Test2 is  an external reference so it needs an external import

we can figure this out by comparing the node ref to the current module node ref to see if it's the same or not
if it's not then we will include the import

we could have a flat map and a counter for how many times it's referenced ?

we need a flat map of all module names & structs/enums/fields so we can map when generating code
we could do this after mapping the data type nodes to the graph

graph:
add modules
-> add enums & classes
  -> add variants and members
-> add data type node to class members

RefLocation {
    name: String,
    module: NodeRef,
    type_: NodeRef,
}

this would be how we keep track of the new names for codegen after graphing

Map<RefLocation, RefLocation>

 */