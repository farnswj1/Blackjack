use super::{deck::Deck, player::Player};

pub struct Blackjack {
    player: Player,
    dealer: Player
}

impl Blackjack {
    /// Creates a game state manager for Blackjack.
    pub fn new() -> Self {
        let player = Player::new();
        let dealer = Player::new();
        Self { player, dealer }
    }

    /// Shows some of the dealer's hand while showing the player's hand entirely.
    fn show_some(&self) {
        println!("\nDealer's {}", self.dealer.show_some());
        println!("\nPlayer's {}", self.player.show_all());
    }

    /// Shows the dealer and player's hands.
    fn show_all(&self) {
        println!("\nDealer's {}", self.dealer.show_all());
        println!("\nPlayer's {}", self.player.show_all());
    }

    /// Declares a tie.
    fn push(&self) {
        println!("Its a push! Player and Dealer tie!");
    }

    /// Runs the game.
    pub fn play(&mut self) {
        println!("Welcome to Blackjack!");

        loop {
            // Ask the player to make a bet.
            self.player.place_bet();

            // Create a shuffled deck.
            let mut deck = Deck::new();
            deck.shuffle();

            // Deal two cards to the player and dealer.
            for _ in 0..2 {
                self.player.add_card(deck.deal());
                self.dealer.add_card(deck.deal());
            }

            // Player wins by Blackjack if the dealer don't have a Blackjack too.
            // Otherwise, the player must hit or stand to try to win.
            if self.player.has_blackjack() {
                if self.dealer.has_blackjack() {
                    self.push();
                }
                else {
                    self.show_all();
                    println!("PLAYER WINS BY BLACKJACK!");
                    self.player.win_by_blackjack();
                }
            }
            else {
                loop {
                    self.show_some();
                    let hit = self.player.hit_or_stand(&mut deck);

                    if self.player.value() > 21 {
                        println!("PLAYER BUSTS!");
                        break;
                    }

                    if !hit {
                        break;
                    }
                }

                let player_value = self.player.value();

                if player_value <= 21 {
                    while self.dealer.value() < 17 {
                        self.dealer.hit(&mut deck);
                    }

                    let dealer_value = self.dealer.value();
                    self.show_all();

                    if dealer_value > 21 {
                        println!("DEALER BUSTS!");
                        self.player.win();
                    }
                    else if dealer_value > player_value {
                        println!("DEALER WINS!");
                        self.player.lose();
                    }
                    else if dealer_value < player_value {
                        println!("PLAYER WINS!");
                        self.player.win();
                    }
                    else {
                        self.push();
                    }
                }
                else {
                    self.show_some();
                    println!("PLAYER BUSTS!");
                    self.player.lose();
                }
            }

            let balance = self.player.balance();
            println!("\nPlayer's winnings stand at {}", balance);

            if balance <= 0 {
                println!("You're out of chips. Thanks for playing!");
                return;
            }
            else if !self.player.play_again() {
                println!("\nThanks for playing!");
                return;
            }
            else {
                self.player.reset();
                self.dealer.reset();
            }
        }
    }
}
