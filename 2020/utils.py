import re
import operator
import functools
import itertools
from typing import List, Set, Tuple, Iterable
from itertools import combinations, chain
from dataclasses import dataclass
from collections import defaultdict, Counter


def data(day: int, parse=str, sep='\n') -> List:
    with open(f'data/day{day:02d}.txt') as f:
        items = re.split(sep, f.read().strip())
        return list(map(parse, items))


def first(iterable, default=None) -> object:
    return next(iter(iterable), default)


def count(iterable, pred=bool) -> int:
    return sum(pred(x) for x in iterable)


def count_matrix(matrix, pred=bool) -> int:
    return sum(pred(x) for row in matrix for x in row)


def product(numbers) -> int:
    return functools.reduce(operator.mul, numbers, 1)


def max_min(lst) -> Tuple:
    return max(lst), min(lst)


def flatten(iterable) -> Iterable:
    return chain.from_iterable(iterable)
