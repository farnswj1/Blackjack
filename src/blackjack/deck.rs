use std::fmt::{Display, Result};

use itertools::{iproduct, Itertools};
use rand::{seq::SliceRandom, thread_rng};

use super::card::Card;

const SUITS: &'static [&'static str] = &["Hearts", "Diamonds", "Spades", "Clubs"];
const RANKS: &'static [&'static str] = &[
    "Two", "Three", "Four", "Five", "Six", "Seven", "Eight",
    "Nine", "Ten", "Jack", "Queen", "King", "Ace"
];

pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    /// Creates a new Deck object.
    pub fn new() -> Self {
        let cards = iproduct!(SUITS, RANKS)
            .map(|(&suit, &rank)| Card { suit, rank })
            .collect::<Vec<Card>>();
        Deck { cards }
    }

    /// Shuffles the deck of cards.
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    /// Draws one card from the deck.
    ///
    /// **NOTE**: this removes a card from the deck.
    pub fn deal(&mut self) -> Card {
        self.cards.pop().unwrap()
    }
}

impl Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        let deck_comp = self.cards
            .iter()
            .map(|card| format!("{card}"))
            .join("\n ");
        write!(f, "The deck has:\n{deck_comp}")
    }
}
