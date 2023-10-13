use generator::Generate;
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

    let modules = vec!["emsg", "enums", "eresult", "steammsg"];

    for module in &modules {
        let path = format!("assets/SteamKit/Resources/SteamLanguage/{module}.steamd");
        let content = fs::read_to_string(&path).unwrap();

        if let Ok((_, document)) = parser::document(&content) {
            if document.entries.is_empty() {
                continue;
            }

            // TODO: go through the document and build a list of imports (support nesting but make sure to not loop indefinitely)
            // TODO: when generating type information, if referencing a foreign type (from an import) and add to a list of "used" import-types
            // TODO: using the import-types list, in the header of the rust codegen file, add the appropriate use statements

            let mut path = out_dir.clone();
            path.push(format!("{module}.rs"));

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
