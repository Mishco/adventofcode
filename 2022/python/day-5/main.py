import re

x = open('../inputs/day05.txt').read()

stackt, instructions = [part.split("\n") for part in x.split("\n\n")]

stacks = [""] * 10
for line in stackt[:-1]:
    for i, box in enumerate(line[1::4]):
        if box != " ": stacks[i + 1] += box

# print(stacks)
# print(instructions)
p1, p2 = stacks[:], stacks[:]
for line in instructions:
    n, src, dest = map(int, re.findall("\\d+", line))
    # print(n, src, dest)
    p1[src], p1[dest] = p1[src][n:],  p1[src][:n][::-1] + p1[dest]
    p2[src], p2[dest] = p2[src][n:],  p2[src][:n]       + p2[dest]

print("Part 1:", "".join(s[0] for s in p1 if s))
print("Part 2:", "".join(s[0] for s in p2 if s))

