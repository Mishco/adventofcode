# --- Day 4: Ceres Search ---

def part1():
    # directions = [(0,1), (0,-1), (1,0), (1,1), (1,-1), (-1,0), (-1,-1), (-1,1)]
    directions = [(dy, dx) for dy in [-1, 0, 1] for dx in [-1, 0, 1] if (dx != 0 or dy != 0)]
    word = 'XMAS'
    result = 0
    word_len = len(word)

    lines = open('../../rust/data/inputs/04.txt', mode='r').readlines()
    h, w = len(lines), len(lines[0]) - 1
    grid = {(y, x): lines[y][x] for y in range(h) for x in range(w)}

    for y, x in grid:
        for dy, dx in directions:
            candidate = "".join(grid.get((y + dy * i, x + dx * i), "") for i in range(word_len))
            result += candidate == word

    return result


def part2():
    pass


if __name__ == '__main__':
    print(part1())
    print(part2())
