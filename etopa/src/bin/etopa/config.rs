//! Configuration utils

use kern::cli::Command;

// Get cli configuration
pub fn conf_cli<'a>(
    cmd: &'a Command<'_>,
    port: &mut &'a str,
    addr: &mut &'a str,
    threads: &mut u8,
    cert: &mut &'a str,
    key: &mut &'a str,
) {
    // parse command-line options
    if let Some(v) = cmd.get_parameter("port") {
        *port = v;
    }
    if let Some(v) = cmd.get_parameter("addr") {
        *addr = v;
    }
    if let Some(v) = cmd.get_parameter("threads") {
        // parse to u8
        if let Ok(v) = v.parse() {
            *threads = v;
        }
    }
    if let Some(v) = cmd.get_parameter("cert") {
        *cert = v;
    }
    if let Some(v) = cmd.get_parameter("key") {
        *key = v;
    }
}

// Parse file config
pub fn conf_file<'a>(
    buf: &'a mut String,
    port: &mut &'a str,
    addr: &mut &'a str,
    threads: &mut u8,
    cert: &mut &'a str,
    key: &mut &'a str,
) {
    // parse file config
    buf.split('\n') // split lines
        .map(|l| l.splitn(2, '=').map(|c| c.trim()).collect()) // seperate and trim key and value
        .for_each(|kv: Vec<&str>| {
            if kv.len() == 2 {
                match kv[0] {
                    "port" => *port = kv[1],
                    "addr" => *addr = kv[1],
                    "threads" => {
                        // parse to u8
                        if let Ok(v) = kv[1].parse() {
                            *threads = v;
                        }
                    }
                    "cert" => *cert = kv[1],
                    "key" => *key = kv[1],
                    _ => {}
                }
            }
        });
}
