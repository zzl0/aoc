from utils import *


def parse(boarding_pass):
    table = {ord('F'): '0', ord('B'): '1', ord('L'): '0', ord('R'): '1'}
    return int(boarding_pass.translate(table), 2)


def day5_1(seats):
    return max(seats)


def day5_2(seats):
    seats = sorted(seats)
    for i, seat in enumerate(seats):
        if i > 0 and seat != seats[i - 1] + 1:
            return seats[i - 1] + 1


def test():
    s = 'FBFBBFFRLR'
    assert parse(s) == 357
    print('test passes')


if __name__ == "__main__":
    seats = data(5, parse)
    print(f'day5_1: {day5_1(seats)}')
    print(f'day5_2: {day5_2(seats)}')
    test()
    # day5_1: 919
    # day5_2: 642
