//! Types that can read and write.

use crate::Result;

/// A type that can read.
pub trait Read: std::io::Read + std::io::Seek + Sized {
    /// Read a value.
    #[inline]
    fn take<T: crate::value::Read>(&mut self) -> Result<T> {
        crate::value::Read::read(self)
    }

    /// Read a value given a parameter.
    #[inline]
    fn take_given<'l, T: crate::walue::Read<'l>>(&mut self, parameter: T::Parameter) -> Result<T> {
        crate::walue::Read::read(self, parameter)
    }

    #[doc(hidden)]
    #[inline]
    fn jump(&mut self, position: u64) -> Result<u64> {
        self.seek(std::io::SeekFrom::Start(position))
    }

    #[doc(hidden)]
    #[inline]
    fn peek<T: crate::value::Read>(&mut self) -> Result<T> {
        self.stay(|tape| crate::value::Read::read(tape))
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

/// A type that can write.
pub trait Write: std::io::Write + Sized {
    /// Write a value.
    #[inline]
    fn give<T: crate::value::Write>(&mut self, value: &T) -> Result<()> {
        crate::value::Write::write(value, self)
    }
}

impl<T: std::io::Read + std::io::Seek> Read for T {}

impl<T: std::io::Write> Write for T {}
