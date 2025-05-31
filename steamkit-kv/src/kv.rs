use lasso::{Rodeo, Spur};
use slotmap::{SlotMap, new_key_type};

new_key_type! { pub struct NodeKey; }

#[cfg(feature = "precision")]
pub type Integer = i64;
#[cfg(feature = "precision")]
pub type Float = f64;

#[cfg(not(feature = "precision"))]
pub type Integer = i32;
#[cfg(not(feature = "precision"))]
pub type Float = f32;

#[cfg(feature = "intern-keys")]
type Symbol = Spur;
#[cfg(not(feature = "intern-keys"))]
type Symbol = Box<str>;

#[cfg(feature = "mutable-string")]
type StringValue = String;
#[cfg(not(feature = "mutable-string"))]
type StringValue = Box<str>;

enum Value {
    String(StringValue),
    Integer(Integer),
    Float(Float),
    Boolean(bool),
    Color { r: u8, g: u8, b: u8, a: u8 },
    Group(Vec<NodeKey>),
}

struct Node {
    key: Symbol,
    value: Value,
    parent: Option<NodeKey>,
}

pub struct KeyValues {
    nodes: SlotMap<NodeKey, Node>,
    symbols: Rodeo,
    #[cfg(feature = "intern-keys")]
    root_node: Option<NodeKey>,
}
