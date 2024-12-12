"""
https://adventofcode.com/2024/day/12
"""

import numpy as np  # 2.2.0
from numpy.lib.stride_tricks import sliding_window_view  # as_strided
from scipy.ndimage import label  # 1.14.1


def cluster_pixels(pixel_indices: list[tuple[int, int]], grid_size: int) -> list[list[tuple[int, int]]]:
    grid = np.zeros((grid_size, grid_size))

    for index in pixel_indices:
        grid[index] = True

    structure = np.ones((3, 3))  # This defines 8-connectivity
    # Turn to only main direction connectivity
    structure[0, 0] = 0
    structure[0, 2] = 0
    structure[2, 0] = 0
    structure[2, 2] = 0
    labeled_array, _ = label(grid, structure)

    clusters: dict[int, list[tuple[int, int]]] = {}
    for pixel in pixel_indices:
        label_value = labeled_array[pixel]
        if label_value not in clusters:
            clusters[label_value] = []
        clusters[label_value].append(pixel)

    return [pixels for pixels in clusters.values() if pixels]


def count_corners(grid: np.ndarray) -> int:
    if grid[1, 1] == 0:
        return 0

    n = (0, 1)
    e = (1, 2)
    ne = (0, 2)
    ret = 0
    for _ in range(4):
        if ((grid[n] == 0 and grid[e] == 0) or (grid[ne] == 0 and grid[n] == 1 and grid[e] == 1)):
            ret += 1
        grid = np.rot90(grid)

    return ret


def main1() -> int:
    """main"""
    with open('input.txt', 'r') as f_in:
        plan = np.array(list(map(np.array, [line.rstrip() for line in f_in.read().splitlines()])))

    len_plan = len(plan)
    unique = set()
    for line in plan:
        unique.update(set(line))

    total = 0
    for item in unique:
        vertices = []
        for i, row in enumerate(plan):
            for j, cell in enumerate(row):
                if cell == item:
                    vertices.append((i, j))

        clusters = cluster_pixels(vertices, len_plan)
        for cluster in clusters:
            perim = 0
            for pixel in cluster:
                for i, j in ((0, 1), (1, 0), (0, -1), (-1, 0)):
                    if (pixel[0] + i, pixel[1] + j) not in cluster:
                        perim += 1
            total += len(cluster) * perim

    return total


def main2() -> int:
    with open('input.txt', 'r') as f_in:
        plan = np.array(list(map(np.array, [line.rstrip() for line in f_in.read().splitlines()])))

    len_plan = len(plan)
    unique = set()
    for line in plan:
        unique.update(set(line))

    total = 0
    for item in unique:
        vertices = []
        for i, row in enumerate(plan):
            for j, cell in enumerate(row):
                if cell == item:
                    vertices.append((i, j))

        clusters = cluster_pixels(vertices, len_plan)
        for cluster in clusters:
            # add a border of 0s - no index out of bound checking, yeah
            plan_to_measure_perim = np.zeros((len_plan + 2, len_plan + 2), dtype=bool)
            for pixel in map(lambda x: (x[0] + 1, x[1] + 1), cluster):
                plan_to_measure_perim[pixel] = True

            if len(cluster) == 1:
                sides = 4
            else:
                sides = 0
                windows = sliding_window_view(plan_to_measure_perim, (3, 3))
                for window_row in windows:
                    for window in window_row:
                        sides += count_corners(window)
            total += len(cluster) * sides

    return total


if __name__ == '__main__':
    print(main1())  # 1546338
    print(main2())  # 978590
