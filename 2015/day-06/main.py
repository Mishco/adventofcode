# --- Day 6: Probably a Fire Hazard ---


def parse_instruction(line):
    parts = line.strip().split()
    if parts[0] == "turn":
        action = parts[1]
        start = tuple(map(int, parts[2].split(",")))
        end = tuple(map(int, parts[4].split(",")))
    else:  # toggle
        action = "toggle"
        start = tuple(map(int, parts[1].split(",")))
        end = tuple(map(int, parts[3].split(",")))
    return action, start, end


def part1():
    data = open("../inputs/day-06.txt", "r").readlines()
    grid = [[0 for _ in range(1000)] for _ in range(1000)]

    for line in data:
        action, start, end = parse_instruction(line)

        for x in range(start[0], end[0] + 1):
            for y in range(start[1], end[1] + 1):
                if action == "on":
                    grid[x][y] = 1
                elif action == "off":
                    grid[x][y] = 0
                else:  # toggle
                    grid[x][y] = 1 - grid[x][y]

    return sum(sum(row) for row in grid)


def part2():
    data = open("../inputs/day-06.txt", "r").readlines()
    grid = [[0 for _ in range(1000)] for _ in range(1000)]

    for line in data:
        action, start, end = parse_instruction(line)

        for x in range(start[0], end[0] + 1):
            for y in range(start[1], end[1] + 1):
                if action == "on":
                    grid[x][y] += 1
                elif action == "off":
                    grid[x][y] = max(0, grid[x][y] - 1)
                else:  # toggle
                    grid[x][y] += 2

    return sum(sum(row) for row in grid)


if __name__ == "__main__":
    print("Part 1:", part1())
    print("Part 2:", part2())
