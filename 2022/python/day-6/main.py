def find_characters_different(data, n):
    for i in range(len(data) - n):
        if len(set(data[i:i + n])) == n:
            return i + n


def tests():
    assert find_characters_different("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4) == 7, "Should be 7"
    assert find_characters_different("bvwbjplbgvbhsrlpgdmjqwftvncz", 4) == 5, "Should be 5"
    assert find_characters_different("nppdvjthqldpwncqszvftbrmjlhg", 4) == 6, "Should be 6"
    assert find_characters_different("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4) == 10, "Should be 10"
    assert find_characters_different("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4) == 11, "Should be 11"

    assert find_characters_different("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14) == 19, "Should be 19"


if __name__ == "__main__":
    tests()
    x = open('../inputs/day06.txt', 'r').read()
    print(find_characters_different(x, 4))
    print(find_characters_different(x, 14))
