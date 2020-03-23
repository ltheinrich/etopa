//! Web handler

use crate::http::{read_header, respond, HttpRequest};
use rustls::{ServerConfig, ServerSession, Stream};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use std::thread;

// Accept connections
pub fn accept_connections(listener: Arc<RwLock<TcpListener>>, tls_config: Arc<ServerConfig>) {
    loop {
        // accept connection
        if let Ok((stream, _)) = listener.read().unwrap().accept() {
            // spawn new thread
            let tls_config = tls_config.clone();
            thread::spawn(move || {
                // handle connection
                handle_connection(stream, tls_config);
            });
        }
    }
}

// Handle connection
fn handle_connection(mut stream: TcpStream, tls_config: Arc<ServerConfig>) {
    let mut session = ServerSession::new(&tls_config);
    let mut stream = Stream::new(&mut session, &mut stream);

    // read header
    if let Ok((header, rest)) = read_header(&mut stream) {
        // parse HTTP request
        let http_request = match HttpRequest::from(&header, rest, &mut stream) {
            Ok(http_request) => http_request,
            Err(err) => {
                // error
                return respond(&mut stream, err.to_string().as_bytes(), "text/html").unwrap();
            }
        };

        // match URL
        match &http_request.url()[1..] {
            /*
            "favicon.ico" => respond(&mut stream, FAVICON_ICO, "image/x-icon").unwrap(),
            "favicon.png" => respond(&mut stream, FAVICON_PNG, "image/png").unwrap(),
            "apple-touch-icon.png" => {
                respond(&mut stream, APPLE_TOUCH_ICON, "image/png").unwrap()
            }
            "bootstrap.min.css" => respond(&mut stream, BOOTSTRAP, "text/css").unwrap(),
            "style.css" => respond(&mut stream, STYLE, "text/css").unwrap(),
            */
            _ => {
                respond(
                    &mut stream,
                    b"Nothing here, this is an index page!",
                    "text/plain",
                )
                .unwrap();
            }
        }
    }
}
