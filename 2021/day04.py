from utils import *
from copy import deepcopy

ROWS, COLS = 5, 5


def parse(s: str) -> List:
    lines = s.split('\n')
    if len(lines) == 1: # random numbers
        return [int(n) for n in lines[0].split(',')]
    # boards
    return [[int(n) for n in line.split()] for line in lines]


def is_winner(board, row, col):
    if all(board[row][j] == None for j in range(COLS)):
        return True
    if all(board[i][col] == None for i in range(ROWS)):
        return True
    return False


def sum_unmarked_num(board):
    return sum(board[i][j] for i in range(ROWS)
                           for j in range(COLS)
                           if board[i][j] != None)


def mark_num(board, num):
    for row in range(ROWS):
        for col in range(COLS):
            if board[row][col] == num:
                board[row][col] = None
                return is_winner(board, row, col)
                    
                            
def part1(nums: List[int], boards: List[List[int]]) -> int:
    for num in nums:
        for board in boards:
            if mark_num(board, num):
                return sum_unmarked_num(board) * num


def part2(nums: List[int], boards: List[List[int]]) -> int:
    total_boards, win_boards = len(boards), 0
    for num in nums:
        new_boards = []
        for i, board in enumerate(boards):
            if mark_num(board, num):
                win_boards += 1
                if win_boards == total_boards:
                    return sum_unmarked_num(board) * num
            else:
                new_boards.append(board)
        boards = new_boards


if __name__ == "__main__":
    nums, *boards = data(4, parse, '\n\n')
    print(f'part1: {part1(nums, deepcopy(boards))}')
    print(f'part2: {part2(nums, deepcopy(boards))}')
