from utils import *


"""
       / \     / \
     /     \ /     \
    | -1,1  |  0,1  |                    y
    |       |       |                   /
   / \     / \     / \                 /
 /     \ /     \ /     \              /
| -1,0  |  0,0  |  1,0  |             -----> x
|       |       |       |
 \     / \     / \     /
   \ /     \ /     \ /
    | 0,-1  |  1,-1 |
    |       |       |
     \     / \     /
       \ /     \ /
"""
DIRECTIONS = {
    'e': (1, 0),
    'w': (-1, 0),
    'se': (1, -1),
    'sw': (0, -1),
    'ne': (0, 1),
    'nw': (-1, 1),
}

def parse_directions(line):
    return re.findall('e|w|se|sw|ne|nw', line)


def walk(directions, start=(0,0)):
    x, y = start
    for dir in directions:
        dx, dy = DIRECTIONS[dir]
        x, y = x + dx, y + dy
    return x, y


def day24_1(directions):
    counter = Counter(map(walk, directions))
    return sum(counter[k] % 2 for k in counter)


def get_adjs(p):
    x, y = p
    return [(x + dx, y + dy) for dx, dy in DIRECTIONS.values()]


def next_generation(blacks):
    counter = Counter([x for p in blacks for x in get_adjs(p)])
    return {k for k, cnt in counter.items()
              if (k not in blacks and cnt == 2) or (k in blacks and cnt in (1, 2))}


def day24_2(directions, times=100):
    grid = defaultdict(bool)  # False means 'white', True means 'black'
    for x, y in map(walk, directions):
        grid[(x, y)] = not grid[(x, y)]

    blacks = {k for k in grid if grid[k]}
    for _ in range(times):
        blacks = next_generation(blacks)
    return len(blacks)


if __name__ == "__main__":
    directions = data(24, parse_directions)
    print(f'day24_1: {day24_1(directions)}')
    print(f'day24_2: {day24_2(directions)}')
    # day24_1: 400
    # day24_2: 3768
    # python3 day24.py  0.67s user 0.01s system 99% cpu 0.694 total
