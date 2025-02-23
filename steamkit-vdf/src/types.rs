use crate::{Error, Result};

cfg_if::cfg_if! {
    if #[cfg(feature = "precision")] {
        pub type Int = i64;
        pub type Float = f64;

        const FLOAT_TYPE: &str = "f64";
        const INT_TYPE: &str = "i64";
    } else {
        pub type Int = i32;
        pub type Float = f32;

        const FLOAT_TYPE: &str = "f32";
        const INT_TYPE: &str = "i32";
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Float(Float),
    Int(Int),
    String(String),
    Bool(bool),
    Group(Group),
}

impl From<Group> for Value {
    fn from(group: Group) -> Self {
        Value::Group(group)
    }
}

impl From<String> for Value {
    fn from(string: String) -> Self {
        Value::String(string)
    }
}

impl From<&str> for Value {
    fn from(string: &str) -> Self {
        Value::String(string.to_string())
    }
}

macro_rules! impl_value_as_type {
    ($variant:ident, $typ:ty, $typ_name:expr, false) => {
        impl<'a> TryFrom<&'a Value> for &'a $typ {
            type Error = Error;

            fn try_from(value: &'a Value) -> Result<Self> {
                match value {
                    Value::$variant(v) => Ok(v),
                    _ => Err(Error::InvalidConversion {
                        variant: stringify!($variant),
                        output_type: $typ_name,
                    }),
                }
            }
        }

        impl<'a> TryFrom<&'a mut Value> for &'a mut $typ {
            type Error = Error;

            fn try_from(value: &'a mut Value) -> Result<Self> {
                match value {
                    Value::$variant(v) => Ok(v),
                    _ => Err(Error::InvalidConversion {
                        variant: stringify!($variant),
                        output_type: $typ_name,
                    }),
                }
            }
        }
    };

    ($variant:ident, $typ:ty, $typ_name:expr, true) => {
        impl_value_as_type!($variant, $typ, $typ_name, false);

        impl<'a> TryFrom<&'a mut Value> for $typ {
            type Error = Error;

            fn try_from(value: &'a mut Value) -> Result<Self> {
                match value {
                    Value::$variant(v) => Ok(*v),
                    _ => Err(Error::InvalidConversion {
                        variant: stringify!($variant),
                        output_type: $typ_name,
                    }),
                }
            }
        }
    };
}

impl_value_as_type!(Float, Float, FLOAT_TYPE, true);
impl_value_as_type!(Int, Int, INT_TYPE, true);
impl_value_as_type!(Bool, bool, "bool", true);
impl_value_as_type!(String, String, "String", false);
impl_value_as_type!(String, str, "str", false);
impl_value_as_type!(Group, Group, "Group", false);

#[derive(Debug, Clone)]
pub struct Entry {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, Clone)]
pub struct Group {
    pub entries: Vec<Entry>,
}

pub trait Accessor {
    fn get(&self, key: &[&str]) -> Option<&Value>;
    fn get_mut(&mut self, key: &[&str]) -> Option<&mut Value>;
    fn get_all(&self, key: &[&str]) -> Vec<&Value>;
    fn get_all_mut(&mut self, key: &[&str]) -> Vec<&mut Value>;
}

impl Accessor for Group {
    fn get(&self, key: &[&str]) -> Option<&Value> {
        if key.is_empty() {
            return None;
        }

        let entry = self.entries.iter().find(|e| e.name == key[0])?;
        let remaining_keys = &key[1..];

        if remaining_keys.is_empty() {
            Some(&entry.value)
        } else {
            match &entry.value {
                Value::Group(group) => group.get(remaining_keys),
                _ => None,
            }
        }
    }

    fn get_mut(&mut self, key: &[&str]) -> Option<&mut Value> {
        if key.is_empty() {
            return None;
        }

        let entry = self.entries.iter_mut().find(|e| e.name == key[0])?;
        let remaining_keys = &key[1..];

        if remaining_keys.is_empty() {
            Some(&mut entry.value)
        } else {
            match &mut entry.value {
                Value::Group(group) => group.get_mut(remaining_keys),
                _ => None,
            }
        }
    }

    fn get_all(&self, key: &[&str]) -> Vec<&Value> {
        let mut v = vec![];

        let entries = self.entries.iter().filter(|e| e.name == key[0]);
        let remaining_keys = &key[1..];

        for entry in entries {
            if remaining_keys.is_empty() {
                v.push(&entry.value);
            } else {
                match &entry.value {
                    Value::Group(group) => v.extend(group.get_all(remaining_keys)),
                    _ => {}
                }
            }
        }

        v
    }

    fn get_all_mut(&mut self, key: &[&str]) -> Vec<&mut Value> {
        let mut v = vec![];

        let entries = self.entries.iter_mut().filter(|e| e.name == key[0]);
        let remaining_keys = &key[1..];

        for entry in entries {
            if remaining_keys.is_empty() {
                v.push(&mut entry.value);
            } else {
                match &mut entry.value {
                    Value::Group(group) => v.extend(group.get_all_mut(remaining_keys)),
                    _ => {}
                }
            }
        }

        v
    }
}

#[test]
fn test() {
    let mut group = Group {
        entries: vec![Entry {
            name: "key".to_string(),
            value: Group {
                entries: vec![Entry {
                    name: "key".into(),
                    value: "hello".into(),
                }],
            }
            .into(),
        }],
    };

    let value: &str = group.get(&["key", "key"]).unwrap().try_into().unwrap();
    println!("{value:?}");
}
