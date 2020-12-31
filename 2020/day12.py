from utils import *


def parse_instruction(s):
    action = s[0]
    return action, int(s[1:])


class Ship:
    def __init__(self, direction='E', x=0, y=0):
        self.direction = direction
        self.x = x # E/W
        self.y = y # N/S
        self.flag = {'E': 1, 'W': -1, 'N': 1, 'S': -1}

    def run_instruction(self, instruction):
        action, val = instruction
        if action == 'F':
            action = self.direction

        if action in 'EW':
            self.x += val * self.flag[action]
        elif action in 'NS':
            self.y += val * self.flag[action]
        else:
            self.direction = Ship.next_direction(self.direction, instruction)

    def manhattan_distance(self):
        return abs(self.x) + abs(self.y)

    @staticmethod
    def next_direction(direction, instruction):
        action, val = instruction
        if not action in 'LR':
            return direction
        directions = 'ESWN'
        offset = val // 90 * (1 if action == 'R' else -1)
        idx = directions.index(direction)
        return directions[(idx + offset) % len(directions)]


def test():
    next_direction = Ship.next_direction
    assert next_direction('W', ('R', 90)) == 'N'
    assert next_direction('W', ('R', 180)) == 'E'
    assert next_direction('W', ('R', 270)) == 'S'
    assert next_direction('W', ('L', 90)) == 'S'
    assert next_direction('W', ('L', 180)) == 'E'
    assert next_direction('W', ('L', 270)) == 'N'
    print('test passes')
test()


def day12_1(instructions):
    pos = Ship()
    for instruction in instructions:
        pos.run_instruction(instruction)
    return pos.manhattan_distance()


if __name__ == "__main__":
    instructions = data(12, parse_instruction)
    print(f"day12_1: {day12_1(instructions)}")
