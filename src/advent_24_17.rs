#![allow(clippy::unused_unit)]

//use std::simd::{Mask, Simd, cmp::SimdPartialEq};  // nightly feature - vectorization
use rayon::prelude::*;

use crate::*;

// https://adventofcode.com/2024/day/17
// 8230 / 22741

fn inst_set(
    option: &str, y_: &str, registers: &mut Dict<&str, u64>, count: usize,
) -> (usize, Option<u64>) {
    let y = match y_ {
        "0" | "1" | "2" | "3" => int::<u64>(y_),
        "4" => *registers.get(&"a").unwrap(),
        "5" => *registers.get(&"b").unwrap(),
        "6" => *registers.get(&"c").unwrap(),
        _ => panic!(),
    };
    // name: (increment, out)
    match option {
        "0" => {
            let base: u64 = 2;
            registers.insert("a", registers.get(&"a").unwrap() / base.checked_pow(y.try_into().unwrap()).unwrap());
            (count + 2, None)
        },  // "adv"
        "1" => {
            let xor = registers.get(&"b").unwrap() ^ int::<u64>(y_);
            registers.insert("b", xor);
            (count + 2, None)
        },  // "bxl"
        "2" => {
            registers.insert("b", y % 8);
            (count + 2, None)
        },  // "bst"
        "4" => {
            let xor = registers.get(&"b").unwrap() ^ registers.get(&"c").unwrap();
            registers.insert("b", xor);
            (count + 2, None)
        },  // "bxc"
        "5" => (count + 2, Some(y % 8)),  // "out"
        "6" => {
            let base: u64 = 2;
            registers.insert("b", registers.get(&"a").unwrap() / base.checked_pow(y.try_into().unwrap()).unwrap());
            (count + 2, None)
        },  // "bdv"
        "7" => {
            let base: u64 = 2;
            registers.insert("c", registers.get(&"a").unwrap() / base.checked_pow(y.try_into().unwrap()).unwrap());
            (count + 2, None)
        },  // "cdv"
        "3" => {  // "jnz"
            if *registers.get(&"a").unwrap() == 0 {
                (count + 2, None)  // do nothing
            } else {
                (int(y_), None)
            }
        },
        _ => { print(&option); panic!() },
    }
}


fn get_output(instructions: &Vec<&str>, registers: &mut Dict<&str, u64>) -> String {
    let mut count = 0;
    let mut res: Vec<u64> = vec![];
    let len_instruct = len(&instructions);
    loop {
        if len_instruct <= count {
            break;
        }
        let maybe_freq;
        if len_instruct <= count + 1 {
            break;
        }
        (count, maybe_freq) = inst_set(
            instructions[count], instructions[count + 1], registers, count,
        );
        if let Some(freq) = maybe_freq {
            res.push(freq);
        }
    }

    res.into_iter().join(",")
}


pub fn main_1() -> String {  // 3,4,3,1,7,6,5,6,0
    let binding = open("day17/input.txt", "r").read();
    let file_in = binding.split("\n\n").collect::<Vec<_>>();

    let mut registers = dict();
    // Range<&str> cannot be iterated directly
    let intermed = ('a'..='z').map(|x| x.to_string()).collect::<Vec<_>>();
    let letters = intermed.iter().map(|x| x.as_str()).collect::<Vec<_>>();

    let instructions = pstr::rstrip(file_in[1].split(": ").collect::<Vec<_>>()[1]).split(",").collect::<Vec<_>>();
    for (count, line) in enumerate(&file_in[0].lines()) {
        let val = int::<u64>(line.split(": ").collect::<Vec<_>>()[1]);
        registers.insert(letters[count], val);
    }

    // end of load_input

    get_output(&instructions, &mut registers)
}


pub fn main_2() -> u64 {  // 109_019_930_331_546
    let binding = open("day17/input.txt", "r").read();
    let file_in = binding.split("\n\n").collect::<Vec<_>>();
    let instructions = pstr::rstrip(file_in[1].split(": ").collect::<Vec<_>>()[1]).split(",").collect::<Vec<_>>();

    let match_str = instructions.join(",");
    let mut pos = match_str.len() - 1;

    let mut ranges = vec![(0, 8)];
    let mut registers = dict();
    registers.insert("b", 0);
    registers.insert("c", 0);
    loop {
        let mut next = Vec::new();
        let suffix = &match_str[pos..];
        for &(low, high) in &ranges {
            for idx in low..high {
                registers.insert("a", idx);
                let output = get_output(&instructions, &mut registers);
                if pos == 0 && output == match_str {
                    return idx;
                }
                if output.ends_with(suffix) {
                    next.push((idx * 8, (idx + 1) * 8 - 1));
                }
            }
        }
        ranges = next;
        pos -= 2;
    }
}

pub fn main_2_bruteforce() -> u64 {
    // impossible to bruteforce using this inefficient algo.
    let binding = open("day17/input.txt", "r").read();
    let file_in = binding.split("\n\n").collect::<Vec<_>>();
    let mut registers = dict();
    let letters = [
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m",
        "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
    ];
    let instructions = pstr::rstrip(file_in[1].split(": ").collect::<Vec<_>>()[1]).split(",").collect::<Vec<_>>();
    for (count, line) in enumerate(&file_in[0].lines()) {
        let val = int::<u64>(line.split(": ").collect::<Vec<_>>()[1]);
        registers.insert(letters[count], val);
    }

    // end of load_input
    let registers_copy = registers.clone();
    let instructions_int = map(int::<u64>, instructions.clone()).collect::<Vec<_>>();
    let len_instruct = len(&instructions);

    // find the order of magnitude, when the input has correct length
    // assume the a_orig is way lower than the target
    let mut a_orig = *registers.get(&"a").unwrap() / 8;
    let mut out = vec![];
    while len(&out) < len_instruct - 1 {
        out = vec![];
        let mut count = 0;
        a_orig *= 8;
        registers.insert("a", a_orig);
        loop {
            if len_instruct <= count + 1 {
                break;
            }
            let maybe_freq;
            (count, maybe_freq) = inst_set(
                instructions[count], instructions[count + 1], &mut registers, count,
            );
            if let Some(freq) = maybe_freq {
                out.push(freq);
            }
        }
    }

    print(&(a_orig * 8));
    let par_len = 7 * a_orig / 12;
    (0..12).into_par_iter().for_each(|i| {
        let mut registers = registers_copy.clone();
        let start = a_orig + i * par_len;
        let mut increment = start;
        loop {
            registers.insert("a", increment);
            let mut count = 0;
            let mut res: Vec<u64> = vec![];
            loop {
                if len_instruct <= count + 1 {
                    break;
                }
                let maybe_freq;
                (count, maybe_freq) = inst_set(
                    instructions[count], instructions[count + 1], &mut registers, count,
                );
                if let Some(freq) = maybe_freq {
                    res.push(freq);
                    let len_res = len(&res);
                    if res[len_res - 1] != instructions_int[len_res - 1] {
                        break;
                    }
                    if len_res == len_instruct {
                        print(registers.get(&"a").unwrap());
                        panic!();
                    }
                }
            }

            increment += 1;
            if increment % 100_000_000 == 0 {
                print(&((increment - start) as f32 / par_len as f32));
            }
        }
    });
    0
}
