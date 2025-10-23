#![allow(clippy::unused_unit)]

use crate::*;
use heapq;

// https://adventofcode.com/2024/day/16
// 6585 / 16167


/// Return all 4 direct neighbor indices
fn neighbors(x: usize, y: usize) -> Vec<(usize, usize)> {
    vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}


#[allow(clippy::type_complexity)]
fn dijkstra(input_map: Vec<Vec<char>>, start: (usize, usize)) -> (u32, Dict<(usize, usize, (i16, i16)), u32>) {
    //let rows = len(&input_map);
    //let cols = len(&input_map[0]);
    let mut visited = dict();  // Dict<(usize, usize, (i16, i16)), u32>
    //let mut visited_ret = dict();
    //visited_ret.insert((start.0, start.1, (0, 1)), 0);
    let mut pq = heapq::heapify(vec![]);

    let current_cost = 0;
    heapq::heappush(&mut pq, (current_cost, start, (0, 1)));

    while len(&pq) > 0 {
        let (current_cost, (x, y), last_direction) = heapq::heappop(&mut pq);
        let visited_key = (x, y, last_direction);

        if visited.contains_key(&visited_key) && visited[&visited_key] <= current_cost {
            continue;
        }

        visited.insert(visited_key, current_cost);

        if input_map[x][y] == 'E' {
            return (current_cost, visited);
        }

        for (dx, dy) in neighbors(x, y) {
            if input_map[dx][dy] != '#' {
                let change_direction = (dx, dy) != ((x as i16 + last_direction.0) as usize, (y as i16 + last_direction.1) as usize);
                let mut curr_weight = current_cost + 1;
                let increment = if change_direction { 1000 } else { 0 };
                curr_weight += increment;

                let new_direction = (dx as i16 - x as i16, dy as i16 - y as i16);

                heapq::heappush(&mut pq, (curr_weight, (dx, dy), new_direction));
            }
        }
    }

    // should not happen
    (current_cost, dict())
}


fn get_input_map() -> Vec<Vec<char>> {
    map(
        |x| pstr::rstrip(&x).chars().collect::<Vec<_>>(),
        open("day16/input.txt", "r").readlines(),
    ).collect::<Vec<Vec<_>>>()
}


fn get_start(input_map: &Vec<Vec<char>>) -> (usize, usize) {
    let mut start = (0, 0);
    for (i, line) in enumerate(input_map) {
        for (j, c) in enumerate(&line.iter()) {
            if *c == 'S' {
                start = (i, j);
                break;
            }
        }
    }
    start
}


pub fn main_1() -> u32 {  // 89460
    let input_map = get_input_map();
    let start = get_start(&input_map);

    dijkstra(input_map, start).0
}


/// Find all nodes making up any shortest path.
/// This is achieved by following the shortest path and adding all nodes,
/// whose cost is 1 or 1001 off the adjacent one.
fn backtrack(node: (usize, usize), direction: (i16, i16), nodes: &mut Set<(usize, usize)>, visited: &Dict<(usize, usize, (i16, i16)), u32>) -> () {
    nodes.add(node);
    let nlist = neighbors(node.0, node.1);

    for (x, y) in nlist {
        let rhs_or_none = visited.get(&(node.0, node.1, direction));
        let rhs = rhs_or_none.unwrap_or(&0);
        //let Some(rhs) = visited.get(&(node.0, node.1, direction)) else {
        //    0
        //};
        if
            let Some(lhs) = visited.get(&(x, y, direction))
            && *lhs + 1 == *rhs
            && !nodes.contains(&(x, y))
        {
            backtrack((x, y), direction, nodes, visited);
        }
        for new_dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let rhs_or_none = visited.get(&(node.0, node.1, direction));
            let rhs = rhs_or_none.unwrap_or(&0);
            if
                let Some(lhs) = visited.get(&(x, y, new_dir)) 
                && *lhs + 1001 == *rhs
                && !nodes.contains(&(x, y))
            {
                backtrack((x, y), new_dir, nodes, visited);
            }
        }
    }
}


pub fn main_2() -> usize {  // 504
    let input_map = get_input_map();
    let start = get_start(&input_map);

    let (min_win, visited) = dijkstra(input_map.clone(), start);
    let mut end = (0, 0);
    for (i, line) in enumerate(&input_map) {
        for (j, char) in enumerate(&line) {
            if char == 'E' {
                end = (i, j);
                break;
            }
        }
    }

    let mut nodes = set();
    // find the direction to end
    for (key, value) in visited.items() {
        let node = (key.0, key.1);
        if node == end && *value == min_win {
            backtrack(node, key.2, &mut nodes, &visited);
            break;
        }
    }

    len(&nodes)
}
