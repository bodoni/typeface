//! Foundation for parsing fonts.

#[macro_use]
mod macros;
mod tape;

pub use tape::{Tape, Value, Walue};

/// An error.
pub type Error = std::io::Error;

/// An error caused by another error.
#[derive(Debug)]
pub struct ErrorWithSource {
    pub description: String,
    pub source: Error,
}

/// A result.
pub type Result<T> = std::io::Result<T>;

impl std::fmt::Display for ErrorWithSource {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}, due to {}", self.description, self.source)
    }
}

impl std::error::Error for ErrorWithSource {
    #[inline]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}
