# --- Day 6: Guard Gallivant ---

import sys

sys.setrecursionlimit(10 ** 6)
directions_deltas = {
    'up': (-1, 0),
    'right': (0, 1),
    'down': (1, 0),
    'left': (0, -1),
}
directions = ['up', 'right', 'down', 'left']


def is_obstacle_ahead(position, direction, rows:int, cols:int, G):
    delta  = directions_deltas[direction]
    new_position = (position[0] + delta[0], position[1] + delta[1])
    row, col = new_position
    if 0 <= row < rows and 0 <= col < cols:
        return G[row][col] == '#'
    return True


def turn_right(direction: str) -> str:
    """
    Return the new direction when the guard turns right.
    :param direction: input direction as string
    :return: new direction as string
    """
    new_index = (directions.index(direction) + 1) % len(directions)
    return directions[new_index]


def move_forward(position, direction):
    """
    Move the guard forward.
    :param position: Starting position
    :param direction: Which direction to move
    :return: new positions
    """
    delta = directions_deltas[direction]
    new_row = position[0] + delta[0]
    new_col = position[1] + delta[1]
    return (new_row, new_col)


def part1():
    with open('../../rust/data/inputs/06.txt', mode='r') as f:
        data = f.read().strip()

    G = data.split('\n')
    rows = len(G)
    cols = len(G[0])
    for row in range(rows):
        for col in range(cols):
            if G[row][col] == '^':
                start_row, start_col = row, col

    guard_position = (start_row, start_col)
    guard_direction = 'up'
    visited = set()
    # Track visited states (position + direction) to detect cycles
    visited_states = set()
    
    while True:
        # Check if we've been in this exact state before
        state = (guard_position, guard_direction)
        if state in visited_states:
            break
            
        visited_states.add(state)
        
        if is_obstacle_ahead(guard_position, guard_direction, rows, cols, G):
            guard_direction = turn_right(guard_direction)
        else:
            guard_position = move_forward(guard_position, guard_direction)
            visited.add(guard_position)

        row, col = guard_position
        if row < 0 or row >= rows or col < 0 or col >= cols:
            break

    return len(visited)
    # for o_r in range(rows):
    #     for o_c in range(cols):
    #         r, c = start_row, start_col
    #         d = 0  # 0=up, 1=right, 2=down, 3=left
    #         SEEN = set()
    #         SEEN_RC = set()
    #         while True:
    #             # if (r, c, d) in SEEN:
    #             #     p2 += 1
    #             #     break
    #             SEEN.add((r, c, d))
    #             SEEN_RC.add((r, c))
    #             dr, dc = [(-1, 0), (0, 1), (1, 0), (0, -1)][d]
    #             rr = r + dr
    #             cc = c + dc
    #             if not (0 <= rr < rows and 0 <= cc < cols):
    #                 if G[o_r][o_c] == '#':
    #                     result = len(SEEN_RC)
    #                 break
    #             if G[rr][cc] == '#' or rr == o_r and cc == o_c:
    #                 d = (d + 1) % 4
    #             else:
    #                 r = rr
    #                 c = cc
    # return result


def part2():
    pass
    # with open(infile, mode='r') as f:
    #     lines = f.read().split("\n\n")
    # result = 0
    # for line in lines:
    #     # Process each line here
    #     pass
    # return result


if __name__ == '__main__':
    print(part1())
    print(part2())
