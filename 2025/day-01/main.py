# --- Day 1: Secret Entrance ---


def part1():
    starting_point = 50
    data = open('example.txt').readlines()
    top_number = 99
    low_number = 0
    # L from 0 makes 99
    # R from 99 makes 0
    # L68 or
    for row in data:
        charac = row[0]
        how_much  = int(row[1:])

        if charac == 'L':
            if low_number > starting_point - how_much:
                starting_point = top_number - (how_much - starting_point) + 1
            else:
                starting_point = starting_point - how_much
        elif charac == 'R':
            if top_number < starting_point + how_much:
                starting_point = top_number - (how_much + starting_point) + 1
            else:
                starting_point = starting_point - how_much


def part2():
    # lines = open('example.txt').readlines()
    lines = open('input.txt', 'r').readlines()
    starting_pos = 50
    zero_count = 0

    for line in lines:
        direction = line[0]
        distance = int(line[1:].strip())

        if direction == 'L':
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
    # part1()
    part2()
