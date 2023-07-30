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
    let text = r#"

    enum EChatEntryType
    {
        Invalid = 0;
    
        ChatMsg = 1;
        Typing = 2;
        InviteGame = 3;
        Emote = 4; removed "No longer supported by clients"
        LobbyGameStart = 5; removed "Listen for LobbyGameCreated_t callback instead"
        LeftConversation = 6;
        Entered = 7;
        WasKicked = 8;
        WasBanned = 9;
        Disconnected = 10;
        HistoricalChat = 11;
        Reserved1 = 12;
        Reserved2 = 13;
        LinkBlocked = 14;
    };
    "#;

    // println!("{:?}", parser::reason(r#""test 123""#));
    let file = fs::read_to_string("assets/SteamKit/Resources/SteamLanguage/enums.steamd").unwrap();
    println!("{:?}", parser::document(&file));
    // println!("{:?}", parser::parse_enum(&text));
}
