from utils import *


def day1_1(nums: List[int]) -> int:
    return sum(curr > prev for curr, prev in zip(nums[1:], nums))


def day1_2(nums: List[int]) -> int:
    windows = [sum(nums[i:i+3]) for i in range(len(nums)-2)]
    return day1_1(windows)


if __name__ == "__main__":
    measurements = data(1, int)
    print(f'day1_1: {day1_1(measurements)}')
    print(f'day1_2: {day1_2(measurements)}')
