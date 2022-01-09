use serde::{Serialize, Deserialize};

// use futures::{channel::oneshot::Receiver, SinkExt, StreamExt};
// use steam_kit::connection::Request;

#[derive(Serialize, Deserialize, Debug)]
struct Test {
    name: String,
    age: String,
    test: bool,
}

#[tokio::main]
async fn main() {
    steam_kit::request::get(&steam_kit::request::Config {
        iface: "ISteamNews",
        method: "GetNewsForApp",
        version: "2",
        query: Some(&[("appid", 440), ("count", 3)]),
    })
    .await
    .unwrap();
    // let (mut client, connection) = steam_kit::connect();

    // // spawn connection off somewhere else
    // tokio::spawn(async move {
    //     if let Err(e) = connection.await {
    //         println!("connection error: {:?}", e);
    //     }
    // });

    // let (sender, mut receiver) = futures::channel::mpsc::channel(1);
    // let _ = client.sender.send(Request { sender }).await;

    // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    // println!("received: {:?}", receiver.next().await.unwrap());
}
