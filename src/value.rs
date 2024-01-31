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
        let mut maybe = std::mem::MaybeUninit::<[u8; $size]>::uninit();
        let buffer = unsafe { &mut *maybe.as_mut_ptr() };
        std::io::Read::read_exact($tape, buffer)?;
        unsafe { maybe.assume_init() }
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

        impl Write for [u8; $count] {
            #[inline]
            fn write<T: crate::tape::Write>(&self, tape: &mut T) -> Result<()> {
                tape.write_all(self)
            }
        }
    };
    ([$type:ident; $count:expr]) => {
        impl Read for [$type; $count] {
            #[inline]
            fn read<T: crate::tape::Read>(tape: &mut T) -> Result<Self> {
                let value = read!(tape, $count);
                Ok(unsafe { std::mem::transmute(value) })
            }
        }

        impl Write for [$type; $count] {
            #[inline]
            fn write<T: crate::tape::Write>(&self, tape: &mut T) -> Result<()> {
                let value: [u8; $count] = unsafe { std::mem::transmute(*self) };
                value.write(tape)
            }
        }
    };
    ($type:ident, $size:expr) => {
        impl Read for $type {
            #[inline]
            fn read<T: crate::tape::Read>(tape: &mut T) -> Result<Self> {
                Ok($type::from_be_bytes(read!(tape, $size)))
            }
        }

        impl Write for $type {
            #[inline]
            fn write<T: crate::tape::Write>(&self, tape: &mut T) -> Result<()> {
                let value = self.to_be_bytes();
                tape.write_all(&value)
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

macro_rules! implement {
    ($($type:ident),*) => {
        impl<$($type),*> Read for ($($type),*)
        where
            $($type: Read,)*
        {
            #[inline]
            fn read<T: crate::tape::Read>(tape: &mut T) -> Result<Self> {
                Ok(($(tape.take::<$type>()?),*))
            }
        }
    };
}

implement!(U, V);
