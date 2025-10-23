#![allow(clippy::unused_unit)]

use indicatif::ParallelProgressIterator;
use rayon::prelude::*;

use crate::*;

// https://adventofcode.com/2024/day/7
// 9296 / 8197


pub fn main_1() -> usize {  // 538191549061
    let lines = open("day07/input.txt", "r").readlines();

    lines.par_iter().map(|line_iter| {
        let keyval = pstr::rstrip(line_iter).split(": ").collect::<Vec<_>>();
        let test = int::<usize>(keyval[0]);
        let equation = keyval[1].split(" ").map(int::<usize>).collect::<Vec<_>>();
        let len_eq = len(&equation) - 1;
        for variation in itertools::itertools::repeat_n(0..2, len_eq).multi_cartesian_product() {
            let mut intermed = equation[0];
            for (count, item) in enumerate(&equation[1..].to_vec()) {
                if variation[count] == 0 {
                    intermed += item;
                } else {
                    intermed *= item;
                }
            }
            if intermed == test {
                return test;
            }
        }
        0
    }).sum()
}


pub fn main_2() -> usize {  // 34612812972206
    let lines = open("day07/input.txt", "r").readlines();

    lines.par_iter().progress().map(|line_iter| {
        let keyval = pstr::rstrip(line_iter).split(": ").collect::<Vec<_>>();
        let test = int::<usize>(keyval[0]);
        let equation = keyval[1].split(" ").map(int::<usize>).collect::<Vec<_>>();
        let len_eq = len(&equation) - 1;
        for variation in itertools::itertools::repeat_n(0..3, len_eq).multi_cartesian_product() {
            let mut intermed = equation[0];
            for (count, item) in enumerate(&equation[1..].to_vec()) {
                if variation[count] == 0 {
                    intermed += item;
                } else if variation[count] == 1 {
                    intermed *= item;
                } else {
                    // concat
                    intermed = int(&format!("{intermed}{item}"));
                }
            }
            if intermed == test {
                return test;
            }
        }
        0
    }).sum()
}
