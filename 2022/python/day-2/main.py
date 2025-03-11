# A for Rock, B for Paper, and C for Scissors.
# X for Rock, Y for Paper, and Z for Scissors

# score:
ii = """A Y
B X
C Z
"""

# shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
shapes = {
    'X': 1,
    'Y': 2,
    'Z': 3
}

# + outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won)
scores = {
    ('A', 'X'): 3,
    ('A', 'Y'): 6,
    ('A', 'Z'): 0,
    ('B', 'X'): 0,
    ('B', 'Y'): 3,
    ('B', 'Z'): 6,
    ('C', 'X'): 6,
    ('C', 'Y'): 0,
    ('C', 'Z'): 3
}

score = 0
content = open('../inputs/day02.txt', 'r').read()
for row in content.splitlines():
    first_col, second_col = row.split()[0], row.split()[1]
    shape = shapes.get(second_col)
    score += scores.get((first_col, second_col), 0) + shape

print(score)  # 11603

# part 2
points = {
    "A X": 0 + 3,
    "A Y": 3 + 1,
    "A Z": 6 + 2,
    "B X": 0 + 1,
    "B Y": 3 + 2,
    "B Z": 6 + 3,
    "C X": 0 + 2,
    "C Y": 3 + 3,
    "C Z": 6 + 1,
}
rounds = content.split("\n")
scores = []
total = 0
for one_round in rounds:
    scores.append(points[one_round])
    total += points[one_round]

print(total)  # 12725
