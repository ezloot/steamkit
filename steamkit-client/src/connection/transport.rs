use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf, ReadHalf, WriteHalf},
        TcpStream, ToSocketAddrs,
    },
};

pub struct TcpTransport {
    rx: OwnedReadHalf,
    tx: OwnedWriteHalf,
}

impl TcpTransport {
    pub const MAGIC: &str = "VT01";

    pub async fn connect<A: ToSocketAddrs>(addr: A) -> anyhow::Result<Self> {
        let stream = TcpStream::connect(addr).await?;
        let (rx, tx) = stream.into_split();
        Ok(Self { rx, tx })
    }

    async fn read(rx: &mut OwnedReadHalf, session_key: Option<Vec<u8>>) -> anyhow::Result<Vec<u8>> {
        // read message length
        let len = rx.read_u32_le().await?;

        // read header magic
        let mut header = vec![0; 4];
        rx.read_exact(&mut header).await?;

        // decode magic from bytes
        let magic = String::from_utf8(header)?;
        if magic != Self::MAGIC {
            anyhow::bail!("Connection out of sync");
        }

        let mut data = vec![0; len as usize];
        rx.read_exact(&mut data).await?;

        // TODO: decrypt

        Ok(data)
    }

    async fn write(tx: &mut OwnedWriteHalf, data: &[u8]) -> anyhow::Result<()> {
        // TODO: encypt data before doing anything

        // get length as u32 le bytes
        let len = data.len();

        tx.write_u32_le(len as u32).await?;
        tx.write_all(Self::MAGIC.as_bytes()).await?;
        tx.write_all(data).await?;

        Ok(())
    }

    // async fn wr
}

// pub struct WsTransport {

// }

pub enum Transport {
    Tcp(TcpTransport),
    // Ws(WsTransport),
}

#[cfg(test)]
mod tests {
    use super::TcpTransport;

    #[tokio::test]
    async fn try_connect() {
        TcpTransport::connect("162.254.193.102:27017")
            .await
            .unwrap();
    }
}
