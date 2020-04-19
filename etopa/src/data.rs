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
use std::string::ToString;

/// Raw data storage file
#[derive(Debug)]
pub struct StorageFile {
    file: File,
}

impl StorageFile {
    /// Open file or create new
    pub fn new(file_name: impl AsRef<str>) -> Result<Self, Fail> {
        // open file and return
        Ok(Self {
            file: open_file(file_name)?,
        })
    }

    /// Read storage file and parse to map
    pub fn parse(&mut self) -> Result<BTreeMap<String, String>, Fail> {
        // read file initialize
        let buf = String::from_utf8(read_file(&mut self.file)?).or_else(Fail::from)?;

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
        write_file(&mut self.file, buf.as_bytes()).or_else(Fail::from)
    }
}

/// Open file or create new
pub fn open_file(file_name: impl AsRef<str>) -> Result<File, Fail> {
    // open and return file
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name.as_ref())
        .or_else(Fail::from)
}

/// Read data from file
pub fn read_file(file: &mut File) -> Result<Vec<u8>, Fail> {
    // start from beginning
    file.seek(std::io::SeekFrom::Start(0)).or_else(Fail::from)?;

    // create buffer
    let mut buf = Vec::with_capacity(match file.metadata() {
        Ok(metadata) => (metadata.len() as usize).try_into().unwrap_or(8192),
        Err(_) => 8192,
    });

    // read and return
    file.read_to_end(&mut buf).or_else(Fail::from)?;
    Ok(buf)
}

/// Write data to file
pub fn write_file(file: &mut File, data: &[u8]) -> Result<(), Fail> {
    // truncate file
    file.set_len(0).or_else(Fail::from)?;

    // start from first byte
    file.seek(std::io::SeekFrom::Start(0)).or_else(Fail::from)?;

    // write data
    file.write_all(data).or_else(Fail::from)
}

/// Intialize Aes256Gcm with custom key
fn init_aes(raw_key: impl AsRef<[u8]>) -> Aes256Gcm {
    // initialize aes with key
    let key = GenericArray::clone_from_slice(raw_key.as_ref());
    Aes256Gcm::new(key)
}

/// Decrypt secure storage
pub fn decrypt(raw_data: impl AsRef<[u8]>, raw_key: impl AsRef<[u8]>) -> Result<String, Fail> {
    // init
    let raw_data = raw_data.as_ref();
    let aead = init_aes(raw_key);

    // check if contains at least nonce (first 12 bytes)
    if raw_data.len() < 13 {
        // no data
        Ok("{}".to_string())
    } else {
        // get nonce and decrypt data
        let nonce = GenericArray::clone_from_slice(&raw_data[..12]);
        let decrypted = aead
            .decrypt(&nonce, &raw_data[12..])
            .or_else(|_| Fail::from("could not decrypt secure storage data"))?;

        // decrypted to string
        let data = String::from_utf8(decrypted).or_else(Fail::from)?;
        Ok(data)
    }
}

/// Encrypt secure storage
pub fn encrypt(data: impl AsRef<[u8]>, raw_key: impl AsRef<[u8]>) -> Result<Vec<u8>, Fail> {
    // init
    let aead = init_aes(raw_key);
    let mut rng = thread_rng();

    // generate random nonce
    let mut raw_data: Vec<u8> = (0..12).map(|_| rng.gen()).collect();
    let nonce = GenericArray::clone_from_slice(&raw_data);

    // encrypt data
    let mut encrypted = aead
        .encrypt(&nonce, data.as_ref())
        .or_else(|_| Fail::from("could not encrypt secure storage data"))?;

    // add encrypted and return
    raw_data.append(&mut encrypted);
    Ok(raw_data)
}
