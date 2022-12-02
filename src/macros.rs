#[doc(hidden)]
#[macro_export]
macro_rules! deref {
    (@itemize $($one:item)*) => ($($one)*);
    ($name:ident::$field:tt => $target:ty) => (deref! {
        @itemize

        impl ::std::ops::Deref for $name {
            type Target = $target;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }

        impl ::std::ops::DerefMut for $name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field
            }
        }
    });
    ($name:ident<$life:tt>::$field:tt => $target:ty) => (deref! {
        @itemize

        impl<$life> ::std::ops::Deref for $name<$life> {
            type Target = $target;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }

        impl<$life> ::std::ops::DerefMut for $name<$life> {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field
            }
        }
    });
}

/// Implement flags.
#[macro_export]
macro_rules! flags {
    ($(#[$attribute:meta])* pub $name:ident($kind:ident) {
        $($mask:expr => $method:ident,)*
    }) => (
        $(#[$attribute])*
        #[derive(Clone, Copy, Default, Eq, PartialEq)]
        pub struct $name(pub $kind);

        impl $name {
            $(
                #[inline(always)]
                pub fn $method(&self) -> bool {
                    self.0 & $mask > 0
                }
            )*
        }

        impl ::typeface::Value for $name {
            #[inline(always)]
            fn read<T: ::typeface::Tape>(tape: &mut T) -> ::typeface::Result<Self> {
                let value = $name(tape.take::<$kind>()?);
                if cfg!(not(feature = "ignore-invalid-flags")) {
                    if value.is_invalid() {
                        raise!("found malformed flags with value {}", value);
                    }
                }
                Ok(value)
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(formatter, concat!(stringify!($name), "({:#b})"), self.0)
            }
        }

        impl ::std::fmt::Display for $name {
            #[inline]
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Debug::fmt(self, formatter)
            }
        }

        impl From<$name> for $kind {
            #[inline(always)]
            fn from(flags: $name) -> $kind {
                flags.0
            }
        }
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! jump_take(
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
);

#[doc(hidden)]
#[macro_export]
macro_rules! jump_take_given(
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
);

#[doc(hidden)]
#[macro_export]
macro_rules! jump_take_maybe(
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
);

/// Raise an exception.
#[macro_export]
macro_rules! raise(
    (@from $error:ident, $($argument:tt)*) => (
        return Err(
            ::typeface::Error::new(
                ::std::io::ErrorKind::Other,
                ::typeface::ErrorWithSource {
                    description: format!($($argument)*),
                    source: $error,
                },
            )
        )
    );
    ($($argument:tt)*) => (
        return Err(
            ::typeface::Error::new(
                ::std::io::ErrorKind::Other,
                format!($($argument)*),
            )
        )
    );
);

/// Implement a table.
#[macro_export]
macro_rules! table {
    ($(#[$attribute:meta])* pub $name:ident {
        $($field:ident ($($kind:tt)+) $(= $value:block)* $(|$($argument:tt),+| $body:block)*,)*
    }) => (
        table! { @define $(#[$attribute])* pub $name { $($field ($($kind)+),)* } }
        table! {
            @implement
            pub $name { $($field ($($kind)+) [$($value)*] $(|$($argument),+| $body)*,)* }
        }
    );
    (@position $(#[$attribute:meta])* pub $name:ident {
        $($field:ident ($($kind:tt)+) $(= $value:block)* $(|$($argument:tt),+| $body:block)*,)*
    }) => (
        table! { @define $(#[$attribute])* pub $name { $($field ($($kind)+),)* } }
        table! {
            @implement @position
            pub $name { $($field ($($kind)+) [$($value)*] $(|$($argument),+| $body)*,)* }
        }
    );
    (@define $(#[$attribute:meta])* pub $name:ident { $($field:ident ($kind:ty),)* }) => (
        $(#[$attribute])*
        #[derive(Clone, Debug, Default)]
        pub struct $name { $(pub $field: $kind,)* }
    );
    (@implement pub $name:ident {
        $($field:ident ($($kind:tt)+) [$($value:block)*] $(|$($argument:tt),+| $body:block)*,)*
    }) => (
        impl ::typeface::Value for $name {
            fn read<T: ::typeface::Tape>(tape: &mut T) -> ::typeface::Result<Self> {
                let mut table: $name = $name::default();
                $({
                    let value = table!(@read $name, table, tape [] [$($kind)+] [$($value)*]
                                       $(|$($argument),+| $body)*);
                    ::std::mem::forget(::std::mem::replace(&mut table.$field, value));
                })*
                Ok(table)
            }
        }
    );
    (@implement @position pub $name:ident {
        $($field:ident ($($kind:tt)+) [$($value:block)*] $(|$($argument:tt),+| $body:block)*,)*
    }) => (
        impl ::typeface::Value for $name {
            fn read<T: ::typeface::Tape>(tape: &mut T) -> ::typeface::Result<Self> {
                let position = tape.position()?;
                let mut table: $name = $name::default();
                $({
                    let value = table!(@read $name, table, tape [position] [$($kind)+] [$($value)*]
                                       $(|$($argument),+| $body)*);
                    ::std::mem::forget(::std::mem::replace(&mut table.$field, value));
                })*
                Ok(table)
            }
        }
    );
    (@read $name:ident, $this:ident, $tape:ident [$($position:tt)*] [$kind:ty] []) => (
        $tape.take()?
    );
    (@read $name:ident, $this:ident, $tape:ident [$($position:tt)*] [$kind:ty]
     [$value:block]) => ({
        let value = $tape.take()?;
        if value != $value {
            raise!("found a malformed or unknown table");
        }
        value
    });
    (@read $name:ident, $this:ident, $tape:ident [] [$kind:ty] []
     |$this_:tt, $tape_:tt| $body:block) => ({
        #[inline(always)]
        fn read<T: ::typeface::Tape>($this_: &$name, $tape_: &mut T)
                                     -> ::typeface::Result<$kind> $body
        read(&$this, $tape)?
    });
    (@read $name:ident, $this:ident, $tape:ident [$position:ident] [$kind:ty] []
     |$this_:tt, $tape_:tt, $position_:tt| $body:block) => ({
        #[inline(always)]
        fn read<T: ::typeface::Tape>($this_: &$name, $tape_: &mut T, $position_: u64)
                                     -> ::typeface::Result<$kind> $body
        read(&$this, $tape, $position)?
    });
}
