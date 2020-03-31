//! Database

use crate::common::*;
use aes_gcm::{
    aead::{generic_array::GenericArray, Aead, NewAead},
    Aes256Gcm,
};
use kern::Fail;
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::str::FromStr;
use std::string::ToString;

/// Encrypted storage
pub struct SecureStorage {
    file_name: String,
    file: File,
    aead: Aes256Gcm,
    nonce: Nonce,
    data: BTreeMap<String, String>,
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

    /// Create new secure storage (parse with )
    pub fn new(file_name: String, raw_key: &str) -> Result<Self, Fail> {
        // read file
        let mut file = open_file(&file_name)?;
        let buf = read_file(&mut file)?;

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

        // get decrypted data
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

        // parse raw data to map
        let data = parse(&raw_data)?;

        // return unparsed secure storage
        Ok(Self {
            file_name,
            file,
            aead,
            nonce,
            data,
        })
    }

    /// Write secure storage to file
    fn write_file(&mut self) -> Result<(), Fail> {
        // truncate file and reopen
        self.file.set_len(0).or_else(Fail::from)?;
        self.file = open_file(&self.file_name)?;

        // encrypt data
        let encrypted = self
            .aead
            .encrypt(&self.nonce, serialize(&self.data)?.as_bytes())
            .or_else(|_| Fail::from("could not encrypt data for secure storage"))?;

        // write file and return
        self.file.write_all(&encrypted).or_else(Fail::from)?;
        Ok(())
    }
}

impl Drop for SecureStorage {
    // save at drop
    fn drop(&mut self) {
        if let Err(err) = self.write_file() {
            eprintln!(
                "Failed to write secure storage to file ({}) at drop: '{}'",
                self.file_name, err
            );
        }
    }
}

// parse string to map
fn parse(raw_data: &str) -> Result<BTreeMap<String, String>, Fail> {
    // split first line and check format
    let mut head_conf = raw_data.splitn(2, '\n');
    if head_conf
        .next()
        .ok_or_else(|| Fail::new("invalid secure storage format"))?
        != "etopa_secure_storage#1"
    {
        return Fail::from("invalid secure storage format");
    }

    // Config parser
    let mut conf = BTreeMap::new();
    head_conf
        .next()
        .ok_or_else(|| Fail::new("invalid secure storage format"))?
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
    Ok(conf)
}

// serialize map to string
fn serialize(data: &BTreeMap<String, String>) -> Result<String, Fail> {
    // create buffer and add header
    let mut buf = String::with_capacity(22 + data.len() * 10);
    buf.push_str("etopa_secure_storage#1\n");

    // add entries
    for (k, v) in data {
        buf.push_str(k);
        buf.push('=');
        buf.push_str(v);
        buf.push('\n');
    }

    // return
    Ok(buf)
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
