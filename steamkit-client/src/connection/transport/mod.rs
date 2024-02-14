pub mod tcp;
pub mod ws;

use futures::Future;

pub trait Transport {
    fn read(&self) -> impl Future<Output = anyhow::Result<Vec<u8>>> + Send;
    fn write(&self) -> impl Future<Output = ()> + Send;
}
