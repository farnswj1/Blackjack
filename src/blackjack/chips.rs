pub struct Chips {
    pub total: i32,
    pub bet: i32
}

impl Chips {
    /// Creates a new Chips objects.
    pub fn new() -> Self {
        Chips { total: 100, bet: 0 }
    }

    /// Increases the total by the value of the bet.
    pub fn win_bet(&mut self) {
        self.total += self.bet;
    }

    /// Decreases the total by the value of the bet.
    pub fn lose_bet(&mut self) {
        self.total -= self.bet;
    }

    /// Increases the total by 1.5x the value of the bet.
    pub fn win_by_blackjack(&mut self) {
        let bonus = self.bet / 2;
        self.total += self.bet + bonus;
    }
}
