use std::env;

// TODO: support classes
// TODO: support enums
// TODO: support flags
// TODO: add a way to map EMsg to message classes
// TODO: support class constants (as pub const on struct impls)
// TODO: `use derive-new` to add default values for new
// TODO: show removed enum variants as commented out (with message if available)
// TODO: show obsolete enum variants as deprecated (with message if available)

fn main() {
    let cwd = env::current_dir().unwrap();
    eprintln!("{:?}", cwd);
    // panic!(cwd);
    panic!();
}
