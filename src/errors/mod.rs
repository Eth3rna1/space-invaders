//! Error Handling for the ASCII Engine
//!
//! This module defines the error types used throughout the engine crate.
//! It provides a lightweight, structured way to categorize and propagate
//! errors encountered during sprite manipulation, rendering, and engine logic.
//!
//! # Components
//!
//! - [`ErrorKind`]: Enumerates common categories of errors, such as boundary
//!   violations or invalid sprite operations.
//! - [`Error`]: A simple, cloneable error struct containing an [`ErrorKind`]
//!   and a human-readable diagnosis message.
//!
//! # Usage
//! Errors are constructed using the `Error::new()` function, where an `ErrorKind`
//! and a diagnostic message are provided. This enables meaningful error messages
//! that can be logged or displayed to users.
//!
//! ```rust
//! use crate::errors::{Error, ErrorKind};
//!
//! let err = Error::new(ErrorKind::OutOfBounds, "Tried to move beyond grid limits");
//! println!("{}", err); // Prints: (OutOfBounds, Tried to move beyond grid limits)
//! ```
//!
//! # Integration
//! The `Error` type is used throughout the `engine`, `sprite`, and `bounding_box`
//! modules to enforce and describe invalid state changes. Since it implements
//! `Display`, it's suitable for use in logs or CLI applications.
//!
//! # Notes
//! - The `Error` type is intentionally simple and avoids lifetimes or
//!   dynamic traits for ease of use and cloning.
//! - This is not a replacement for `Result<T, Box<dyn std::error::Error>>`, but
//!   a domain-specific error type tailored for this crate's logic.
use std::convert::AsRef;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    OutOfBounds,
    InexistentSprite,
    InexistentCoordinate,
    OverlappingSprite,
    Other,
}

#[derive(Debug, Clone)]
pub struct Error {
    kind: ErrorKind,
    diagnosis: String,
}

impl Error {
    pub fn new<T: AsRef<str> + ToString>(kind: ErrorKind, diagnosis: T) -> Self {
        Self {
            kind,
            diagnosis: diagnosis.to_string(),
        }
    }

    /// Returns the error kind
    pub fn kind(&self) -> ErrorKind {
        self.kind.clone()
    }

    /// Returns the string diagnosis given
    pub fn diagnosis(&self) -> String {
        self.diagnosis.clone()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?}, {})", &self.kind, &self.diagnosis)
    }
}
