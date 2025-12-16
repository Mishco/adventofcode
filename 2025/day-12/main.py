def part1():
    data = open('../inputs/day-12.txt').read()

    chunks = data.replace(":", "").replace("x", " ").split("\n\n")
    regions = chunks.pop()
    areas = [x.count("#") for x in chunks]

    a = 0
    for line in regions.splitlines():
        w, h, *counts = map(int, line.split())
        area_needed = sum(areas[i] * x for i, x in enumerate(counts))
        a += area_needed <= w * h

    return a


def part2():
    pass


if __name__ == "__main__":
    print("Part 1:", part1())
    print("Part 2:", part2())
