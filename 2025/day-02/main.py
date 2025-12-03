# --- Day 2: Gift Shop ---


def part1():
    lines = """
    11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
    1698522-1698528,446443-446449,38593856-38593862,565653-565659,
    824824821-824824827,2121212118-2121212124
    """
    lines = open("../inputs/day-02.txt", "r").read().strip()
    # lines
    invalid_ids = []
    ranges = [r.strip() for r in lines.split(",") if r.strip()]
    for r in ranges:
        start_str, end_str = r.split("-")
        start = int(start_str)
        end = int(end_str)
        for i in range(start, end + 1):
            s = str(i)
            if len(s) % 2 == 0:
                half = len(s) // 2
                if s[:half] == s[half:]:
                    invalid_ids.append(i)

    # print(f"Part 1: {len(invalid_ids)} invalid IDs")
    # print(f"Invalid IDs: {invalid_ids}")
    print(sum(invalid_ids))


def is_repeated_substring(substr: str) -> bool:
    n = len(substr)
    for l in range(1, n // 2 + 1):
        if n % l == 0 and substr == substr[:l] * (n // l):
            return True
    return False


def part2():
    lines = """
        11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,
        824824821-824824827,2121212118-2121212124
        """
    lines = open("../inputs/day-02.txt", "r").read().strip()  # lines
    invalid_any = []
    ranges = [r.strip() for r in lines.split(",") if r.strip()]
    for r in ranges:
        start_str, end_str = r.split("-")
        start = int(start_str)
        end = int(end_str)
        for i in range(start, end + 1):
            substring = str(i)
            if is_repeated_substring(substring):
                invalid_any.append(i)

    # print(f"Part 2: {len(invalid_any)} invalid IDs")
    # print(f"Invalid IDs: {invalid_any}")
    print(sum(invalid_any))


if __name__ == "__main__":
    part1()
    part2()
