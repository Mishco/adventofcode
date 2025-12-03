# --- Day 3: Lobby ---


def part1():
    lines = """
    987654321111111
    811111111111119
    234234234234278
    818181911112111
    """.split(
        "\n"
    )

    lines = open("../inputs/day-03.txt", "r").readlines()
    total = 0
    for line in lines:
        # split to digits
        numbers = line.strip()
        numbers = [int(num) for num in numbers]
        # find the two largest possible digits within numbers you cannot rearange order
        max_joltage = 0
        for i in range(len(numbers)):
            for j in range(i + 1, len(numbers)):
                joltage = numbers[i] * 10 + numbers[j]
                if joltage > max_joltage:
                    max_joltage = joltage
        # print(f"{max_one}{max_two}")
        total += max_joltage
    return total


def part2():
    lines = """987654321111111
       811111111111119
       234234234234278
       818181911112111""".split(
        "\n"
    )

    lines = open("../inputs/day-03.txt", "r").readlines()
    # total = 0
    max_digits = 12
    total = 0
    for line in lines:
        digits = [int(d) for d in line.strip()]
        n = len(digits)

        # Skip lines with fewer than 12 digits
        if n < max_digits:
            continue

        result = []
        start = 0

        for i in range(max_digits):
            # Find the max digit in the range where enough digits remain
            end = n - (max_digits - len(result)) + 1
            max_digit = -1
            max_pos = -1
            for j in range(start, end):
                if digits[j] > max_digit:
                    max_digit = digits[j]
                    max_pos = j
            result.append(max_digit)
            start = max_pos + 1

        max_joltage = int("".join(map(str, result)))
        total += max_joltage
    return total


if __name__ == "__main__":
    print("Part 1:", part1())
    print("Part 2:", part2())
