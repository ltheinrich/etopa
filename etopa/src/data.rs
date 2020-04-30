//! Database

use kern::Fail;
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::fs::{remove_file, rename, File, OpenOptions};
use std::io::prelude::*;
use std::string::ToString;

/// Raw data storage file
#[derive(Debug)]
pub struct StorageFile {
    file: File,
    cache: BTreeMap<String, String>,
}

impl StorageFile {
    /// Open file or create new
    pub fn new(file_name: impl AsRef<str>) -> Result<Self, Fail> {
        // open file and parse
        let mut file = open_file(file_name)?;
        let cache = parse(read_file(&mut file)?)?;

        // return
        Ok(Self { file, cache })
    }

    /// Get map from cache
    pub fn cache(&self) -> &BTreeMap<String, String> {
        &self.cache
    }

    /// Get map from cache mutably
    pub fn cache_mut(&mut self) -> &mut BTreeMap<String, String> {
        &mut self.cache
    }

    /// Serialize map to string and write to file
    pub fn write(&mut self) -> Result<(), Fail> {
        // serialize and write
        let buf = serialize(self.cache())?;
        write_file(&mut self.file, buf.as_bytes()).or_else(Fail::from)
    }
}

/// Parse storage file buf to map
pub fn parse(buf: Vec<u8>) -> Result<BTreeMap<String, String>, Fail> {
    // to string
    let buf = String::from_utf8(buf).or_else(Fail::from)?;

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

/// Serialize map to string
pub fn serialize(data: &BTreeMap<String, String>) -> Result<String, Fail> {
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
    Ok(buf)
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

/// Delete file if exists
pub fn delete_file(file_name: impl AsRef<str>) -> Result<(), Fail> {
    // delete file
    remove_file(file_name.as_ref()).or_else(Fail::from)
}

/// Move file
pub fn move_file(file_name: impl AsRef<str>, new_file_name: impl AsRef<str>) -> Result<(), Fail> {
    // delete file
    rename(file_name.as_ref(), new_file_name.as_ref()).or_else(Fail::from)
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
