from collections import Counter

from utils import data


DIGITS = {
    0: 'abcefg',
    1: 'cf',
    2: 'acdeg',
    3: 'acdfg',
    4: 'bcdf',
    5: 'abdfg',
    6: 'abdefg',
    7: 'acf',
    8: 'abcdefg',
    9: 'abcdfg'
}
PATTERNS = {v: k for k, v in DIGITS.items()}
LEN_COUNTER = Counter(len(v) for v in DIGITS.values())
SIGNAL_COUNTER = Counter(c for v in DIGITS.values() for c in v)


def get_char_stats(uniq_patterns):
    return Counter(c for p in uniq_patterns for c in p)


def get_char_map(uniq_patterns):
    char_stats = get_char_stats(uniq_patterns)
    char_map = {}
    for c, cnt in SIGNAL_COUNTER.items():
        char_map[c] = {k for k, v in char_stats.items() if v == cnt}

    special_digit = [None] * 10
    for p in uniq_patterns:
        if len(p) == 2:
            special_digit[1] = p
        elif len(p) == 3:
            special_digit[7] = p
        elif len(p) == 4:
            special_digit[4] = p 
    
    diff_1_7 = special_digit[7] - special_digit[1]
    char_map['a'] = diff_1_7
    char_map['c'] -= diff_1_7

    diff_1_4 = special_digit[4] - special_digit[1]
    char_map['d'] &= diff_1_4
    char_map['g'] -= char_map['d']

    return {v.pop(): k for k, v in char_map.items()}


def parse_line(line):
    patterns, digits = line.split(' | ')
    uniq_patterns = [set(p) for p in patterns.split()]
    digits = digits.split()
    return uniq_patterns, digits


def part1(records):
    res = 0
    for uniq_patterns, digits in records:
        char_map = get_char_map(uniq_patterns)
        for d in digits:
            if len(d) in {2, 3, 4, 7}:
                new = ''.join(sorted(char_map[c] for c in d))
                if new in PATTERNS:
                    res += 1
    return res


def part2(records):
    res = 0
    for uniq_patterns, digits in records:
        char_map = get_char_map(uniq_patterns)
        v = 0
        for d in digits:
            new = ''.join(sorted(char_map[c] for c in d))
            v = v * 10 + PATTERNS[new]
        res += v
    return res



if __name__ == '__main__':
    records = data(8, parse_line)
    print(f'part1: {part1(records)}')
    print(f'part2: {part2(records)}')