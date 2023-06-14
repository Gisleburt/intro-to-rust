use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::{Card, Suit, Value};

#[derive(Debug)]
pub struct Deck(Vec<Card>);

impl Deck {
    pub fn new() -> Self {
        let cards = Suit::all()
            .into_iter()
            .flat_map(|suit| {
                Value::all()
                    .into_iter()
                    .map(move |value| Card { suit, value })
            })
            .collect();
        Deck(cards)
    }

    pub fn shuffle(&mut self) {
        self.0.shuffle(&mut thread_rng())
    }

    pub fn take(&mut self) -> Option<Card> {
        self.0.pop()
    }
}
