#![allow(clippy::unused_unit)]

use crate::*;
use indicatif::ProgressIterator;

// https://adventofcode.com/2024/day/20
// 5552 / 12403

/// Return all 4 direct neighbor indices
fn neighbors(x: usize, y: usize) -> Vec<(usize, usize)> {
    vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}


#[allow(clippy::type_complexity)]
fn dijkstra(input_map: Vec<Vec<char>>, start: (usize, usize)) -> (Option<u32>, Dict<(usize, usize), u32>) {
    let mut visited = dict();
    let mut pq = heapq::heapify(vec![]);

    let current_cost = 0;
    heapq::heappush(&mut pq, (current_cost, start));

    while len(&pq) > 0 {
        let (current_cost, (x, y)) = heapq::heappop(&mut pq);
        let visited_key = (x, y);

        if visited.contains_key(&visited_key) && visited[&visited_key] <= current_cost {
            continue;
        }

        visited.insert(visited_key, current_cost);

        if input_map[x][y] == 'E' {
            return (Some(current_cost), visited);
        }

        for (dx, dy) in neighbors(x, y) {
            if input_map[dx][dy] != '#' {
                let curr_weight = current_cost + 1;
                heapq::heappush(&mut pq, (curr_weight, (dx, dy)));
            }
        }
    }

    // should not happen
    (None, dict())
}


pub fn main_1() -> usize {
    let grid = map(
        |x| pstr::rstrip(&x).chars().collect::<Vec<_>>(),
        open("day20/input.txt", "r").readlines(),
    ).collect::<Vec<Vec<_>>>();

    let mut start = (0, 0);
    for (i, line) in enumerate(&grid) {
        for (j, char) in enumerate(&line) {
            if char == 'S' {
                start = (i, j);
                break;
            }
        }
    }

    let mut grids = vec![];
    for (mut i, line) in enumerate(&grid[1..len(&grid) - 1].to_vec()) {
        i += 1;
        for (mut j, _) in enumerate(&line[1..len(&line) - 1].to_vec()) {
            j += 1;
            let mut tmp = line[0..j].to_vec();
            if line[j] == '#' {
                tmp.push('.');
            } else {
                tmp.push(line[j]);
            }
            tmp.extend_from_slice(&line[j + 1..]);
            let mut current = grid[0..i].to_vec();
            current.push(tmp);
            current.extend_from_slice(&grid[i + 1..]);
            grids.push(current);
        }
    }

    let mut res = 0;
    let orig_cost_or_none = dijkstra(grid, start).0;
    let orig_cost = match orig_cost_or_none {
        Some(orig_cost) => orig_cost,
        None => panic!("No path to exit"),
    };

    for grid in grids.into_iter().progress() {
        if orig_cost - (dijkstra(grid, start).0.unwrap_or(0)) >= 100 {
            res += 1;
        }
    }

    res
}


struct Graph {
    visited: Dict<(isize, isize), isize>,
    input_map: Vec<Vec<char>>,
}

impl Graph {
    fn dp(
        &mut self,
        coords: (isize, isize),
        weight: isize,
    ) -> () {
        let (i, j) = coords;
        if self.input_map[i as usize][j as usize] == '#' {
            self.visited.insert(coords, u32::MAX as isize);
            return;
        }

        self.visited.insert(coords, weight);

        if self.input_map[i as usize][j as usize] == 'E' {
            return;
        }

        for (x, y) in neighbors(i as usize, j as usize) {
            let curr_weight = weight + 1;
            let visited_key = (x as isize, y as isize);

            // Check if we've already visited this node with a better or equal cost
            if self.visited.contains_key(&visited_key) && self.visited[&visited_key] <= weight {
                continue;
            }

            self.dp(visited_key, curr_weight);
        }
    }
}


pub fn main_2() -> u64 {
    let grid = map(
        |x| pstr::rstrip(&x).chars().collect::<Vec<_>>(),
        open("day20/input.txt", "r").readlines(),
    ).collect::<Vec<Vec<_>>>();

    let mut start = (0, 0);
    for (i, line) in enumerate(&grid) {
        for (j, character) in enumerate(&line) {
            if character == 'S' {
                start = (i as isize, j as isize);
                break;
            }
        }
    }

    let mut graph = Graph{
        visited: dict(),
        input_map: grid,
    };
    graph.dp(start, 0);
    // if the cheat saves at least 100 picoseconds,
    // it needs to connect 2 points on path whose diff in cost (minus shortcut len) is >= 100
    let mut res = 0;
    let to_iter = list(filter(|(_, val)| **val != u32::MAX as isize, &graph.visited.items()));
    let path_len = len(&to_iter);
    for (node, orig_cost) in &to_iter {
        let mut idx = 0;
        while idx < path_len {
            let (other, cost) = to_iter[idx];
            let md = (abs(node.0 - other.0) + abs(node.1 - other.1)) as usize;  // distance on grid
            if md > 20 {
                idx += 1;
                continue;
            }
            if *orig_cost - cost - md as isize >= 100 {
                res += 1;
            }
            idx += 1;
        }
    }

    res
}
