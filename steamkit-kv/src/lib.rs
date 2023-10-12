mod error;
mod parser;

use std::collections::HashMap;

pub use error::*;
use indexmap::IndexMap;

pub type Path = Vec<String>;

#[derive(Debug, Clone, PartialEq)]
pub enum KeyValue {
    String(String),
    Map(IndexMap<String, Self>),
}

impl From<String> for KeyValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for KeyValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

fn merge(entries: Vec<parser::Entry>) -> IndexMap<String, KeyValue> {
    let mut map = IndexMap::new();

    for entry in entries {
        let kv = match entry.value {
            parser::Value::String(s) => KeyValue::String(s),
            parser::Value::Map(v) => KeyValue::Map(merge(v)),
        };

        // no need to check collisions if it doesn't exist
        if !map.contains_key(&entry.key) {
            map.insert(entry.key, kv);
            continue;
        }

        // don't need to check if existing entry is a map because we're replacing it with a string
        if let KeyValue::String(_) = kv {
            map.insert(entry.key, kv);
            continue;
        }

        // if we're here, we know that the existing entry is a map and we're trying to insert a map
        match map.get_mut(&entry.key).unwrap() {
            KeyValue::Map(map) => {
                if let KeyValue::Map(value_map) = kv {
                    map.extend(value_map);
                }
            }
            KeyValue::String(_) => {
                map.insert(entry.key, kv);
            }
        }
    }

    map
}

impl KeyValue {
    pub fn parse(input: &str) -> Result<Self> {
        let (input, entry) = parser::key_value(input.trim()).map_err(|_| Error::Parse)?;

        // check if input is empty
        if !input.is_empty() {
            return Err(Error::UnexpectedInput(input.to_string()));
        }

        Ok(Self::Map(merge(vec![entry])))
    }

    pub fn get<I, T>(&self, path: I) -> Option<&Self>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        let mut iter = path.into_iter().peekable();
        let path = iter.next();

        if let Some(path) = path {
            let path = path.as_ref();
            match self {
                Self::String(_) => None,
                Self::Map(map) => match map.get(path) {
                    Some(kv) => {
                        if iter.peek().is_some() {
                            kv.get(iter.collect::<Vec<_>>())
                        } else {
                            Some(kv)
                        }
                    }
                    None => None,
                },
            }
        } else {
            Some(self)
        }
    }

    pub fn get_mut<I, T>(&mut self, path: I) -> Option<&mut Self>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        let mut iter = path.into_iter().peekable();
        let path = iter.next();

        if let Some(path) = path {
            let path = path.as_ref();
            match self {
                Self::String(_) => None,
                Self::Map(map) => match map.get_mut(path) {
                    Some(kv) => {
                        if iter.peek().is_some() {
                            kv.get_mut(iter.collect::<Vec<_>>())
                        } else {
                            Some(kv)
                        }
                    }
                    None => None,
                },
            }
        } else {
            Some(self)
        }
    }
}

#[derive(Debug, Clone)]
pub struct FlatKeyValues {
    map: HashMap<Path, String>,
}

impl FlatKeyValues {
    pub fn parse(input: &str) -> Result<Self> {
        let mut map = HashMap::new();
        let (input, entry) = parser::key_value(input.trim()).map_err(|_| Error::Parse)?;

        // check if input is empty
        if !input.is_empty() {
            return Err(Error::UnexpectedInput(input.to_string()));
        }

        fn process(map: &mut HashMap<Path, String>, entry: parser::Entry, path: &Path) {
            let mut path = path.clone();
            path.push(entry.key.clone());

            match entry.value {
                parser::Value::String(s) => {
                    map.insert(path, s);
                }
                parser::Value::Map(entries) => {
                    for entry in entries {
                        process(map, entry, &path);
                    }
                }
            }
        }

        process(&mut map, entry, &vec![]);

        Ok(Self { map })
    }

    pub fn get<I, T>(&self, path: I) -> Option<&String>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        let path = path
            .into_iter()
            .map(|s| s.as_ref().to_string())
            .collect::<Vec<_>>();
        self.map.get(&path)
    }

    pub fn get_str<I, T>(&self, path: I) -> Option<&str>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        self.get(path).map(|s| s.as_str())
    }

    pub fn get_mut<I, T>(&mut self, path: I) -> Option<&mut String>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        let path = path
            .into_iter()
            .map(|s| s.as_ref().to_string())
            .collect::<Vec<_>>();
        self.map.get_mut(&path)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let input = r#"
            "key1"
            {
                "key2" "value1"
                "key3"
                {
                    "key4" "value2"
                }
            }
        "#;

        let kv = KeyValue::parse(input).unwrap();
        assert_eq!(
            kv.get(&["key1", "key2"]),
            Some(&KeyValue::String("value1".into()))
        );
        assert_eq!(
            kv.get(&["key1", "key3", "key4"]),
            Some(&KeyValue::String("value2".into()))
        );
    }

    #[test]
    fn test_parse_flat() {
        let input = r#"
            "key1"
            {
                "key2" "value1"
                "key3"
                {
                    "key4" "value2"
                }
            }
        "#;

        let kv = FlatKeyValues::parse(input).unwrap();
        assert_eq!(kv.get_str(&["key1", "key2"]), Some("value1"));
        assert_eq!(kv.get_str(&["key1", "key3", "key4"]), Some("value2"));
    }
}
