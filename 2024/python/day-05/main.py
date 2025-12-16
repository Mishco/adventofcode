# --- Day 5: Print Queue ---


def part1():
    lines = open("../../rust/data/inputs/05.txt", mode="r").read().splitlines()
    result = 0
    rules = set()
    for line in lines:
        if len(line) <= 1:
            break
        pages = [int(n) for n in line.split("|") if n.isdigit()]
        rules.add((pages[0], pages[1]))

    for line in lines[len(rules) + 1 :]:
        update = [int(n) for n in line.split(",") if n.isdigit()]
        if all((b, a) not in rules for a, b, in zip(update, update[1:])):
            result += update[len(update) // 2]

    return result


def part2():
    lines = open("../../rust/data/inputs/05.txt", mode="r").read().split("\n\n")
    rules = lines[0].split("\n")
    updates = lines[1].split("\n")
    result = 0
    order = {}

    for r in rules:
        key, val = r.split("|")
        if key in order:
            order[key].append(val)
        else:
            order[key] = [val]

    for i in updates:
        update = i.split(",")
        flag = True
        j = 0
        while j < len(update):
            for k in range(j + 1, len(update)):
                if update[k] not in order[update[j]]:
                    update[k], update[j] = update[j], update[k]
                    j -= 1
                    flag = False
                    break
            j += 1
        if not flag:
            result += int(update[len(update) // 2])

    return result


if __name__ == "__main__":
    print(part1())
    print(part2())
