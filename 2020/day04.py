from utils import *


# byr (Birth Year) - four digits; at least 1920 and at most 2002.
# iyr (Issue Year) - four digits; at least 2010 and at most 2020.
# eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
# hgt (Height) - a number followed by either cm or in:
#     If cm, the number must be at least 150 and at most 193.
#     If in, the number must be at least 59 and at most 76.
# hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
# ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
# pid (Passport ID) - a nine-digit number, including leading zeroes.
# cid (Country ID) - ignored, missing or not.
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
    return dict(kv.split(':') for kv in s.split() if not kv.startswith('cid'))


def day4_1(passports):
    return sum(set(passport.keys()) == REQURIED_FIELDS for passport in passports)


def day4_2(passports):
    def is_valid(passport):
        return all(k in passport and VALIDATORS[k](passport[k]) for k in REQURIED_FIELDS)
    return count(passports, is_valid)


if __name__ == "__main__":
    passports = data(4, parse_passport, sep='\n\n')
    print(f'day4_1: {day4_1(passports)}')
    print(f'day4_2: {day4_2(passports)}')
    # day4_1: 239
    # day4_2: 188
