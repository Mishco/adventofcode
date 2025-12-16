# --- Day 7: Bridge Repair ---


def part1():
    operators = ["+", "*"]
    # all_lines = open(r'..\..\rust\data\examples\07.txt').readlines()
    all_lines = open(r"..\..\rust\data\inputs\07.txt").readlines()
    total_result = 0
    output_list = []

    for line in all_lines:
        result, values = line.split(":")
        numbers = values.strip().split()
        left_result = int(result)
        output_list.append((left_result, numbers))

    for output in output_list:
        left_result, numbers_list = output
        gaps = len(numbers_list) - 1
        combinations_counts = 2**gaps

        for i in range(0, combinations_counts):
            test_res = int(numbers_list[0])

            for j in range(gaps):
                current_number = numbers_list[j + 1]
                if ((i >> j) & 1) == 0:
                    test_res += int(current_number)
                else:
                    test_res *= int(current_number)

            if test_res == left_result:
                total_result = total_result + left_result
                break
    print(total_result)


def part2():
    pass


if __name__ == "__main__":
    part1()
    part2()
