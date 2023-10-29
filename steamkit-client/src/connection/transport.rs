// use tokio::{
//     io::{AsyncReadExt, AsyncWriteExt},
//     net::{
//         tcp::{OwnedReadHalf, OwnedWriteHalf, ReadHalf, WriteHalf},
//         TcpStream, ToSocketAddrs,
//     },
// };
//
// pub struct TcpTransport {
//     rx: OwnedReadHalf,
//     tx: OwnedWriteHalf,
// }
//
// impl TcpTransport {
//     pub const MAGIC: &str = "VT01";
//
//     pub async fn connect<A: ToSocketAddrs>(addr: A) -> anyhow::Result<Self> {
//         let stream = TcpStream::connect(addr).await?;
//         let (rx, tx) = stream.into_split();
//         Ok(Self { rx, tx })
//     }
//
//     pub async fn read(
//         rx: &mut OwnedReadHalf,
//         // session_key: Option<Vec<u8>>,
//     ) -> anyhow::Result<Vec<u8>> {
//         // read message length
//         let len = rx.read_u32_le().await?;
//
//         // read header magic
//         let mut header = vec![0; 4];
//         rx.read_exact(&mut header).await?;
//
//         // decode magic from bytes
//         let magic = String::from_utf8(header)?;
//         if magic != Self::MAGIC {
//             anyhow::bail!("Connection out of sync");
//         }
//
//         let mut data = vec![0; len as usize];
//         rx.read_exact(&mut data).await?;
//
//         // TODO: decrypt
//
//         Ok(data)
//     }
//
//     pub async fn write(tx: &mut OwnedWriteHalf, data: &[u8]) -> anyhow::Result<()> {
//         // TODO: encypt data before doing anything
//
//         // get length as u32 le bytes
//         let len = data.len();
//
//         tx.write_u32_le(len as u32).await?;
//         tx.write_all(Self::MAGIC.as_bytes()).await?;
//         tx.write_all(data).await?;
//
//         Ok(())
//     }
// }
//
// // pub struct WsTransport {
//
// // }
//
// pub enum Transport {
//     Tcp(TcpTransport),
//     // Ws(WsTransport),
// }
//
// impl Transport {
//     pub async fn read(
//         rx: &mut OwnedReadHalf,
//         // session_key: Option<Vec<u8>>,
//     ) -> anyhow::Result<Vec<u8>> {
//     }
//
//     pub async fn write(tx: &mut OwnedWriteHalf, data: &[u8]) -> anyhow::Result<()> {}
// }
//
// // #[cfg(test)]
// // mod tests {
// //     use bytes::{Buf, Bytes};
// //     use steamkit_protos::{enums_clientserver::EMsg, protobuf::Enum};
// //     use steamkit_util::crypto::SessionKey;
//
// //     use super::TcpTransport;
//
// //     struct MessageHeader {
// //         msg: EMsg,
// //         target_job_id: u64,
// //         source_job_id: u64,
// //     }
//
// //     #[tokio::test]
// //     async fn try_connect() {
// //         const PROTO_MASK: u32 = 0x80000000;
//
// //         let mut transport = TcpTransport::connect("162.254.193.102:27017")
// //             .await
// //             .unwrap();
//
// //         let mut data = Bytes::from(TcpTransport::read(&mut transport.rx, None).await.unwrap());
//
// //         let raw_emsg = data.get_u32_le();
// //         let emsg = EMsg::from_i32((raw_emsg & !PROTO_MASK) as i32).unwrap();
// //         let is_protobuf = raw_emsg & PROTO_MASK > 0;
//
// //         if emsg == EMsg::k_EMsgChannelEncryptRequest || emsg == EMsg::k_EMsgChannelEncryptResult {
// //             let target_job_id = data.get_u64_le();
// //             let source_job_id = data.get_u64_le();
//
// //             handle_channel_encrypt_request(&mut data);
// //         } else if is_protobuf {
// //             // TODO: protobuf
// //         } else {
// //             data.advance(3);
// //             let target_job_id = data.get_u64_le();
// //             let source_job_id = data.get_u64_le();
// //             data.advance(1);
// //             let steam_id = data.get_u64_le();
// //             let session_id = data.get_u32_le();
// //         }
// //     }
//
// //     fn handle_channel_encrypt_request(data: &mut Bytes) {
// //         let test = data.get(0).cloned();
//
// //         let protocol = data.get_u32_le();
// //         let universe = data.get_u32_le();
// //         let nonce: [u8; 16] = data.get(0..16).expect("invalid nonce").try_into().unwrap();
// //         data.advance(16);
//
// //         // generate session key from nonce
// //         let session_key = SessionKey::generate(&nonce);
//
// //         println!("{protocol} {universe} {} {session_key:?}", steamkit_util::crypto::hex(nonce));
//
// //         // let nonce = data.get
// //     }
// // }
