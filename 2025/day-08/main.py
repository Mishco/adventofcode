# --- Day 8: Playground  ---
from collections import defaultdict


class UnionFind:
    """Union-Find (Disjoint Set Union) data structure for tracking connected components."""

    def __init__(self, n):
        """Initialize n separate sets."""
        self.parent = {i: i for i in range(n)}

    def find(self, x):
        """Find the root of the set containing x with path compression."""
        if x == self.parent[x]:
            return x
        self.parent[x] = self.find(self.parent[x])
        return self.parent[x]

    def union(self, x, y):
        """Union the sets containing x and y. Returns True if they were merged."""
        root_x = self.find(x)
        root_y = self.find(y)

        if root_x == root_y:
            return False  # Already in the same set

        self.parent[root_x] = root_y
        return True

    def get_component_sizes(self):
        """Get the sizes of all connected components."""
        sizes = defaultdict(int)
        for i in self.parent:
            sizes[self.find(i)] += 1
        return sorted(sizes.values())


def parse_and_setup():
    data = open("../inputs/day-08.txt").readlines()
    # data = open('example.txt').readlines()

    parsed = []
    for line in data:
        x, y, z = line.split(",")
        parsed.append((int(x), int(y), int(z)))

    map_data = []
    for i, (x1, y1, z1) in enumerate(parsed):
        for j, (x2, y2, z2) in enumerate(parsed):
            distance = (x1 - x2) ** 2 + (y1 - y2) ** 2 + (z1 - z2) ** 2
            if i > j:
                map_data.append((distance, i, j))
    map_data = sorted(map_data)

    return parsed, map_data


def part1():
    """
    Connect the 1000 closest pairs of junction boxes.
    Return the product of the three largest circuit sizes.
    """
    parsed, data = parse_and_setup()
    uf = UnionFind(len(parsed))

    # Process first 1000 connections
    connections = 0
    for t, (_d, i, j) in enumerate(data):
        if t == 1000:
            # Calculate circuit sizes after 1000 connections
            sizes = uf.get_component_sizes()
            result = sizes[-1] * sizes[-2] * sizes[-3]
            return result

        if uf.union(i, j):
            connections += 1

    return None


def part2():
    """
    Continue connecting until all junction boxes form a single circuit.
    Return the product of x-coordinates of the last two boxes connected.
    """
    parsed, data = parse_and_setup()
    uf = UnionFind(len(parsed))

    # Process connections until we have a single circuit
    connections = 0
    for t, (_d, i, j) in enumerate(data):
        if uf.union(i, j):
            connections += 1

            # When we form a single circuit (n-1 connections)
            if connections == len(parsed) - 1:
                result = parsed[i][0] * parsed[j][0]
                return result

    return None


if __name__ == "__main__":
    print("Part 1:", part1())
    print("Part 2:", part2())
