use generator::Generate;
use glob::glob;
use std::{env, fs, path::PathBuf};

mod generator;
mod parser;

fn main() {
    // use cargo out dir for build files
    let cargo_out_dir = env::var("OUT_DIR").expect("OUT_DIR env var not set");
    let mut out_dir = PathBuf::from(cargo_out_dir);
    out_dir.push("generated");

    // remove previous build files
    if out_dir.exists() {
        fs::remove_dir_all(&out_dir).unwrap();
    }

    // create dir
    fs::create_dir(&out_dir).unwrap();

    let mut modules = vec![];
    let paths = glob("assets/SteamKit/Resources/SteamLanguage/*.steamd").unwrap();

    for path in paths {
        let path = path.unwrap();
        let content = fs::read_to_string(&path).unwrap();

        if let Ok((_, document)) = parser::document(&content) {
            if document.entries.is_empty() {
                continue;
            }

            let module = path.file_stem().unwrap().to_str().unwrap().to_owned();
            let mut path = out_dir.clone();
            path.push(format!("{module}.rs"));
            modules.push(module);

            // todo generate content for module
            let content = document.generate_stream().to_string();
            fs::write(path, content).unwrap();
        }
    }

    // create mod.rs file for all sub-modules
    let mut path = out_dir;
    path.push("mod.rs");

    // convert module name array into code
    let modules = modules
        .into_iter()
        .map(|module| format!("pub mod {module};\n"))
        .collect::<String>();
    fs::write(path, modules).unwrap();
}
