from utils import *


def parse_grid(s, ndim=3):
    grid = {}
    for x, line in enumerate(s.split('\n')):
        for y, c in enumerate(line):
            cube = (x, y) + (0,) * (ndim - 2)
            grid[cube] = c == '#'
    return grid


@functools.lru_cache()
def deltas(ndim):
    return set(itertools.product((-1, 0, 1), repeat=ndim)) - {(0,) * ndim}


@functools.lru_cache(maxsize=None)
def neighbors(cube):
    return [tuple(i + di for i, di in zip(cube, delta)) for delta in deltas(len(cube))]


def new_generation(grid):
    new_grid = {}
    dim_ranges = [range(min-1, max+2) for (max, min) in map(max_min, zip(*grid.keys()))]
    for cube in itertools.product(*dim_ranges):
        actives = sum(1 for _cube in neighbors(cube) if grid.get(_cube))
        new_grid[cube] = actives == 3 or (actives == 2 and grid.get(cube, False))
    return new_grid


def day17_1(state, cycles=6, ndim=3):
    grid = parse_grid(state, ndim)
    for _ in range(cycles):
        grid = new_generation(grid)
    return sum(grid.values())


def day17_2(state, cycles=6, ndim=4):
    return day17_1(state, cycles, ndim)


def show(grid):
    _x, _rest = None, None
    for x, y, *rest in sorted(grid.keys(), key=lambda x: x[2:]):
        rest = tuple(rest)
        if rest != _rest: print(f'\n===== rest={rest} ======', end='')
        if x != _x: print()
        _x, _rest = x, rest
        print('.#'[grid[(x, y) + rest]], end='')
    print()


if __name__ == "__main__":
    state = data(17, sep='\n\n')[0]
    print(f"day17_1: {day17_1(state)}")
    print(f"day17_2: {day17_2(state)}")
    # day17_1: 207
    # day17_2: 2308
    # python3 day17.py  11.34s user 0.35s system 99% cpu 11.735 total
