#![allow(clippy::unused_unit)]

use regex::Regex;

use crate::*;

// word search (crossout) puzzle

// https://adventofcode.com/2024/day/4
// 38515 / 34148


pub fn main_1() -> usize {  // 2530
    let re = Regex::new(r"(XMAS)").unwrap();
    let mut res = 0;
    let lines = open("day04/input.txt", "r").readlines().iter().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();
    let rows = len(&lines);
    let cols = len(&lines[0]);
    // transpose
    let lines_vetical = (0..cols).map(|col| {
        (0..rows)
            .map(|row| lines[row][col])
            .collect()
    }).collect::<Vec<Vec<char>>>();

    let mut lines_diag: Vec<Vec<char>> = vec![vec![]];
    for diff in 0..(cols - 3) {
        // or use ndarray/struct.ArrayBase.html#method.diag
        let mut intermed1 = vec![];
        let mut intermed2 = vec![];
        let mut intermed3 = vec![];
        let mut intermed4 = vec![];
        for i in (0..(rows - diff)).rev() {
            // yeah, could have rotated the matrix and have less logic
            intermed1.push(lines[i           ][i + diff]);  // right up over main diag
            intermed2.push(lines[i + diff    ][i       ]);  // right up under diag
            intermed3.push(lines[rows - i - 1][i + diff]);  // right down under diag
            intermed4.push(lines[rows - i - 1 - diff][i]);  // right down over diag
        }
        lines_diag.push(intermed1);
        lines_diag.push(intermed2);
        lines_diag.push(intermed3);
        lines_diag.push(intermed4);
    }
    lines_diag = lines_diag[1..].to_vec();  // remove empty
    // frankly, does not work with test data - counts the main diag twice
    // println!("{:?}", sorted(&lines_diag, len).collect::<Vec<_>>());
    for item_set in [
        lines.clone(), lines_vetical.clone(), lines_diag.clone(),
        lines.into_iter().rev().collect::<Vec<_>>(),
        lines_vetical.into_iter().rev().collect::<Vec<_>>(),
        lines_diag.into_iter().rev().collect::<Vec<_>>(),
    ] {
        for line in &item_set {
            let line_str = line.iter().collect::<String>();

            res += re.find_iter(&line_str).collect::<Vec<_>>().len();
            // res += re.captures_iter(&line_str).map(|c| c.extract::<1>().1.len()).sum::<usize>();
        }
    }

    res
}


pub fn main_2() -> u32 {  // 1921
    let mut res = 0;
    let lines = open("day04/input.txt", "r").readlines().iter().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();
    let rows = len(&lines);
    let cols = len(&lines[0]);

    for i in 1..(rows - 1) {
        for j in 1..(cols - 1) {
            if lines[i][j] != 'A' {
                // A in middle
                continue;
            }
            let to_compare = [
                lines[i - 1][j - 1], lines[i + 1][j + 1],
                lines[i - 1][j + 1], lines[i + 1][j - 1],
            ].into_iter().collect::<String>();
            if pstr::count(&to_compare, "M") != 2 || pstr::count(&to_compare, "S") != 2 {
                // 2 Ms and 2 Ss
                continue;
            }
            if lines[i - 1][j - 1] == 'M' && (
                (lines[i - 1][j + 1] == 'S' && lines[i + 1][j - 1] == 'M')
                ||
                (lines[i + 1][j - 1] == 'S' && lines[i - 1][j + 1] == 'M')
            ) {
                res += 1;
                continue;
            }
            if lines[i - 1][j - 1] == 'S' && (
                (lines[i - 1][j + 1] == 'M' && lines[i + 1][j - 1] == 'S')
                ||
                (lines[i + 1][j - 1] == 'M' && lines[i - 1][j + 1] == 'S')
            ) {
                res += 1;
            }
        }
    }

    res
}
