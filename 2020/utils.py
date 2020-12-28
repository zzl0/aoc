import re
from typing import List, Set
from itertools import combinations
from dataclasses import dataclass


def data(day: int, parse=str, sep='\n') -> List:
    with open(f'data/day{day:02d}.txt') as f:
        items = f.read().strip().split(sep)
        return list(map(parse, items))


def first(iterable, default=None) -> object:
    return next(iter(iterable), default)


def count(iterable, pred=bool) -> int:
    return sum(pred(x) for x in iterable)
