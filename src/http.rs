//! HTTP parsing

use crate::{common::*, version};
use kern::Error;
use std::collections::BTreeMap;
use std::io;
use std::io::prelude::{Read, Write};

/// HTTP request method (GET or POST)
#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
}

/// HTTP request structure
#[derive(Debug)]
pub struct HttpRequest<'a> {
    method: HttpMethod,
    url: &'a str,
    headers: BTreeMap<&'a str, &'a str>,
    get: BTreeMap<&'a str, &'a str>,
    body: String,
}

// HTTP request implementation
impl<'a> HttpRequest<'a> {
    pub fn from(
        raw_header: &'a str,
        mut raw_body: Vec<u8>,
        stream: &mut Stream,
    ) -> Result<Self, Error> {
        // split header
        let mut header = raw_header.lines();
        let mut reqln = header
            .next()
            .ok_or_else(|| Error::new("empty header"))?
            .split(' ');

        // parse method
        let method = if reqln
            .next()
            .ok_or_else(|| Error::new("no method in header"))?
            == "POST"
        {
            HttpMethod::POST
        } else {
            HttpMethod::GET
        };

        // parse url and split raw get parameters
        let mut get_raw = "";
        let url = if let Some(full_url) = reqln.next() {
            let mut split_url = full_url.splitn(2, '?');
            let url = split_url
                .next()
                .ok_or_else(|| Error::new("no url in header"))?;
            if let Some(params) = split_url.next() {
                get_raw = params;
            }
            url
        } else {
            "/"
        };

        // parse headers
        let mut headers = BTreeMap::new();
        header.for_each(|hl| {
            let mut hls = hl.splitn(2, ':');
            if let (Some(key), Some(value)) = (hls.next(), hls.next()) {
                headers.insert(key.trim(), value.trim());
            }
        });

        // set time out
        /* TODO stream
        .set_read_timeout(Some(std::time::Duration::from_millis(2000)))
        .ok()
        .ok_or_else(|| Error::new("can not set timeout"))?;*/

        // get content length
        let buf_len = if let Some(buf_len) = headers.get("Content-Length") {
            Some(buf_len)
        } else {
            headers.get("content-length")
        };

        // check max log size and read body
        let mut body = String::new();
        if let Some(buf_len) = buf_len {
            // parse buffer length
            let con_len = buf_len
                .parse::<usize>()
                .ok()
                .ok_or_else(|| Error::new("content-length is not of type usize"))?;
            // read body
            while raw_body.len() < con_len {
                let mut rest_body = vec![0u8; 65536];
                let length = stream
                    .read(&mut rest_body)
                    .ok()
                    .ok_or_else(|| Error::new("stream broken"))?;
                rest_body.truncate(length);
                raw_body.append(&mut rest_body);
            }
            body = String::from_utf8(raw_body)
                .ok()
                .ok_or_else(|| Error::new("body is not utf-8"))?;
        }

        // parse GET parameters and return
        let get =
            parse_parameters(get_raw).ok_or_else(|| Error::new("can not parse get parameters"))?;
        Ok(Self {
            method,
            url,
            headers,
            get,
            body,
        })
    }

    /* unused
    /// Parse POST parameters
    pub fn post(&self) -> Option<BTreeMap<&str, &str>> {
        // check if POST method used
        if self.method == HttpMethod::POST {
            // parse POST parameters
            parse_upload(&self.body)
        } else {
            // no POST request: return empty map
            Some(BTreeMap::new())
        }
    }
    */

    /* unused
    /// Get HTTP request method
    pub fn method(&self) -> &HttpMethod {
        // return HTTP request method
        &self.method
    }
    */

    /// Get URL
    pub fn url(&self) -> &str {
        // return URL
        self.url
    }

    /* unused
    /// Get headers map
    pub fn headers(&self) -> &BTreeMap<&str, &str> {
        // return headers map
        &self.headers
    }
    */

    /* unused
    /// Get GET parameters
    pub fn get(&self) -> &BTreeMap<&str, &str> {
        // return GET parameters map
        &self.get
    }
    */
}

/* unused
// Parse POST file upload with parameters to map
fn parse_upload(body: &str) -> Option<BTreeMap<&str, &str>> {
    // parameters map
    let mut params = BTreeMap::new();

    // split file upload body into sections
    for content in body.split("\r\n---") {
        // split lines (max 4)
        let mut lines = content.splitn(4, "\r\n").skip(1);
        let mut name = "";

        // split in phrases
        for line in lines.next()?.split(';').map(|line| line.trim()) {
            // check if phrase contains name
            if line.starts_with("name=") {
                if line.len() > 6 {
                    // get name
                    name = &line[6..(line.len() - 1)];
                    break;
                } else {
                    // no name
                    return None;
                }
            }
        }

        // get next line
        if let Some(value) = lines.next() {
            // check for empty line
            if value == "" {
                // add next line to parameters map
                params.insert(name, lines.next()?);
            } else {
                // ignore first empty line and add second line to parameters map
                let mut a = lines.next()?.splitn(2, "\r\n");
                params.insert(name, a.nth(1)?);
            }
        }
    }

    // return parameters map
    Some(params)
}
*/

// Parse GET parameters to map
fn parse_parameters(raw: &str) -> Option<BTreeMap<&str, &str>> {
    // parameters map
    let mut params = BTreeMap::new();

    // split parameters by ampersand
    for p in raw.split('&') {
        // split key and value and add to map
        let mut ps = p.splitn(2, '=');
        params.insert(
            ps.next()?.trim(), // trimmed key
            if let Some(value) = ps.next() {
                value.trim() // trimmed value
            } else {
                "" // no value, is option
            },
        );
    }

    // return parameters map
    Some(params)
}

/// HTTP responder
pub fn respond(stream: &mut Stream, content: &[u8], content_type: &str) -> io::Result<()> {
    // write headers to stream
    stream
        .write_all(format!(
            "HTTP/1.1 200 OK\r\nServer: ltheinrich.de etopa/{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
            version(),
            content_type,
            content.len() + 2, // bugfix (proxying)
        )
        .as_bytes())?;

    // write body and end
    stream.write_all(content)?;
    stream.write_all(b"\r\n")?;
    stream.flush()
}

/* unused
/// HTTP redirecter
pub fn redirect(stream: &mut TcpStream, url: &str) -> io::Result<()> {
    // write redirect headers and simple body
    stream.wa(format!(
        "HTTP/1.1 303 See Other\r\nServer: ltheinrich.de etopa/{}\r\nLocation: {1}\r\n\r\n<html><head><title>Moved</title></head><body><h1>Moved</h1><p><a href=\"{1}\">{1}</a></p></body></html>\r\n",
        version(),
        url
    )
    .as_bytes())
}
*/

/// Read until \r\n\r\n (just working, uncommented)
pub fn read_header(stream: &mut Stream) -> Result<(String, Vec<u8>), Error> {
    let mut header = Vec::new();
    let mut rest = Vec::new();
    let mut buf = vec![0u8; 8192];

    'l: loop {
        let length = match stream.read(&mut buf) {
            Ok(length) => length,
            Err(err) => return Error::from(err),
        };
        for (i, &c) in buf.iter().enumerate() {
            if c == b'\r' {
                if buf.len() < i + 4 {
                    let mut buf_temp = vec![0u8; buf.len() - (i + 4)];
                    match stream.read(&mut buf_temp) {
                        Ok(_) => {}
                        Err(err) => return Error::from(err),
                    };
                    let buf2 = [&buf[..], &buf_temp[..]].concat();
                    if buf2[i + 1] == b'\n' && buf2[i + 2] == b'\r' && buf2[i + 3] == b'\n' {
                        header.append(&mut buf);
                        header.append(&mut buf_temp);
                        break 'l;
                    }
                } else if buf[i + 1] == b'\n' && buf[i + 2] == b'\r' && buf[i + 3] == b'\n' {
                    for &b in buf.iter().take(i + 4) {
                        header.push(b);
                    }
                    for &b in buf.iter().take(length).skip(i + 4) {
                        rest.push(b);
                    }
                    break 'l;
                } else if i + 1 == buf.len() {
                    for &b in buf.iter().take(i + 4) {
                        header.push(b);
                    }
                    for &b in buf.iter().take(length).skip(i + 4) {
                        rest.push(b);
                    }
                }
            }
        }
    }
    Ok((
        match String::from_utf8(header) {
            Ok(header) => header,
            Err(err) => return Error::from(err),
        },
        rest,
    ))
}
