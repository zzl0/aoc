from utils import *


def part1(nums: List[int]) -> int:
    return sum(curr > prev for curr, prev in zip(nums[1:], nums))


def part2(nums: List[int]) -> int:
    windows = [sum(nums[i:i+3]) for i in range(len(nums)-2)]
    return part1(windows)


if __name__ == "__main__":
    measurements = data(1, int)
    print(f'part1: {part1(measurements)}')
    print(f'part2: {part2(measurements)}')
