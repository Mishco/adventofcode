# --- Day 11: Reactor ---
from functools import cache


def part1():
    # data = open('example.txt').readlines()
    data = open("../inputs/day-11.txt").readlines()
    graph = {}

    for line in data:
        device, connections = line.strip().split(":")
        device = device.strip()
        # Parse the connections - they are space-separated
        connections = connections.strip().split()
        graph[device] = connections

    @cache
    def paths(start, stop):
        return (
            1
            if start == stop
            else sum(paths(step, stop) for step in graph.get(start, []))
        )

    return paths("you", "out")


def part2():
    # data = open('example2.txt').readlines()
    data = open("../inputs/day-11.txt").readlines()
    graph = {}

    for line in data:
        device, connections = line.strip().split(":")
        device = device.strip()
        # Parse the connections - they are space-separated
        connections = connections.strip().split()
        graph[device] = connections

    @cache
    def paths(start, stop):
        return (
            1
            if start == stop
            else sum(paths(step, stop) for step in graph.get(start, []))
        )

    tot = paths("svr", "dac") * paths("dac", "fft") * paths("fft", "out") + paths(
        "svr", "fft"
    ) * paths("fft", "dac") * paths("dac", "out")
    return tot


if __name__ == "__main__":
    print("Part 1:", part1())
    print("Part 2:", part2())
