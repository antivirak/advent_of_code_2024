"""
https://adventofcode.com/2024/day/16
"""

import sys

sys.setrecursionlimit(100_000)

visited: dict[tuple[tuple[int, int], tuple[int, int]], float] = {}
win: set[int] = set()


def neighbors(x: int, y: int) -> list[tuple[int, int]]:
    """Return all 4 direct neighbor indices"""
    return [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]


def dp(
    input_map: tuple[tuple[str, ...], ...],
    coords: tuple[int, int],
    weight: int,
    direction: tuple[int, int],
):
    i, j = coords
    if input_map[i][j] == '#':
        visited[(coords, direction)] = float('inf')
        return

    if visited.get((coords, direction), float('inf')) > weight:
        visited[(coords, direction)] = weight

    if input_map[i][j] == 'E':
        win.add(weight)
        return

    for x, y in neighbors(i, j):
        curr_weight = weight + 1
        if not(x == i + direction[0] and y == j + direction[1]):
            curr_weight += 1000
        visited_key = ((x, y), direction)

        # If we've already visited this cell in this direction with a lower or equal cost, skip it
        if visited_key in visited and visited[visited_key] <= curr_weight:
            continue
        if x != i - direction[0] or y != j - direction[1]:
            dp(input_map, (x, y), curr_weight, (x - i, y - j))


def main1() -> int:
    """part1"""
    # TODO make faster with heapqueue
    with open('input.txt', 'r') as f:
        input_map = tuple(map(lambda x: tuple(x.rstrip()), f.readlines()))

    start = (0, 0)
    for i, line in enumerate(input_map):
        for j, char in enumerate(line):
            if char == 'S':
                start = (i, j)
                break

    visited[(start, (0, 1))] = 0
    for x, y in neighbors(*start):
        if x == start[0] and y == start[1] + 1:  # east
            dp(input_map, (x, y), 1, (0, 1))
        else:
            dp(input_map, (x, y), 1001, (x - start[0], y - start[1]))

    return min(win)


def backtrack(node: tuple[int, int], direction: tuple[int, int], nodes: set[tuple[int, int]]):
    """
    Find all nodes making up any shortest path.
    This is achieved by following the shortest path and adding all nodes,
    whose cost is 1 or 1001 off the adjacent one.
    """
    nodes.add(node)
    nlist = neighbors(*node)

    for x, y in nlist:
        if visited.get(((x, y), direction)) == visited.get((node, direction), 0) - 1:
            if (x, y) not in nodes:
                backtrack((x, y), direction, nodes)
        for new_dir in ((0, 1), (0, -1), (1, 0), (-1, 0)):
            if visited.get(((x, y), new_dir)) == visited.get((node, direction), 0) - 1001:
                if (x, y) not in nodes:
                    backtrack((x, y), new_dir, nodes)


def main2() -> int:
    print(main1())  # fill the visited dict and win set
    with open('input.txt', 'r') as f:
        input_map = tuple(map(lambda x: tuple(x.rstrip()), f.readlines()))

    end = (0, 0)
    for i, line in enumerate(input_map):
        for j, char in enumerate(line):
            if char == 'E':
                end = (i, j)
                break

    nodes: set[tuple[int, int]] = set()
    # find the direction to end
    for key, value in visited.items():
        node, (x, y) = key
        if node == end and value == min(win):
            backtrack(node, (x, y), nodes)
            break

    for i, line in enumerate(input_map):
        for j, char in enumerate(line):
            if (i, j) in nodes:
                print('0', end='')
            else:
                print(char, end='')
        print()

    return len(nodes)


if __name__ == '__main__':
    # print(main1())  # 89460
    print(main2())  # 504
