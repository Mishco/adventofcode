test_input = """199
200
208
210
200
207
240
269
260
263
"""

test_input2 = """199  A      
200  A B    
208  A B C  
210    B C D
200  E   C D
207  E F   D
240  E F G  
269    F G H
260      G H
263        H
"""


def part1(input_str: str):
    previous_value = -1
    counter = 0
    for line in input_str.splitlines():
        if previous_value == -1:
            print(f"{line} (N/A - no previous measurement)")
            previous_value = int(line.strip())
        elif int(line.strip()) < previous_value:
            print(f"{line} (decreased)")
            previous_value = int(line.strip())
        else:
            print(f"{line} (increase)")
            previous_value = int(line.strip())
            counter += 1
    return counter


def part2(input_str: str):
    previous_value = -1
    counter = 0
    slice_size = 3
    overlap = 2
    step_size = slice_size - overlap

    for i in range(0, len(input_str.splitlines()) - slice_size + 1, step_size):
        line_slice = input_str.splitlines()[i:i + slice_size]

        sum_line = sum([int(line.strip()) for line in line_slice])

        if previous_value == -1:
            print(f"{sum_line} (N/A - no previous measurement)")
            previous_value = sum_line
        elif sum_line < previous_value:
            print(f"{sum_line} (decreased)")
            previous_value = sum_line
        elif sum_line == previous_value:
            print(f"{sum_line} (no change)")
        else:
            print(f"{sum_line} (increased)")
            previous_value = sum_line
            counter += 1
    return counter


if __name__ == '__main__':
    res = part1(test_input)
    assert res == 7

    f = open('../inputs/2021-01.txt', mode='r').read()
    res = part1(f)
    print(res)

    res = part2(test_input)
    assert res == 5

    res = part2(f)
    print(res)
