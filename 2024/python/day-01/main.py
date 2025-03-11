def part1():
    columns1 = []
    columns2 = []
    result = 0

    with open('../../rust/data/inputs/01.txt', mode='r') as f:
        lines = f.read().splitlines()

        for line in lines:
            first, second = line.split()
            columns1.append(first)
            columns2.append(second)

    columns1.sort()
    columns2.sort()

    for i in range(0, len(columns1)):
        left = int(columns1[i])
        right = int(columns2[i])
        result += abs(left - right)

    return result


if __name__ == '__main__':
    print(part1())
