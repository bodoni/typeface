//! Types that can be read and written.

use crate::Result;

/// A type that can be read.
pub trait Read: Sized {
    /// Read a value.
    fn read<T: crate::tape::Read>(_: &mut T) -> Result<Self>;
}

/// A type that can be written.
pub trait Write: Sized {
    /// Write the value.
    fn write<T: crate::tape::Write>(&self, _: &mut T) -> Result<()>;
}

macro_rules! read {
    ($tape:ident, $size:expr) => {{
        let mut buffer: [u8; $size] = unsafe { std::mem::zeroed() };
        std::io::Read::read_exact($tape, &mut buffer)?;
        buffer
    }};
}

macro_rules! implement {
    ([u8; $count:expr]) => {
        impl Read for [u8; $count] {
            #[inline]
            fn read<T: crate::tape::Read>(tape: &mut T) -> Result<Self> {
                Ok(read!(tape, $count))
            }
        }
    };
    ([$kind:ident; $count:expr]) => {
        impl Read for [$kind; $count] {
            #[inline]
            fn read<T: crate::tape::Read>(tape: &mut T) -> Result<Self> {
                let value = read!(tape, $count);
                Ok(unsafe { std::mem::transmute(value) })
            }
        }
    };
    ($kind:ident, $size:expr) => {
        impl Read for $kind {
            #[inline]
            fn read<T: crate::tape::Read>(tape: &mut T) -> Result<Self> {
                Ok($kind::from_be_bytes(read!(tape, $size)))
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
implement!([u8; 3]);
implement!([i8; 4]);
implement!([u8; 4]);
implement!([u8; 10]);

impl<U, V> Read for (U, V)
where
    U: Read,
    V: Read,
{
    #[inline]
    fn read<T: crate::tape::Read>(tape: &mut T) -> Result<Self> {
        Ok((tape.take()?, tape.take()?))
    }
}
