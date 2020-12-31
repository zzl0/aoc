from utils import *


@dataclass
class BusData:
    earliest: int
    raw_buses: List[str]

    def buses(self):
        return [int(x) for x in self.raw_buses if x != 'x']

    def bus_idxs(self):
        return [(int(x), i) for i, x in enumerate(self.raw_buses) if x != 'x']


def parse(s):
    earliest, buses = s.split('\n')
    buses = [x for x in buses.split(',')]
    return BusData(int(earliest), buses)


def day13_1(bus_data):
    def next_depart(ts, bus):
        return int((ts + bus - 1) / bus) * bus
    next_departs = [(next_depart(bus_data.earliest, bus), bus) for bus in bus_data.buses()]
    ts, bus = min(next_departs)
    return bus * (ts - bus_data.earliest)


def day13_2(bus_data):
    bus_idxs = sorted(bus_data.bus_idxs(), reverse=True)
    ts, step = 0, 1
    for bus, idx in bus_idxs:
        while (ts + idx) % bus != 0:
            ts += step
        step *= bus
    return ts


if __name__ == "__main__":
    bus_data = data(13, parse, "_do_not_exist_")[0]
    print(f"day13_1: {day13_1(bus_data)}")
    print(f"day13_2: {day13_2(bus_data)}")
    # day13_1: 2305
    # day13_2: 552612234243498
    # python3 day13.py  0.03s user 0.01s system 91% cpu 0.048 total
