use crate::tape::Tape;
use crate::Result;

/// A type that can be read.
pub trait Value: Sized {
    /// Read a value.
    fn read<T: Tape>(_: &mut T) -> Result<Self> {
        crate::error!("not implemented yet")
    }

    /// Write the value.
    fn write<T: Tape>(&self, _: &mut T) -> Result<()> {
        crate::error!("not implemented yet")
    }
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
