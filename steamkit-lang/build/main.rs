mod generator;
mod parser;

use anyhow::Context;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

fn get_modules() -> anyhow::Result<HashMap<String, parser::Document>> {
    let mut m = HashMap::new();
    let dir = fs::read_dir("assets/SteamKit/Resources/SteamLanguage")?;

    for entry in dir {
        let entry = entry?;
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            continue;
        }

        // make sure valid extension
        if entry.path().extension().is_none() || entry.path().extension().unwrap() != "steamd" {
            continue;
        }

        let name = entry
            .path()
            .file_stem()
            .unwrap()
            .to_str()
            .context("invalid file stem")?
            .to_owned();

        let content = fs::read_to_string(&entry.path())?;
        let document = parser::parse(&content)?;

        m.insert(name, document);
    }

    Ok(m)
}

fn main() -> anyhow::Result<()> {
    // use cargo out dir for build files
    let cargo_out_dir = env::var("OUT_DIR").expect("OUT_DIR env var not set");
    let mut out_dir = PathBuf::from(cargo_out_dir);
    out_dir.push("generated");

    // remove previous build files
    if out_dir.exists() {
        fs::remove_dir_all(&out_dir).unwrap();
    }

    // create dir
    fs::create_dir(&out_dir)?;

    let modules = get_modules()?;
    let outputs = generator::generate(modules)?;

    for (name, output) in &outputs {
        let mut path = out_dir.clone();
        path.push(format!("{name}.rs"));
        fs::write(path, output)?;
    }

    {
        let mut path = out_dir.clone();
        path.push("mod.rs");

        let root = outputs
            .keys()
            .map(|k| format!("pub mod {};\n", k))
            .collect::<String>();
        fs::write(path, root)?;
    }

    Ok(())
}
