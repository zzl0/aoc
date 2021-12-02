from utils import *


UNITS = {
    'forward': 1 + 0j,
    'down': 0 + 1j,
    'up': 0 + -1j,
}


def parse_cmd(s: str) -> Tuple:
    cmd, val = s.split()
    return cmd, int(val)


def part1(commands: Tuple) -> int:
    pos = 0 + 0j
    for cmd, val in commands:
        pos += UNITS[cmd] * val
    return int(pos.real * pos.imag)


def part2(commands: Tuple) -> int:
    x, y, aim = 0, 0, 0
    for cmd, val in commands:
        if cmd == 'forward':
            x += val
            y += aim * val
        elif cmd == 'down':
            aim += val
        else: # up
            aim -= val
    return x * y


if __name__ == "__main__":
    commands = data(2, parse_cmd)
    print(f'part1: {part1(commands)}')
    print(f'part2: {part2(commands)}')
