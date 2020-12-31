from utils import *

DIRECTIONS = dict(E=(1, 0), W=(-1, 0), N=(0, 1), S=(0, -1))
ROTATE_FACTOR = dict(R=(1, -1), L=(-1, 1))


def parse_instruction(s):
    return s[0], int(s[1:])


# This is a refactored version based on Peter Norvig's solution.
class Ship:
    def __init__(self, pos=(0, 0), direction=DIRECTIONS['E']):
        self.direction = direction
        self.pos = pos

    def run(self, instructions, waypoint=False):
        for action, val in instructions:
            if action in 'LR':
                self.direction = self.rotate(action, val, *self.direction)
            elif action == 'F':
                self.pos = self.go(val, *self.pos, *self.direction)
            else:
                field = 'direction'if waypoint else 'pos'
                setattr(self, field, self.go(val, *getattr(self, field), *DIRECTIONS[action]))

    def rotate(self, action, degrees, x, y):
        dx, dy = ROTATE_FACTOR[action]
        for _ in range(degrees // 90):
            x, y = y * dx, x * dy
        return x, y

    def go(self, val, x, y, dx, dy):
        return x + dx * val, y + dy * val

    def manhattan_distance(self):
        return sum(map(abs, self.pos))


def day12_1(instructions):
    ship = Ship()
    ship.run(instructions)
    return ship.manhattan_distance()


def day12_2(instructions):
    ship = Ship(direction=(10, 1))
    ship.run(instructions, True)
    return ship.manhattan_distance()


if __name__ == "__main__":
    instructions = data(12, parse_instruction)
    print(f"day12_1: {day12_1(instructions)}")
    print(f"day12_2: {day12_2(instructions)}")
    # day12_1: 1603
    # day12_2: 52866
    # python3 day12.py  0.03s user 0.01s system 88% cpu 0.049 total
