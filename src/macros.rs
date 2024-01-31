/// Implement choices.
#[macro_export]
macro_rules! choices {
    ($(#[$attribute:meta])* pub $name:ident($type:ty) {
        $($value:expr => $variant:ident,)*
        _ => $other:ident,
    }) => (
        $(#[$attribute])*
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
        pub enum $name {
            #[default]
            $($variant,)*
            $other($type),
        }

        impl From<$name> for $type {
            fn from(value: $name) -> $type {
                match value {
                    $($name::$variant => $value,)*
                    $name::$other(value) => value,
                }
            }
        }

        impl From<$type> for $name {
            fn from(value: $type) -> $name {
                match value {
                    $($value => $name::$variant,)*
                    value => $name::$other(value),
                }
            }
        }

        impl $crate::value::Read for $name {
            fn read<T: $crate::tape::Read>(tape: &mut T) -> $crate::Result<Self> {
                match tape.take::<$type>()? {
                    $($value => Ok($name::$variant),)*
                    value => Ok($name::$other(value)),
                }
            }
        }
    );
    ($(#[$attribute:meta])* pub $name:ident($type:ty) {
        $($value:expr => $variant:ident,)*
    }) => (
        $(#[$attribute])*
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
        pub enum $name {
            #[default]
            $($variant = $value,)*
        }

        impl From<$name> for $type {
            #[inline]
            fn from(value: $name) -> $type {
                value as $type
            }
        }

        impl TryFrom<$type> for $name {
            type Error = $crate::Error;

            #[inline]
            fn try_from(value: $type) -> $crate::Result<$name> {
                match value {
                    $($value => Ok($name::$variant),)*
                    value => $crate::raise!(
                        concat!(
                            "found a malformed field of type ",
                            stringify!($name),
                            " with value {:?}",
                        ),
                        value,
                    ),
                }
            }
        }

        impl $crate::value::Read for $name {
            fn read<T: $crate::tape::Read>(tape: &mut T) -> $crate::Result<Self> {
                match tape.take::<$type>()? {
                    $($value => Ok($name::$variant),)*
                    value => $crate::raise!(
                        concat!(
                            "found a malformed field of type ",
                            stringify!($name),
                            " with value {:?}",
                        ),
                        value,
                    ),
                }
            }
        }
    );
    ($(#[$attribute:meta])* pub $name:ident($type:tt) {
        $($value:expr => $variant:ident($string:expr),)*
    }) => (
        $(#[$attribute])*
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
        pub enum $name {
            #[default]
            $($variant = $value,)*
        }

        impl From<$name> for $type {
            #[inline]
            fn from(value: $name) -> Self {
                value as $type
            }
        }

        impl From<$name> for &'static str {
            #[inline]
            fn from(value: $name) -> Self {
                match value {
                    $($name::$variant => $string,)*
                }
            }
        }

        impl TryFrom<$type> for $name {
            type Error = $crate::Error;

            fn try_from(value: $type) -> $crate::Result<$name> {
                match value {
                    $($value => Ok($name::$variant),)*
                    value => $crate::raise!(
                        concat!(
                            "found a malformed field of type ",
                            stringify!($name),
                            " with value {:?}",
                        ),
                        value,
                    ),
                }
            }
        }

        impl $crate::value::Read for $name {
            fn read<T: $crate::tape::Read>(tape: &mut T) -> $crate::Result<Self> {
                match tape.take::<$type>()? {
                    $($value => Ok($name::$variant),)*
                    value => $crate::raise!(
                        concat!(
                            "found a malformed field of type ",
                            stringify!($name),
                            " with value {:?}",
                        ),
                        value,
                    ),
                }
            }
        }
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! dereference {
    (@itemize $($one:item)*) => ($($one)*);
    ($name:ident::$field:tt => $target:ty) => (dereference! {
        @itemize

        impl std::ops::Deref for $name {
            type Target = $target;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }

        impl std::ops::DerefMut for $name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field
            }
        }
    });
    ($name:ident<$life:tt>::$field:tt => $target:ty) => (dereference! {
        @itemize

        impl<$life> std::ops::Deref for $name<$life> {
            type Target = $target;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }

        impl<$life> std::ops::DerefMut for $name<$life> {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field
            }
        }
    });
}

/// Create an error.
#[macro_export]
macro_rules! error {
    (@from $error:ident, $($argument:tt)*) => (
        Err(
            std::io::Error::new(
                std::io::ErrorKind::Other,
                $crate::ErrorWithSource {
                    description: format!($($argument)*),
                    source: $error,
                },
            )
        )
    );
    ($($argument:tt)*) => (
        Err(std::io::Error::new(std::io::ErrorKind::Other, format!($($argument)*)))
    );
}

/// Implement flags.
#[macro_export]
macro_rules! flags {
    ($(#[$attribute:meta])* pub $name:ident($type:ty) {
        $($value:expr => $variant:ident,)*
    }) => (
        flags!(@base $(#[$attribute])* pub $name($type) { $($value => $variant,)* });
        flags!(@read pub $name($type));
    );
    (@base $(#[$attribute:meta])* pub $name:ident($type:ty) {
        $($value:expr => $variant:ident,)*
    }) => (
        $(#[$attribute])*
        #[derive(Clone, Copy, Default, Eq, PartialEq)]
        pub struct $name(pub $type);

        impl $name {
            $(
                #[inline]
                pub fn $variant(&self) -> bool {
                    self.0 & $value > 0
                }
            )*
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, concat!(stringify!($name), "({:#b})"), self.0)
            }
        }

        impl std::fmt::Display for $name {
            #[inline]
            fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Debug::fmt(self, formatter)
            }
        }

        impl From<$name> for $type {
            #[inline]
            fn from(flags: $name) -> $type {
                flags.0
            }
        }
    );
    (@read pub $name:ident($type:ty)) => (
        impl $crate::value::Read for $name {
            #[inline]
            fn read<T: $crate::tape::Read>(tape: &mut T) -> $crate::Result<Self> {
                let value = $name(tape.take::<$type>()?);
                if value.is_invalid() {
                    $crate::raise!(
                        concat!(
                            "found a malformed field of type ",
                            stringify!($name),
                            " with value {:?}",
                        ),
                        value,
                    );
                }
                Ok(value)
            }
        }
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! jump_take {
    (@unwrap $tape:ident, $position:ident, $offset:expr) => ({
        $tape.jump($position + $offset as u64)?;
        $tape.take()?
    });
    (@unwrap $tape:ident, $position:ident, $count:expr, $offsets:expr) => (
        jump_take!(@unwrap $tape, $position, $count, i => $offsets[i])
    );
    (@unwrap $tape:ident, $position:ident, $count:expr, $i:ident => $iterator:expr) => ({
        let mut values = Vec::with_capacity($count as usize);
        for $i in 0..($count as usize) {
            $tape.jump($position + $iterator as u64)?;
            values.push($tape.take()?);
        }
        values
    });
    ($tape:ident, $position:ident, $offset:expr) => (
        Ok(jump_take!(@unwrap $tape, $position, $offset))
    );
    ($tape:ident, $position:ident, $count:expr, $offsets:expr) => (
        Ok(jump_take!(@unwrap $tape, $position, $count, i => $offsets[i]))
    );
    ($tape:ident, $position:ident, $count:expr, $i:ident => $iterator:expr) => (
        Ok(jump_take!(@unwrap $tape, $position, $count, $i => $iterator))
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! jump_take_given {
    (@unwrap $tape:ident, $position:ident, $offset:expr, $parameter:expr) => ({
        $tape.jump($position + $offset as u64)?;
        $tape.take_given($parameter)?
    });
    (@unwrap $tape:ident, $position:ident, $count:expr, $offsets:expr, $parameter:expr) => (
        jump_take_given!(@unwrap $tape, $position, $count, i => $offsets[i], $parameter)
    );
    (@unwrap $tape:ident, $position:ident, $count:expr, $i:ident => $iterator:expr,
     $parameter:expr) => ({
        let mut values = Vec::with_capacity($count as usize);
        for $i in 0..($count as usize) {
            $tape.jump($position + $iterator as u64)?;
            values.push($tape.take_given($parameter)?);
        }
        values
    });
    ($tape:ident, $position:ident, $offset:expr, $parameter:expr) => (
        Ok(jump_take_given!(@unwrap $tape, $position, $offset, $parameter))
    );
    ($tape:ident, $position:ident, $count:expr, $offsets:expr, $parameter:expr) => (
        Ok(jump_take_given!(@unwrap $tape, $position, $count, i => $offsets[i], $parameter))
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! jump_take_maybe {
    (@unwrap $tape:ident, $position:ident, $offset:expr) => (
        if $offset > 0 {
            $tape.jump($position + $offset as u64)?;
            Some($tape.take()?)
        } else {
            None
        }
    );
    (@unwrap $tape:ident, $position:ident, $count:expr, $i:ident => $iterator:expr) => ({
        let mut values = Vec::with_capacity($count as usize);
        for $i in 0..($count as usize) {
            if $iterator > 0 {
                $tape.jump($position + $iterator as u64)?;
                values.push(Some($tape.take()?));
            } else {
                values.push(None);
            }
        }
        values
    });
    ($tape:ident, $position:ident, $offset:expr) => (
        Ok(jump_take_maybe!(@unwrap $tape, $position, $offset))
    );
    ($tape:ident, $position:ident, $count:expr, $offsets:expr) => (
        Ok(jump_take_maybe!(@unwrap $tape, $position, $count, i => $offsets[i]))
    );
}

/// Raise an exception.
#[macro_export]
macro_rules! raise {
    ($($argument:tt)*) => ($crate::error!($($argument)*)?);
}

/// Implement a table.
#[macro_export]
macro_rules! table {
    ($(#[$attribute:meta])* pub $name:ident {
        $($field:ident ($($type:tt)+) $(= $value:block)* $(|$($argument:tt),+| $body:block)*,)*
    }) => (
        table! {
            @define
            $(#[$attribute])* pub $name { $($field ($($type)+),)* }
        }
        table! {
            @read
            pub $name { $($field ($($type)+) [$($value)*] $(|$($argument),+| $body)*,)* }
        }
    );
    (@position $(#[$attribute:meta])* pub $name:ident {
        $($field:ident ($($type:tt)+) $(= $value:block)* $(|$($argument:tt),+| $body:block)*,)*
    }) => (
        table! {
            @define
            $(#[$attribute])* pub $name { $($field ($($type)+),)* }
        }
        table! {
            @read @position
            pub $name { $($field ($($type)+) [$($value)*] $(|$($argument),+| $body)*,)* }
        }
    );
    (@write $(#[$attribute:meta])* pub $name:ident {
        $($field:ident ($($type:tt)+) $(= $value:block)* $(|$($argument:tt),+| $body:block)*,)*
    }) => (
        table! {
            @define
            $(#[$attribute])* pub $name { $($field ($($type)+),)* }
        }
        table! {
            @read
            pub $name { $($field ($($type)+) [$($value)*] $(|$($argument),+| $body)*,)* }
        }
        table! {
            @write
            pub $name { $($field ($($type)+) [],)* }
        }
    );
    (@define $(#[$attribute:meta])* pub $name:ident { $($field:ident ($type:ty),)* }) => (
        $(#[$attribute])*
        #[derive(Clone, Debug, Default)]
        pub struct $name { $(pub $field: $type,)* }
    );

    (@read pub $name:ident {
        $($field:ident ($($type:tt)+) [$($value:block)*] $(|$($argument:tt),+| $body:block)*,)*
    }) => (
        impl $crate::value::Read for $name {
            fn read<T: $crate::tape::Read>(tape: &mut T) -> $crate::Result<Self> {
                let mut table: $name = $name::default();
                $({
                    let value = table!(@read $name, table.$field, tape [] [$($type)+] [$($value)*]
                                       $(|$($argument),+| $body)*);
                    #[allow(forgetting_copy_types)]
                    std::mem::forget(std::mem::replace(&mut table.$field, value));
                })*
                Ok(table)
            }
        }
    );
    (@read @position pub $name:ident {
        $($field:ident ($($type:tt)+) [$($value:block)*] $(|$($argument:tt),+| $body:block)*,)*
    }) => (
        impl $crate::value::Read for $name {
            fn read<T: $crate::tape::Read>(tape: &mut T) -> $crate::Result<Self> {
                let position = tape.position()?;
                let mut table: $name = $name::default();
                $({
                    let value = table!(@read $name, table.$field, tape [position] [$($type)+] [$($value)*]
                                       $(|$($argument),+| $body)*);
                    #[allow(forgetting_copy_types, clippy::forget_non_drop)]
                    std::mem::forget(std::mem::replace(&mut table.$field, value));
                })*
                Ok(table)
            }
        }
    );

    (@read $name:ident, $this:ident . $field:ident, $tape:ident [$($position:tt)*] [$type:ty] []) => (
        $tape.take()?
    );
    (@read $name:ident, $this:ident . $field:ident, $tape:ident [$($position:tt)*] [$type:ty]
     [$value:block]) => ({
        let value = $tape.take()?;
        if value != $value {
            $crate::raise!(
                concat!(
                    "found a malformed field ",
                    stringify!($name), "::", stringify!($field),
                    " with value {:?} unequal to {:?}",
                ),
                value,
                $value,
            );
        }
        value
    });
    (@read $name:ident, $this:ident . $field:ident, $tape:ident [] [$type:ty] []
     |$this_:tt, $tape_:tt| $body:block) => ({
        #[inline]
        fn read<T: $crate::tape::Read>($this_: &$name, $tape_: &mut T) -> $crate::Result<$type> $body
        read(&$this, $tape)?
    });
    (@read $name:ident, $this:ident . $field:ident, $tape:ident [$position:ident] [$type:ty] []
     |$this_:tt, $tape_:tt, $position_:tt| $body:block) => ({
        #[inline]
        fn read<T: $crate::tape::Read>($this_: &$name, $tape_: &mut T, $position_: u64)
                                 -> $crate::Result<$type> $body
        read(&$this, $tape, $position)?
    });

    (@write pub $name:ident {
        $($field:ident ($($type:tt)+) [],)*
    }) => (
        impl $crate::value::Write for $name {
            fn write<T: $crate::tape::Write>(&self, _: &mut T) -> $crate::Result<()> {
                $(table!(@write $name, table.$field, tape);)*
                Ok(())
            }
        }
    );
    (@write $name:ident, $this:ident . $field:ident, $tape:ident) => (
    );
}

#[cfg(test)]
mod tests {
    table! {
        pub Read {
            major_version (u16) = { 1 },
            minor_version (u16),

            records (Vec<u16>) |_, tape| {
                tape.take_given(0)
            },
        }
    }

    table! {
        @write
        pub Write {
            major_version (u16),
            minor_version (u16),
        }
    }
}
