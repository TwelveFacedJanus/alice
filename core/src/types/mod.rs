//! This module provides some alice types.

use thiserror::Error;
use serde::{Serialize, Deserialize};

/// Default result type for alice.
pub type AliceResult<T> = std::io::Result<T>;


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum AliceErrorStatus {
    UndefinedError,
    InternalError,
    ConnectionError,
}


#[derive(Debug, Serialize, Deserialize,)]
pub struct AliceError {
    message: String,
    status: AliceErrorStatus,
}

impl AliceError {
    pub fn new(message: &str, status: AliceErrorStatus) -> AliceResult<Self> {
        Ok(Self {
            message: message.to_string(),
            status,
        })
    }
}

impl std::fmt::Display for AliceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
