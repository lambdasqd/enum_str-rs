#![cfg(test)]

#[macro_use]
extern crate enum_str;

use std::str::FromStr;
use enum_str::{Error, AsStr};

enum_str! {
    Vegetable,
    (Apple, "🍎"),
    (Pineapple, "🍍"),
    (Strawberry, "🍓"),
}

#[test]
fn test_as_str() {
    let _ = Vegetable::Apple;
    assert_eq!("🍎", Vegetable::Apple.as_str());
}

#[test]
fn test_from_str_ok() {
    let _ = Vegetable::Apple;
    assert_eq!(Vegetable::Apple, Vegetable::from_str("🍎").unwrap());
    assert_eq!(Vegetable::Strawberry, Vegetable::from_str("🍓").unwrap());
}

#[test]
fn test_from_str_err() {
    let result: Result<Vegetable, Error> = Vegetable::from_str("Strawberry");
    let _ = result.and_then(
        |_v| -> Result<(), Error> {
            panic!("expected error");
        }
    ).map_err(
        |e| {
            match e {
                Error::ParseStrError { input, to } => {
                    assert_eq!("Strawberry", input);
                    assert_eq!("Vegetable", to);
                },
                /* _ => panic!("wrong error") */
            }
        });
}