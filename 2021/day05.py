from utils import *
from collections import Counter


def parse_point(p):
    x, y = p.split(',')
    return int(x), int(y)


def parse_segment(s):
    "'0,9 -> 5,9' -> ((0, 9), (5, 9))"
    p1, p2 = s.split(' -> ')
    return parse_point(p1), parse_point(p2)


def get_points(segment, skip_diagonal=True):
    (x1, y1), (x2, y2) = segment
    if x1 == x2: # vertial
        return [(x1, y) for y in range(min(y1, y2), max(y1, y2) + 1)]
    if y1 == y2: # horizontal
        return [(x, y1) for x in range(min(x1, x2), max(x1, x2) + 1)]
    
    if skip_diagonal:
        return []
    if x1 > x2: # make sure x1 is always less than x2
        x1, y1, x2, y2 = x2, y2, x1, y1
    if x1 < x2 and y1 < y2: # up-diagonal
        return [(x1 + i, y1 + i) for i in range(x2 - x1 + 1)]
    # down-diagonal
    return [(x1 + i, y1 - i) for i in range(x2 - x1 + 1)]


def part1(segments: Tuple) -> int:
    counter = Counter([p for segment in segments for p in get_points(segment)])
    return count(counter.values(), lambda v: v >= 2)


def part2(segments: Tuple) -> int:
    counter = Counter([p for segment in segments for p in get_points(segment, False)])
    return count(counter.values(), lambda v: v >= 2)


if __name__ == "__main__":
    segments = data(5, parse_segment)
    print(f'part1: {part1(segments)}')
    print(f'part2: {part2(segments)}')
