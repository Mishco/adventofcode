# --- Day 4: Ceres Search ---


def part1():
    # directions = [(0,1), (0,-1), (1,0), (1,1), (1,-1), (-1,0), (-1,-1), (-1,1)]
    directions = [
        (dy, dx) for dy in [-1, 0, 1] for dx in [-1, 0, 1] if (dx != 0 or dy != 0)
    ]
    word = "XMAS"
    result = 0
    word_len = len(word)

    lines = open("../../rust/data/inputs/04.txt", mode="r").readlines()
    h, w = len(lines), len(lines[0]) - 1
    grid = {(y, x): lines[y][x] for y in range(h) for x in range(w)}

    for y, x in grid:
        for dy, dx in directions:
            candidate = "".join(
                grid.get((y + dy * i, x + dx * i), "") for i in range(word_len)
            )
            result += candidate == word

    return result


def part2():
    lines = open("../../rust/data/inputs/04.txt", mode="r").readlines()
    result = 0
    h, w = len(lines), len(lines[0]) - 1
    grid = {(y, x): lines[y][x] for y in range(h) for x in range(w)}

    for i in range(0, h):
        for j in range(0, w):
            if grid[i, j] == "A" and i > 0 and j > 0 and i < w - 1 and j < h - 1:
                c1 = grid[i - 1, j - 1]
                c2 = grid[i + 1, j + 1]
                c3 = grid[i - 1, j + 1]
                c4 = grid[i + 1, j - 1]

                if ((c1 == "M" and c2 == "S") or (c1 == "S" and c2 == "M")) and (
                    (c3 == "M" and c4 == "S") or (c3 == "S" and c4 == "M")
                ):
                    result += 1

    return result


if __name__ == "__main__":
    print(part1())
    print(part2())
