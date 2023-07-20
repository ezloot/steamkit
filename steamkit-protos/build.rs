use std::fs;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/protos");

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
