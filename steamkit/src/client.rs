use futures::channel::mpsc;

use crate::connection::Request;

#[derive(Debug)]
pub enum Response {
    Ok,
}

pub struct Client {
    pub sender: mpsc::UnboundedSender<Request>,
}

impl Client {
    pub fn new(sender: mpsc::UnboundedSender<Request>) -> Self {
        Self { sender }
    }
}
