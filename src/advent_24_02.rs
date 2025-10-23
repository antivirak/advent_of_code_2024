#![allow(clippy::unused_unit)]

use crate::*;

// https://adventofcode.com/2024/day/2
// 11618 / 58528


fn helper(split_res_new: &Vec<i16>) -> bool {
    for (item, shift) in split_res_new[..(len(&split_res_new) - 1)].iter().zip(
        split_res_new[1..].iter()
    ) {
        let diff = abs(item - shift);
        if diff == 0 || diff > 3 {
            return false;
        }
    }
    true
}


pub fn main_1() -> i16 {  // 332
    let lines = open("day02/input.txt", "r").readlines();
    let mut count = 0;
    for line_iter in lines {
        let line = pstr::rstrip(&line_iter);
        let split_res = map(int::<i16>, line.split(" ")).collect::<Vec<_>>();

        let sorted_res = sorted(&split_res, |x| *x).collect::<Vec<i16>>();
        if split_res != sorted_res && split_res != sorted_res.clone().into_iter().rev().collect::<Vec<i16>>() {//reversed(&sorted_res).collect::<Vec<i16>>() {
            continue;
        }
        if !helper(&split_res) {
            continue;
        }
        //println!("{:?}", split_res);
        count += 1;
    }

    count
}


fn test2(split_res: &Vec<i16>, inner_count: usize) -> bool {
    let mut split_res_new = split_res.clone();
    for inner_count2 in 1..(len(&split_res) + 1) {
        for (item, shift) in split_res[0..(len(&split_res) - 1)].iter().zip(
            split_res[1..].iter()
        ) {
            let diff = abs(item - shift);
            if diff == 0 || diff > 3 {
                if inner_count != 1 {
                    // already used the Problem Dampener to fulfill prev condition
                    return false;
                }
                split_res_new = map(|x| *x, split_res[0..(inner_count2 - 1)].iter().chain(
                    split_res[inner_count2..].iter()
                )).collect::<Vec<i16>>();
                break;
            }
        }

        if len(&split_res_new) == len(&split_res) {
            // speed up the process - no need to check the same list
            return true;
        }
        // And check one last time after the removal
        if !helper(&split_res_new) {
            continue;
        }
        return true;
    }
    false
}


pub fn main_2() -> i16 {  // 398
    // The Problem Dampener added interestingly many edge cases
    let lines = open("day02/input.txt", "r").readlines();
    let mut count = 0;
    'outer: for line_iter in lines {
        let line = pstr::rstrip(&line_iter);
        let mut split_res = map(int::<i16>, line.split(" ")).collect::<Vec<_>>();
        let split_res_orig = split_res.clone();
        let len_split_res = len(&split_res);

        // monotonicity:
        let mut sorted_res = sorted(&split_res, |x| *x).collect::<Vec<i16>>();
        let mut inner_count = 1;
        while
            split_res != sorted_res
            && split_res != sorted_res.clone().into_iter().rev().collect::<Vec<i16>>()
        {
            if inner_count > len_split_res {
                continue 'outer;
            }
            split_res = map(|x| *x, split_res_orig[0..(inner_count - 1)].iter().chain(
                split_res_orig[inner_count..].iter()
            )).collect::<Vec<i16>>();
            sorted_res = sorted(&split_res, |x| *x).collect::<Vec<i16>>();  // sort is stable - does not reorder equal elements
            inner_count += 1;

            while !test2(&split_res, inner_count) {
                // if I did not call test2 and caught duplicates edge case only, I would have to check for records like
                // [74, 75, 71, 68, 66, 64]
                // which should have replaced 2nd item later
                if inner_count > len_split_res {
                    continue 'outer;
                }
                split_res = map(|x| *x, split_res_orig[0..(inner_count - 1)].iter().chain(
                    split_res_orig[inner_count..].iter()
                )).collect::<Vec<i16>>();
                sorted_res = sorted(&split_res, |x| *x).collect::<Vec<i16>>();
                inner_count += 1;
            }
        }

        // second rule: no more than 3 steps between elements
        if len(&split_res) != len_split_res {
            // speed up the process - no need to check the same list
            count += 1;
        } else if test2(&split_res, inner_count) {
            count += 1;
        }
    }

    count
}
