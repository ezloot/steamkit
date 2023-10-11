mod error;
mod parser;

use std::collections::HashMap;

pub use error::*;
use parser::*;

pub type Path = Vec<String>;
pub struct KeyValue(HashMap<Path, String>);

impl KeyValue {
    pub fn parse(input: &str) -> Result<Self> {
        let mut map = HashMap::new();
        let (_input, entry) = key_value(input).unwrap();

        fn process(map: &mut HashMap<Path, String>, entry: Entry, path: &Path) {
            let mut path = path.clone();
            path.push(entry.key.clone());

            match entry.value {
                Value::String(s) => {
                    map.insert(path, s);
                }
                Value::Map(entries) => {
                    for entry in entries {
                        process(map, entry, &path);
                    }
                }
            }
        }

        process(&mut map, entry, &vec![]);

        Ok(Self(map))
    }

    pub fn get(&self, path: &Path) -> Option<&String> {
        self.0.get(path)
    }
}
