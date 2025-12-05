# --- Day 5: Cafeteria ---


def part1():
    fresh_ingredients = 0
    available_ingredients = 0

    lines = open("../inputs/day-05.txt", "r").read().strip()
    # lines = open('example.txt', 'r').read().strip()
    # lines until first blank line are fresh ingredients
    fresh_section, available_section = lines.split("\n\n", 1)

    fresh_ranges = []
    for line in fresh_section.strip().split("\n"):
        if "-" in line:
            start, end = map(int, line.split("-"))
            fresh_ranges.append((start, end))

    available_ids = [
        int(line.strip())
        for line in available_section.strip().split("\n")
        if line.strip()
    ]

    # how many fresh ingredients are also in available ingredients
    count = 0
    for ingredient_id in available_ids:
        for start, end in fresh_ranges:
            if start <= ingredient_id <= end:
                count += 1
                break

    return count


def part2():
    # lines = open('example.txt', 'r').read().strip()
    lines = open("../inputs/day-05.txt", "r").read().strip()

    fresh_section, _ = lines.split("\n\n", 1)
    fresh_ids = set()
    ranges = []
    for line in fresh_section.strip().split("\n"):
        if "-" in line:
            start, end = map(int, line.split("-"))
            ranges.append((start, end))

    ranges.sort()
    merged = []

    for start, end in ranges:
        if merged and start <= merged[-1][1] + 1:
            merged[-1] = (merged[-1][0], max(merged[-1][1], end))
        else:
            # No overlap - add as new range
            merged.append((start, end))

    total = 0
    for start, end in merged:
        total += end - start + 1

    return total


if __name__ == "__main__":
    print("Part 1:", part1())
    print("Part 2:", part2())
