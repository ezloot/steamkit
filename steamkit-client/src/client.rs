// use std::sync::Arc;
//
// use futures::{
//     channel::mpsc::{UnboundedReceiver, UnboundedSender},
//     lock::Mutex,
// };
//
// #[derive(Clone, Debug)]
// pub struct Client {
//     inner: ClientInner,
// }
//
// #[derive(Clone, Debug)]
// struct ClientInner {
//     tx: UnboundedSender<Vec<u8>>,
//     rx: UnboundedReceiver<Vec<u8>>,
//     state: Arc<Mutex<ClientState>>,
// }
//
// struct ClientState {}
