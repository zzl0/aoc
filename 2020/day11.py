from utils import *

DIRECTIONS = [(i, j) for i in (-1, 0, 1) for j in (-1, 0, 1) if not (i == 0 and j == 0)]
FLOOR, OCCUPIED, EMPTY = '.#L'


def occupied_adj_seats(matrix, i, j):
    m, n = len(matrix), len(matrix[0])
    cnt = 0
    for (di, dj) in DIRECTIONS:
        I, J = i + di, j + dj
        if 0 <= I < m and 0 <= J < n and matrix[I][J] == OCCUPIED:
            cnt += 1
    return cnt


def occupied_seats(matrix, i, j):
    m, n = len(matrix), len(matrix[0])
    cnt = 0
    for (di, dj) in DIRECTIONS:
        I, J = i + di, j + dj
        while 0 <= I < m and 0 <= J < n:
            if matrix[I][J] != FLOOR:
                cnt += matrix[I][J] == OCCUPIED
                break
            I, J = I + di, J + dj
    return cnt


def run(matrix, rules):
    m, n = len(matrix), len(matrix[0])
    pre_matrix = [] # previous matrix
    while matrix != pre_matrix:
        pre_matrix, matrix = matrix[:], []
        for i in range(m):
            row = []
            for j in range(n):
                state = pre_matrix[i][j]
                if state == FLOOR:
                    row.append(FLOOR)
                elif state == EMPTY:
                    row.append(OCCUPIED if rules[EMPTY](pre_matrix, i, j) else EMPTY)
                else: # state == OCCUPIED
                    row.append(EMPTY if rules[OCCUPIED](pre_matrix, i, j) else OCCUPIED)
            matrix.append(''.join(row))
    return count_matrix(matrix, lambda x: x == OCCUPIED)


def day11_1(matrix):
    rules = {
        EMPTY: lambda matrix, i, j: not occupied_adj_seats(matrix, i, j),
        OCCUPIED: lambda matrix, i, j: occupied_adj_seats(matrix, i, j) >= 4
    }
    return run(matrix, rules)


def day11_2(matrix):
    rules = {
        EMPTY: lambda matrix, i, j: not occupied_seats(matrix, i, j),
        OCCUPIED: lambda matrix, i, j: occupied_seats(matrix, i, j) >= 5
    }
    return run(matrix, rules)


if __name__ == "__main__":
    matrix = data(11)
    print(f'day11_1: {day11_1(matrix)}')
    print(f'day11_2: {day11_2(matrix)}')
    # day11_1: 2281
    # day11_2: 2085
    # python3 day11.py  4.83s user 0.02s system 99% cpu 4.868 total
