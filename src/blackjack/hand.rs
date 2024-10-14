use itertools::Itertools;

use super::card::Card;

pub struct Hand {
    pub cards: Vec<Card>,
    pub value: u8,
    pub aces: u8
}

impl Hand {
    /// Creates a new Hand object.
    pub fn new() -> Self {
        Hand { cards: vec![], value: 0, aces: 0 }
    }

    /// Adds a card to the hand and adjusts for aces if needed.
    pub fn add_card(&mut self, card: Card) {
        if card.rank == "Ace" {
            self.aces += 1;
        }

        self.cards.push(card);
        self.value += card.value().unwrap();
        self.adjust_for_ace();
    }

    /// Internal function for adjusting the value of the hand.
    fn adjust_for_ace(&mut self) {
        while self.value > 21 && self.aces > 0 {
            self.value -= 10;
            self.aces -= 1;
        }
    }

    /// Checks if the player has a Blackjack.
    pub fn has_blackjack(&self) -> bool {
        self.cards.len() == 2 && self.value == 21
    }

    /// Shows only the last card along with a "missing" card.
    pub fn show_some(&self) -> String {
        format!("Hand:\n  <card hidden>\n  {}", self.cards.last().unwrap())
    }

    /// Shows all cards in the hand.
    pub fn show_all(&self) -> String {
        format!("Hand:\n  {}", self.cards.iter().map(|card| format!("{card}")).join("\n  "))
    }
}
