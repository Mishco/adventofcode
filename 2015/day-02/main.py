# --- Day 2: I Was Told There Would Be No Math ---

def solve_part_one(lines: list[str]):
    total = 0
    for line in lines:
        numbers = line.split("x")
        l = int(numbers[0])
        w = int(numbers[1])
        h = int(numbers[2])

        side_a = l * w
        side_b = w * h
        side_c = h * l
        surface = (2 * side_a) + (2 * side_b) + (2 * side_c)
        min_side = min(side_a, side_b, side_c)
        result = surface + min_side
        total += result
    return total


def solve_part_two(lines: list[str]):
    total_feet_of_ribbon = 0
    for line in lines:
        numbers = line.split("x")
        l = int(numbers[0])
        w = int(numbers[1])
        h = int(numbers[2])

        perimeters = [2 * (l + w), 2 * (w + h), 2 * (h + l)]
        min_perimeter = min(perimeters)
        bow = l * w * h
        result = min_perimeter + bow
        total_feet_of_ribbon += result
    return total_feet_of_ribbon


def part1():
    lines = ["2x3x4", "1x1x10"]
    assert solve_part_one(lines) == 101  # 58 feet + 43 feet

    lines = open('..\inputs\day-02.txt').readlines()
    total = solve_part_one(lines)
    print(f"part1: {total}")


def part2():
    lines = ["2x3x4", "1x1x10"]
    assert solve_part_two(lines) == 48  # 34 feet + 14 feet

    lines = open('..\inputs\day-02.txt').readlines()
    total = solve_part_two(lines)
    print(f"part2: {total}")


if __name__ == '__main__':
    part1()
    part2()
