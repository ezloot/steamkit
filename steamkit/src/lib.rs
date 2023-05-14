use client::Client;
use connection::Connection;
use futures::channel::mpsc;

#[macro_use]
extern crate lazy_static;

pub mod client;
pub mod connection;
pub mod id;
pub mod request;

pub use steamkit_vdf as vdf;

pub fn connect() -> (Client, Connection) {
    let (sender, receiver) = mpsc::unbounded();
    let client = Client::new(sender);
    let connection = Connection::new(receiver);

    (client, connection)
}
