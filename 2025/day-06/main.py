# --- Day 6: Trash Compactor ---
from operator import add, mul


def part1():
    # *nums, ops = open('example.txt', 'r')
    *nums, ops = open("../inputs/day-06.txt", "r")

    # add '+' and mul '*' functions based on operator characters
    new_ops = []
    for c in ops.split():
        if c == "+":
            new_ops.append(add)
        else:
            new_ops.append(mul)
    ops = new_ops

    result, *ints = [[*map(int, l.split())] for l in nums]

    for row in ints:
        result = [op(a, b) for op, a, b in zip(ops, result, row)]
    p1 = sum(result)
    return p1


def part2():
    # *nums, ops = open('example.txt', 'r')
    *nums, ops = open("../inputs/day-06.txt", "r")
    # add '+' and mul '*' functions based on operator characters
    new_ops = []
    for c in ops.split():
        if c == "+":
            new_ops.append(add)
        else:
            new_ops.append(mul)
    ops = new_ops

    result = 0
    col = 0
    acc = None
    for digits in zip(*nums):
        if not (c := "".join(digits).strip()):
            col += 1
            result += acc
            # clean up for next column
            acc = None
            c = 0
        else:
            d = int(c)
            acc = d if acc is None else ops[col](acc, d)

    if acc is not None:
        result += acc

    return result


if __name__ == "__main__":
    print("Part 1:", part1())
    print("Part 2:", part2())
