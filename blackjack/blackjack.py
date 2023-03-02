from .chips import Chips
from .deck import Deck
from .hand import Hand


class Blackjack:
    def __init__(self):
        self.playing = True
        self.player_turn = True

    @staticmethod
    def take_bet(chips: Chips):
        wagered = False
        print(f'You have {chips.total} chips.')

        while not wagered:
            try:
                chips.bet = int(input('How many chips would you like to bet? '))
            except ValueError:
                print('Sorry! Please submit a number: ')
            else:
                if chips.bet > chips.total:
                    print(f"You bet can't exceed {chips.total}!")
                elif chips.bet <= 0:
                    print('You must bet at least 1 chip.')
                else:
                    wagered = True

    @staticmethod
    def hit(deck: Deck, hand: Hand):
        hand.add_card(deck.deal())
        hand.adjust_for_ace()

    def hit_or_stand(self, deck: Deck, hand: Hand):
        made_choice = False

        while not made_choice:
            try:
                ask = input(
                    "\nWould you like to hit or stand? Please enter 'h' or 's': "
                )
                option = ask[0].lower()
            except IndexError:
                print('You must hit or stand.')
            else:
                if option == 'h':
                    self.hit(deck, hand)
                    made_choice = True
                elif option == 's':
                    print('Player stands, Dealer is playing.')
                    self.player_turn = False
                    made_choice = True
                else:
                    print('Sorry! I did not understand that! Please try again!')

    @staticmethod
    def show_some(player: Hand, dealer: Hand):
        print("\nDealer's Hand: ")
        print(" <card hidden>")
        print("", dealer.cards[1])
        print("\nPlayer's Hand: ", *player.cards, sep='\n ')

    @staticmethod
    def show_all(player: Hand, dealer: Hand):
        print("\nDealer's Hand: ", *dealer.cards, sep='\n ')
        print("Dealer's Hand =", dealer.value)
        print("\nPlayer's Hand: ", *player.cards, sep='\n ')
        print("Player's Hand =", player.value)

    @staticmethod
    def player_busts(chips: Chips):
        print('PLAYER BUSTS!')
        chips.lose_bet()

    @staticmethod
    def player_wins(chips: Chips):
        print('PLAYER WINS!')
        chips.win_bet()

    @staticmethod
    def dealer_busts(chips: Chips):
        print('DEALER BUSTS!')
        chips.win_bet()

    @staticmethod
    def dealer_wins(chips: Chips):
        print('DEALER WINS!')
        chips.lose_bet()

    @staticmethod
    def push():
        print("Its a push! Player and Dealer tie!")

    def play(self):
        print('Welcome to Blackjack!')

        # Set up the player's chips
        player_chips = Chips()

        while self.playing:
            # Create a shuffled deck
            deck = Deck()
            deck.shuffle()

            player_hand = Hand()
            player_hand.add_card(deck.deal())
            player_hand.add_card(deck.deal())

            dealer_hand = Hand()
            dealer_hand.add_card(deck.deal())
            dealer_hand.add_card(deck.deal())

            # Ask the player for a bet
            self.take_bet(player_chips)

            # Show the cards
            self.show_some(player_hand, dealer_hand)

            self.player_turn = True

            while self.player_turn:
                self.hit_or_stand(deck, player_hand)
                self.show_some(player_hand, dealer_hand)

                if player_hand.value > 21:
                    self.player_busts(player_chips)
                    self.player_turn = False

            if player_hand.value <= 21:
                while dealer_hand.value < 17:
                    self.hit(deck, dealer_hand)

                self.show_all(player_hand, dealer_hand)

                if dealer_hand.value > 21:
                    self.dealer_busts(player_chips)
                elif dealer_hand.value > player_hand.value:
                    self.dealer_wins(player_chips)
                elif dealer_hand.value < player_hand.value:
                    self.player_wins(player_chips)
                else:
                    self.push()

            print("\nPlayer's winnings stand at", player_chips.total)

            if player_chips.total == 0:
                print("You're out of chips. Thanks for playing!")
                self.playing = False
            else:
                made_choice = False

                while not made_choice:
                    try:
                        new_game = input("\nWould you like to play again? Enter 'y' or 'n': ")
                        option = new_game[0].lower()
                    except IndexError:
                        print('You must select an option.')
                    else:
                        made_choice = True

                        if option != 'y':
                            print('\nThanks for playing!')
                            self.playing = False
