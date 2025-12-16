import os
import sys
from collections import Counter

sys.path.insert(
    0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "utils"))
)
from benchmark import benchmark


@benchmark
def part1():
    result = 0
    columns1, columns2 = load_input_to_two_cols()

    for i in range(0, len(columns1)):
        left = int(columns1[i])
        right = int(columns2[i])
        result += abs(left - right)

    return result


@benchmark
def part2():
    columns1, columns2 = load_input_to_two_cols()
    result = 0
    count_map = Counter()

    for num in columns2:
        count_map[num] += 1

    for left in columns1:
        count = count_map.get(left)
        if count:
            result += int(left) * int(count)

    return result


def load_input_to_two_cols():
    columns1 = []
    columns2 = []

    with open("../../rust/data/inputs/01.txt", mode="r") as f:
        lines = f.read().splitlines()

        for line in lines:
            first, second = line.split()
            columns1.append(first)
            columns2.append(second)

    columns1.sort()
    columns2.sort()
    return columns1, columns2


if __name__ == "__main__":
    print(part1())
    print(part2())
