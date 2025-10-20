"""https://adventofcode.com/2024/day/21

5074 / 19786
"""

from math import copysign
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
) -> list[tuple[int, int]]:
    # simple path finding by going straight to the end
    # correctly deciding, if we should go up-down or left-right first
    visited = [start]
    vector = (end[0] - start[0], end[1] - start[1])
    vector_sgn = (copysign(1, vector[0]), copysign(1, vector[1]))
    i = 0
    condition = vector_sgn[1] == -1  # if going left, go left first
    if reverse_:
        condition = not condition
    if condition:
        if vector[1] < 0:
            visited.extend([(start[0], start[1] - i) for i in range(1, -vector[1] + 1)])
        else:
            visited.extend([(start[0], start[1] + i) for i in range(1, vector[1] + 1)])
        i = vector[1]

        if vector[0] < 0:
            visited.extend([(start[0] - j, start[1] + i) for j in range(1, -vector[0] + 1)])
        else:
            visited.extend([(start[0] + j, start[1] + i) for j in range(1, vector[0] + 1)])
        return visited

    if vector[0] < 0:
        visited.extend([(start[0] - i, start[1]) for i in range(1, -vector[0] + 1)])
    else:
        visited.extend([(start[0] + i, start[1]) for i in range(1, vector[0] + 1)])
    i = vector[0]

    if vector[1] < 0:
        visited.extend([(start[0] + i, start[1] - j) for j in range(1, -vector[1] + 1)])
    else:
        visited.extend([(start[0] + i, start[1] + j) for j in range(1, vector[1] + 1)])

    return visited


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
            visited = simple(start, end)
            if (3, 0) in visited:
                visited = simple(start, end, reverse_=True)
            start = visited[-1]
            to_visit.append(diff(visited))
    else:
        to_visit = to_visit_

    for _ in range(layers):
        # TODO we construct the enormous list to measure its len, which will likely have billion elements
        to_press = []
        start = (0, 2)
        for visited_ in to_visit:
            for key_ in [*visited_, (0, 0)]:  # Add A
                end = DIRECTIONS_1[DIRECTIONS_REV[key_]]
                pressed = simple(start, end)
                if (0, 0) in pressed:
                    pressed = simple(start, end, reverse_=True)
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
        sol = solve(start, code, layers)
        res += val * len(sol)

    return res


def main1() -> int:
    """part1"""
    return solve_part(3)


if __name__ == '__main__':
    print(main1())
