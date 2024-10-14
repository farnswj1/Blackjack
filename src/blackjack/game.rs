use std::io;

use super::{chips::Chips, deck::Deck, hand::Hand};

pub struct Blackjack {
    playing: bool,
    player_turn: bool
}

impl Blackjack {
    /// Creates a game state manager for Blackjack.
    pub fn new() -> Self {
        Blackjack { playing: true, player_turn: true }
    }

    /// Allows the player to make a bet.
    /// The bet must be a positive integer no greater than the chips they have.
    fn take_bet(&self, chips: &mut Chips) {
        let mut wagered = false;
        println!("You have {} chips.", chips.total);

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
            if bet > chips.total {
                println!("Your bet can't exceed {}!", chips.total);
            }
            else if bet <= 0 {
                println!("Your bet must be at least 1 chip.");
            }
            else {
                chips.bet = bet;
                wagered = true;
            }
        }
    }

    /// Draw a card from the deck and append it to the hand.
    fn hit(&self, deck: &mut Deck, hand: &mut Hand) {
        hand.add_card(deck.deal());
    }

    /// Allows the player to decide whether to hit or stand.
    fn hit_or_stand(&mut self, deck: &mut Deck, hand: &mut Hand) {
        let mut made_choice = false;

        while !made_choice {
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
                    self.hit(deck, hand);
                    made_choice = true;
                },
                "s" => {
                    println!("Player stands, Dealer is playing.");
                    self.player_turn = false;
                    made_choice = true;
                },
                _ => println!("Not a valid option. Please try again!")
            }
        }
    }

    /// Shows some of the dealer's hand while showing the player's hand entirely.
    fn show_some(&self, player: &Hand, dealer: &Hand) {
        println!("\nDealer's {}", dealer.show_some());
        println!("\nPlayer's {}", player.show_all());
    }

    /// Shows the dealer and player's hands.
    fn show_all(&self, player: &Hand, dealer: &Hand) {
        println!("\nDealer's {}", dealer.show_all());
        println!("\nPlayer's {}", player.show_all());
    }

    /// Decrements the player's total by the value of the bet.
    fn player_busts(&self, chips: &mut Chips) {
        println!("PLAYER BUSTS!");
        chips.lose_bet();
    }

    /// Increments the player's total by the value of the bet.
    fn player_wins(&self, chips: &mut Chips) {
        println!("PLAYER WINS!");
        chips.win_bet();
    }

    /// Increments the player's total by 1.5x the value of the bet.
    fn player_wins_by_blackjack(&self, chips: &mut Chips) {
        println!("PLAYER WINS BY BLACKJACK!");
        chips.win_by_blackjack();
    }

    /// Increments the player's total by the value of the bet.
    fn dealer_busts(&self, chips: &mut Chips) {
        println!("DEALER BUSTS!");
        chips.win_bet();
    }

    /// Decrements the player's total by the value of the bet.
    fn dealer_wins(&self, chips: &mut Chips) {
        println!("DEALER WINS!");
        chips.lose_bet();
    }

    /// Declares a tie.
    fn push(&self) {
        println!("Its a push! Player and Dealer tie!");
    }

    /// Allows the player to continue playing or quit.
    fn play_again_or_quit(&mut self) {
        let mut made_choice = false;

        while !made_choice {
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

            made_choice = true;
            if choice != "y" {
                println!("\nThanks for playing!");
                self.playing = false;
            }
        }
    }

    /// Runs the game.
    pub fn play(&mut self) {
        println!("Welcome to Blackjack!");

        // Set up the player's chips
        let mut player_chips = Chips::new();

        while self.playing {
            // Ask the player to make a bet
            self.take_bet(&mut player_chips);

            // Create a shuffled deck
            let mut deck = Deck::new();
            deck.shuffle();

            // Create hands for the player and dealer
            let mut player_hand = Hand::new();
            let mut dealer_hand = Hand::new();

            // Deal two cards to the player and dealer
            for _ in 0..2 {
                player_hand.add_card(deck.deal());
                dealer_hand.add_card(deck.deal());
            }

            // Player wins by Blackjack if the dealer don't have a Blackjack too.
            // Otherwise, the player must hit or stand to try to win
            if player_hand.has_blackjack() {
                if dealer_hand.has_blackjack() {
                    self.push();
                }
                else {
                    self.show_all(&player_hand, &dealer_hand);
                    self.player_wins_by_blackjack(&mut player_chips);
                }
            }
            else {
                self.player_turn = true;

                while self.player_turn {
                    self.show_some(&player_hand, &dealer_hand);
                    self.hit_or_stand(&mut deck, &mut player_hand);

                    // Player's turn ends if they bust
                    if player_hand.value > 21 {
                        self.player_turn = false;
                    }
                }

                if player_hand.value <= 21 {
                    while dealer_hand.value < 17 {
                        self.hit(&mut deck, &mut dealer_hand);
                    }

                    self.show_all(&player_hand, &dealer_hand);
                    if dealer_hand.value > 21 {
                        self.dealer_busts(&mut player_chips);
                    }
                    else if dealer_hand.value > player_hand.value {
                        self.dealer_wins(&mut player_chips);
                    }
                    else if dealer_hand.value < player_hand.value {
                        self.player_wins(&mut player_chips);
                    }
                    else {
                        self.push();
                    }
                }
                else {
                    self.show_some(&player_hand, &dealer_hand);
                    self.player_busts(&mut player_chips);
                }
            }

            println!("\nPlayer's winnings stand at {}", player_chips.total);

            if player_chips.total > 0 {
                self.play_again_or_quit();
            }
            else {
                println!("You're out of chips. Thanks for playing!");
                self.playing = false;
            }
        }
    }
}
