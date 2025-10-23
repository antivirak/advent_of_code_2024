#![allow(clippy::unused_unit)]

use crate::*;
use itertools::Itertools;

// https://adventofcode.com/2024/day/8
// 6559 / 6049


fn get_input() -> (Dict<char, Vec<(i16, i16)>>, i16) {
    let lines = open("day08/input.txt", "r").readlines()
        .iter()
        .map(
            |line| line.chars().collect::<Vec<_>>()
        ).collect::<Vec<Vec<_>>>();

    let mut antennas = dict::<char, Vec<(i16, i16)>>();
    for (i, line) in enumerate(&lines) {
        for (j, c) in enumerate(&line) {
            if c == '.' {
                continue;
            }
            if antennas.contains_key(&c) {
                antennas.get_mut(&c).unwrap().push((i as i16, j as i16));
                continue;
            }
            antennas.insert(c, vec![(i as i16, j as i16)]);
        }
    }

    // assuming square
    (antennas, len(&lines) as i16)
}


pub fn main_1() -> usize {  // 280
    let (antennas, plan_len) = get_input();

    let mut antinodes = set();
    for pos in antennas.values() {
        for pair in pos.iter().combinations(2) {
            let &(mut dx, mut dy) = pair[0];
            dx -= pair[1].0;
            dy -= pair[1].1;
            let x_new = pair[1].0 - dx;
            if x_new < plan_len && x_new >= 0 {
                let y_new = pair[1].1 - dy;
                if y_new < plan_len && y_new >= 0 {
                    antinodes.insert((x_new, y_new));
                }
            }
            let x_new = pair[0].0 + dx;
            if x_new < plan_len && x_new >= 0 {
                let y_new = pair[0].1 + dy;
                if y_new < plan_len && y_new >= 0 {
                    antinodes.insert((x_new, y_new));
                }
            }
        }
    }

    len(&antinodes)
}


pub fn main_2() -> usize {  // 958
    let (antennas, plan_len) = get_input();

    let mut antinodes = set();
    for pos in antennas.values() {
        for pair in pos.iter().combinations(2) {
            antinodes.insert(*pair[0]);
            antinodes.insert(*pair[1]);
            let &(mut dx, mut dy) = pair[0];
            dx -= pair[1].0;
            dy -= pair[1].1;
            let &(mut x, mut y) = pair[1];
            loop {
                x -= dx;
                if x < plan_len && x >= 0 {
                    y -= dy;
                    if y < plan_len && y >= 0 {
                        antinodes.insert((x, y));
                    } else { break; }
                } else { break; }
            }
            (x, y) = *pair[0];
            loop {
                x += dx;
                if x < plan_len && x >= 0 {
                    y += dy;
                    if y < plan_len && y >= 0 {
                        antinodes.insert((x, y));
                    } else { break; }
                } else { break; }
            }
        }
    }

    len(&antinodes)
}
