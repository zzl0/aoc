from utils import *
from functools import lru_cache


def parse_nums(s):
    return [int(i) for i in s.strip().split(',')]


def part1(values: List[int], DAYS=80) -> int:
    @lru_cache
    def dfs(v, d):
        if d == 0: return 1
        return dfs(v-1, d-1) if v else dfs(6, d-1) + dfs(8, d-1)
    return sum(dfs(v, DAYS) for v in values)


def part2(values: List[int]) -> int:
    return part1(values, 256)


if __name__ == "__main__":
    values = data(6, parse_nums, sep='__none__')[0]
    print(f'part1: {part1(values[:])}')
    print(f'part2: {part2(values[:])}')
