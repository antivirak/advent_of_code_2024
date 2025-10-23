#![allow(clippy::unused_unit)]

use dashmap::DashMap;
use rayon::prelude::*;
//use indicatif::ProgressIterator;
use indicatif::ParallelProgressIterator;

use crate::*;

// https://adventofcode.com/2024/day/22
// 2823 / 5685


pub fn main_1() -> u64{  // 15006633487
    let input = map(
        |x| int::<u64>(&x),
        open("day22/input.txt", "r").readlines(),
    ).collect::<Vec<_>>();

    let mut res = 0;
    for mut line in input {
        for _ in 0..2000 {
            let mut intermed = line * 64;
            line = (intermed ^ line) % 16777216;
            intermed = line / 32;
            line = (intermed ^ line) % 16777216;
            intermed = line * 2048;
            line = (intermed ^ line) % 16777216;
        }
        res += line;
    }

    res
}


fn prepare_kmp<T, FEq>(x: &[T], next: &mut [usize], equal: &mut FEq)
    where FEq: FnMut(&T, &T) -> bool
{
    let mut i = 0;
    let mut j = !0;
    next[0] = !0;
    while i < x.len() {
        while let Some(&next_j) = next.get(j) {
            if equal(&x[i], &x[j]) {
                break;
            }
            j = next_j;
        }
        i += 1;
        j = j.wrapping_add(1);
        if i != x.len() && equal(&x[i], &x[j]) {
            next[i] = next[j];
        } else {
            next[i] = j;
        }
    }
}

const STACK_NEXT_SIZE: usize = 32;

/// Search for the first occurence of `pattern` as a substring of `text`,
/// if any. Return the start of the substring as an offset from the start of
/// the text inside a `Some`. If the pattern is not found, return `None`.
pub fn knuth_morris_pratt<T>(text: &[T], pattern: &[T]) -> Option<usize>
    where T: Eq
{
    knuth_morris_pratt_by(text, pattern, PartialEq::eq)
}

/// Search for the first occurence of `pattern` as a substring of `text`,
/// if any. Return the start of the substring as an offset from the start of
/// the text inside a `Some`. If the pattern is not found, return `None`.
///
/// Use the function `equal` for equality comparison.
pub fn knuth_morris_pratt_by<T, FEq>(text: &[T], pattern: &[T], mut equal: FEq)
    -> Option<usize>
    where FEq: FnMut(&T, &T) -> bool
{
    // empty pattern is a trivial match
    if pattern.is_empty() {
        return Some(0);
    } else if pattern.len() > text.len() {
        return None;
    }

    // use the stack for short patterns
    let mut next_vec;
    let mut next_stack = [0; STACK_NEXT_SIZE];
    let next;
    if pattern.len() >= STACK_NEXT_SIZE {
        next_vec = vec![0; pattern.len() + 1];
        next = &mut next_vec[..];
    } else {
        next = &mut next_stack[..];
    }
    prepare_kmp(pattern, next, &mut equal);
    
    let mut i = 0;
    let mut j = 0;
    while j < text.len() {
        while let Some(&next_i) = next.get(i) {
            if equal(&pattern[i], &text[j]) {
                break;
            }
            i = next_i;
        }
        i = i.wrapping_add(1);
        j += 1;
        if i >= pattern.len() {
            return Some(j - i);
        }
    }
    None
}


pub fn main_2() -> usize {  // 1710
    let input = map(
        |x| int::<usize>(&x),
        open("day22/input.txt", "r").readlines(),
    ).collect::<Vec<_>>();

    let len_input = input.len();
    let mut table = np::zeros((len_input, 2001), (0, 0, 0));
    // let mut cache = dict();
    let cache = DashMap::new();
    for (idx, mut line) in enumerate(&input) {
        // let one_pos = int::<i64>(&format!("{line}").chars().rev().collect::<Vec<_>>()[0].to_string());
        let one_pos = line % 10;
        table[idx][0] = (line, one_pos, 0);
        let mut last = one_pos as i64;
        for count in 1..2001 {
            let mut intermed = line * 64;
            line = (intermed ^ line) % 16777216;
            intermed = line / 32;
            line = (intermed ^ line) % 16777216;
            intermed = line * 2048;
            line = (intermed ^ line) % 16777216;
            let one_pos = line % 10;
            table[idx][count] = (line, one_pos, one_pos as i64 - last);
            last = one_pos as i64;
        }
        for (count, row) in enumerate(&table[idx][1..1997].iter()) {
            cache.insert((row.2, table[idx][count + 2].2, table[idx][count + 3].2, table[idx][count + 4].2), 0);
        }
    }

    table.par_iter().progress().for_each(|line| {
        for needle_ in cache.clone().iter() {
            // let needle = comprehension![{ x } for x in needle_];
            let key = needle_.key();
            let needle = vec![key.0, key.1, key.2, key.3];
            // naive:
            // for (position, window) in enumerate(&line[1..].windows(4)) {
            //     if map(|x| x.2, window).collect::<Vec<_>>() == needle {
            //         *cache.get_mut(needle_).unwrap() += line[position + 4].1;
            //         break;
            //     }
            // }
            if let Some(position) = knuth_morris_pratt(&line[1..].iter().map(|x| x.2).collect::<Vec<_>>(), &needle) {
                *cache.get_mut(key).unwrap() += line[position + 4].1;
            }
        }
    });

    *cache.into_read_only().values().max().unwrap()
}
