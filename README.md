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
       Vegetable,
       (Apple, "🍎"),
       (Pineapple, "🍍"),
       (Strawberry, "🍓"),
    }
    assert_eq!("🍎", Vegetable::Apple.as_str());
    assert_eq!(Vegetable::Apple, Vegetable::from_str("🍎").unwrap());
}
```