from typing import List, Set
from itertools import combinations


def data(day: int, type=str, sep='\n') -> List:
    with open(f'data/day{day:02d}.txt') as f:
        items = f.read().strip().split(sep)
        return list(map(type, items))


def first(iterable, default=None) -> object:
    return next(iter(iterable), default)
