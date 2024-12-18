"""
https://adventofcode.com/2024/day/18
"""

import heapq
from typing import Optional

import numpy as np


def neighbors(x: int, y: int) -> list[tuple[int, int]]:
    """Return all 4 direct neighbor indices"""
    return [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]


def dijkstra(input_map: tuple[tuple[str, ...], ...], start: tuple[int, int]) -> Optional[int]:
    visited: dict[tuple[int, int], int] = {}
    pq: list[tuple[int, tuple[int, int]]] = []
    current_cost = 0
    heapq.heappush(pq, (current_cost, start))

    while pq:
        current_cost, (x, y) = heapq.heappop(pq)

        visited_key = (x, y)
        if visited_key in visited and visited[visited_key] <= current_cost:
            continue

        # store current cost
        visited[visited_key] = current_cost

        # exit tile
        if input_map[x][y] == 2:
            return current_cost

        for dx, dy in neighbors(x, y):
            if input_map[dx][dy] != 1:
                curr_weight = current_cost + 1
                heapq.heappush(pq, (curr_weight, (dx, dy)))

    return None


def load_input(nanoseconds: int, grid_len: int):
    with open('input.txt', 'r') as f:
        coords = tuple(map(lambda x: tuple(map(int, x.rstrip().split(","))), f.readlines()))

    grid = np.zeros((grid_len + 2, grid_len + 2), dtype=int)

    for idx in range(nanoseconds):
        coord = coords[idx]
        grid[coord[0] + 1][coord[1] + 1] = 1

    # Add frame around - no need to check bounds later
    grid[:, 0] = 1
    grid[0, :] = 1
    grid[:, grid_len + 1] = 1
    grid[grid_len + 1, :] = 1
    grid[grid_len, grid_len] = 2

    return grid, coords[nanoseconds:]


def main1() -> Optional[int]:
    """part1"""
    grid_len = 71
    grid, _ = load_input(1024, grid_len)
    print(grid)

    start = (1, 1)

    cost = dijkstra(grid, start)

    return cost


def main2() -> int:
    grid_len = 71  # 7
    grid, coords = load_input(1024, grid_len)  # 12

    start = (1, 1)

    for coord in coords:
        grid[coord[0] + 1][coord[1] + 1] = 1
        res = dijkstra(grid, start)
        if res is None:
            return coord

    return -1


if __name__ == '__main__':
    print(main1())  # 344
    print(main2())  # 46, 18
