use std::io;

use super::{card::Card, chips::Chips, deck::Deck, hand::Hand};

pub struct Player {
    chips: Chips,
    hand: Hand
}

impl Player {
    /// Creates the player object.
    pub fn new() -> Self {
        let chips = Chips::new();
        let hand = Hand::new();
        Self { chips, hand }
    }

    /// Allows the player to make a bet.
    /// The bet must be a positive integer no greater than the chips they have.
    pub fn place_bet(&mut self) {
        let mut wagered = false;
        println!("You have {} chips.", self.chips.total);

        while !wagered {
            println!("How many chips would you like to bet?");
            let mut input = String::new();
            let result = io::stdin().read_line(&mut input);

            if let Err(_) = result {
                println!("Please submit a number!");
                continue;
            }

            let parsed_input = input.trim().parse::<i32>();
            if let Err(_) = parsed_input {
                println!("Please enter a whole number!");
                continue;
            }

            let bet = parsed_input.unwrap();
            if bet > self.chips.total {
                println!("Your bet can't exceed {}!", self.chips.total);
            }
            else if bet <= 0 {
                println!("Your bet must be at least 1 chip.");
            }
            else {
                self.chips.bet = bet;
                wagered = true;
            }
        }
    }

    /// Draw a card from the deck and append it to the player's hand.
    pub fn hit(&mut self, deck: &mut Deck) {
        self.hand.add_card(deck.deal());
    }

    /// Allows the player to decide whether to hit or stand.
    /// If the player hits, returns true; if they stand, false.
    pub fn hit_or_stand(&mut self, deck: &mut Deck) -> bool {
        loop {
            println!("Would you like to hit or stand? Please enter 'h' or 's':");
            let mut input = String::new();
            let result = io::stdin().read_line(&mut input);

            if let Err(_) = result {
                println!("There was an issue processing your choice. Try again!");
                continue;
            }

            let choice = input.trim();
            if choice.len() < 1 {
                println!("You must hit or stand.");
                continue;
            }

            match choice {
                "h" => {
                    self.hit(deck);
                    return true;
                },
                "s" => {
                    println!("Player stands, Dealer is playing.");
                    return false;
                },
                _ => println!("Not a valid option. Please try again!")
            }
        }
    }

    /// Adds a card to the player's hand.
    pub fn add_card(&mut self, card: Card) {
        self.hand.add_card(card);
    }

    /// Increments the player's total by the value of the bet.
    pub fn win(&mut self) {
        self.chips.win_bet();
    }

    /// Increments the player's total by 1.5x the value of the bet.
    pub fn win_by_blackjack(&mut self) {
        self.chips.win_by_blackjack();
    }

    /// Decrements the player's total by the value of the bet.
    pub fn lose(&mut self) {
        self.chips.lose_bet();
    }

    /// Checks if the player has a Blackjack.
    pub fn has_blackjack(&self) -> bool {
        self.hand.has_blackjack()
    }

    /// Show all but one card in the player's hand.
    pub fn show_some(&self) -> String {
        self.hand.show_some()
    }

    /// Show all cards in the player's hand.
    pub fn show_all(&self) -> String {
        self.hand.show_all()
    }

    /// Returns the value of the player's hand.
    pub fn value(&self) -> u8 {
        self.hand.value
    }

    /// Returns the balance of the player's chips.
    pub fn balance(&self) -> i32 {
        self.chips.total
    }

    /// Allows the player to continue playing or quit.
    /// Returns true if the player wants to play again, otherwise false.
    pub fn play_again(&mut self) -> bool {
        loop {
            println!("Would you like to play again? Enter 'y' or 'n':");
            let mut input = String::new();
            let result = io::stdin().read_line(&mut input);

            if let Err(_) = result {
                println!("There was an issue processing your choice. Try again!");
                continue;
            }

            let choice = input.trim();
            if choice.len() < 1 {
                println!("You must select an option.");
                continue;
            }

            return choice == "y";
        }
    }

    /// Resets the player's hand.
    pub fn reset(&mut self) {
        self.hand.reset();
    }
}
