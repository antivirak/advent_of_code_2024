#![allow(clippy::unused_unit)]

use crate::*;

// https://adventofcode.com/2024/day/5
// 15227 / 33181


fn load_input() -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let lines = open("day05/input.txt", "r").read();
    let splitted = lines.split("\n\n").collect::<Vec<_>>();

    let numbers_list = splitted[1].lines().map(|line| line.split(",").map(int).collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();
    let rules = splitted[0].lines().map(|line| line.split("|").map(int).collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();

    // transpose
    let rows = len(&rules);
    let cols = len(&rules[0]);
    let rules_zip = (0..cols).map(|col| {
        (0..rows)
            .map(|row| rules[row][col])
            .collect()
    }).collect::<Vec<Vec<i32>>>();
    // or maybe dict?

    (numbers_list, rules_zip)
}


pub fn main_1() -> i32 {  // 4905
    let (numbers_list, rules_zip) = load_input();

    let mut res = 0;
    'outer: for numbers in numbers_list {
        for (count, number) in enumerate(&numbers.clone()) {
            let mut rules_trim = rules_zip[1].clone();
            let mut rules_trim_0 = rules_zip[0].clone();
            while rules_trim_0.contains(&number) {
                let idx = list::index(&rules_trim_0, number);
                if numbers[..count].contains(&rules_trim[idx]) {
                    continue 'outer;
                }
                rules_trim = rules_trim[idx + 1..].to_vec();
                rules_trim_0 = rules_trim_0[idx + 1..].to_vec();
            }
        }
        res += numbers[len(&numbers) / 2];  // integer div is floor automatically
    }

    res
}


pub fn main_2() -> i32 {  // 6204
    let (numbers_list, rules_zip) = load_input();

    let mut res = 0;
    for numbers in numbers_list {
        let mut ordered = vec![];
        let mut incorrect = false;
        let mut to_correct = vec![];
        for (count, number) in enumerate(&numbers.clone()) {
            let mut rules_trim = rules_zip[1].clone();
            let mut rules_trim_0 = rules_zip[0].clone();
            ordered.push(number);
            while rules_trim_0.contains(&number) {
                let idx = list::index(&rules_trim_0, number);
                if numbers[..count].contains(&rules_trim[idx]) {
                    let maybe_none = ordered.pop();
                    if Option::is_none(&maybe_none) {
                        break;
                    }
                    to_correct.push(number);
                    incorrect = true;
                    break;
                }
                rules_trim = rules_trim[idx + 1..].to_vec();
                rules_trim_0 = rules_trim_0[idx + 1..].to_vec();
            }
        }
        // push incorrect numbers to right positions
        let mut ordered_intermed = collections::deque(ordered);
        for number in to_correct {
            let mut rules_trim = rules_zip[1].clone();
            let mut rules_trim_0 = rules_zip[0].clone();
            ordered_intermed.push_back(number);
            let mut rotation = 1;
            while rules_trim_0.contains(&number) {
                let idx = list::index(&rules_trim_0, number);
                ordered_intermed.make_contiguous();
                while ordered_intermed.as_slices().0[..len(&ordered_intermed) - rotation].contains(&rules_trim[idx]) {
                    ordered_intermed.rotate_right(rotation - 1);
                    ordered_intermed.pop_back().unwrap();
                    ordered_intermed.rotate_right(1);
                    ordered_intermed.push_back(number);
                    ordered_intermed.rotate_left(rotation);
                    rotation += 1;
                    ordered_intermed.make_contiguous();
                }
                rules_trim = rules_trim[idx + 1..].to_vec();
                rules_trim_0 = rules_trim_0[idx + 1..].to_vec();
            }
        }
        ordered = ordered_intermed.into();

        if incorrect {
            res += ordered[len(&ordered) / 2];
        }
    }

    res
}
