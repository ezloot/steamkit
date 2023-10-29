use generator::Generate;
use std::{env, fs, path::PathBuf};
use petgraph::Graph;
use petgraph_graphml::GraphMl;

mod generator;
mod parser;

fn main() {
    // use cargo out dir for build files
    // let cargo_out_dir = env::var("OUT_DIR").expect("OUT_DIR env var not set");
    let mut out_dir = PathBuf::from("src");
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

            // TODO: document classes/enums should have a map of any changed names (from the steamd file) to the rust name
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
}