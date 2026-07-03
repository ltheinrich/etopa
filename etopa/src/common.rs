//! Commons

use aes_gcm::aead::array::{
    Array,
    typenum::{
        bit::{B0, B1},
        uint::{UInt, UTerm},
    },
};

/// AES 256-bit key representation
pub type Aes256Key = Array<u8, UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>>;

/// AES-GCM nonce representation
pub type Nonce = Array<u8, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>>;
