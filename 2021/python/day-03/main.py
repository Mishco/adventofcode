from collections import Counter


def part1(test_input: str) -> int:
    ll = test_input.strip().splitlines()

    gamma_rate = ''
    epsilon_rate = ''
    for i in range(len(ll[0])):
        x = ''
        for j in ll:
            x += j[i]

        gamma_rate += Counter(x).most_common()[0][0]
        epsilon_rate += Counter(x).most_common()[1][0]

    result = int(gamma_rate, 2) * int(epsilon_rate, 2)
    return result

def part2(test_input: str) -> int:
    ll = test_input.strip().splitlines()
    theta = ''
    epsilon = ''
    for i in range(len(ll[0])):
        common = Counter([x[i] for x in ll])

        if common['0'] > common['1']:
            ll = [x for x in ll if x[i] == '0']
        else:
            ll = [x for x in ll if x[i] == '1']
        theta = ll[0]

    ll = test_input.strip().splitlines()
    for i in range(len(ll[0])):
        common = Counter([x[i] for x in ll])

        if common['0'] > common['1']:
            ll = [x for x in ll if x[i] == '1']
        else:
            ll = [x for x in ll if x[i] == '0']
        if ll:
            epsilon = ll[0]
    return int(theta, 2) * int(epsilon, 2)


if __name__ == '__main__':
    test_in = """00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"""

    assert part1(test_in) == 198

    f = open('../inputs/2021-03.txt', mode='r').read()
    res = part1(f)
    print(res)

    assert part2(test_in) == 230
    res = part2(f)
    print(res)
