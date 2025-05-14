mod card;
mod chips;
mod deck;
mod game;
mod hand;
mod player;

use game::Blackjack;

/// Play Blackjack on the command line interface.
pub fn play() {
    let mut game = Blackjack::new();
    game.play();
}
