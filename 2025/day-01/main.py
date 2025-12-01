# --- Day 1: Secret Entrance ---


def part1():
    starting_pos = 50
    zero_count = 0
    # lines = open('example.txt').readlines()
    lines = open("../inputs/day-01.txt", "r").readlines()

    for line in lines:
        direction = line[0]
        distance = int(line[1:].strip())

        if direction == "L":
            starting_pos = (starting_pos - distance) % 100
        else:
            starting_pos = (starting_pos + distance) % 100

        if starting_pos == 0:
            zero_count += 1

    print(f"Part 1: {zero_count}")


def part2():
    # lines = open('example.txt').readlines()
    lines = open("../inputs/day-01.txt", "r").readlines()
    starting_pos = 50
    zero_count = 0

    for line in lines:
        direction = line[0]
        distance = int(line[1:].strip())

        if direction == "L":
            new_pos = (starting_pos - distance) % 100
            zero_count += distance // 100
            if starting_pos != 0 and (new_pos > starting_pos or new_pos == 0):
                zero_count += 1
            starting_pos = new_pos

        else:
            new_pos = (starting_pos + distance) % 100
            zero_count += distance // 100
            if new_pos < starting_pos:
                zero_count += 1
            starting_pos = new_pos

    print(f"Part 2: {zero_count}")


if __name__ == "__main__":
    part1()
    part2()
