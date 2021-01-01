from utils import *


def parse_nums(s):
    return tuple(int(i) for i in s.strip().split(','))


def day15_1(nums, nth=2020):
    seen = {n: i + 1 for i, n in enumerate(nums[:-1])}
    last = nums[-1]
    for i in range(len(nums), nth):
        curr = 0 if last not in seen else i - seen[last]
        seen[last] = i
        last = curr
    return last


def day15_2(nums):
    return day15_1(nums, 30000000)


if __name__ == "__main__":
    nums = data(15, parse_nums, sep='__none__')[0]
    print(f'day15_1: {day15_1(nums)}')
    print(f'day15_2: {day15_2(nums)}')
    # day15_1: 758
    # day15_2: 814
    # python3 day15.py  11.05s user 0.34s system 99% cpu 11.434 total
