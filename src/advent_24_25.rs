#![allow(clippy::unused_unit)]

use crate::*;

// https://adventofcode.com/2024/day/25
// 4613 /


pub fn main_1() -> usize {  // 2978
    let binding = open("day25/input.txt", "r").read();  // make borowchecker happy
    let input = pstr::rstrip(&binding).split("\n\n").collect::<Vec<_>>();

    // or create only one dequeue and push locks to the front and keys to the back
    let mut locks = vec![];
    let mut keys = vec![];
    for lock_or_key in input {
        let lines = lock_or_key.split("\n").collect::<Vec<_>>();
        let rows = len(&lines);
        
        let lines_of_chars = map(|line| line.chars().collect::<Vec<_>>(), lines).collect::<Vec<Vec<char>>>();
        let cols = len(&lines_of_chars[0]);
        let mut to_comp = np::zeros((1, cols), 'a')[0].clone();
        for tc in to_comp.iter_mut().take(cols) {
            *tc = '#';
        }
        //if lines[0].chars().collect::<Vec<_>>() == to_comp {
        if lines_of_chars[0] == to_comp {
            // transpose
            let lines_vetical = (0..cols).map(|col| {
                (1..rows)
                    .map(|row| lines_of_chars[row][col])
                    .collect()
            }).collect::<Vec<Vec<char>>>();
            locks.push(lines_vetical);
        } else {
            let lines_vetical = (0..cols).map(|col| {
                (0..rows - 1)
                    .map(|row| lines_of_chars[row][col])
                    .collect()
            }).collect::<Vec<Vec<char>>>();  // reversed to have the code below work for both.
            // but likely not needed, as we only count #s
            keys.push(lines_vetical);
        }
    }

    let mut pins_hs = vec![vec![]; len(&locks)];
    for (count, lock) in enumerate(&locks) {
        for line in lock {
            pins_hs[count].push(line.iter().filter(|x| **x == '#').collect::<Vec<_>>().len());
        }
    }
    let mut keys_hs = vec![vec![]; len(&keys)];
    for (count, key) in enumerate(&keys) {
        for line in key {
            keys_hs[count].push(line.iter().filter(|x| **x == '#').collect::<Vec<_>>().len());
        }
    }

    let mut res = 0;
    let len_row = len(&pins_hs[0]);
    for key in &keys_hs {
        'outer: for lock in &pins_hs {
            for (count, line) in enumerate(&lock) {
                if line + key[count] > len_row {
                    continue 'outer;
                }
            }
            res += 1;
        }
    }

    res
}
