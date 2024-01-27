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
