#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EUdpPacketType(pub u8);

impl EUdpPacketType {
    pub const INVALID: Self = Self(0);
    pub const CHALLENGE_REQ: Self = Self(1);
    pub const CHALLENGE: Self = Self(2);
    pub const CONNECT: Self = Self(3);
    pub const ACCEPT: Self = Self(4);
    pub const DISCONNECT: Self = Self(5);
    pub const DATA: Self = Self(6);
    pub const DATAGRAM: Self = Self(7);
    pub const MAX: Self = Self(8);
}

impl PartialEq<u8> for EUdpPacketType {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EUdpPacketType> for u8 {
    fn eq(&self, other: &EUdpPacketType) -> bool {
        *self == other.0
    }
}

impl Default for EUdpPacketType {
    fn default() -> Self {
        Self::INVALID
    }
}
#[derive(Debug, Clone, new)]
pub struct UdpHeader {
    #[new(default = "todo!()")]
    pub magic: u32,
    pub payload_size: u16,
    #[new(default = "todo!()")]
    pub packet_type: TODO,
    pub flags: u8,
    #[new(default = "512")]
    pub source_conn_id: u32,
    pub dest_conn_id: u32,
    pub seq_this: u32,
    pub seq_ack: u32,
    pub packets_in_msg: u32,
    pub msg_start_seq: u32,
    pub msg_size: u32,
}

impl UdpHeader {
    pub const MAGIC: u32 = 0x31305356;
}

#[derive(Debug, Clone, new)]
pub struct ChallengeData {
    pub challenge_value: u32,
    pub server_load: u32,
}

impl ChallengeData {
    pub const CHALLENGE_MASK: u32 = 0xA426DF2B;
}

#[derive(Debug, Clone, new)]
pub struct ConnectData {
    pub challenge_value: u32,
}

impl ConnectData {
    pub const CHALLENGE_MASK: u32 = todo!();
}

#[derive(Debug, Clone, Default)]
pub struct Accept;

#[derive(Debug, Clone, Default)]
pub struct Datagram;

#[derive(Debug, Clone, Default)]
pub struct Disconnect;

