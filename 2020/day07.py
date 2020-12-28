from utils import *


class BagGraph:
    def __init__(self):
        self.graph = defaultdict(dict)

    def add_rule(self, s):
        color_type, contents = s.split(' bags contain ')
        if contents == 'no other bags.':
            self.graph[color_type]
            return
        contents = [x for x in re.split(r' bags?[,.]\s*', contents) if x]
        for content in contents:
            num, _type = content.split(' ', 1)
            self.graph[color_type][_type] = int(num)

    def reverse(self):
        rs = BagGraph()
        new_graph = rs.graph
        for color in self.graph:
            new_graph[color]
            for sub_color, num in self.graph[color].items():
                new_graph[sub_color][color] = num
        return rs

    def uniq_reachable_bags(self, bag):
        rs = set()
        def dfs(bag):
            if bag not in rs:
                rs.add(bag)
                for sub_bag in self.graph[bag]:
                    dfs(sub_bag)
        dfs(bag)
        return rs - {bag}

    def count_reachable_bags(self, bag):
        def dfs(bag):
            return sum((dfs(sub_bag) + 1) * num for sub_bag, num in self.graph[bag].items())
        return dfs(bag)


def day7_1(graph):
    return len(graph.reverse().uniq_reachable_bags('shiny gold'))


def day7_2(graph):
    return graph.count_reachable_bags('shiny gold')


if __name__ == "__main__":
    graph = BagGraph()
    data(7, graph.add_rule)
    print(f'day7_1: {day7_1(graph)}')
    print(f'day7_2: {day7_2(graph)}')
    # day7_1: 128
    # day7_2: 20189
