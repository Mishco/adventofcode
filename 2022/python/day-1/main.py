ii = """
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"""
content = open('../inputs/day01.txt').readlines()

current_val = 0
sum_elves = []
for ln in content:
    if ln != '\n':
        current_val += int(ln)
    else:
        sum_elves.append(current_val)
        current_val = 0

print(max(sum_elves))

sum_elves.sort(reverse=True)
print(sum(sum_elves[:3]))
