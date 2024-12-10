"""
https://adventofcode.com/2024/day/10
"""

from copy import copy
from functools import lru_cache

res_set: set[frozenset[tuple[int, int]]] = set()


def neighbors(x: int, y: int) -> list[tuple[int, int]]:
    """Return all 4 direct neighbor indices"""
    return [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]


@lru_cache(maxsize=None)
def dp(input_map: tuple[tuple[int, ...], ...], coords: tuple[int, int], next_needed: int) -> set[tuple[int, int]]:
    i, j = coords
    if next_needed == 10:
        return set([(i, j)])

    res = set()
    len_map = len(input_map)  # assume square
    for x, y in neighbors(i, j):
        if 0 <= x < len_map and 0 <= y < len_map and input_map[x][y] == next_needed:
            res.update(dp(input_map, (x, y), next_needed + 1))

    return res


def dp_rank(
    input_map: tuple[tuple[int, ...], ...], coords: tuple[int, int], next_needed: int, res: set[tuple[int, int]],
) -> set[tuple[int, int]]:
    i, j = coords
    res.add((i, j))
    if next_needed == 10:
        return res

    len_map = len(input_map)
    for x, y in neighbors(i, j):
        if 0 <= x < len_map and 0 <= y < len_map and input_map[x][y] == next_needed:
            res_new = dp_rank(input_map, (x, y), next_needed + 1, copy(res))
            res_set.add(frozenset(res_new))
        else:
            res_new = set()

    return res_new


def main1() -> int:
    """part1"""
    with open('input.txt', 'r') as f:
        input_map = tuple(map(lambda x: tuple(map(int, x.rstrip())), f.readlines()))

    res = 0
    for i, line in enumerate(input_map):
        for j, char in enumerate(line):
            if char == 0:
                res += len(dp(input_map, (i, j), 1))

    return res


def main2() -> int:
    with open('input.txt', 'r') as f:
        input_map = tuple(map(lambda x: tuple(map(int, x.rstrip())), f.readlines()))

    res = 0
    for i, line in enumerate(input_map):
        for j, char in enumerate(line):
            if char == 0:
                dp_rank(input_map, (i, j), 1, set())
                res += len(res_set - {frozenset()})
                res_set.clear()

    return res


if __name__ == '__main__':
    print(main1())  # 496
    print(main2())  # 1120
