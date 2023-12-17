mod generator;
mod parser;

use anyhow::Context;
use heck::ToShoutySnakeCase;
use once_cell::sync::Lazy;
use petgraph::prelude::*;
use regex::Regex;

use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs;
use std::str::FromStr;

fn get_modules() -> anyhow::Result<HashMap<String, String>> {
    let mut m = HashMap::new();
    let dir = fs::read_dir("assets/SteamKit/Resources/SteamLanguage")?;

    for entry in dir {
        let entry = entry?;
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            continue;
        }

        // make sure valid extension
        if entry.path().extension().is_none() || entry.path().extension().unwrap() != "steamd" {
            continue;
        }

        let name = entry
            .path()
            .file_stem()
            .unwrap()
            .to_str()
            .context("invalid file stem")?
            .to_owned();
        m.insert(name, fs::read_to_string(&entry.path())?);
    }

    Ok(m)
}

fn get_data_type_nodes(
    graph: &Graph<Node, NodeEdge>,
    module_node_idx: NodeIndex,
) -> HashMap<String, NodeIndex> {
    let mut m = HashMap::new();

    let imports = graph
        .edges_directed(module_node_idx, Direction::Outgoing)
        .filter(|edge_ref| *edge_ref.weight() == NodeEdge::Import)
        .map(|edge_ref| edge_ref.target())
        .collect::<Vec<_>>();

    // add imported to map
    for import in imports {
        m.extend(get_data_type_nodes(graph, import));
    }

    let structures = graph
        .neighbors_directed(module_node_idx, Direction::Outgoing)
        .filter(|node_idx| graph[*node_idx].is_class() || graph[*node_idx].is_enum())
        .collect::<Vec<_>>();

    for node_idx in structures {
        let name = match &graph[node_idx] {
            Node::Class(Class { name, .. }) => name.clone(),
            Node::Enum(Enum { name, .. }) => name.clone(),
            _ => panic!(),
        };

        m.insert(name, node_idx);
    }

    m
}

fn build_graph() -> anyhow::Result<(Graph<Node, NodeEdge>, NodeIndex)> {
    let modules = get_modules()?;

    let mut graph = Graph::<Node, NodeEdge>::new();
    let root = graph.add_node(Node::Root);

    let mut module_imports = HashMap::new();
    let mut module_map = HashMap::new();

    for (module_name, content) in &modules {
        let (_, document) =
            parser::document(content).map_err(|_| anyhow::anyhow!("failed to parse"))?;
        if document.entries.is_empty() {
            continue;
        }

        let mut imports = vec![];
        let module = graph.add_node(Node::Module(Module {
            name: module_name.to_owned(),
        }));

        graph.add_edge(root, module, NodeEdge::Module);

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
                            name: variant.name.to_shouty_snake_case(),
                            removed: variant.removed,
                            obsolete: variant.obsolete,
                            reason: variant.reason,
                            comment: variant.comment,
                            value: variant.value.into(),
                        }));

                        graph.add_edge(node, variant_node, NodeEdge::EnumVariant);
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

                        graph.add_edge(node, member_node, NodeEdge::ClassMember);
                    }
                }
                parser::DocumentEntry::Import(import) => {
                    let import_name = import
                        .split(".steamd")
                        .next()
                        .expect("invalid import")
                        .to_owned();

                    imports.push(import_name);
                }
            }
        }

        module_imports.insert(module, imports);
        module_map.insert(module_name.to_owned(), module);
    }

    // go through and map any imports to the necessary modules
    for (module, imports) in module_imports {
        for import in imports {
            let imported_module = module_map[&import];
            graph.add_edge(module, imported_module, NodeEdge::Import);
        }
    }

    Ok((graph, root))
}

fn build_type_nodes(graph: &mut Graph<Node, NodeEdge>, root: NodeIndex) {
    let modules = graph
        .neighbors_directed(root, Direction::Outgoing)
        .filter(|node_idx| graph[*node_idx].is_module())
        .collect::<Vec<_>>();

    for module_idx in modules {
        let type_node_map = get_data_type_nodes(&graph, module_idx);
        let classes = graph
            .neighbors_directed(module_idx, Direction::Outgoing)
            .filter(|node_idx| graph[*node_idx].is_class())
            .collect::<Vec<_>>();

        for class_idx in classes {
            let members = graph
                .neighbors_directed(class_idx, Direction::Outgoing)
                .filter(|node_idx| graph[*node_idx].is_class_member())
                .collect::<Vec<_>>();

            for member_idx in members {
                let Node::ClassMember(member) = &graph[member_idx] else {
                    continue;
                };

                let DataType::Reference(type_name) = &member.type_ else {
                    continue;
                };

                let Some(type_node) = type_node_map.get(type_name) else {
                    continue;
                };

                graph.add_edge(member_idx, *type_node, NodeEdge::DataTypeReference);
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let (mut graph, root) = build_graph()?;
    build_type_nodes(&mut graph, root);

    Ok(())
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
    pub value: EnumVariantValue,
}

#[derive(Debug, Clone)]
pub enum EnumVariantValue {
    Number(String),
    Hex(String),
    Union(Vec<String>),
}

impl From<parser::EnumVariantValue> for EnumVariantValue {
    fn from(value: parser::EnumVariantValue) -> Self {
        match value {
            parser::EnumVariantValue::Number(num) => Self::Number(num),
            parser::EnumVariantValue::Hex(hex) => Self::Hex(hex),
            parser::EnumVariantValue::Union(list) => Self::Union(
                list.into_iter()
                    .map(|name| name.to_shouty_snake_case())
                    .collect(),
            ),
        }
    }
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
    Enum(Enum),
    EnumVariant(EnumVariant),
    Class(Class),
    ClassMember(ClassMember),
}

impl Node {
    pub fn is_module(&self) -> bool {
        match &self {
            Self::Module(_) => true,
            _ => false,
        }
    }

    pub fn is_class(&self) -> bool {
        match &self {
            Self::Class(_) => true,
            _ => false,
        }
    }

    pub fn is_class_member(&self) -> bool {
        match &self {
            Self::ClassMember(_) => true,
            _ => false,
        }
    }

    pub fn is_enum(&self) -> bool {
        match &self {
            Self::Enum(_) => true,
            _ => false,
        }
    }
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
    FixedLengthArray { type_: Box<Self>, length: usize },
}

impl From<&str> for DataType {
    fn from(s: &str) -> Self {
        static RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(?<type>[^<>]+)<(?<length>\d+)>").unwrap());

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
