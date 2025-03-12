import time


def part1():
    safe_count = 0
    with open('../../rust/data/inputs/02.txt', mode='r') as f:
        lines = f.read().splitlines()

        for line in lines:
            nums = line.split()
            if checking_level(nums):
                safe_count += 1

    return safe_count


def checking_level(nums):
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


def part2():
    pass


if __name__ == '__main__':
    start_time = time.perf_counter()
    print(part1())
    print(part2())
    end_time = time.perf_counter()
    elapsed_time = (end_time - start_time) * 1_000_000  # Convert to microseconds
    print(f"Execution time: {elapsed_time:.4f} µs")
