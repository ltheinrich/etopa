//! Type definitions

use aes_gcm::aead::generic_array::{
    typenum::{
        bit::{B0, B1},
        uint::{UInt, UTerm},
    },
    GenericArray,
};

pub type Aes256Key =
    GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>>;
pub type Nonce = GenericArray<u8, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>>;
