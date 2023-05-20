use std::{env, fs, path::PathBuf};

use indexmap::IndexMap;
use parse_int::parse;
use regex::Regex;

fn protobufs() {
    let paths = fs::read_dir("src/protos")
        .expect("failed to get protobuf files")
        .filter_map(|path| {
            let path = path.unwrap().path();
            if let Some(ext) = path.extension() {
                if ext == "proto" {
                    return Some(path);
                }
            }
            None
        });

    protobuf_codegen::Codegen::new()
        .protoc()
        .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
        .includes(["src/protos"])
        .inputs(paths)
        .cargo_out_dir("protos")
        .run_from_script();
}

fn resources() {
    // get resources from source files
    let paths = fs::read_dir("src/resources")
        .expect("failed to get resources")
        .filter_map(|path| {
            let path = path.unwrap().path();
            if let Some(ext) = path.extension() {
                if ext == "steamd" {
                    return Some(path);
                }
            }
            None
        });

    let enum_regex =
        Regex::new(r"(?i)enum\s+(?P<name>[a-z]+)[\s\n][^{]*\{(?P<inner>[^}]+)\}").unwrap();
    let variant_regex =
        Regex::new(r"(?i)(?P<key>[a-z0-9_]+)\s*=\s*\s*(?P<value>[a-z0-9_|\s]+)\s*;(?P<comment>.*)")
            .unwrap();
    let mut data = String::new();

    for path in paths {
        let res = fs::read_to_string(&path).expect("failed to read resource");
        for capture in enum_regex.captures_iter(&res) {
            let name = &capture["name"];
            let inner = &capture["inner"];
            let mut map = IndexMap::<String, (u32, Option<String>)>::new();

            for capture in variant_regex.captures_iter(inner) {
                let key = &capture["key"];
                let value = &capture["value"].trim();

                let comment = &capture["comment"].trim();
                let comment = if !comment.is_empty() {
                    Some(comment.to_string())
                } else {
                    None
                };

                let num = match parse::<u32>(value.trim()) {
                    Ok(num) => num,
                    Err(_) => {
                        let mut num = 0;
                        for x in value.split('|') {
                            num |= map.get(x.trim()).unwrap().0;
                        }
                        num
                    }
                };

                map.insert(key.to_owned(), (num, comment));
            }

            if !map.is_empty() {
                // create enum
                data.push_str(&format!(
                    "#[allow(non_camel_case_types)]\n#[derive(Debug, Clone, PartialEq, Eq, Hash)]\npub enum {name} {{\n"
                ));
                for (key, (_, comment)) in map.iter() {
                    data.push_str(&format!("    {key},"));

                    if let Some(comment) = comment {
                        data.push_str(&format!(" // {}", comment));
                    }

                    data.push('\n');
                }
                data.push_str("}\n\n");

                // from enum impl
                data.push_str(&format!("impl From<{name}> for u32 {{\n    fn from(value: {name}) -> Self {{\n        match value {{\n"));
                for (key, (value, _)) in map.iter() {
                    data.push_str(&format!("            {name}::{key} => {value},\n"));
                }
                data.push_str("        }\n    }\n}\n\n");

                // from u32 impl
                let mut used = vec![];
                data.push_str(&format!("impl From<u32> for {name} {{\n    fn from(value: u32) -> Self {{\n        match value {{\n"));
                for (key, (value, _)) in map.iter().rev() {
                    if !used.contains(&value) {
                        data.push_str(&format!("            {value} => {name}::{key},\n"));
                        used.push(value);
                    }
                }
                data.push_str("            _ => panic!(),\n        }\n    }\n}\n\n");
            }
        }
    }

    // get resources output path
    let cargo_out_dir = env::var("OUT_DIR").expect("OUT_DIR env var not set");
    let mut res_path = PathBuf::from(cargo_out_dir);
    res_path.push("resources.rs");

    // write generated file
    fs::write(res_path, data).unwrap();
}

fn main() {
    resources();
    protobufs();
}
