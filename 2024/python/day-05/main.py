# --- Day 5: Print Queue ---


def part1():
    lines = open('../../rust/data/inputs/05.txt', mode='r').read().splitlines()
    result = 0
    rules = set()
    for line in lines:
        if len(line) <= 1:
            break
        pages = [int(n) for n in line.split("|") if n.isdigit()]
        rules.add((pages[0], pages[1]))

    for line in lines[len(rules) + 1:]:
        update = [int(n) for n in line.split(",") if n.isdigit()]
        if all((b, a) not in rules for a, b, in zip(update, update[1:])):
            result += update[len(update) // 2]

    return result


def part2():
    pass


if __name__ == '__main__':
    print(part1())
    print(part2())
