import re


def part1():
    pattern = r"mul\((\d+),\s*(\d+)\)"
    total = 0
    with open('../../rust/data/inputs/03.txt', mode='r') as f:
        lines = f.read().splitlines()

        for line in lines:
            for item in re.finditer(pattern, line):
                first = int(item[1])
                second = int(item[2])

                total += (first * second)
    return total


def part2():
    pass


if __name__ == '__main__':
    print(part1())
    print(part2())
