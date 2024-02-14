mod error;
mod parser;

use std::collections::HashMap;

pub use error::*;
use indexmap::IndexMap;

pub type Path = Vec<String>;

#[derive(Debug, Clone, PartialEq, Eq)]
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
        match map.entry(entry.key) {
            indexmap::map::Entry::Occupied(mut occupied) => match entry.value {
                parser::Value::String(s) => {
                    *occupied.get_mut() = KeyValue::String(s);
                }
                parser::Value::Map(new_entries) => {
                    if let KeyValue::Map(existing_map) = occupied.get_mut() {
                        existing_map.extend(merge(new_entries));
                    } else {
                        *occupied.get_mut() = KeyValue::Map(merge(new_entries));
                    }
                }
            },
            indexmap::map::Entry::Vacant(vacant) => {
                let kv = match entry.value {
                    parser::Value::String(s) => KeyValue::String(s),
                    parser::Value::Map(v) => KeyValue::Map(merge(v)),
                };
                vacant.insert(kv);
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

    pub fn iter(&self) -> KeyValueIter {
        match self {
            Self::String(_) => KeyValueIter::Empty,
            Self::Map(map) => KeyValueIter::Map(map.iter()),
        }
    }

    pub fn as_vec(&self) -> Vec<&KeyValue> {
        let mut v = self.iter().collect::<Vec<_>>();

        v.sort_by(|(a, _), (b, _)| {
            let a_num = a.parse::<u32>().unwrap_or(u32::MAX);
            let b_num = b.parse::<u32>().unwrap_or(u32::MAX);
            a_num.cmp(&b_num)
        });

        v.into_iter().map(|(_, kv)| kv).collect()
    }

    pub fn as_str_vec(&self) -> Vec<&str> {
        self.as_vec()
            .iter()
            .filter_map(|kv| match kv {
                KeyValue::String(s) => Some(s.as_str()),
                _ => None,
            })
            .collect()
    }
}

pub enum KeyValueIter<'a> {
    Empty,
    Map(indexmap::map::Iter<'a, String, KeyValue>),
}

impl<'a> Iterator for KeyValueIter<'a> {
    type Item = (&'a String, &'a KeyValue);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Empty => None,
            Self::Map(iter) => iter.next(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlatKeyValues {
    map: HashMap<Path, String>,
}

impl From<KeyValue> for FlatKeyValues {
    fn from(kv: KeyValue) -> Self {
        // calculate necessary map capacity
        let mut capacity = 0;

        // since this is just a guess, we don't care above duplicate keys
        fn calculate_capacity(kv: &KeyValue, capacity: &mut usize) {
            match kv {
                KeyValue::String(_) => *capacity += 1,
                KeyValue::Map(entries) => {
                    for kv in entries.values() {
                        calculate_capacity(kv, capacity);
                    }
                }
            }
        }

        calculate_capacity(&kv, &mut capacity);

        let mut map = HashMap::with_capacity(capacity);

        fn process(map: &mut HashMap<Path, String>, kv: KeyValue, path: &mut Path) {
            match kv {
                KeyValue::String(s) => {
                    map.insert(path.clone(), s);
                }
                KeyValue::Map(entries) => {
                    for (key, kv) in entries {
                        path.push(key);
                        process(map, kv, path);
                        path.pop();
                    }
                }
            }
        }

        // this is a guess at the capacity, not sure what needs more than 10 levels of nesting
        process(&mut map, kv, &mut Vec::with_capacity(10));

        Self { map }
    }
}

impl FlatKeyValues {
    pub fn parse(input: &str) -> Result<Self> {
        let (input, entry) = parser::key_value(input.trim()).map_err(|_| Error::Parse)?;
        // check if input is empty
        if !input.is_empty() {
            return Err(Error::UnexpectedInput(input.to_string()));
        }

        // calculate necessary map capacity
        let mut capacity = 0;

        // since this is just a guess, we don't care above duplicate keys
        fn calculate_capacity(entry: &parser::Entry, capacity: &mut usize) {
            match &entry.value {
                parser::Value::String(_) => *capacity += 1,
                parser::Value::Map(entries) => {
                    for entry in entries {
                        calculate_capacity(entry, capacity);
                    }
                }
            }
        }

        calculate_capacity(&entry, &mut capacity);

        let mut map = HashMap::with_capacity(capacity);

        fn process(map: &mut HashMap<Path, String>, entry: parser::Entry, path: &mut Path) {
            path.push(entry.key);

            match entry.value {
                parser::Value::String(s) => {
                    // BUG: if a lower-level key is inserted after a higher-level key, the higher level key will stay in the map
                    map.insert(path.clone(), s);
                }
                parser::Value::Map(entries) => {
                    map.remove(path);

                    for entry in entries {
                        process(map, entry, path);
                    }
                }
            }

            // remove the last key since it's done processing
            path.pop();
        }

        // this is a guess at the capacity, not sure what needs more than 10 levels of nesting
        process(&mut map, entry, &mut Vec::with_capacity(10));

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

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Path, &String)> {
        self.map.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LARGE_DATA: &str = include_str!("../assets/items_game.txt");

    #[test]
    fn nested() {
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
            kv.get(["key1", "key2"]),
            Some(&KeyValue::String("value1".into()))
        );
        assert_eq!(
            kv.get(["key1", "key3", "key4"]),
            Some(&KeyValue::String("value2".into()))
        );
    }

    #[test]
    fn nested_merge_maps() {
        let input = r#"
            "root"
            {
                "key1"
                {
                    "key2"
                    {
                        "key3" "value1"
                    }
                    "key2"
                    {
                        "key4" "value2"
                    }
                }
                "key2" "value3"
            }
        "#;

        let kv = KeyValue::parse(input).unwrap();
        assert_eq!(
            kv.get(["root", "key1", "key2", "key3"]),
            Some(&KeyValue::String("value1".into()))
        );
        assert_eq!(
            kv.get(["root", "key1", "key2", "key4"]),
            Some(&KeyValue::String("value2".into()))
        );
        assert_eq!(
            kv.get(["root", "key2"]),
            Some(&KeyValue::String("value3".into()))
        );
    }

    #[test]
    fn nested_duplicate_strings() {
        let input = r#"
            "root"
            {
                "key1" "value1"
                "key1" "value2"
                "key2" "value3"
            }
        "#;

        let kv = KeyValue::parse(input).unwrap();
        assert_eq!(
            kv.get(["root", "key1"]),
            Some(&KeyValue::String("value2".into()))
        );
        assert_eq!(
            kv.get(["root", "key2"]),
            Some(&KeyValue::String("value3".into()))
        );
    }

    #[test]
    fn nested_large() {
        let kv = KeyValue::parse(LARGE_DATA).unwrap();
        assert_eq!(
            kv.get(["items_game", "game_info", "first_valid_item_slot"]),
            Some(&KeyValue::String("0".into()))
        );
        assert_eq!(
            kv.get(["items_game", "items", "507", "name"]),
            Some(&KeyValue::String("weapon_knife_karambit".into()))
        );
    }

    #[test]
    fn flat() {
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
        assert_eq!(kv.get_str(["key1", "key2"]), Some("value1"));
        assert_eq!(kv.get_str(["key1", "key3", "key4"]), Some("value2"));
    }

    #[test]
    fn flat_merge_maps() {
        let input = r#"
            "root"
            {
                "key1"
                {
                    "key2"
                    {
                        "key3" "value1"
                    }
                    "key2"
                    {
                        "key4" "value2"
                    }
                }
                "key2" "value3"
            }
        "#;

        let kv = FlatKeyValues::parse(input).unwrap();
        assert_eq!(kv.get_str(["root", "key1", "key2", "key3"]), Some("value1"));
        assert_eq!(kv.get_str(["root", "key1", "key2", "key4"]), Some("value2"));
        assert_eq!(kv.get_str(["root", "key2"]), Some("value3"));
    }

    #[test]
    fn flat_duplicate_strings() {
        let input = r#"
            "root"
            {
                "key1" "value1"
                "key1" "value2"
                "key2" "value3"
            }
        "#;

        let kv = FlatKeyValues::parse(input).unwrap();
        assert_eq!(kv.get_str(["root", "key1"]), Some("value2"));
        assert_eq!(kv.get_str(["root", "key2"]), Some("value3"));
    }

    #[test]
    fn flat_large() {
        let kv = FlatKeyValues::parse(LARGE_DATA).unwrap();
        assert_eq!(
            kv.get_str(["items_game", "game_info", "first_valid_item_slot"]),
            Some("0")
        );
        assert_eq!(
            kv.get_str(["items_game", "items", "507", "name"]),
            Some("weapon_knife_karambit")
        );
    }

    #[test]
    fn flat_from_nested_large() {
        let kv = FlatKeyValues::from(KeyValue::parse(LARGE_DATA).unwrap());
        assert_eq!(
            kv.get_str(["items_game", "game_info", "first_valid_item_slot"]),
            Some("0")
        );
        assert_eq!(
            kv.get_str(["items_game", "items", "507", "name"]),
            Some("weapon_knife_karambit")
        );
    }

    #[test]
    fn empty_quotes() {
        assert!(KeyValue::parse(r#""key" """#).is_ok());
    }

    #[test]
    fn escaped() {
        let kv = KeyValue::parse(r#""key" "\n hello world""#);
        assert!(kv.is_ok());
        assert_eq!(
            kv.unwrap().get(["key"]),
            Some(&KeyValue::String("\n hello world".into()))
        );
    }
}
