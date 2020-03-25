//! Etopa library

pub mod common;
pub mod data;
pub mod totp;

use std::error::Error;
use std::fmt;

/// Custom Error type
#[derive(Clone, Debug)]
pub struct Fail(String);

// Fail implementation
impl Fail {
    /// Create Error from any Display
    pub fn new<E>(err: E) -> Self
    where
        E: fmt::Display,
    {
        Self(err.to_string())
    }

    /// Create Result with Error from any Display
    pub fn from<T, E>(err: E) -> Result<T, Self>
    where
        E: fmt::Display,
    {
        Err(Self::new(err))
    }

    /// Get error message
    pub fn err_msg(&self) -> &str {
        &self.0
    }
}

/// Display implementation for Fail
impl fmt::Display for Fail {
    // fmt implementation
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// Error implementation for Fail
impl Error for Fail {}
