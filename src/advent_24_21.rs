#![allow(clippy::unused_unit)]

use crate::*;

//use std::pin::Pin;
//use std::pin::pin;

// https://adventofcode.com/2024/day/21
// 5074 / 19786


fn simple(
    start: (i8, i8),
    end: (i8, i8),
    reverse_: bool,
) -> Vec<(i8, i8)> {
    // simple path finding by going straight to the end
    // correctly deciding, if we should go up-down or left-right first
    let mut visited = vec![start];
    let vector = (end.0 - start.0, end.1 - start.1);
    let vector_sgn = (math::sgn(vector.0), math::sgn(vector.1));
    let mut condition = vector_sgn.1 == -1;  // if going left, go left first
    if reverse_ {
        condition = !condition;
    }
    if condition {
        if vector.1 < 0 {
            for i in 1..-vector.1 + 1 {
                visited.push((start.0, start.1 - i));
            }
        } else {
            for i in 1..vector.1 + 1 {
                visited.push((start.0, start.1 + i));
            }
        }
        let i = vector.1;

        if vector.0 < 0 {
            for j in 1..-vector.0 + 1 {
                visited.push((start.0 - j, start.1 + i));
            }
        } else {
            for j in 1..vector.0 + 1 {
                visited.push((start.0 + j, start.1 + i));
            }
        }
        return visited;
    }

    if vector.0 < 0 {
        for i in 1..-vector.0 + 1 {
            visited.push((start.0 - i, start.1));
        }
    } else {
        for i in 1..vector.0 + 1 {
            visited.push((start.0 + i, start.1));
        }
    }
    let i = vector.0;

    if vector.1 < 0 {
        for j in 1..-vector.1 + 1 {
            visited.push((start.0 + i, start.1 - j));
        }
    } else {
        for j in 1..vector.1 + 1 {
            visited.push((start.0 + i, start.1 + j));
        }
    }

    visited
}


fn _get_sequence_length(
    layer: u8,
    to_visit: &Vec<Vec<(i8, i8)>>,
) -> usize {
    if layer == 0 {
        return len(&to_visit);
    }
    let mut start = (0, 2);
    let mut len_to_visit = 0;
    for visited_ in to_visit {
        for key_ in visited_.iter().chain([&(0, 0)].into_iter()) {  // Add A
            let end = match key_ {
                (-1, 0) => (0, 1),
                (1, 0) => (1, 1),
                (0, -1) => (1, 0),
                (0, 1) => (1, 2),
                (0, 0) => (0, 2),
                _ => (0, 0),  // panic!
            };
            let mut pressed = simple(start, end, false);
            if pressed.contains(&(0, 0)) {
                pressed = simple(start, end, true);
            }
            len_to_visit += _get_sequence_length(
                layer - 1, &vec![np::diff_pairs(&pressed)],
            );
            start = end;
        }
    }

    len_to_visit
}


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct MemoKey {
    layer: u8,
    sequences: Vec<Vec<(i8, i8)>>,
}


fn get_sequence_length_dp(
    layer: u8,
    to_visit: &Vec<Vec<(i8, i8)>>,
    memo: &mut Dict<MemoKey, usize>,
) -> usize {
    let key = MemoKey {
        layer,
        sequences: to_visit.clone(),
    };
    
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }
    
    if layer == 0 {
        let result = len(to_visit);
        memo.insert(key, result);
        return result;
    }
    
    let mut start = (0, 2);
    let mut len_to_visit = 0;
    for visited_ in to_visit {
        for key_ in visited_.iter().chain([&(0, 0)].into_iter()) {
            let end = match key_ {
                (-1, 0) => (0, 1),
                (1, 0) => (1, 1),
                (0, -1) => (1, 0),
                (0, 1) => (1, 2),
                (0, 0) => (0, 2),
                _ => (0, 0),
            };
            let mut pressed = simple(start, end, false);
            if pressed.contains(&(0, 0)) {
                pressed = simple(start, end, true);
            }
            len_to_visit += get_sequence_length_dp(
                layer - 1, 
                &vec![np::diff_pairs(&pressed)],
                memo,
            );
            start = end;
        }
    }
    
    memo.insert(key, len_to_visit);
    len_to_visit
}


fn solve_next_pad(
    mut start: (i8, i8), code: Vec<char>, layers: u8,
    memo: &mut Dict<MemoKey, usize>,
) -> usize {
    let mut to_visit = vec![];  // : list[tuple[(i8, i8), ...]]
    for key in code {
        let end = match key {
            '#' => (3, 0),
            '0' => (3, 1),
            'A' => (3, 2),
            '1' => (2, 0),
            '2' => (2, 1),
            '3' => (2, 2),
            '4' => (1, 0),
            '5' => (1, 1),
            '6' => (1, 2),
            '7' => (0, 0),
            '8' => (0, 1),
            '9' => (0, 2),
            _ => (0, 0),  // panic
        };
        let mut visited = simple(start, end, false);
        if visited.contains(&(3, 0)) {
            visited = simple(start, end, true);
        }
        start = visited[len(&visited) - 1];
        to_visit.push(np::diff_pairs(&visited));
    }

    get_sequence_length_dp(layers, &to_visit, memo)
}


fn solve_part(layers: u8) -> usize {
    let codes = map(
        |x| pstr::rstrip(&x).chars().collect::<Vec<_>>(),
        open("day21/input.txt", "r").readlines(),
    ).collect::<Vec<Vec<_>>>();
    let vals: Vec<_> = comp![int::<usize>(&String::from_iter(&code[0..len(&code) - 1])) for code in &codes];

    let memo: std::cell::RefCell<Dict<MemoKey, usize>> = std::cell::RefCell::new(dict());

    let mut res = 0;
    let start = (3, 2);
    for (code, val) in zip(codes, vals) {
        let sol = solve_next_pad(start, code, layers, &mut memo.borrow_mut());
        res += val * sol;
    }

    res
}


pub fn main_1() -> usize {
    solve_part(3)
}


pub fn main_2() -> usize {
    solve_part(26)
}
