use self::transport::Transport;

pub mod transport;

pub struct Connection<T: Transport> {
    transport: T,
}

impl<T: Transport> Connection<T> {
    pub fn new(transport: T) -> Self {
        Connection { transport }
    }
}
