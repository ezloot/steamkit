





#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EUdpPacketType(pub i32);

impl EUdpPacketType {
    pub const MAX: Self = Self(8);
    pub const DATAGRAM: Self = Self(7);
    pub const DATA: Self = Self(6);
    pub const DISCONNECT: Self = Self(5);
    pub const ACCEPT: Self = Self(4);
    pub const CONNECT: Self = Self(3);
    pub const CHALLENGE: Self = Self(2);
    pub const CHALLENGE_REQ: Self = Self(1);
    pub const INVALID: Self = Self(0);
}

impl PartialEq<i32> for EUdpPacketType {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<EUdpPacketType> for i32 {
    fn eq(&self, other: &EUdpPacketType) -> bool {
        *self == other.0
    }
}
