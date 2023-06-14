use std::fmt::{Display, Formatter};

use crate::{Card, Value};

fn face_value_to_blackjack_value(value: Value) -> u32 {
    match value {
        Value::Jack | Value::Queen | Value::King => 10,
        _ => value.value(),
    }
}

fn card_value(card: &Card) -> u32 {
    face_value_to_blackjack_value(card.value)
}

#[derive(Debug, Clone)]
pub enum BlackjackError {
    Bust(BlackjackHand),
}

#[derive(Debug, Clone)]
pub struct BlackjackHand(Vec<Card>);

impl Display for BlackjackHand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|card| card.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl BlackjackHand {
    pub fn max() -> u32 {
        21
    }

    pub fn new() -> Self {
        BlackjackHand(Vec::with_capacity(8))
    }

    /// Tells you the current value of the hand
    ///
    /// ```
    /// # use intro_to_rust::{BlackjackHand, Card, Suit, Value};
    /// let mut hand = BlackjackHand::new();
    ///
    /// let _ = hand.add(Card { suit: Suit::Hearts, value: Value::King });
    /// assert_eq!(hand.value(), 10);
    ///
    /// let _ = hand.add(Card { suit: Suit::Hearts, value: Value::Ace });
    /// assert_eq!(hand.value(), 21); // Yay
    ///
    /// let _ = hand.add(Card { suit: Suit::Hearts, value: Value::Two });
    /// assert_eq!(hand.value(), 13); // Whoops!
    /// ```
    pub fn value(&self) -> u32 {
        // Get the base value without handling aces
        let value = self.0.iter().map(card_value).sum();

        // We only need to check for a single ace as two aces at 11 would be bust anyway
        if value + 10 <= BlackjackHand::max() && self.0.iter().any(|card| card.value == Value::Ace) {
            value + 10
        } else {
            value
        }
    }

    /// Add a card to the hand, if the hand value would be over 21, an error is returned as the hand is bust
    ///
    /// ```
    /// # use intro_to_rust::{BlackjackHand, Card, Suit, Value};
    /// let mut hand = BlackjackHand::new();
    ///
    /// assert!(hand.add(Card { suit: Suit::Hearts, value: Value::King }).is_ok()); // Total: 10
    /// assert!(hand.add(Card { suit: Suit::Hearts, value: Value::Queen }).is_ok()); // Total: 20
    /// assert!(hand.add(Card { suit: Suit::Hearts, value: Value::Two }).is_err()); // Total: 22
    /// ```
    pub fn add(&mut self, card: Card) -> Result<(), BlackjackError> {
        self.0.push(card);
        if self.value() > BlackjackHand::max() {
            Err(BlackjackError::Bust(self.clone()))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_face_value_to_blackjack_value() {
        assert_eq!(face_value_to_blackjack_value(Value::Ace), 1);
        assert_eq!(face_value_to_blackjack_value(Value::Two), 2);
        assert_eq!(face_value_to_blackjack_value(Value::Three), 3);
        assert_eq!(face_value_to_blackjack_value(Value::Four), 4);
        assert_eq!(face_value_to_blackjack_value(Value::Five), 5);
        assert_eq!(face_value_to_blackjack_value(Value::Six), 6);
        assert_eq!(face_value_to_blackjack_value(Value::Seven), 7);
        assert_eq!(face_value_to_blackjack_value(Value::Eight), 8);
        assert_eq!(face_value_to_blackjack_value(Value::Nine), 9);
        assert_eq!(face_value_to_blackjack_value(Value::Ten), 10);
        assert_eq!(face_value_to_blackjack_value(Value::Jack), 10);
        assert_eq!(face_value_to_blackjack_value(Value::Queen), 10);
        assert_eq!(face_value_to_blackjack_value(Value::King), 10);
    }
}
