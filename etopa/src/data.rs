//! Data handling

use kern::Fail;
use std::collections::BTreeMap;

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
