#![allow(clippy::unused_unit)]

use crate::*;

// https://adventofcode.com/2024/day/1
// 17880 / 18831


fn take_input() -> (Vec<i32>, Vec<i32>) {
    let lines = {
        open("day01/input.txt", "r").readlines()
    };
    let lines_len = lines.len();
    let mut vec1 = vec![0; lines_len];
    let mut vec2 = vec![0; lines_len];
    for line_iter in lines {
        let line = pstr::rstrip(&line_iter);
        let split_res = line.split("   ").collect::<Vec<_>>();
        // what about storing split_res and then zip?
        let prefix = int(split_res[0]);
        let end = int(split_res[1]);
        vec1.push(prefix);
        vec2.push(end);
    }

    (vec1, vec2)
}


pub fn main_1() -> i32 {  // 2166959
    let (vec1, vec2) = take_input();

    sum(&np::vec_abs(np::substract(vec1.into_iter().sorted().collect(), vec2.into_iter().sorted().collect())))
}


pub fn main_2() -> i32 {  // 23741109
    let (vec1, vec2) = take_input();

    let mut count_map = Dict::from_iter(map(|x| (x, 0), vec1.clone()));
    let vec2_iter = vec2.into_iter();
    for unique in Set::from_iter(vec1.clone()) {
        count_map.entry(unique).and_modify(|current| *current += vec2_iter.clone().filter(|x| *x == unique).collect::<Vec<_>>().len());
    }

    let mut res = 0;
    for key in vec1 {
        res += key * *count_map.get(&key).unwrap() as i32;
    }
    res
}
