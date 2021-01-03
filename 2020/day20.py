# This solution is based on https://github.com/norvig/pytudes/blob/master/ipynb/Advent-2020.ipynb
from utils import *


def parse_tiles(sections: List[List[str]]) -> Dict[int, List[str]]:
    return {int(first.replace(":", "").split()[1]): rest
            for first, *rest in sections}


def edges(tile):
    return [tile[0],
            tile[-1],
            ''.join(x[0] for x in tile),
            ''.join(x[-1] for x in tile)]


def canonical(edge):
    return min(edge, edge[::-1])


def day20_1(tiles):
    edge_counter = Counter(canonical(e) for tile in tiles.values() for e in edges(tile))
    is_outermost = lambda edge: edge_counter[canonical(edge)] == 1
    is_corner = lambda tile: count(edges(tile), is_outermost) == 2
    corners = [id for id in tiles if is_corner(tiles[id])]
    return product(corners)


if __name__ == "__main__":
    tiles = parse_tiles(data(20, lines, '\n\n'))
    print(f'day20_1: {day20_1(tiles)}')
