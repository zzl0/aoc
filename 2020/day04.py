from utils import *

VALIDATORS = {
    'byr': lambda x: len(x) == 4 and '1920' <= x <= '2002',
    'iyr': lambda x: len(x) == 4 and '2010' <= x <= '2020',
    'eyr': lambda x: len(x) == 4 and '2020' <= x <= '2030',
    'hgt': lambda x: '150' <= x[:-2] <= '193' if x[-2:] == 'cm' else x[-2:] == 'in' and '59' <= x[:-2] <= '76',
    'hcl': lambda x: re.match(r'^#[0-9a-f]{6}$', x),
    'ecl': lambda x: x in {'amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'},
    'pid': lambda x: len(x) == 9 and x.isdigit(),
}
REQURIED_FIELDS = set(VALIDATORS.keys())


def parse_passport(s):
    return [kv.split(':') for kv in s.split() if not kv.startswith('cid')]


def day4_1(passports):
    return sum({k for k, _ in passport} == REQURIED_FIELDS for passport in passports)


def day4_2(passports):
    def is_valid(passport):
        keys = {k for k, _ in passport}
        return keys == REQURIED_FIELDS and all(VALIDATORS[k](v) for k, v in passport)
    return count(passports, is_valid)



if __name__ == "__main__":
    passports = data(4, parse_passport, sep='\n\n')
    print(f'day4_1: {day4_1(passports)}')
    print(f'day4_2: {day4_2(passports)}')
    # day4_1: 239
    # day4_2: 188
