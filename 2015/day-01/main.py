# --- Day 1: Not Quite Lisp ---

def solve(lines: list[str]) -> int:
    ground_floor = 0
    for i in lines[0].strip():
        curr_item = i
        if curr_item == "(":
            ground_floor += 1
        else:
            ground_floor -= 1
    return ground_floor


def part1():
    lines = open("..\inputs\day-01.txt").readlines()
    # Test cases
    assert solve(["(())"]) == 0
    assert solve(["))((((("]) == 3
    assert solve(["())"]) == -1
    assert solve([")))"]) == -3
    # Actual input
    ground_floor = solve(lines)
    print(f"part 1: {ground_floor}")


def part2():
    lines = open("..\inputs\day-01.txt").readlines()
    ground_floor = 0
    position = 0
    for i in lines[0].strip():
        position += 1
        curr_item = i
        if curr_item == "(":
            ground_floor += 1
        else:
            ground_floor -= 1
        if ground_floor == -1:
            break
    print(f"part 2: {position}")


if __name__ == '__main__':
    part1()
    part2()
