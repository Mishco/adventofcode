import os
import sys

sys.path.insert(
    0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "utils"))
)
from benchmark import benchmark


@benchmark
def part1():
    safe_count = 0
    with open("../../rust/data/inputs/02.txt", mode="r") as f:
        lines = f.read().splitlines()

        for line in lines:
            nums = line.split()
            if checking_level(nums):
                safe_count += 1

    return safe_count


def checking_level(nums: []) -> bool:
    is_increasing = True
    is_decreasing = True

    for i in range(0, len(nums) - 1):
        curr_item = int(nums[i])
        next_item = int(nums[i + 1])

        diff = abs(next_item - curr_item)
        if diff not in range(1, 4):
            return False

        if curr_item < next_item:
            is_decreasing = False
        else:
            is_increasing = False

    return is_increasing or is_decreasing


@benchmark
def part2():
    safe_count = 0
    with open("../../rust/data/inputs/02.txt", mode="r") as f:
        lines = f.read().splitlines()
        for line in lines:
            nums = line.split()
            if checking_level(nums):
                safe_count += 1
            else:
                for i in range(0, len(nums)):
                    cleaned_level = nums.copy()
                    cleaned_level.pop(i)
                    if checking_level(cleaned_level):
                        safe_count += 1
                        break
    return safe_count


if __name__ == "__main__":
    print(part1())
    print(part2())
