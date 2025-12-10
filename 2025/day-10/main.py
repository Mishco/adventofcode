# --- Day 10: Factory ---
import re
from itertools import combinations, product


def parse_input(filename):
    machines = []
    with open(filename) as f:
        for line in f:
            if line.strip():
                machines.append(parse_machine_line(line))
    return machines


def parse_machine_line(line):
    # Extract light diagram [.##.]
    light_match = re.search(r"\[(.*?)\]", line)
    lights = [1 if c == "#" else 0 for c in light_match.group(1)]

    # Extract button configurations ()
    button_matches = re.findall(r"\(([0-9,]+)\)", line)
    buttons = []
    for match in button_matches:
        button = [int(x) for x in match.split(",")]
        buttons.append(button)

    # Extract joltage requirements {}
    joltage_match = re.search(r"\{([0-9,]+)\}", line)
    joltages = []
    if joltage_match:
        joltages = [int(x) for x in joltage_match.group(1).split(",")]

    return lights, buttons, joltages


def create_button_matrix(n_items, buttons):
    matrix = []
    for item_idx in range(n_items):
        row = [1 if item_idx in button else 0 for button in buttons]
        matrix.append(row)
    return matrix


def gaussian_elimination_gf2(matrix, target):
    """
    Perform Gaussian elimination over GF(2) (binary field with XOR).
    Returns (pivot_columns, reduced_matrix) or None if no solution.
    """
    n_rows = len(matrix)
    n_cols = len(matrix[0])

    # Create augmented matrix [A | b]
    aug_matrix = [row[:] + [target[i]] for i, row in enumerate(matrix)]

    pivot_cols = []
    current_row = 0

    for col in range(n_cols):
        # Find pivot
        pivot_row = None
        for row in range(current_row, n_rows):
            if aug_matrix[row][col] == 1:
                pivot_row = row
                break

        if pivot_row is None:
            continue

        # Swap rows
        aug_matrix[current_row], aug_matrix[pivot_row] = (
            aug_matrix[pivot_row],
            aug_matrix[current_row],
        )
        pivot_cols.append(col)

        # Eliminate column using XOR
        for row in range(n_rows):
            if row != current_row and aug_matrix[row][col] == 1:
                for c in range(n_cols + 1):
                    aug_matrix[row][c] ^= aug_matrix[current_row][c]

        current_row += 1

    # Check for inconsistency (0 = 1)
    for row in range(current_row, n_rows):
        if aug_matrix[row][n_cols] == 1:
            return None

    return pivot_cols, aug_matrix


def solve_binary_system(target, buttons):
    n_lights = len(target)
    n_buttons = len(buttons)

    matrix = create_button_matrix(n_lights, buttons)
    result = gaussian_elimination_gf2(matrix, target)

    if result is None:
        return None

    pivot_cols, aug_matrix = result

    # Extract basic solution
    solution = [0] * n_buttons
    for i, col in enumerate(pivot_cols):
        solution[col] = aug_matrix[i][n_buttons]

    # Find free variables
    free_vars = [i for i in range(n_buttons) if i not in pivot_cols]

    if not free_vars:
        return sum(solution)

    # Try all combinations of free variables to minimize presses
    min_presses = sum(solution)

    for num_free in range(1, len(free_vars) + 1):
        for free_combo in combinations(free_vars, num_free):
            test_solution = solution[:]
            for var in free_combo:
                test_solution[var] = 1

            # Recompute dependent variables
            for i, col in enumerate(pivot_cols):
                val = aug_matrix[i][n_buttons]
                for j in range(n_buttons):
                    if j != col and test_solution[j] == 1:
                        val ^= aug_matrix[i][j]
                test_solution[col] = val

            min_presses = min(min_presses, sum(test_solution))

    return min_presses


def gaussian_elimination_real(matrix, target):
    """
    Perform Gaussian elimination with real arithmetic.
    Returns (pivot_columns, reduced_matrix) or None if no solution.
    """
    n_rows = len(matrix)
    n_cols = len(matrix[0])

    # Create augmented matrix [A | b]
    aug_matrix = [row[:] + [float(target[i])] for i, row in enumerate(matrix)]

    pivot_cols = []
    current_row = 0

    for col in range(n_cols):
        # Find pivot
        pivot_row = None
        for row in range(current_row, n_rows):
            if abs(aug_matrix[row][col]) > 1e-9:
                pivot_row = row
                break

        if pivot_row is None:
            continue

        # Swap rows
        aug_matrix[current_row], aug_matrix[pivot_row] = (
            aug_matrix[pivot_row],
            aug_matrix[current_row],
        )
        pivot_cols.append(col)
        pivot_val = aug_matrix[current_row][col]

        # Eliminate column
        for row in range(n_rows):
            if row != current_row and abs(aug_matrix[row][col]) > 1e-9:
                factor = aug_matrix[row][col] / pivot_val
                for c in range(n_cols + 1):
                    aug_matrix[row][c] -= factor * aug_matrix[current_row][c]

        current_row += 1

    # Check for inconsistency
    for row in range(current_row, n_rows):
        if abs(aug_matrix[row][n_cols]) > 1e-9:
            return None

    return pivot_cols, aug_matrix


def is_valid_integer_solution(val):
    return val > -1e-9 and abs(val - round(val)) < 1e-9


def solve_integer_system(target, buttons):
    n_counters = len(target)
    n_buttons = len(buttons)

    matrix = create_button_matrix(n_counters, buttons)
    result = gaussian_elimination_real(matrix, target)

    if result is None:
        return None

    pivot_cols, aug_matrix = result
    free_vars = [i for i in range(n_buttons) if i not in pivot_cols]

    if not free_vars:
        # Unique solution
        solution = [0] * n_buttons
        for i, col in enumerate(pivot_cols):
            val = aug_matrix[i][n_buttons] / aug_matrix[i][col]
            if not is_valid_integer_solution(val):
                return None
            solution[col] = round(val)
        return sum(solution)

    # Search for minimum with free variables
    max_target = max(target) if target else 0
    max_free = max_target + 10
    min_presses = float("inf")

    for free_vals in product(range(max_free), repeat=len(free_vars)):
        solution = [0] * n_buttons

        # Set free variables
        for idx, val in zip(free_vars, free_vals):
            solution[idx] = val

        valid = True
        for i, col in enumerate(pivot_cols):
            val = aug_matrix[i][n_buttons]
            for j in range(n_buttons):
                if j != col:
                    val -= aug_matrix[i][j] * solution[j]
            val = val / aug_matrix[i][col]

            if not is_valid_integer_solution(val):
                valid = False
                break
            solution[col] = round(val)

        if valid:
            presses = sum(solution)
            min_presses = min(min_presses, presses)

            if presses < max_target:
                break

    return min_presses if min_presses != float("inf") else None


def part1():
    # machines = parse_input('example.txt')
    machines = parse_input("../inputs/day-10.txt")

    total_presses = 0
    for lights, buttons, _ in machines:
        presses = solve_binary_system(lights, buttons)
        if presses is not None:
            total_presses += presses

    return total_presses


def part2():
    # machines = parse_input('example.txt')
    machines = parse_input("../inputs/day-10.txt")

    total_presses = 0
    for _, buttons, joltages in machines:
        if joltages:
            presses = solve_integer_system(joltages, buttons)
            if presses is not None:
                total_presses += presses

    return total_presses


if __name__ == "__main__":
    print("Part 1:", part1())
    print("Part 2:", part2())
