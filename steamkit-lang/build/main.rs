use std::fs;

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
    let file = fs::read_to_string("assets/SteamKit/Resources/SteamLanguage/enums.steamd").unwrap();
    println!("{:?}", parser::document(&file));
}
