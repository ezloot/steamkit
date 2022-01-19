use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use steam_kit::vdf::Value;

// use futures::{channel::oneshot::Receiver, SinkExt, StreamExt};
// use steam_kit::connection::Request;

#[derive(Serialize, Debug)]
enum Test {
    #[serde(rename = "app_news")]
    A(usize),
}

// #[tokio::main]
fn main() {
    let test = steam_kit::vdf::to_string(&Test::A(1234))
    .unwrap();

    println!("{}", test);
}
