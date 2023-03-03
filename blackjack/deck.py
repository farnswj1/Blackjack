from .card import Card
import random


class Deck:
    suits = ('Hearts', 'Diamonds', 'Spades', 'Clubs')
    ranks = (
        'Two', 'Three', 'Four', 'Five', 'Six', 'Seven',
        'Eight', 'Nine', 'Ten', 'Jack', 'Queen', 'King', 'Ace'
    )

    def __init__(self):
        self.deck = [Card(suit, rank) for suit in self.suits for rank in self.ranks]

    def __str__(self):
        deck_comp = '\n  '.join(str(card) for card in self.deck)
        return 'The deck has:\n ' + deck_comp

    def shuffle(self):
        random.shuffle(self.deck)

    def deal(self):
        card = self.deck.pop()
        return card
