from utils import *


def day23_1(cups, moves=100):
    curr, i, n = cups[0], 0, len(cups)
    sorted_labels = sorted(cups, reverse=True)
    for _ in range(moves):
        picked = cups[i+1: i+4] + cups[0: max(i+4-n, 0)]
        cups = cups[max(i+4-n, 0):i+1] + cups[i+4:]
        dest = max((c for c in cups if c < curr), default=max(cups))
        idx = cups.index(dest)
        cups = cups[:idx+1] + picked + cups[idx+1:]
        i = (cups.index(curr) + 1) % n
        curr = cups[i]
    j = cups.index('1')
    return cups[j+1:] + cups[:j]


def build_linked_list(cups, n):
    size = len(cups) + 1 # cup number starts from 1
    next_links = [0] * size
    for i, cup in enumerate(cups):
        if i > 0: next_links[cups[i - 1]] = cup
    next_links += list(range(size + 1, n + 2))
    next_links[n] = cups[0]
    next_links[cups[-1]] = size
    return next_links


def play(next_links, curr, moves):
    for _ in range(moves):
        packed = pickup(next_links, curr)
        dest = destination(next_links, curr, packed)
        place(next_links, dest, packed)
        curr = next_links[curr]


def pickup(next_links, curr):
    first = next_links[curr]
    second = next_links[first]
    third = next_links[second]
    next_links[curr] = next_links[third]
    return [first, second, third]


def destination(next_links, curr, packed):
    _max = len(next_links) - 1
    dest = curr - 1 if curr > 1 else _max
    while dest in packed:
        dest = dest - 1 if dest > 1 else _max
    return dest


def place(next_links, dest, packed):
    next_links[packed[-1]] = next_links[dest]
    next_links[dest] = packed[0]


def day23_2(cups, moves=10000000, n=1000000):
    cups = [int(x) for x in cups]
    next_links = build_linked_list(cups, n)
    play(next_links, cups[0], moves)
    return next_links[1] * next_links[next_links[1]]


if __name__ == "__main__":
    cups = '586439172'
    print(f'day23_1: {day23_1(cups)}')
    print(f'day23_2: {day23_2(cups)}')
    # day23_1: 28946753
    # day23_2: 519044017360
    # python3 day23.py  14.39s user 0.10s system 99% cpu 14.596 total
