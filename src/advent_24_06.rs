#![allow(clippy::unused_unit)]

use indicatif::ProgressIterator;
use crate::*;

// https://adventofcode.com/2024/day/6
// 11988 / 6755


fn plan_rotate(plan: &[Vec<bool>], rows: usize) -> Vec<Vec<bool>> {
    let mut res = vec![vec![false; rows]; rows];
    for i in 0..rows {
        for (j, plan_j) in enumerate(&plan) {
            // res[j][rows - i - 1] = plan[i][j]; - clockwise
            res[rows - i - 1][j] = plan_j[i];
        }
    }

    res
}


fn load_input() -> (Vec<Vec<bool>>, usize, usize) {
    let lines = open("day06/input.txt", "r")
        .readlines()
        .iter()
        .map(
            |line| line.chars().collect::<Vec<_>>()
        ).collect::<Vec<Vec<_>>>();
    // find the starting point
    let mut count = 0;
    let (idx1, idx2) = loop {
        //let idx = pstr::index(&line, '^');
        let line = &lines[count];
        let idx_none = line.iter().position(|&c| c == '^');
        match idx_none {
            Some(idx) => { break (count, idx); },
            None => {
                count += 1;
                continue;
            },
        }
    };
    (lines.into_iter().map(|line| line.into_iter().map(|c| c == '#').collect::<Vec<_>>()).collect::<Vec<Vec<_>>>(), idx1, idx2)
}


pub fn main_1() -> usize {  // 5269
    let (mut plan, mut idx1, mut idx2) = load_input();
    let rows = len(&plan);

    let mut res = [set(), set(), set(), set()];
    let mut deg = 0;
    loop {
        if idx1 == 0 {
            break;
        }
        idx1 -= 1;
        if plan[idx1][idx2] {
            idx1 += 1;
            plan = plan_rotate(&plan, rows);
            deg = (deg + 1) % 4;  // (deg + 90) % 360;
            let intermed = idx1;
            idx1 = rows - idx2 - 1;
            idx2 = intermed;
            continue;
        }
        res[deg].insert((idx1, idx2));
    }
    let mut final_res = res[0].clone();
    for (count, idxs) in enumerate(&res[1..].iter()) {
        for idx in idxs {
            let mut idx1 = idx.0;
            let mut idx2 = idx.1;
            for _ in 0..4 - (count + 1) {
                let intermed = idx1;
                idx1 = rows - idx2 - 1;
                idx2 = intermed;
            }
            final_res.insert((idx1, idx2));
        }
    }

    len(&final_res)
}


pub fn main_2() -> usize {  // 1957
    let (plan_orig, idx1_orig, idx2_orig) = load_input();
    let rows = len(&plan_orig);

    // bruteforcing, takes about a minute on Intel Macbook pro
    // could be optimized by checking only path from part1
    let mut loop_count = 0;
    for i in (0..rows).progress() {
        for j in 0..rows {
            if plan_orig[i][j] || (idx1_orig == i && idx2_orig == j) {
                continue;
            }
            let mut plan = plan_orig.clone();
            let mut idx1 = idx1_orig;
            let mut idx2 = idx2_orig;
            plan[i][j] = true;
            let mut res = [set(), set(), set(), set()];
            let mut deg = 0;
            loop {
                if idx1 == 0 {
                    break;
                }
                idx1 -= 1;
                if plan[idx1][idx2] {
                    idx1 += 1;
                    plan = plan_rotate(&plan, rows);
                    deg = (deg + 1) % 4;
                    let intermed = idx1;
                    idx1 = rows - idx2 - 1;
                    idx2 = intermed;
                    continue;
                }
                if res[deg].contains(&(idx1, idx2)) {
                    loop_count += 1;
                    break;
                }
                res[deg].insert((idx1, idx2));
            }
        }
    }

    loop_count
}
