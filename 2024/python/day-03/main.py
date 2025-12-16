import re
import sys
import os

sys.path.insert(
    0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "utils"))
)
from benchmark import benchmark


@benchmark
def part1():
    pattern = r"mul\((\d+),\s*(\d+)\)"
    total = 0
    with open("../../rust/data/inputs/03.txt", mode="r") as f:
        lines = f.read().splitlines()

        for line in lines:
            for item in re.finditer(pattern, line):
                first = int(item[1])
                second = int(item[2])

                total += first * second
    return total


@benchmark
def part2():
    pattern = re.compile(r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)")
    is_enabled = True
    result = 0

    with open("../../rust/data/inputs/03.txt", mode="r") as f:
        lines = f.read().splitlines()
        for line in lines:
            for match in pattern.finditer(line):
                if match.group(0).startswith("mul"):
                    x = int(match.group(1))
                    y = int(match.group(2))
                    if is_enabled:
                        result += x * y
                elif match.group(0).startswith("don't()"):
                    is_enabled = False
                elif match.group(0).startswith("do()"):
                    is_enabled = True
    return result


if __name__ == "__main__":
    print(part1())
    print(part2())
