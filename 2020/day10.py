from utils import *


def day10_1(ratings):
    ratings = [0] + ratings + [max(ratings) + 3]
    counter = Counter(ratings[i] - ratings[i-1] for i in range(1, len(ratings)))
    return counter[1] * counter[3]


def day10_2(ratings):
    ratings = ratings + [max(ratings) + 3]
    dp = defaultdict(int)
    dp[0] = 1
    for n in ratings:
        dp[n] = dp[n-1] + dp[n-2] + dp[n-3]
    return dp[ratings[-1]]


if __name__ == "__main__":
    ratings = sorted(data(10, int))
    print(f'day10_1: {day10_1(ratings)}')
    print(f'day10_2: {day10_2(ratings)}')
    # day10_1: 1656
    # day10_2: 56693912375296
    # python3 day10.py  0.03s user 0.01s system 89% cpu 0.048 total
