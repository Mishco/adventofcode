# --- Day 3: Perfectly Spherical Houses in a Vacuum ---

north = "^"
south = "v"
east = ">"
west = "<"


def solve_part_one(lines: list[str]):
    visited_map = {}
    x, y = 0, 0
    # > delivers presents to 2 houses: one at the starting location, and one to the east.
    # ^>v< delivers presents to 4 houses in a square, including twice to the house at his starting/ending location.
    # ^v^v^v^v^v delivers a bunch of presents to some very lucky children at only 2 houses.
    visited_map[(x, y)] = visited_map.get((x, y), 0) + 1

    for direction in lines[0].strip():
        if direction == north:
            y += 1
        elif direction == south:
            y -= 1
        elif direction == east:
            x += 1
        elif direction == west:
            x -= 1

        visited_map[(x, y)] = visited_map.get((x, y), 0) + 1
    return len(visited_map)


def solve_part_two(lines: list[str]):
    pass


def part1():
    assert solve_part_one([">"]) == 2
    assert solve_part_one(["^>v<"]) == 4
    assert solve_part_one(["^v^v^v^v^v"]) == 2

    lines = open("../inputs/day-03.txt").read().strip().splitlines()
    result = solve_part_one(lines)
    print(f"Part 1: {result}")


def part2():
    # lines = ['^v'] ### 3
    # lines = ['^>v<'] ### 5
    # lines = ['^v^v^v^v^v'] ### 11
    lines = open("../inputs/day-03.txt").read().strip().splitlines()
    visited_map = {}
    santa_x, santa_y = 0, 0
    robo_x, robo_y = 0, 0

    # Both start at the origin
    visited_map[(0, 0)] = visited_map.get((0, 0), 0) + 1

    directions = lines[0].strip()
    for i, direction in enumerate(directions):
        # Santa even indices (0, 2, 4) Robo-Santa odd indices (1, 3, 5)
        if i % 2 == 0:
            # Santa
            if direction == north:
                santa_y += 1
            elif direction == south:
                santa_y -= 1
            elif direction == east:
                santa_x += 1
            elif direction == west:
                santa_x -= 1
            visited_map[(santa_x, santa_y)] = visited_map.get((santa_x, santa_y), 0) + 1
        else:
            # Robo-Santa
            if direction == north:
                robo_y += 1
            elif direction == south:
                robo_y -= 1
            elif direction == east:
                robo_x += 1
            elif direction == west:
                robo_x -= 1
            visited_map[(robo_x, robo_y)] = visited_map.get((robo_x, robo_y), 0) + 1

    total = len(visited_map)
    print(f"Part 2: {total}")
    return total


if __name__ == "__main__":
    part1()
    part2()
