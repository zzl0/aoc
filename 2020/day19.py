# This solution is copied from https://github.com/norvig/pytudes/blob/master/ipynb/Advent-2020.ipynb
from utils import *

Char = str
Message = str   # A string we are trying to match, e.g. "ababba"
Choice  = tuple # A choice of any of the elements, e.g. Choice(([5, 6], [7]))
Pattern = List[Union[Char, int, Choice]]


def parse_inputs(rules, messages) -> Tuple[Dict[int, Pattern], List[Message]]:
    "Return a dict of {rule_id: pattern} and a list of messages."
    return dict(map(parse_rule, rules)), messages


def parse_rule(line):
    """Parse line to (id, pattern) pair.

    >>> parse_rule('1: 2 3')
    (1, [2, 3])
    >>> parse_rule('4: 5 6 | 7 8')
    (4, [([5, 6], [7, 8])])
    """
    items = line.replace(':', ' ').replace('"', ' ').split()
    n, *rhs = [int(x) if x.isdigit() else x for x in items]
    if '|' in rhs:
        i = rhs.index('|')
        rhs = [Choice((rhs[:i], rhs[i + 1:]))]
    return n, rhs


def match(pat, msg, rules):
    if pat and not msg:
        return False
    elif not pat:
        return msg == ''
    elif pat[0] == msg[0]:
        return match(pat[1:], msg[1:], rules)
    elif isinstance(pat[0], int):
        return match(rules[pat[0]] + pat[1:], msg, rules)
    elif isinstance(pat[0], Choice):
        for choice in pat[0]:
            m = match(choice + pat[1:], msg, rules)
            if m: return m
    return False


def day19_1(rules, msgs):
    return count(msgs, lambda msg: match(rules[0], msg, rules))


def day19_2(rules, msgs):
    rules2 = {**rules, 8: [42, maybe(8)], 11: [42, maybe(11), 31]}
    return day19_1(rules2, msgs)


def maybe(n):
    return Choice(([], [n]))


if __name__ == "__main__":
    rules, msgs = parse_inputs(*data(19, lines, sep='\n\n'))
    print(f'day19_1: {day19_1(rules, msgs)}')
    print(f'day19_2: {day19_2(rules, msgs)}')
    # day19_1: 122
    # day19_2: 287
    # python3 day19.py  0.39s user 0.01s system 98% cpu 0.410 total
