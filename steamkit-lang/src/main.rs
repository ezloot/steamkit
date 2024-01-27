mod generator;
mod parser;

use anyhow::Context;

use std::collections::HashMap;
use std::fs;

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
    let modules = get_modules()?;
    let outputs = generator::generate(modules)?;

    for (name, output) in outputs {
        fs::write(format!("generated/{}.rs", name), output)?;
    }

    Ok(())
}
