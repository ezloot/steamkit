use lazy_static::lazy_static;
use rand::Rng;
use rsa::{pkcs8::DecodePublicKey, Oaep, RsaPublicKey};
use sha1::Sha1;

lazy_static! {
    static ref PUBLIC_KEY: RsaPublicKey =
        DecodePublicKey::from_public_key_pem(include_str!("system.pem")).unwrap();
}

#[derive(Debug, Clone)]
pub struct SessionKey {
    pub plain: Vec<u8>,
    pub encrypted: Vec<u8>,
}

impl SessionKey {
    pub fn generate(nonce: &[u8]) -> Self {
        let mut rng = rand::thread_rng();
        let session_key = rng.gen::<[u8; 32]>().to_vec();

        let mut buf = session_key.clone();
        buf.extend_from_slice(&nonce);

        let padding = Oaep::new::<Sha1>();
        let encrypted = PUBLIC_KEY
            .encrypt(&mut rng, padding, &buf)
            .expect("failed to encrypt");

        Self {
            plain: session_key,
            encrypted,
        }
    }
}

pub fn symmetric_decrypt(data: &[u8], key: &[u8], check_hmac: bool) {
    
}

pub mod hash {
    use crc::{Crc, CRC_32_ISO_HDLC};
    use sha1::{Digest, Sha1};

    pub fn sha1(data: &[u8]) -> Vec<u8> {
        let mut sha = Sha1::new();
        sha.update(data);
        sha.finalize().to_vec()
    }

    pub fn crc32(data: &[u8]) -> u32 {
        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        crc.checksum(data)
    }
}

pub use hex::encode as hex;

#[cfg(test)]
mod tests {
    use super::hash;

    #[test]
    fn sha1() {
        let value = hash::sha1(b"the quick brown fox jumps over the lazy dog");
        assert_eq!(hex::encode(value), "16312751ef9307c3fd1afbcb993cdc80464ba0f1");
    }

    #[test]
    fn crc32() {
        let value = hash::crc32(b"the quick brown fox jumps over the lazy dog");
        assert_eq!(value, 3456913684);
    }
}
