"""
https://adventofcode.com/2024/day/21
"""

from random import getrandbits
from sys import maxsize
from typing import Optional

import numpy as np

KEYPAD = (
    ('#', '^', 'A'),
    ('<', 'v', '>'),
)

DIRECTIONS_0 = {
    '#': (3, 0),
    '0': (3, 1),
    'A': (3, 2),
    '1': (2, 0),
    '2': (2, 1),
    '3': (2, 2),
    '4': (1, 0),
    '5': (1, 1),
    '6': (1, 2),
    '7': (0, 0),
    '8': (0, 1),
    '9': (0, 2),
}

DIRECTIONS_1 = {
    '^': (0, 1),
    'v': (1, 1),
    '<': (1, 0),
    '>': (1, 2),
    'A': (0, 2),
}

DIRECTIONS_REV: dict[tuple[int, ...], str] = {
    (-1, 0): '^',
    (1, 0): 'v',
    (0, -1): '<',
    (0, 1): '>',
    (0, 0): 'A',
}


def simple(
    start: tuple[int, int],
    end: tuple[int, int],
    reverse_: Optional[bool] = None,
) -> tuple[list[tuple[int, int]], bool]:
    # simple path finding by going straight to the end
    # randomly choosing, if we should go up-down or left-right first
    visited = [start]
    vector = (end[0] - start[0], end[1] - start[1])
    if reverse_ is None:
        reverse = bool(getrandbits(1))
    else:
        reverse = reverse_
    i = 0
    if reverse:
        if vector[1] < 0:
            for i in range(1, -vector[1] + 1):
                visited.append((start[0], start[1] - i))
                i = -i
        else:
            for i in range(1, vector[1] + 1):
                visited.append((start[0], start[1] + i))

        if vector[0] < 0:
            for j in range(1, -vector[0] + 1):
                visited.append((start[0] - j, start[1] + i))
        else:
            for j in range(1, vector[0] + 1):
                visited.append((start[0] + j, start[1] + i))
        return visited, reverse

    if vector[0] < 0:
        for i in range(1, -vector[0] + 1):
            visited.append((start[0] - i, start[1]))
            i = -i
    else:
        for i in range(1, vector[0] + 1):
            visited.append((start[0] + i, start[1]))

    if vector[1] < 0:
        for j in range(1, -vector[1] + 1):
            visited.append((start[0] + i, start[1] - j))
    else:
        for j in range(1, vector[1] + 1):
            visited.append((start[0] + i, start[1] + j))

    return visited, reverse


def diff(inp: list[tuple[int, int]]) -> tuple[tuple[int, ...], ...]:
    return tuple(map(tuple, np.diff(inp, axis=0)))


def solve(
    start: tuple[int, int],
    code: tuple[str, ...],  # tuple[str, str, str, str]
    layers: int,
    to_visit_: Optional[list[tuple[tuple[int, ...], ...]]] = None,
) -> list[tuple[tuple[int, ...], ...]]:
    """solve"""
    if to_visit_ is None:
        to_visit: list[tuple[tuple[int, ...], ...]] = []
        for key in code:
            end = DIRECTIONS_0[key]
            visited, reverse = simple(start, end)
            if (3, 0) in visited:
                visited, _ = simple(start, end, reverse_=not reverse)
            start = visited[-1]
            to_visit.append(diff(visited))
    else:
        to_visit = to_visit_

    for _ in range(layers):
        # TODO we construct the enormous list to measure its len, which will likely have billion elements
        to_press = []
        start = (0, 2)
        cache: dict[tuple[tuple[int, int], tuple[int, int]], list[tuple[int, int]]] = {}  # My caching try - not working well so far
        for visited_ in to_visit:
            for key_ in [*visited_, (0, 0)]:  # Add A
                end = DIRECTIONS_1[DIRECTIONS_REV[key_]]
                if pressed := cache.get((start, end)):  # first walrus this year :) Probably because I solved half problems in another lang
                    to_press.append(tuple(diff(pressed)))
                    start = end
                    continue
                pressed1, reverse = simple(start, end)
                pressed2, _ = simple(start, end, reverse_=not reverse)
                if (0, 0) in pressed1:
                    pressed = pressed2
                elif (0, 0) in pressed2:
                    pressed = pressed1
                else:
                    pressed = min(pressed1, pressed2, key=len)
                cache[(start, end)] = pressed
                to_press.append(tuple(diff(pressed)))
                start = end

        to_visit = to_press

    return to_visit


def solve_part(layers: int) -> int:
    """solve part"""
    with open('input.txt', 'r') as f:
        codes = tuple(map(lambda x: tuple(x.rstrip()), f.readlines()))
    vals = [int(''.join(code[:-1])) for code in codes]

    res = 0
    start = (3, 2)
    for code, val in zip(codes, vals):
        # just repeat the random choosing of first direction
        # until we get minumum
        mult = maxsize
        for _ in range(10):  # 1000
            sol = solve(start, code, layers)
            mult = min(mult, len(sol))
        res += val * mult

    return res


def main1() -> int:
    """part1"""
    return solve_part(3)


if __name__ == '__main__':
    print(main1())  # 94284
