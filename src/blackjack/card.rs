use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy)]
pub struct Card {
    pub suit: &'static str,
    pub rank: &'static str
}

impl Card {
    /// Maps the rank of the card to a numerical value.
    pub fn value(&self) -> Option<u8> {
        match self.rank {
            "Two" => Some(2),
            "Three" => Some(3),
            "Four" => Some(4),
            "Five" => Some(5),
            "Six" => Some(6),
            "Seven" => Some(7),
            "Eight" => Some(8),
            "Nine" => Some(9),
            "Ten" | "Jack" | "Queen" | "King" => Some(10),
            "Ace" => Some(11),
            _ => None
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}
