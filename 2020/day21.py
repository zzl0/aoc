# This solution is based on https://github.com/norvig/pytudes/blob/master/ipynb/Advent-2020.ipynb
from utils import *


def parse_food(line):
    ingredients, allergens = line.split(' (contains ')
    return set(ingredients.split()), set(allergens[:-1].split(', '))


def eliminate_others(possible, i):
    for j in possible:
        if i != j:
            possible[j] -= possible[i]


def bad_ingredients(foods):
    possible = defaultdict(set)
    for ingredients, allergens in foods:
        for a in allergens:
            possible[a] |= ingredients
    while any(len(possible[a]) > 1 for a in possible):
        for ingredients, allergens in foods:
            for a in allergens:
                possible[a] &= ingredients
                if len(possible[a]) == 1:
                    eliminate_others(possible, a)
    return possible


def day21_1(foods):
    bad = bad_ingredients(foods)
    _ingredients = set(flatten(bad.values()))
    return sum(len(ingredients - _ingredients) for ingredients, _ in foods)


def day21_2(foods):
    bad = bad_ingredients(foods)
    return ','.join(next(iter(bad[a])) for a in sorted(bad))


if __name__ == "__main__":
    foods = data(21, parse_food)
    print(f'day21_1: {day21_1(foods)}')
    print(f'day21_2: {day21_2(foods)}')
    # day21_1: 1829
    # day21_2: mxkh,gkcqxs,bvh,sp,rgc,krjn,bpbdlmg,tdbcfb
    # python3 day21.py  0.03s user 0.01s system 91% cpu 0.048 total
