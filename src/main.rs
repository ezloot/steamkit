use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use steam_kit::vdf::{Value, Tokens};

// use futures::{channel::oneshot::Receiver, SinkExt, StreamExt};
// use steam_kit::connection::Request;

#[derive(Serialize, Debug)]
enum Test {
    #[serde(rename = "app_news")]
    A(usize),
}

// #[tokio::main]
fn main() {
    // let test = steam_kit::vdf::to_string(&Test::A(1234))
    // .unwrap();

    // println!("{}", test);

    let tokens = Tokens::new(r#"
    test
    {
        "3"		"DMR ALİ"
        "4"		"무슨 일이"
        "5"		"發生了什麼"
        "6"		"Zażółć gęślą jaźń"
        "7"		"ماذا يحدث"
    }
    
    "#.trim());

    println!("{:?}", tokens);
}
