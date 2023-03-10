from .card import Card


class Hand:
    values = {
        'Two': 2,
        'Three': 3,
        'Four': 4,
        'Five': 5,
        'Six': 6,
        'Seven': 7,
        'Eight': 8,
        'Nine': 9,
        'Ten': 10,
        'Jack': 10,
        'Queen': 10,
        'King': 10,
        'Ace': 11
    }

    def __init__(self):
        self.cards = []
        self.value = 0
        self.aces = 0  # Keep track of aces

    def add_card(self, card: Card):
        if card.rank == 'Ace':
            self.aces += 1

        self.cards.append(card)
        self.value += self.values[card.rank]
        self.adjust_for_ace()

    def adjust_for_ace(self):
        while self.value > 21 and self.aces > 0:
            self.value -= 10
            self.aces -= 1

    def has_blackjack(self):
        return len(self.cards) == 2 and self.value == 21
