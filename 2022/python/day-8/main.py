# --- Day 8: Treetop Tree House ---
from pprint import pprint


test = """30373
25512
65332
33549
35390
"""

lines = [list(x) for x in test.split('\n')]

# create grid
grid = [list(map(int, row)) for row in test.split()]
pprint(grid)

rows, cols = len(grid), len(grid[0])
directions = [(-1, 0), (1, 0), (0, -1), (0,1)]

def travel(r, c, dr, dc):
    while 0 <= (r := r + dr) < rows and 0 <= (c := c + dc) < cols:
        yield grid[r][c]

# tree_can_be_visible()
# print(list(travel(0,0,0,1)))
# print(list(travel(0,0,1,1)))
# print(list(travel(0,0,1,0)))




visible = []
edges = 4 * (len(lines) - 1)  # 16
# visible = (len(trees) - 1) * 4
# count_visibles = edges
# for line in lines:
#     # data = re.findall('\\d', line)
#     # print(data)
#
#     # just inner trees to pick up
#     for item in line:
#         print(f"{item}")
#
#         for i in range(-1, 2):
#             for j in range(-1, 2):
#                 # print((i, j))
#                 if item > lines[i][j]:
#                     visible.append(item)

#print('----')
#print(count_visibles)
