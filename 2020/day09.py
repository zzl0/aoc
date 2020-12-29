from utils import *


def search(counter, target):
    for a in counter:
        b = target - a
        if (a == b and counter[b] >= 2) or (a != b and counter[b]):
            return True
    return False


def day9_1(nums):
    counter, i = Counter(nums[:25]), 0
    for j in range(25, len(nums)):
        if not search(counter, nums[j]):
            return nums[j]
        counter[nums[j]] += 1
        counter[nums[i]] -= 1
        i += 1


def day9_2(nums):
    i, j, s = 0, 0, 0
    target = 57195069
    while j < len(nums):
        s += nums[j]
        while s > target:
            s -= nums[i]
            i += 1
        if s == target and i != j:
            break
        j += 1
    arr = nums[i: j+1]
    return min(arr) + max(arr)


if __name__ == "__main__":
    nums = data(9, int)
    print(f'day9_1: {day9_1(nums)}')
    print(f'day9_2: {day9_2(nums)}')
    # day9_1: 57195069
    # day9_2: 7409241
    # python3 day09.py  0.08s user 0.01s system 95% cpu 0.092 total
