# --- Day 9: Movie Theater ---


def parse_coordinates(filename):
    coordinates = []
    with open(filename) as f:
        for line in f:
            if line.strip():
                x, y = map(int, line.strip().split(","))
                coordinates.append((x, y))
    return coordinates


def calculate_rectangle_area(point1, point2):
    width = abs(point1[0] - point2[0]) + 1
    height = abs(point1[1] - point2[1]) + 1
    return width * height


def create_coordinate_mapping(points):
    x_coords = sorted(set(p[0] for p in points))
    y_coords = sorted(set(p[1] for p in points))

    x_indices = {coord: 2 * i + 2 for i, coord in enumerate(x_coords)}
    y_indices = {coord: 2 * i + 2 for i, coord in enumerate(y_coords)}

    return x_indices, y_indices


def create_grid(x_indices, y_indices):
    grid_size_x = 2 * len(x_indices) + 8
    grid_size_y = 2 * len(y_indices) + 8
    return [["?" for _ in range(grid_size_y)] for _ in range(grid_size_x)]


def fill_polygon_boundary(grid, points, x_indices, y_indices):

    def fill_line_segment(point1, point2):
        min_x = min(point1[0], point2[0])
        max_x = max(point1[0], point2[0])
        min_y = min(point1[1], point2[1])
        max_y = max(point1[1], point2[1])

        for x_idx in range(x_indices[min_x], x_indices[max_x] + 1):
            for y_idx in range(y_indices[min_y], y_indices[max_y] + 1):
                grid[x_idx][y_idx] = "#"

    # Connect consecutive points
    for i in range(len(points) - 1):
        fill_line_segment(points[i], points[i + 1])

    # Close the polygon by connecting last to first
    fill_line_segment(points[-1], points[0])


def flood_fill_exterior(grid):
    grid[0][0] = "."
    queue = [[0, 0]]

    while queue:
        current = queue.pop(0)

        # Check all 8 neighbors (including diagonals)
        for dx in range(-1, 2):
            for dy in range(-1, 2):
                x = current[0] + dx
                y = current[1] + dy

                # Check bounds and if tile is unexplored
                if 0 <= x < len(grid) and 0 <= y < len(grid[0]):
                    if grid[x][y] == "?":
                        queue.append([x, y])
                        grid[x][y] = "."


def is_rectangle_valid(point1, point2, grid, x_indices, y_indices):
    min_x = min(point1[0], point2[0])
    max_x = max(point1[0], point2[0])
    min_y = min(point1[1], point2[1])
    max_y = max(point1[1], point2[1])

    for x_idx in range(x_indices[min_x], x_indices[max_x] + 1):
        for y_idx in range(y_indices[min_y], y_indices[max_y] + 1):
            if grid[x_idx][y_idx] == ".":
                return False
    return True


def find_largest_rectangle_area(points, validator=None):
    max_area = 0

    for i in range(len(points)):
        for j in range(i + 1, len(points)):
            if validator is None or validator(points[i], points[j]):
                area = calculate_rectangle_area(points[i], points[j])
                max_area = max(max_area, area)

    return max_area


def part1():
    # coordinates = parse_coordinates('example.txt')
    coordinates = parse_coordinates("../inputs/day-09.txt")
    return find_largest_rectangle_area(coordinates)


def part2():
    # coordinates = parse_coordinates('example.txt')
    coordinates = parse_coordinates("../inputs/day-09.txt")

    # Create compressed coordinate mapping
    x_indices, y_indices = create_coordinate_mapping(coordinates)

    # Create and populate grid
    grid = create_grid(x_indices, y_indices)
    fill_polygon_boundary(grid, coordinates, x_indices, y_indices)
    flood_fill_exterior(grid)

    # Find largest valid rectangle
    validator = lambda p1, p2: is_rectangle_valid(p1, p2, grid, x_indices, y_indices)
    return find_largest_rectangle_area(coordinates, validator)


if __name__ == "__main__":
    print("Part 1:", part1())
    print("Part 2:", part2())
