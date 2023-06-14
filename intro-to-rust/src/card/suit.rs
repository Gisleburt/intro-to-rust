#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl ToString for Suit {
    fn to_string(&self) -> String {
        match self {
            Suit::Clubs => "♣".to_string(),
            Suit::Diamonds => "♦".to_string(),
            Suit::Hearts => "♥".to_string(),
            Suit::Spades => "♠".to_string(),
        }
    }
}

impl Suit {
    pub fn all() -> [Suit; 4] {
        [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades]
    }
}
