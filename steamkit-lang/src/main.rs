use std::fmt::{Display, Formatter};
use std::fs;
use petgraph::dot::{Dot, Config};
use petgraph::Graph;
use std::fs::File;
use dot::render;

fn main() {
    let mut graph = Graph::<Node, NodeEdge>::new();

    let module = graph.add_node(Node::Module {
        name: "Steam".to_owned(),
    });

    let enum_ = graph.add_node(Node::Enum {
        name: "EAccountType".to_owned(),
        flags: false,
    });

    let invalid = graph.add_node(Node::EnumVariant {
        name: "Invalid".to_owned(),
        removed: false,
        obsolete: false,
        reason: None,
        comment: None,
    });


    let ok = graph.add_node(Node::EnumVariant {
        name: "Ok".to_owned(),
        removed: false,
        obsolete: false,
        reason: None,
        comment: None,
    });

    graph.add_edge(module, enum_, NodeEdge::Enum);
    graph.add_edge(enum_, invalid, NodeEdge::EnumVariant("Invalid".to_owned()));
    graph.add_edge(enum_, ok, NodeEdge::EnumVariant("Ok".to_owned()));

    println!("{:?}", graph.edges(enum_).collect::<Vec<_>>());

    let dot_string = format!("{:?}", Dot::with_config(&graph, &[]));
    fs::write("test.dot", dot_string).unwrap();
}

#[derive(Debug, Clone)]
pub enum NodeEdge {
    EnumVariant(String),
    ClassMember(String),
    Import(String),
    Class,
    Enum,
}

impl Display for NodeEdge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    Module {
        name: String,
    },
    Import(String),
    Enum {
        name: String,
        flags: bool,
    },
    EnumVariant {
        name: String,
        removed: bool,
        obsolete: bool,
        reason: Option<String>,
        comment: Option<String>,
    },
    EnumVariantValue {},
    Class {
        name: String,
        generic: Option<String>,
        removed: bool,
    },
    ClassMember {
        name: String,
        // type_: String,
        modifier: Option<String>,
        constant: bool,
    },
    ClassMemberValue {},
    Typing {
        // TODO: Figure out how to represent typings?
    },
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}