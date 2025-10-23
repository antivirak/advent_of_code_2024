#![allow(clippy::unused_unit)]

use regex::Regex;

use crate::*;

// https://adventofcode.com/2024/day/3
// 19234 / 18786


pub fn main_1() -> i32 {  // 183_380_722
    let lines = open("day03/input.txt", "r").readlines();
    let mut res = 0;
    let re = Regex::new(r"mul\(([0-9]*),([0-9]*)\)").unwrap();
    for line_iter in lines {
        let line = pstr::rstrip(&line_iter);
        res += re.captures_iter(line).map(|caps| caps.extract::<2>().1)
            .map(|tuple| np::prod(tuple.map(int::<i32>).into_iter())).sum::<i32>();//.collect::<Vec<_>>();
    }

    res
}


pub fn main_2() -> i32 {  // 82733683
    let lines = open("day03/input.txt", "r").readlines();
    let mut res = 0;
    let re = Regex::new(r"mul\(([0-9]*),([0-9]*)\)").unwrap();
    let redo = Regex::new(r"do\(\)").unwrap();
    let redont = Regex::new(r"don't\(\)").unwrap();
    let mut do_ = true;
    for line_iter in lines {
        for item in map(|item| item.to_owned() + ")", pstr::rstrip(&line_iter).split(")")) {
            if !do_ && redo.is_match(&item) {
                do_ = true;
                continue;
            }
            if redont.is_match(&item) {
                do_ = false;
                continue;
            }
            if do_ {
                match re.captures(&item) {
                    None => (),
                    Some(caps) => {
                        res += np::prod(caps.extract::<2>().1.into_iter()
                            .map(int::<i32>));
                    }
                }
            }
        };
    }

    res
}
