# --- Day 5: Doesn't He Have Intern-Elves For This? ---


def part1():
    # at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou
    # at least one letter that appears twice in a row, like xx, abcdde
    # does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.
    data = open("../inputs/day-05.txt", "r").readlines()

    assert is_nice("ugknbfddgicrmopn") == True
    assert is_nice("aaa") == True
    assert is_nice("jchzalrnumimnmhp") == False  ## no double letter
    assert is_nice("haegwjzuvuyypxyu") == False  ## contains xy
    assert is_nice("dvszwmarrgswjxmb") == False  ## only one vowel

    nice_count = sum(1 for line in data if is_nice(line.strip()))
    return nice_count


def is_nice(s):
    vowels = "aeiou"
    forbidden = ["ab", "cd", "pq", "xy"]

    vowel_count = sum(1 for char in s if char in vowels)
    has_double = any(s[i] == s[i + 1] for i in range(len(s) - 1))
    has_forbidden = any(sub in s for sub in forbidden)
    if vowel_count >= 3 and has_double and not has_forbidden:
        return True
    else:
        return False


def part2():
    data = open("../inputs/day-05.txt", "r").readlines()

    assert is_nice2("qjhvhtzxzqqjkmpb") == True
    assert is_nice2("xxyxx") == True
    assert is_nice2("uurcxstgmygtbstg") == False  ## no repeat with one between
    assert is_nice2("ieodomkazucvgmuy") == False  ## no pair that appears twice

    nice_count = sum(1 for line in data if is_nice2(line.strip()))
    return nice_count


def is_nice2(s):
    has_pair = any(s[i : i + 2] in s[i + 2 :] for i in range(len(s) - 1))
    has_repeat_with_one_between = any(s[i] == s[i + 2] for i in range(len(s) - 2))
    if has_pair and has_repeat_with_one_between:
        return True
    else:
        return False


if __name__ == "__main__":
    print("Part 1:", part1())
    print("Part 2:", part2())
