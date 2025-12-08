# --- Day 7: Laboratories ---
from collections import defaultdict


def part1():
    data = open("../inputs/day-07.txt").readlines()
    # data = open('example.txt').readlines()

    split_points = set()
    beams = [defaultdict(lambda: 0) for _ in range(len(data))]
    # find the start
    beams[0][data[0].find("S")] = 1

    for row in range(1, len(data)):
        for col, amount in beams[row - 1].items():
            if data[row][col] == "^":
                beams[row][col - 1] = beams[row][col - 1] + amount
                beams[row][col + 1] = beams[row][col + 1] + amount
                split_points.add((col, row))
            else:
                beams[row][col] = beams[row][col] + amount

    return len(split_points)


def part2():
    data = open("../inputs/day-07.txt").readlines()
    # data = open('example.txt').readlines()

    beams = [defaultdict(lambda: 0) for _ in range(len(data))]
    # find the start
    beams[0][data[0].find("S")] = 1

    for row in range(1, len(data)):
        for col, amount in beams[row - 1].items():
            if data[row][col] == "^":
                beams[row][col - 1] = beams[row][col - 1] + amount
                beams[row][col + 1] = beams[row][col + 1] + amount
            else:
                beams[row][col] = beams[row][col] + amount

    return sum(beams[-1].values())


if __name__ == "__main__":
    print("Part 1:", part1())
    print("Part 2:", part2())
