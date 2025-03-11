# --- Day 7: No Space Left On Device ---
from collections import defaultdict

test = """
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"""


def calc(data, debug=False):
    sizes = defaultdict(int)
    ancestry = ['/']  # stack of parents

    for line in data:
        # cmds_ls = re.findall('^\$', line)
        match line.split():
            case ['$', 'cd', '/']:
                ancestry = ['/']
            case ['$', 'cd', '..']:
                ancestry.pop()
            case ['$', 'cd', child]:
                ancestry.append(f'{ancestry[-1]}{child}/')
            case ['$', 'ls']:
                pass
            case ['dir', child]:
                pass
            case [size, file]:
                for path in ancestry:
                    sizes[path] += int(size)
        if debug:
            print(f'{line: <20} {ancestry}')

    sum = 0
    for k, s in sizes.items():
        if s <= 100000:
            sum += s

    to_delete = sizes['/'] - 40000000
    minimal_val = min(size for size in sizes.values() if size >= to_delete)
    return sum, sizes, minimal_val


def tests():
    a, _, _ = calc(test.splitlines())
    assert a == 95437, "Should be 95437"

    _, _, c = calc(test.splitlines())
    assert c == 24933642


if __name__ == '__main__':
    tests()

    f = open('../inputs/day07.txt', 'r').read().splitlines()

    part1, _, part2 = calc(f, debug=False)

    print(part1)
    print(part2)
