//! Type definitions

use rustls::{ServerSession, Stream as RustlsStream};
use std::net::TcpStream;

pub type Stream<'a> = RustlsStream<'a, ServerSession, TcpStream>;
