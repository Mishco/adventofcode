# --- Day 4: Printing Department ---


def part1():
    lines = """..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
    """
    lines = open("../inputs/day-04.txt", "r").read().strip()
    # lines
    lines = lines.split("\n")

    width = len(lines[0])
    height = len(lines)
    total = 0

    for y in range(height):
        for x in range(width):
            if lines[y][x] != "@":
                continue
            count = 0
            for dy in [-1, 0, 1]:
                for dx in [-1, 0, 1]:
                    if dx == 0 and dy == 0:
                        continue
                    nx, ny = x + dx, y + dy
                    if 0 <= nx < width and 0 <= ny < height and lines[ny][nx] == "@":
                        count += 1
            if count < 4:
                total += 1

    return total


def part2():
    total_removed = 0
    grid = [
        list(line)
        for line in open("../inputs/day-04.txt", "r").read().strip().split("\n")
    ]
    #     grid = """..@@.@@@@.
    # @@@.@.@.@@
    # @@@@@.@.@@
    # @.@@@@..@.
    # @@.@@@@.@@
    # .@@@@@@@.@
    # .@.@.@.@@@
    # @.@@@.@@@@
    # .@@@@@@@@.
    # @.@.@@@.@.""".strip().split('\n')
    width = len(grid[0])
    height = len(grid)
    while True:
        accessible = []

        for y in range(height):
            for x in range(width):
                if grid[y][x] != "@":
                    continue

                count = 0
                for dy in [-1, 0, 1]:
                    for dx in [-1, 0, 1]:
                        if dx == 0 and dy == 0:
                            continue
                        nx, ny = x + dx, y + dy
                        if 0 <= nx < width and 0 <= ny < height and grid[ny][nx] == "@":
                            count += 1
                if count < 4:
                    accessible.append((x, y))
        if not accessible:
            break

        for x, y in accessible:
            grid[y][x] = "."

        total_removed += len(accessible)

    return total_removed


if __name__ == "__main__":
    print("Part 1:", part1())
    print("Part 2:", part2())
