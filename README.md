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