#![allow(clippy::unused_unit)]

use crate::*;
use functools::LruCache;
use std::rc::Rc;

// https://adventofcode.com/2024/day/19
// 21968 / 19261

fn can_split(_can_split: &dyn Fn((Rc<str>, Rc<str>)) -> bool, input: (Rc<str>, Rc<str>)) -> bool {
    let (s, substrings) = input;
    if s.is_empty() {
        return true;
    }

    let s_len = s.chars().count();
    for idx in range(s_len + 1) {
        if substrings.split(", ").contains(&&s[0..idx]) && _can_split((s[idx..].into(), substrings.clone())) {
            return true;
        }
    }

    false
}

fn count_split(_count_split: &dyn Fn((Rc<str>, Rc<str>)) -> u64, input: (Rc<str>, Rc<str>)) -> u64 {
    let (s, substrings) = input;
    if s.is_empty() {
        return 1;
    }

    let s_len = s.chars().count();
    let mut res = 0;
    for idx in range(s_len + 1) {
        if substrings.split(", ").contains(&&s[0..idx]) {
            res += _count_split(((s[idx..]).into(), substrings.clone()))
        }
    }

    res
}


fn load_input() -> (Rc<str>, Vec<Rc<str>>) {
    let whole_input = open("day19/input.txt", "r").read();
    
    // Leak the string to make it 'static to avoid lifetime issues
    let whole_input_static: &'static str = Box::leak(whole_input.into_boxed_str());
    let input_split = whole_input_static.split("\n\n").collect::<Vec<_>>();
    let patterns = input_split[1].split("\n").collect::<Vec<_>>();
    (Rc::from(input_split[0]), map(|x| Rc::from(x.to_owned()), patterns[0..len(&patterns) - 1].iter()).collect::<Vec<_>>())
}


pub fn main_1() -> usize {
    let (towels, patterns) = load_input();

    let mut possible = 0;
    let cache = LruCache::new_recursive(Box::new(can_split));
    for pattern in patterns {
        if cache.call((pattern, towels.clone())) {
            possible += 1;
        }
    }

    possible
}


pub fn main_2() -> u64 {
    let (towels, patterns) = load_input();

    let mut res = 0;
    let cache = LruCache::new_recursive(Box::new(count_split));
    for pattern in patterns {
        res += cache.call((pattern, towels.clone()));
    }

    res
}
