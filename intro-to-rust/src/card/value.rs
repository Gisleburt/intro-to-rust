#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Ace => "A".to_string(),
            Value::Two => "2".to_string(),
            Value::Three => "3".to_string(),
            Value::Four => "4".to_string(),
            Value::Five => "5".to_string(),
            Value::Six => "6".to_string(),
            Value::Seven => "7".to_string(),
            Value::Eight => "8".to_string(),
            Value::Nine => "9".to_string(),
            Value::Ten => "10".to_string(),
            Value::Jack => "J".to_string(),
            Value::Queen => "Q".to_string(),
            Value::King => "K".to_string(),
        }
    }
}

impl Value {
    pub fn all() -> [Value; 13] {
        [
            Value::Ace,
            Value::Two,
            Value::Three,
            Value::Four,
            Value::Five,
            Value::Six,
            Value::Seven,
            Value::Eight,
            Value::Nine,
            Value::Ten,
            Value::Jack,
            Value::Queen,
            Value::King,
        ]
    }

    pub fn value(&self) -> u32 {
        (*self as u32) + 1
    }
}
