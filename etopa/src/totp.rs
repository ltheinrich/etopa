//! Time-based one-time password

use base32::Alphabet::{self, RFC4648};
use kern::Fail;
use ring::hmac::{sign, Algorithm, Key};
use std::time::{SystemTime, UNIX_EPOCH};

/// Hashing algorithms
pub mod algorithms {
    pub use ring::hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY as SHA1;
    pub use ring::hmac::HMAC_SHA256 as SHA256;
    pub use ring::hmac::HMAC_SHA384 as SHA384;
    pub use ring::hmac::HMAC_SHA512 as SHA512;
}

/// Alphabet for Base32 secret decoding
static ALPHABET: Alphabet = RFC4648 { padding: false };

/// TOTP generator
#[derive(Clone, Debug)]
pub struct Generator {
    secret: Vec<u8>,
    key: Key,
    digits: usize,
}

impl Generator {
    /// Create new TOTP generator
    pub fn new(secret: impl AsRef<str>) -> Result<Self, Fail> {
        // decode base32 secret
        let decoded = base32::decode(ALPHABET, secret.as_ref());
        let secret = decoded.ok_or_else(|| Fail::new("invalid base32 secret"))?;

        // generate key with algorithm
        let key = Key::new(algorithms::SHA1, &secret);
        Ok(Self {
            secret,
            key,
            digits: 6,
        })
    }

    /// Change number of digits
    pub fn digits(mut self, digits: usize) -> Self {
        self.digits = digits;
        self
    }

    /// Change hashing algorithm
    pub fn algorithm(mut self, algorithm: Algorithm) -> Self {
        // set algorithm and generate new key
        self.key = Key::new(algorithm, &self.secret);
        self
    }

    /// Generate new token
    pub fn token(&self) -> Result<String, Fail> {
        let elapsed_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .or_else(Fail::from)?;
        self.token_at(elapsed_time.as_secs())
    }

    /// Generate new token at time (in secs)
    pub fn token_at(&self, time: u64) -> Result<String, Fail> {
        let signed = sign(&self.key, &(time / 30).to_be_bytes());
        let sr = signed.as_ref();
        let offset = (sr[sr.len() - 1] & 0xf) as usize;
        let hash = sr[offset..offset + 4].to_vec();
        let code: u32 = ((u32::from(hash[0]) & 0x7f) << 24)
            | ((u32::from(hash[1]) & 0xff) << 16)
            | ((u32::from(hash[2]) & 0xff) << 8)
            | (u32::from(hash[3]) & 0xff);
        let mut token = (code % 10u32.pow(self.digits as u32)).to_string();
        while token.len() < self.digits {
            token.insert(0, '0');
        }
        Ok(token)
    }
}

// TODO: add tests
