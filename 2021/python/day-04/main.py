import typing

import numpy as np

import re

def ints(s: str) -> typing.List[int]:  # thanks mserrano
    return list(map(int, re.findall(r"(?:(?<!\d)-)?\d+", s)))


def part1(test_in):

    # load first line
    raw_data = test_in.split("\n\n")

    numbers = raw_data[0]
    list_of_numbers = [int(n) for n in numbers.split(",")]
    list_of_cards = []

    for card in raw_data[1:]:
        rows = card.split("\n")
        card_array = np.array([[int(n) for n in (row.split())] for row in rows])
        card = {"card": card_array, "bingo": False}
        list_of_cards.append(card)

    # load puzzles
    pass




if __name__ == '__main__':
    test_in = """7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"""

    assert part1(test_in) == 4512

