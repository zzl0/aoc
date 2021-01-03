from utils import *

def parse_decks(player1, player2):
    # ignore player id
    deck1 = [int(x) for x in player1[1:]]
    deck2 = [int(x) for x in player2[1:]]
    return deck1, deck2


def play(deck1, deck2):
    while deck1 and deck2:
        a, b = deck1.pop(0), deck2.pop(0)
        if a > b:
            deck1 += [a, b]
        else: # a < b
            deck2 += [b, a]
    return deck1, deck2


def recursive_play(deck1, deck2):
    seen = set()
    while deck1 and deck2:
        key = (tuple(deck1), tuple(deck2))
        if key in seen:
            return deck1, []
        seen.add(key)
        a, b = deck1.pop(0), deck2.pop(0)
        if len(deck1) >= a and len(deck2) >= b:
            sub_deck1, sub_deck2 = recursive_play(deck1[:a], deck2[:b])
            winner = 1 if sub_deck1 else 2
        else:
            winner = 1 if a > b else 2
        if winner == 1:
            deck1 += [a, b]
        else: # winner == 2
            deck2 += [b, a]
    return deck1, deck2


def day22_1(deck1, deck2):
    deck1, deck2 = play(deck1[:], deck2[:])
    deck = deck1 or deck2
    return sum(a * b for a, b in zip(deck, range(len(deck), 0, -1)))


def day22_2(deck1, deck2):
    deck1, deck2 = recursive_play(deck1[:], deck2[:])
    deck = deck1 or deck2
    return sum(a * b for a, b in zip(deck, range(len(deck), 0, -1)))


if __name__ == "__main__":
    decks = parse_decks(*data(22, lines, '\n\n'))
    print(f'day22_1: {day22_1(*decks)}')
    print(f'day22_2: {day22_2(*decks)}')
    # day22_1: 34664
    # day22_2: 32018
    # python3 day22.py  2.25s user 0.02s system 99% cpu 2.282 total
