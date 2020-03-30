//! Database

use crate::common::*;
use aes_gcm::{
    aead::{generic_array::GenericArray, Aead, NewAead},
    Aes256Gcm,
};
use kern::{Config, Fail};
use std::convert::TryInto;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

/// Encrypted storage
pub struct SecureStorage {
    file_name: String,
    file: File,
    aead: Aes256Gcm,
    nonce: Nonce,
    raw_data: String,
}

impl SecureStorage {
    /// Get raw data
    pub fn raw_data(&self) -> &str {
        &self.raw_data
    }

    /// Create new secure storage
    pub fn new(file_name: String, raw_key: &str) -> Result<Self, Fail> {
        // read file
        let mut file = Self::open_file(&file_name)?;
        let buf = Self::read_file(&mut file)?;

        // initialize crypto
        let key = GenericArray::clone_from_slice(raw_key.as_bytes());
        let aead = Aes256Gcm::new(key);
        let file_nonce = [
            &b"etopa+crypto"[..],
            &file_name
                .split('/')
                .last()
                .unwrap_or("etopa+crypto")
                .as_bytes()[..],
        ]
        .concat();
        let nonce =
            GenericArray::clone_from_slice(&file_nonce[(file_nonce.len() - 12)..file_nonce.len()]);

        let raw_data = if buf.len() <= 1 {
            // encrypt initialization
            let initial_data = b"etopa_secure_storage#1\n".to_vec();
            let encrypted = aead
                .encrypt(&nonce, initial_data.as_ref())
                .or_else(|_| Fail::from("could not initialize secure storage"))?;

            // write
            file.write_all(&encrypted).or_else(Fail::from)?;

            // set initial data
            String::from_utf8(initial_data).or_else(Fail::from)?
        } else {
            // decrypt data
            let decrypted = aead
                .decrypt(&nonce, buf.as_ref())
                .or_else(|_| Fail::from("could not decrypt secure storage"))?;
            String::from_utf8(decrypted).or_else(Fail::from)?
        };

        // return secure storage
        Ok(Self {
            file_name,
            file,
            aead,
            nonce,
            raw_data,
        })
    }

    // overwrite file with data
    pub fn write_file(&mut self, data: String) -> Result<(), Fail> {
        // truncate file and reopen
        self.file.set_len(0).or_else(Fail::from)?;
        self.file = Self::open_file(&self.file_name)?;

        // encrypt data
        let encrypted = self
            .aead
            .encrypt(&self.nonce, data.as_bytes())
            .or_else(|_| Fail::from("could not encrypt data for secure storage"))?;

        // truncate and write file
        self.file.write_all(&encrypted).or_else(Fail::from)?;

        // set raw_data
        self.raw_data = data;
        Ok(())
    }

    // opens a file
    fn open_file(file_name: &str) -> Result<File, Fail> {
        // open file
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_name)
            .or_else(Fail::from)
    }

    // reads a file
    fn read_file(file: &mut File) -> Result<Vec<u8>, Fail> {
        // create buffer
        let mut buf = Vec::with_capacity(match file.metadata() {
            Ok(metadata) => (metadata.len() as usize).try_into().unwrap_or(8192),
            Err(_) => 8192,
        });

        // read and return
        file.read_to_end(&mut buf).or_else(Fail::from)?;
        Ok(buf)
    }

    /// Parses the decrypted storage
    pub fn parse(&self) -> Result<Config, Fail> {
        let mut head_conf = self.raw_data.splitn(2, '\n');
        let head = head_conf
            .next()
            .ok_or_else(|| Fail::new("invalid secure storage format"))?;
        let conf = head_conf
            .next()
            .ok_or_else(|| Fail::new("invalid secure storage format"))?;
        if head == "etopa_secure_storage#1" {
            Ok(Config::from(&conf))
        } else {
            Fail::from("invalid secure storage format")
        }
    }
}
