"""
https://adventofcode.com/2024/day/20
"""

import heapq
import sys
from typing import Optional

sys.setrecursionlimit(100_000)


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
        if input_map[x][y] == 'E':
            return current_cost

        for dx, dy in neighbors(x, y):
            if input_map[dx][dy] != '#':
                curr_weight = current_cost + 1
                heapq.heappush(pq, (curr_weight, (dx, dy)))

    return None


def main1() -> int:
    """part1"""
    with open('input.txt', 'r') as f:
        grid = tuple(map(lambda x: tuple(x.rstrip()), f.readlines()))

    start = (0, 0)
    for i, line in enumerate(grid):
        for j, char in enumerate(line):
            if char == 'S':
                start = (i, j)
                break

    grids = []
    for i, line in enumerate(grid[1:-1], 1):
        for j, _ in enumerate(line[1:-1], 1):
            current = tuple(grid[:i] + tuple(
                [tuple([*line[:j], line[j].replace('#', '.'), *line[j + 1:]])]
            ) + grid[i + 1:])
            grids.append(current)

    res = 0
    orig_cost = dijkstra(grid, start)
    if orig_cost is None:
        raise ValueError('No path to exit')

    len_grids = len(grids) - 1
    for count, grid in enumerate(grids):
        if count % 100 == 0:
            print(f'{count} / {len_grids})')
        if orig_cost - (dijkstra(grid, start) or 0) >= 100:
            res += 1

    # from collections import defaultdict
    # res_d = defaultdict(int)
    # for count, grid in enumerate(grids):
    #     if count % 100 == 0:
    #         print(f'{count} / {len_grids})')
    #     cost = orig_cost - dijkstra(grid, start)
    #     res_d[cost] += 1
    # res_d.pop(0)
    # keys = sorted(res_d.keys())
    # for key in keys:
    #     print(f'{key}: {res_d[key]}')
    return res


class Graph:
    def __init__(self, grid: tuple[tuple[str, ...], ...]):
        self.visited: dict[tuple[int, int], float] = {}
        self.input_map = grid

    def dp(
        self,
        coords: tuple[int, int],
        weight: int,
    ):
        # could modify the more performant iterative algo
        i, j = coords
        if self.input_map[i][j] == '#':
            self.visited[coords] = float('inf')
            return

        self.visited[coords] = weight

        if self.input_map[i][j] == 'E':
            return

        for x, y in neighbors(i, j):
            curr_weight = weight + 1
            visited_key = (x, y)

            if visited_key in self.visited and self.visited[visited_key] <= curr_weight:
                continue
            self.dp(visited_key, curr_weight)


def main2() -> int:
    with open('input.txt', 'r') as f:
        grid = tuple(map(lambda x: tuple(x.rstrip()), f.readlines()))

    start = (0, 0)
    for i, line in enumerate(grid):
        for j, char in enumerate(line):
            if char == 'S':
                start = (i, j)
                break

    graph = Graph(grid)
    graph.dp(start, 0)
    # if the cheat saves at least 100 picoseconds,
    # it needs to connect 2 points on path whose diff in cost (minus shortcut len) is >= 100
    res = 0
    to_iter = list(filter(lambda x: x[1] != float('inf'), graph.visited.items()))
    path_len = len(to_iter)
    for node, orig_cost in to_iter:
        idx = 0
        while idx < path_len:
            other, cost = to_iter[idx]
            md = abs(node[0] - other[0]) + abs(node[1] - other[1])  # distance on grid
            if md > 20:
                idx += md - 20  # skip ahead - optimization
                continue
            if orig_cost - cost - md >= 100:  # 50
                res += 1
            idx += 1

    # from collections import defaultdict
    # res_d = defaultdict(int)
    # for node, orig_cost in graph.visited.items():
    #     if orig_cost == float('inf'):
    #         continue
    #     for other, cost in graph.visited.items():
    #         md = abs(node[0] - other[0]) + abs(node[1] - other[1])
    #         if cost == float('inf') or md > 20:
    #             continue
    #         cost = orig_cost - cost - md
    #         if cost >= 50:
    #             res_d[cost] += 1
    # for key in sorted(res_d.keys()):
    #     print(f'{key}: {res_d[key]}')
    return res


if __name__ == '__main__':
    print(main1())  # 1286
    print(main2())  # 989316
