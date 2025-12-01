# --- Day 3: Perfectly Spherical Houses in a Vacuum ---

def solve_part_one(lines: list[str]):
    north = '^'
    south = 'v'
    east = '>'
    west = '<'
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
    assert solve_part_one(['>']) == 2
    assert solve_part_one(['^>v<']) == 4
    assert solve_part_one(['^v^v^v^v^v']) == 2

    lines = open('../inputs/day-03.txt').read().strip().splitlines()
    result = solve_part_one(lines)
    print(f"Part 1: {result}")


def part2():
    pass


if __name__ == '__main__':
    part1()
    part2()
