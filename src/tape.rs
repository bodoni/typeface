use std::io::{Read, Seek, SeekFrom};

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

/// A type that can be read.
pub trait Value: Sized {
    /// Read a value.
    fn read<T: Tape>(tape: &mut T) -> Result<Self>;
}

/// A type that can be read given a parameter.
pub trait Walue<'l>: Sized {
    /// The parameter type.
    type Parameter;

    /// Read a value.
    fn read<T: Tape>(tape: &mut T, parameter: Self::Parameter) -> Result<Self>;
}

impl<T: Read + Seek> Tape for T {}

macro_rules! read {
    ($tape:ident, $size:expr) => {
        unsafe {
            let mut buffer: [u8; $size] = ::std::mem::zeroed();
            ::std::io::Read::read_exact($tape, &mut buffer)?;
            #[allow(clippy::useless_transmute)]
            ::std::mem::transmute(buffer)
        }
    };
}

macro_rules! implement {
    ([$kind:ident; $count:expr], 1) => {
        impl Value for [$kind; $count] {
            #[inline]
            fn read<T: Tape>(tape: &mut T) -> Result<Self> {
                Ok(read!(tape, $count))
            }
        }
    };
    ($kind:ident, 1) => {
        impl Value for $kind {
            #[inline]
            fn read<T: Tape>(tape: &mut T) -> Result<Self> {
                Ok(read!(tape, 1))
            }
        }
    };
    ($kind:ident, $size:expr) => {
        impl Value for $kind {
            #[inline]
            fn read<T: Tape>(tape: &mut T) -> Result<Self> {
                Ok($kind::from_be(read!(tape, $size)))
            }
        }
    };
}

implement!(i8, 1);
implement!(u8, 1);
implement!(i16, 2);
implement!(u16, 2);
implement!(i32, 4);
implement!(u32, 4);
implement!(i64, 8);
implement!([u8; 3], 1);
implement!([i8; 4], 1);
implement!([u8; 4], 1);
implement!([u8; 10], 1);

impl<U, V> Value for (U, V)
where
    U: Value,
    V: Value,
{
    #[inline]
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok((tape.take()?, tape.take()?))
    }
}

impl<V> Walue<'static> for Vec<V>
where
    V: Value,
{
    type Parameter = usize;

    fn read<T: Tape>(tape: &mut T, count: usize) -> Result<Self> {
        let mut values = Vec::with_capacity(count);
        for _ in 0..count {
            values.push(Value::read(tape)?);
        }
        Ok(values)
    }
}
