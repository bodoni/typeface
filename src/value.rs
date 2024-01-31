//! Types that can be read and written.

use crate::Result;

/// A type that can be read.
pub trait Read: Sized {
    /// Read a value.
    fn read<T: crate::tape::Read>(_: &mut T) -> Result<Self>;
}

macro_rules! read {
    ($tape:ident, $size:expr) => {
        unsafe {
            let mut buffer: [u8; $size] = std::mem::zeroed();
            std::io::Read::read_exact($tape, &mut buffer)?;
            #[allow(clippy::useless_transmute)]
            std::mem::transmute(buffer)
        }
    };
}

macro_rules! implement {
    ([$kind:ident; $count:expr], 1) => {
        impl Read for [$kind; $count] {
            #[inline]
            fn read<T: crate::tape::Read>(tape: &mut T) -> Result<Self> {
                Ok(read!(tape, $count))
            }
        }
    };
    ($kind:ident, 1) => {
        impl Read for $kind {
            #[inline]
            fn read<T: crate::tape::Read>(tape: &mut T) -> Result<Self> {
                Ok(read!(tape, 1))
            }
        }
    };
    ($kind:ident, $size:expr) => {
        impl Read for $kind {
            #[inline]
            fn read<T: crate::tape::Read>(tape: &mut T) -> Result<Self> {
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
