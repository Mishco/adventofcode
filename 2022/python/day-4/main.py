# --- Day 4: Camp Cleanup ---
pairs = """2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"""

total_sections = 0
# overlaps = 0
content = open('../inputs/day04.txt').read()

overlaps = 0
overlaps2 = 0
for line in content.splitlines():
    first_pair, second_pair = [list(map(int, pair.split('-'))) for pair in line.split(",")]
    if first_pair[0] != second_pair[0]:
        if first_pair[0] > second_pair[0]:
            inside_range, outside_range = first_pair, second_pair
        else:
            inside_range, outside_range = second_pair, first_pair
    else:
        if first_pair[1] < second_pair[1]:
            inside_range, outside_range = first_pair, second_pair
        else:
            inside_range, outside_range = second_pair, first_pair

    if inside_range[1] <= outside_range[1]:
        overlaps += 1

    # part2
    if first_pair[0] > second_pair[0]:
        inside_range_index, outside_range_index = first_pair, second_pair
    else:
        inside_range_index, outside_range_index = second_pair, first_pair

    if inside_range_index[0] <= outside_range_index[1]:
        overlaps2 += 1

print(overlaps)
print("Task 2 result: " + str(overlaps2))
