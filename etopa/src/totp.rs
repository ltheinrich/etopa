//! Time-based one-time password

use data_encoding::BASE32;
use kern::Fail;
use ring::hmac::{sign, Algorithm, Key, HMAC_SHA1_FOR_LEGACY_USE_ONLY};
use std::convert::TryInto;
use std::time::{SystemTime, UNIX_EPOCH};

/// TOTP generator
pub struct Generator {
    secret: Vec<u8>,
    key: Key,
    digits: u32,
    algorithm: Algorithm,
}

impl Generator {
    /// Create new TOTP generator
    pub fn new(secret: String) -> Result<Self, Fail> {
        let secret = secret.as_bytes();
        let mut output = vec![0; BASE32.decode_len(secret.len()).or_else(Fail::from)?];
        let len = BASE32
            .decode_mut(secret, &mut output)
            .or_else(|e| Fail::from(e.error))?;
        let secret = Vec::from(&output[0..len]);
        let key = Key::new(HMAC_SHA1_FOR_LEGACY_USE_ONLY, &secret);
        Ok(Self {
            secret,
            key,
            digits: 6,
            algorithm: HMAC_SHA1_FOR_LEGACY_USE_ONLY,
        })
    }

    /// Change number of digits
    pub fn set_digits(mut self, digits: u32) -> Self {
        self.digits = digits;
        self
    }

    /// Change hashing algorithm
    pub fn set_algorithm(mut self, algorithm: Algorithm) -> Self {
        self.algorithm = algorithm;
        self.key = Key::new(self.algorithm, &self.secret);
        self
    }

    /// Generate new token
    pub fn token(&self) -> Result<String, Fail> {
        let elapsed_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .or_else(Fail::from)?;
        let time = (elapsed_time.as_secs() / 30).to_be_bytes();
        let mut token = self.token_at(&time).to_string();
        while token.len() < self.digits.try_into().or_else(Fail::from)? {
            token.insert_str(0, "0")
        }
        Ok(token)
    }

    /// Generate new token at time
    fn token_at(&self, time: &[u8]) -> u32 {
        let signed = sign(&self.key, &time);
        let sr = signed.as_ref();
        let offset = (sr[sr.len() - 1] & 0xf) as usize;
        let hash = sr[offset..offset + 4].to_vec();
        let code: u32 = ((u32::from(hash[0]) & 0x7f) << 24)
            | ((u32::from(hash[1]) & 0xff) << 16)
            | ((u32::from(hash[2]) & 0xff) << 8)
            | (u32::from(hash[3]) & 0xff);
        code % 10u32.pow(self.digits)
    }
}

// TODO: add tests
