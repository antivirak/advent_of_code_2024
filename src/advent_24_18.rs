#![allow(clippy::unused_unit)]

use crate::*;
use heapq;
use np;

// https://adventofcode.com/2024/day/18
// 3933 / 3432


/// Return all 4 direct neighbor indices
fn neighbors(x: usize, y: usize) -> Vec<(usize, usize)> {
    vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}


#[allow(clippy::type_complexity)]
fn dijkstra(input_map: &[Vec<i32>], start: (usize, usize)) -> Option<i32> {
    let mut visited = dict();  // : dict[tuple[int, int], int]
    let mut pq = heapq::heapify(vec![]);  // : list[tuple[int, tuple[int, int]]]
    let current_cost = 0;
    heapq::heappush(&mut pq, (current_cost, start));

    while !pq.is_empty() {
        let (current_cost, (x, y)) = heapq::heappop(&mut pq);

        let visited_key = (x, y);
        if visited.contains_key(&visited_key) && visited[&visited_key] <= current_cost {
            continue;
        }

        // store current cost
        visited.insert(visited_key, current_cost);

        // exit tile
        if input_map[x][y] == 2 {
            return Some(current_cost);
        }

        for (dx, dy) in neighbors(x, y) {
            if input_map[dx][dy] != 1 {
                let curr_weight = current_cost + 1;
                heapq::heappush(&mut pq, (curr_weight, (dx, dy)));
            }
        }
    }
                
    // should not happen
    None
}


fn load_input(nanoseconds: usize, grid_len: usize) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let f = open("day18/input.txt", "r");
    let coords: Vec<Vec<i32>> = map(|x| map(int, pstr::rstrip(&x).split(",").collect::<Vec<&str>>()).collect::<Vec<i32>>(), f.readlines()).collect();

    let mut grid = np::zeros((grid_len + 2, grid_len + 2), 0);

    for idx in range(nanoseconds) {
        let coord = &coords[idx];
        grid[coord[0] as usize + 1][coord[1] as usize + 1] = 1;
    }

    // Add frame around - no need to check bounds later
    for i in 0..=grid_len + 1 {
        grid[i][0] = 1;
        grid[0][i] = 1;
        grid[i][grid_len + 1] = 1;
        grid[grid_len + 1][i] = 1;
    }
    grid[grid_len][grid_len] = 2;

    (grid, coords[nanoseconds..len(&coords)].to_vec())
}


pub fn main_1() -> u32 {
    let grid_len = 71;
    let grid = load_input(1024, grid_len).0;
    // print(&grid);

    let start = (1, 1);

    dijkstra(&grid, start).unwrap() as u32
}


pub fn main_2() -> Vec<i32> {
    let grid_len = 71;
    let (mut grid, coords) = load_input(1024, grid_len);

    let start = (1, 1);

    for coord in coords {
        grid[coord[0] as usize + 1][coord[1] as usize + 1] = 1;
        let res = dijkstra(&grid, start);
        if res.is_none() {
            return coord;
        }
    }

    panic!("No valid coordinates found");
}
