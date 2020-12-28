from utils import *

def parse_group(s):
    return s.split('\n')


def day6_1(groups):
    return sum(len(set(''.join(group))) for group in groups)


def day6_2(groups):
    def common_questions(group):
        return len(set.intersection(*(set(person) for person in group)))
    return sum(common_questions(group) for group in groups)


if __name__ == "__main__":
    groups = data(6, parse_group, '\n\n')
    print(f'day6_1: {day6_1(groups)}')
    print(f'day6_2: {day6_2(groups)}')
    # day6_1: 6633
    # day6_2: 3202
