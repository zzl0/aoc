from utils import *


def day1_1(nums: Set[int]) -> int:
    return first(x * y
                 for x in nums
                 for y in nums & {2020 - x}
                 if x != y)


def day1_2(nums: Set[int]) -> int:
    return first(x * y * z
                 for x, y in combinations(nums, 2)
                 for z in nums & {2020 - x - y}
                 if x != y != z)


if __name__ == "__main__":
    nums = set(data(day=1, type=int))
    print(f'day1_1: {day1_1(nums)}')
    print(f'day1_2: {day1_2(nums)}')

    # day1_1: 972576
    # day1_2: 199300880
