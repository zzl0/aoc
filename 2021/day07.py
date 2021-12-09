from utils import *


def parse_nums(s):
    return [int(i) for i in s.strip().split(',')]


def part1(nums: List[int]) -> int:
    # ? is the median the best position
    def total_fuel(pos):
        return sum(abs(n-pos) for n in nums)
    return min(total_fuel(pos) for pos in nums)


def part2(nums: List[int]) -> int:
    def total_fuel(pos):
        s = 0
        for n in nums:
            step = abs(n - pos)
            s += (step + 1) * step // 2
        return s
    return min(total_fuel(pos) for pos in nums)


if __name__ == "__main__":
    nums = data(7, parse_nums, sep='__none__')[0]
    print(f'part1: {part1(nums)}')
    print(f'part2: {part2(nums)}')
