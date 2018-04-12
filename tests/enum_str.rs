#![cfg(test)]

#[macro_use]
extern crate enum_str;

use std::str::FromStr;
use enum_str::{Error, AsStr};

enum_str! {
    Fruit,
    (Apple, "🍎"),
    (Pineapple, "🍍"),
    (Strawberry, "🍓"),
}

#[test]
fn test_as_str() {
    let _ = Fruit::Apple;
    assert_eq!("🍎", Fruit::Apple.as_str());
}

#[test]
fn test_from_str_ok() {
    let _ = Fruit::Apple;
    assert_eq!(Fruit::Apple, Fruit::from_str("🍎").unwrap());
    assert_eq!(Fruit::Strawberry, Fruit::from_str("🍓").unwrap());
}

#[test]
fn test_from_str_err() {
    let result: Result<Fruit, Error> = Fruit::from_str("Strawberry");
    let _ = result.and_then(
        |_v| -> Result<(), Error> {
            panic!("expected error");
        }
    ).map_err(
        |e| {
            match e {
                Error::ParseStrError { input, to } => {
                    assert_eq!("Strawberry", input);
                    assert_eq!("Fruit", to);
                },
                /* _ => panic!("wrong error") */
            }
        });
}