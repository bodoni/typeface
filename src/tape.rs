use std::io::{Read, Seek, SeekFrom};

use crate::value::Value;
use crate::walue::Walue;
use crate::Result;

/// A type that can read.
pub trait Tape: Read + Seek + Sized {
    /// Read a value.
    #[inline]
    fn take<T: Value>(&mut self) -> Result<T> {
        Value::read(self)
    }

    /// Read a value given a parameter.
    #[inline]
    fn take_given<'l, T: Walue<'l>>(&mut self, parameter: T::Parameter) -> Result<T> {
        Walue::read(self, parameter)
    }

    #[doc(hidden)]
    #[inline]
    fn jump(&mut self, position: u64) -> Result<u64> {
        self.seek(SeekFrom::Start(position))
    }

    #[doc(hidden)]
    #[inline]
    fn peek<T: Value>(&mut self) -> Result<T> {
        self.stay(|tape| Value::read(tape))
    }

    #[doc(hidden)]
    #[inline]
    fn position(&mut self) -> Result<u64> {
        self.stream_position()
    }

    #[doc(hidden)]
    fn stay<F, T>(&mut self, mut body: F) -> Result<T>
    where
        F: FnMut(&mut Self) -> Result<T>,
    {
        let position = self.position()?;
        let result = body(self);
        self.jump(position)?;
        result
    }

    #[doc(hidden)]
    #[inline]
    fn take_bytes(&mut self, count: usize) -> Result<Vec<u8>> {
        let mut buffer = vec![0; count];
        self.read_exact(&mut buffer)?;
        Ok(buffer)
    }
}

impl<T: Read + Seek> Tape for T {}
