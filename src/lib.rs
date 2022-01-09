use client::Client;
use connection::Connection;
use futures::channel::mpsc;

#[macro_use]
extern crate lazy_static;

pub mod id;
pub mod client;
pub mod connection;
pub mod request;
pub mod vdf;

pub fn connect() -> (Client, Connection) {
    let (sender, receiver) = mpsc::unbounded();
    let client = Client::new(sender);
    let connection = Connection::new(receiver);

    (client, connection)
}