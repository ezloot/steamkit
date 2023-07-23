use std::{env, fs, path::PathBuf};

use glob::glob;

fn generate_protos(folder: &str, out_dir: &PathBuf) {
    let path = format!("assets/Protobufs/{folder}");
    let paths = glob(&format!("{path}/**/*.proto"))
        .unwrap()
        .map(|path| path.unwrap())
        .collect::<Vec<_>>();

    // let cargo_out_dir = env::var("OUT_DIR").expect("OUT_DIR env var not set");
    // let mut out_dir = PathBuf::from(cargo_out_dir);
    // out_dir.push("protos");
    // out_dir.push(folder);

    let mut out_dir = out_dir.clone();
    out_dir.push(folder);

    if out_dir.exists() {
        fs::remove_dir_all(&out_dir).unwrap();
    }

    fs::create_dir(&out_dir).unwrap();

    protobuf_codegen::Codegen::new()
        .protoc()
        .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
        .includes([
            &path,
            "assets/Protobufs/steam",
            "assets/Protobufs/google/protobuf",
        ])
        .inputs(paths)
        .out_dir(&out_dir)
        .run_from_script();
}

fn generate_mod(folders: &[&str], out_dir: &PathBuf) {
    let mut path = out_dir.clone();
    path.push("mod.rs");

    let src = folders
        .iter()
        .map(|folder| format!("pub mod {folder};\n"))
        .collect::<String>();

    fs::write(&path, src).unwrap();
}

fn generate(folders: &[&str]) {
    let cargo_out_dir = env::var("OUT_DIR").expect("OUT_DIR env var not set");
    let mut out_dir = PathBuf::from(cargo_out_dir);
    out_dir.push("protos");

    if out_dir.exists() {
        fs::remove_dir_all(&out_dir).unwrap();
    }

    fs::create_dir(&out_dir).unwrap();

    for folder in folders {
        let out_dir = out_dir.clone();
        let folder = (*folder).to_owned();
        std::thread::spawn(move || generate_protos(&folder, &out_dir));
    }

    let handles = folders
        .iter()
        .map(|folder| {
            let out_dir = out_dir.clone();
            let folder = (*folder).to_owned();
            std::thread::spawn(move || generate_protos(&folder, &out_dir))
        })
        .collect::<Vec<_>>();

    generate_mod(folders, &out_dir);

    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets/Protobufs");

    generate(&["steam"]);
}
