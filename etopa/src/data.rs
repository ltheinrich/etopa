//! Database

use aes_gcm::{
    aead::{generic_array::GenericArray, Aead, NewAead},
    Aes256Gcm,
};
use kern::Fail;
use rand::{thread_rng, Rng};
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::str::FromStr;
use std::string::ToString;

/// Raw data storage file
pub struct StorageFile {
    file: File,
}

/// Encrypted storage (AES-GCM-256)
pub struct SecureStorage {
    aead: Aes256Gcm,
    data: BTreeMap<String, String>,
}

impl StorageFile {
    /// Read storage file and parse to map
    pub fn parse(&mut self) -> Result<BTreeMap<String, String>, Fail> {
        // read file initialize
        let buf = String::from_utf8(self.read()?).or_else(Fail::from)?;

        // initialize map and split lines
        let mut conf = BTreeMap::new();
        buf.split('\n')
            // seperate and trim
            .map(|l| l.splitn(2, '=').map(|c| c.trim()).collect())
            // iterate through seperated lines
            .for_each(|kv: Vec<&str>| {
                // check if contains key and value
                if kv.len() == 2 {
                    conf.insert(kv[0].to_lowercase(), kv[1].to_string());
                }
            });

        // return
        Ok(conf)
    }

    /// Serialize map to string and write to file
    pub fn serialize(&mut self, data: &BTreeMap<String, String>) -> Result<(), Fail> {
        // create buffer
        let mut buf = String::with_capacity(data.len() * 10);

        // add entries
        for (k, v) in data {
            buf.push_str(k);
            buf.push('=');
            buf.push_str(v);
            buf.push('\n');
        }

        // write
        self.write(buf.as_bytes()).or_else(Fail::from)
    }

    /// Open file or create new
    pub fn new(file_name: &str) -> Result<Self, Fail> {
        // open file
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_name)
            .or_else(Fail::from)?;

        // return
        Ok(Self { file })
    }

    /// Read data from file
    pub fn read(&mut self) -> Result<Vec<u8>, Fail> {
        // start from beginning
        self.file
            .seek(std::io::SeekFrom::Start(0))
            .or_else(Fail::from)?;

        // create buffer
        let mut buf = Vec::with_capacity(match self.file.metadata() {
            Ok(metadata) => (metadata.len() as usize).try_into().unwrap_or(8192),
            Err(_) => 8192,
        });

        // read and return
        self.file.read_to_end(&mut buf).or_else(Fail::from)?;
        Ok(buf)
    }

    /// Write data to file
    pub fn write(&mut self, data: &[u8]) -> Result<(), Fail> {
        // truncate file
        self.file.set_len(0).or_else(Fail::from)?;

        // start from first byte
        self.file
            .seek(std::io::SeekFrom::Start(0))
            .or_else(Fail::from)?;

        // write data
        self.file.write_all(data).or_else(Fail::from)
    }
}

impl SecureStorage {
    /// Set value in secure storage
    pub fn set<T: ToString>(&mut self, name: &str, value: T) {
        // add to data map
        self.data.insert(name.to_lowercase(), value.to_string());
    }

    /// Set string value in secure storage
    pub fn insert(&mut self, name: &str, value: String) {
        // add to data map
        self.data.insert(name.to_lowercase(), value);
    }

    /// Remove value from secure storage
    pub fn remove(&mut self, name: &str) {
        // remove from data map
        self.data.remove(name);
    }

    /// Get value from secure storage
    pub fn get<T: FromStr>(&self, name: &str) -> Result<T, Fail> {
        match self.data.get(name) {
            Some(value) => value
                .parse()
                .or_else(|_| Fail::from("could not parse value into required type")),
            None => Fail::from("no value for key"),
        }
    }

    /// Get string value from secure storage
    pub fn value(&self, name: &str) -> Option<&str> {
        // return value
        Some(self.data.get(&name.to_lowercase())?)
    }

    /// Check if name is key in data map
    pub fn exists(&self, name: &str) -> bool {
        // return map
        self.data.contains_key(name)
    }

    /// Get data map
    pub fn data(&self) -> &BTreeMap<String, String> {
        // return map
        &self.data
    }

    /// Create new secure storage
    pub fn new(raw_data: &[u8], raw_key: &str) -> Result<Self, Fail> {
        // initialize aes
        let key = GenericArray::clone_from_slice(raw_key.as_bytes());
        let aead = Aes256Gcm::new(key);

        // check if contains at least nonce (first 12 bytes)
        if raw_data.len() < 13 {
            // no data
            Ok(Self {
                aead,
                data: BTreeMap::new(),
            })
        } else {
            // get nonce and decrypt data
            let nonce = GenericArray::clone_from_slice(&raw_data[..12]);
            let decrypted = aead
                .decrypt(&nonce, &raw_data[12..])
                .or_else(|_| Fail::from("could not decrypt secure storage data"))?;

            // decrypted to string
            let dec_data = String::from_utf8(decrypted).or_else(Fail::from)?;
            Ok(Self {
                aead,
                data: parse(&dec_data),
            })
        }
    }

    /// Serialize and encrypt secure storage
    pub fn encrypt(&self) -> Result<Vec<u8>, Fail> {
        // generate random nonce
        let mut rng = thread_rng();
        let mut raw_data: Vec<u8> = (0..12).map(|_| rng.gen()).collect();
        let nonce = GenericArray::clone_from_slice(&raw_data);

        // encrypt data
        let mut encrypted = self
            .aead
            .encrypt(&nonce, serialize(&self.data).as_bytes())
            .or_else(|_| Fail::from("could not encrypt secure storage data"))?;

        // add encrypted and return
        raw_data.append(&mut encrypted);
        Ok(raw_data)
    }
}

/// Parse string to map
fn parse(dec_data: &str) -> BTreeMap<String, String> {
    // initialize map and split lines
    let mut conf = BTreeMap::new();
    dec_data
        .split('\n')
        // seperate and trim
        .map(|l| l.splitn(2, '=').map(|c| c.trim()).collect())
        // iterate through seperated lines
        .for_each(|kv: Vec<&str>| {
            // check if contains key and value
            if kv.len() == 2 {
                conf.insert(kv[0].to_lowercase(), kv[1].to_string());
            }
        });

    // return
    conf
}

/// Serialize map to string
fn serialize(data: &BTreeMap<String, String>) -> String {
    // create buffer
    let mut buf = String::with_capacity(data.len() * 10);

    // add entries
    for (k, v) in data {
        buf.push_str(k);
        buf.push('=');
        buf.push_str(v);
        buf.push('\n');
    }

    // return
    buf
}
