from utils import *
from collections import Counter, defaultdict


def part1(nums: List[str]) -> int:
    gamma_rate, epsilon_rate = '', ''
    for col in zip(*nums):
        (first, _), (second, _) = Counter(col).most_common()
        gamma_rate += first
        epsilon_rate += second
    return int(gamma_rate, 2) * int(epsilon_rate, 2)


def part2(nums: List[str]) -> int:
    def helper(queue, v=1):
        if len(queue) == 1:
            return ''.join(b for b in queue[0])
        d = defaultdict(list)
        for it in queue:
            bit = next(it)
            d[bit].append(it)
        if len(d['1']) >= len(d['0']):
            return str(v) + helper(d[str(v)], v)
        else:
            return str(1-v) + helper(d[str(1-v)], v)
    
    oxygen_rate = helper([iter(n) for n in nums], 1)
    co2_rate = helper([iter(n) for n in nums], 0)
    return int(oxygen_rate, 2) * int(co2_rate, 2)


if __name__ == "__main__":
    nums = data(3, str)
    print(f'part1: {part1(nums)}')
    print(f'part2: {part2(nums)}')
