use glob::glob;
use std::{env, fs, path::PathBuf};

mod parser;

// use std::env;

// TODO: support classes
// TODO: support enums
// TODO: support flags
// TODO: add a way to map EMsg to message classes
// TODO: support class constants (as pub const on struct impls)
// TODO: `use derive-new` to add default values for new
// TODO: show removed enum variants as commented out (with message if available)
// TODO: show obsolete enum variants as deprecated (with message if available)

/*

// Do some api like this?

let client = Client::new();

client.inbound.on(EMsg::ChannelEncryptRequest, |client, message| {
    let message = message.read::<ChannelEncryptRequest>().unwrap();

    client.outbound.send(...);
});

client.inbound.on_async(EMsg::ChannelEncryptRequest, async |client, message| {
    let message = message.read::<ChannelEncryptRequest>().unwrap();

    client.outbound.send(...);
}).await;

*/

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
    let paths = glob("assets/SteamKit/Resources/SteamLanguage/enums.steamd").unwrap();
    
    for path in paths {
        let path = path.unwrap();
        let content = fs::read_to_string(&path).unwrap();

        if let Ok((_, _document)) = parser::document(&content) {
            let module = path.file_stem().unwrap().to_str().unwrap().to_owned();
            let mut path = out_dir.clone();
            path.push(format!("{module}.rs"));
            modules.push(module);

            // todo generate content for module
            fs::write(path, "yay").unwrap();
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
