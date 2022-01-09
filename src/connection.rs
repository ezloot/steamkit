use std::{pin::Pin, task::{Context, Poll}};

use futures::{Future, StreamExt, channel::mpsc, ready, stream::FusedStream};

use crate::client::Response;

#[derive(Debug)]
pub struct Request {
    pub sender: mpsc::Sender<Response>,
}

pub struct Connection {
    pub receiver: mpsc::UnboundedReceiver<Request>,
}

impl Connection {
    pub fn new(receiver: mpsc::UnboundedReceiver<Request>) -> Self {
        Self { receiver }
    }

    pub fn poll_message(&mut self, cx: &mut Context<'_>) -> Poll<Option<Result<(), ()>>> {
        if self.receiver.is_terminated() {
            println!("we closed!");
            return Poll::Ready(None);
        }

        if let Poll::Ready(Some(mut request)) = self.receiver.poll_next_unpin(cx) {
            println!("we got something!");
            let _ = request.sender.try_send(Response::Ok);
        }

        Poll::Pending
    }
}

impl Future for Connection {
    type Output = Result<(), ()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        while let Some(request) = ready!(self.poll_message(cx)) {
            println!("poll");
        }

        println!("dead!");

        Poll::Ready(Ok(()))
    }
}
