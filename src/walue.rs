//! Types that can be read and written given a parameter.

use crate::Result;

/// A type that can be read given a parameter.
pub trait Read<'l>: Sized {
    /// The parameter type.
    type Parameter;

    /// Read a value.
    fn read<T: crate::tape::Read>(_: &mut T, _: Self::Parameter) -> Result<Self>;
}

/// A type that can be written given a parameter.
pub trait Write<'l> {
    /// The parameter type.
    type Parameter;

    /// Write the value.
    fn write<T: crate::tape::Write>(&self, _: &mut T, _: Self::Parameter) -> Result<()>;
}

impl<V> Read<'static> for Vec<V>
where
    V: crate::value::Read,
{
    type Parameter = usize;

    fn read<T: crate::tape::Read>(tape: &mut T, count: usize) -> Result<Self> {
        let mut values = Vec::with_capacity(count);
        for _ in 0..count {
            values.push(crate::value::Read::read(tape)?);
        }
        Ok(values)
    }
}
