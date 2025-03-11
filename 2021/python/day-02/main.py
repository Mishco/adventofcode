test_input = """forward 5
down 5
forward 8
up 3
down 8
forward 2
"""

def part1(input_str: str):
    horizontal_position = 0
    depth = 0
    for line in input_str.splitlines():
        instruction, value = line.split(" ")
        if instruction == "forward":
            horizontal_position += int(value)
        elif instruction == "down":
            depth += int(value)
        elif instruction == "up":
            depth -= int(value)

    return horizontal_position * depth

def part2(input_str: str):
    horizontal_position = 0
    depth = 0
    aim = 0
    for line in input_str.splitlines():
        instruction, value = line.split(" ")
        if instruction == "forward":
            horizontal_position += int(value)
            if aim == 0:
                pass
            else:
                depth += int(value) * aim

        elif instruction == "down":
            aim += int(value)
        elif instruction == "up":
            aim -= int(value)

    return horizontal_position * depth


if __name__ == '__main__':
    f = open('../inputs/2021-02.txt', mode='r').read()

    res = part1(test_input)
    assert res == 150
    res = part1(f)
    print(res)

    res = part2(test_input)
    assert res == 900
    res = part2(f)
    print(res)