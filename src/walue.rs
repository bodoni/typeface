use crate::tape::Tape;
use crate::value::Value;
use crate::Result;

/// A type that can be read given a parameter.
pub trait Walue<'l>: Sized {
    /// The parameter type.
    type Parameter;

    /// Read a value.
    fn read<T: Tape>(_: &mut T, _: Self::Parameter) -> Result<Self> {
        crate::error!("not implemented yet")
    }

    /// Write the value.
    fn write<T: Tape>(&self, _: &mut T, _: Self::Parameter) -> Result<()> {
        crate::error!("not implemented yet")
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
