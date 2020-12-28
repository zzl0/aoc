from utils import *


def trees(matrix, slope):
    down, right = slope
    return sum(row[(i * right) % len(row)] == '#' for i, row in enumerate(matrix[::down]))


def day3_1(matrix,):
    return trees(matrix, (1, 3))


def day3_2(matrix):
    slopes = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
    return functools.reduce(operator.mul, (trees(matrix, slope) for slope in slopes))


if __name__ == "__main__":
    matrix = data(3)
    print(f"day3_1: {day3_1(matrix)}")
    print(f"day3_2: {day3_2(matrix)}")
