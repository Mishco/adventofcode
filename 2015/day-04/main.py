# --- Day 4: The Ideal Stocking Stuffer ---
import hashlib


def solve(data: str, desired_zeros: str) -> int:
    orig_data = data
    iteration = int(0)
    while True:
        res = hashlib.md5(data.encode())
        if res.hexdigest().startswith(desired_zeros):
            # print(data.encode())
            return iteration
        else:
            iteration += 1
            data = orig_data + str(iteration)


def part1():
    assert solve("abcdef", "00000") == 609043
    assert solve("pqrstuv", "00000") == 1048970
    data = open("../inputs/day-04.txt").readlines()[0].strip()

    res = solve(data, "00000")
    print(f"part1: {res}")


def part2():
    data = open("../inputs/day-04.txt").readlines()[0].strip()
    res = solve(data, "000000")
    print(f"part2: {res}")


if __name__ == "__main__":
    part1()
    part2()
