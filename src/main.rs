use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use steam_kit::vdf::VDF;

// use futures::{channel::oneshot::Receiver, SinkExt, StreamExt};
// use steam_kit::connection::Request;

// #[derive(Serialize, Deserialize, Debug)]
// struct appnews {
//     appid: u64,
//     newsitems: Vec<newsitem>,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct newsitem {
//     gid: String,
//     title: String,
//     url: String,
//     is_external_url: bool,
//     author: String,
//     contents: String,
//     feedlabel: String,
//     date: u64,
//     feedname: String,
//     feed_type: u8,
//     appid: u64,
// }

#[tokio::main]
async fn main() {
    let vdf = steam_kit::vdf!({
        "test": true,
        "arr": [
            { "test": "123" }
        ]
    });

    println!("{:?}", vdf["arr"][0]["test"].to::<u32>());
    println!("{:?}", vdf.get("arr").unwrap().get(0));
}

// #[tokio::main]
// async fn main() {
//     let vdf = steam_kit::vdf!({
//         "test": true,
//         "test": true,
//     });

//     println!("{:?}", vdf);

//     // let news: appnews = steam_kit::request::get(&steam_kit::request::Config {
//     //     iface: "ISteamNews",
//     //     method: "GetNewsForApp",
//     //     version: "2",
//     //     query: Some(&[("appid", 440), ("count", 3)]),
//     // })
//     // .await
//     // .unwrap();

//     // let (mut client, connection) = steam_kit::connect();

//     // // spawn connection off somewhere else
//     // tokio::spawn(async move {
//     //     if let Err(e) = connection.await {
//     //         println!("connection error: {:?}", e);
//     //     }
//     // });

//     // let (sender, mut receiver) = futures::channel::mpsc::channel(1);
//     // let _ = client.sender.send(Request { sender }).await;

//     // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
//     // println!("received: {:?}", receiver.next().await.unwrap());
// }
