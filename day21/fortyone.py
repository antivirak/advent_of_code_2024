"""https://adventofcode.com/2024/day/21

5074 / 19786
"""

from functools import lru_cache
from math import copysign
from typing import Optional

import numpy as np

# KEYPAD = (
#     ('#', '^', 'A'),
#     ('<', 'v', '>'),
# )

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


def diff(inp: list[tuple[int, int]]) -> tuple[tuple[int, int], ...]:
    return tuple(map(tuple, np.diff(inp, axis=0)))


@lru_cache
def get_sequence_length(
    layer: int,
    to_visit: tuple[tuple[tuple[int, int], ...], ...],
) -> int:
    """Solve() with memoization"""
    if not layer:
        return len(to_visit)
    to_press = []
    start = (0, 2)
    for visited_ in to_visit:
        for key_ in [*visited_, (0, 0)]:  # Add A
            end = DIRECTIONS_1[DIRECTIONS_REV[key_]]
            pressed = simple(start, end)
            if (0, 0) in pressed:
                pressed = simple(start, end, reverse_=True)
            to_press.append(diff(pressed))
            start = end

    len_to_visit = 0
    for item in to_press:
        len_to_visit += get_sequence_length(layer - 1, (item, ))

    return len_to_visit


def solve_next_pad(start: tuple[int, int], code: tuple[str, ...], layers: int) -> int:
    to_visit: list[tuple[tuple[int, int], ...]] = []
    for key in code:
        end = DIRECTIONS_0[key]
        visited = simple(start, end)
        if (3, 0) in visited:
            visited = simple(start, end, reverse_=True)
        start = visited[-1]
        to_visit.append(diff(visited))

    return get_sequence_length(layers, tuple(to_visit))


def solve_part(layers: int) -> int:
    with open("input.txt", 'r') as f:
        codes = tuple(map(lambda x: tuple(x.rstrip()), f.readlines()))
    vals = [int(''.join(code[:-1])) for code in codes]

    res = 0
    start = (3, 2)
    for code, val in zip(codes, vals):
        mult = solve_next_pad(start, code, layers)

        res += val * mult

    return res


def main1() -> int:
    """part1"""
    return solve_part(3)


def main2() -> int:
    """part2"""
    return solve_part(26)


if __name__ == '__main__':
    print(main1())
    print(main2())
