use crate::{Suit, Value};

pub mod suit;
pub mod value;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Card {
    pub suit: Suit,
    pub value: Value,
}

impl ToString for Card {
    fn to_string(&self) -> String {
        format!("{}{}", self.value.to_string(), self.suit.to_string())
    }
}
