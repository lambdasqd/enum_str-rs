/*!
This crate provides a macro to create a unitary enum and conversions from enum variants to a string
representation and vice versa.

# Why another crate?
The string representation does not need to be the same as the enum variant's identifier. See the
example below for clarification.

# Example
```rust
#[macro_use] extern crate enum_str;

use std::str::FromStr;
use enum_str::{Error, AsStr};

fn main() {
    enum_str! {
       Fruit,
       (Apple, "🍎"),
       (Pineapple, "🍍"),
       (Strawberry, "🍓"),
    }
    assert_eq!("🍎", Fruit::Apple.as_str());
    assert_eq!(Fruit::Apple, Fruit::from_str("🍎").unwrap());
}
```
*/
extern crate failure;

#[macro_use]
extern crate failure_derive;

/// Defines errors for this module
#[derive(Debug, Fail)]
pub enum Error {
    /// Failure to parse string used in `FromStr` trait
    #[fail(display = "failed to parse '{}' to {}", input, to)]
    ParseStrError {
        /// string we are trying to convert
        input: String,
        /// name of the enum we are converting to
        to: String,
    }
}

/// Trait for representing a structure as a string
pub trait AsStr {
    /// Get the string representation of a structure
    fn as_str(&self) -> &str;
}

/**
Macro used to generate an enum with `FromStr` and `AsStr` trait implementations.

The enum is identified by the `name` passed to the macro. Enum values are identified by
the `key` passed in each tuple. The 'value' is used as the string representation for
FromStr and AsStr traits.

See crate [Example](index.html#example) for usage.
*/
#[macro_export]
macro_rules! enum_str {
    ($name:ident, $(($key:ident, $value:expr),)*) => {
       #[derive(Debug, PartialEq)]
       enum $name
        {
            $($key),*
        }

        impl AsStr for $name {
            fn as_str(&self) -> &str {
                match self {
                    $(
                        &$name::$key => $value
                    ),*
                }
            }
        }

        impl FromStr for $name {
            type Err = Error;

            fn from_str(val: &str) -> Result<Self, Self::Err> {
                match val
                 {
                    $(
                        $value => Ok($name::$key)
                    ),*,
                    _ => Err(Error::ParseStrError{input: val.to_string(), to: stringify!($name).to_string()})
                }
            }
        }
    }
}