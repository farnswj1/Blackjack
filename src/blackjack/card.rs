use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy)]
pub struct Card {
    pub suit: &'static str,
    pub rank: &'static str
}

impl Card {
    /// Maps the rank of the card to a numerical value.
    pub fn value(&self) -> u8 {
        match self.rank {
            "Two" => 2,
            "Three" => 3,
            "Four" => 4,
            "Five" => 5,
            "Six" => 6,
            "Seven" => 7,
            "Eight" => 8,
            "Nine" => 9,
            "Ten" => 10,
            "Jack" => 10,
            "Queen" => 10,
            "King" => 10,
            "Ace" => 11,
            _ => 0
        }
    } 
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}
