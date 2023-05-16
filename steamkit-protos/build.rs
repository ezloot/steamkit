use std::{env, fs, path::PathBuf};

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
        Regex::new(r"enum\s+(?<name>[a-zA-Z]+)[\s\n][^{]*\{(?<inner>[^}]+)\}").unwrap();
    let variant_regex =
        Regex::new(r"(?<key>[a-zA-Z0-9_]+)\s*=\s*\s*(?<value>[a-zA-Z0-9_]+)\s*;").unwrap();
    let mut data = String::new();

    for path in paths {
        let res = fs::read_to_string(&path).expect("failed to read resource");
        for capture in enum_regex.captures_iter(&res) {
            let name = &capture["name"];
            let inner = &capture["inner"];

            let variants = variant_regex.captures_iter(inner).collect::<Vec<_>>();
            if !variants.is_empty() {
                data.push_str(&format!("enum {name} {{\n"));

                for capture in variants {
                    let key = &capture["key"];
                    let value = &capture["value"];

                    data.push_str(&format!("    {key} = {value},\n"));
                }

                data.push_str("}\n");
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
    // protobufs();
}
