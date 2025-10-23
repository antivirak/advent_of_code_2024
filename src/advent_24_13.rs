#![allow(clippy::unused_unit)]

use crate::*;

// https://adventofcode.com/2024/day/13
// 7901 / 8587

// TODO create decimal module in lib and solve using it


pub fn main_1() -> usize {  // 30413
    let instructions = open("day13/input.txt", "r").readlines();
    let btns_a = map(
        |line| map(
            |item| int(item.split("+").collect::<Vec<&str>>()[1]),
            line.split(": ")
                .collect::<Vec<&str>>()[1]
                .split(", ")
                .collect::<Vec<&str>>()
        ).collect::<Vec<_>>(),
        filter(|x| x.starts_with("Button A"), &instructions)
    ).collect::<Vec<Vec<f64>>>();
    let btns_b = map(
        |line| map(
            |item| int(item.split("+").collect::<Vec<&str>>()[1]),
            line.split(": ")
                .collect::<Vec<&str>>()[1]
                .split(", ")
                .collect::<Vec<&str>>()
        ).collect::<Vec<_>>(),
        filter(|x| x.starts_with("Button B"), &instructions)
    ).collect::<Vec<Vec<f64>>>();
    let prizes = map(
        |line| map(
            |item| int(item.split("=").collect::<Vec<&str>>()[1]),
            line.split(": ")
                .collect::<Vec<&str>>()[1]
                .split(", ")
                .collect::<Vec<&str>>()
        ).collect::<Vec<_>>(),
        filter(|x| x.starts_with("Prize"), &instructions)
    ).collect::<Vec<Vec<f64>>>();

    let mut total = 0.0;
    for (count, prize) in enumerate(&prizes) {
        let a = &btns_a[count];
        let b = &btns_b[count];
        // eq1 = prize[0] = a[0] * c1 + b[0] * c2;
        // eq2 = prize[1] = a[1] * c1 + b[1] * c2;
        // c1 = (prize[0] - b[0] * c2) / a[0];
        // prize[1] = a[1] * (prize[0] - b[0] * c2) / a[0] + b[1] * c2;
        // prize[1] = a[1] / a[0] * prize[0] - a[1] / a[0] * b[0] * c2 + b[1] * c2;
        // prize[1] = a[1] / a[0] * prize[0] - (a[1] / a[0] * b[0] - b[1]) * c2;
        let c2 = (a[1] / a[0] * prize[0] - prize[1]) / (a[1] / a[0] * b[0] - b[1]);
        let c1 = (prize[0] - b[0] * c2) / a[0];
        if c1 < 0.0 || c2 < 0.0 || c1 > 100.0 || c2 > 100.0 {
            continue;
        }

        if (c1.round() - c1).abs() > 0.0001 {
            continue;
        }
        total += c1 * 3.0 + c2;
    }

    total as usize
}

/*
pub fn main_2() -> usize {  // 90_636_188_447_378 too low, 92_974_408_363_862 too high, 92827349540204
    let instructions = open("day13/input.txt", "r").readlines();
    //    map(
    //    |line| pstr::rstrip(&line).clone(),
    //).collect::<Vec<&str>>();
    let btns_a = map(
        |line| map(
            |item| int::<u32>(item.split("+").collect::<Vec<&str>>()[1]) as f128,
            line.split(": ")
                .collect::<Vec<&str>>()[1]
                .split(", ")
                .collect::<Vec<&str>>()
        ).collect::<Vec<_>>(),
        //instructions
        //    .into_iter()
        //    .filter(|x| x.starts_with("Button A"))
        //    .collect::<Vec<&str>>()
        filter(|x| x.starts_with("Button A"), &instructions)
    ).collect::<Vec<Vec<f128>>>();
    let btns_b = map(
        |line| map(
            |item| int::<u32>(item.split("+").collect::<Vec<&str>>()[1]) as f128,
            line.split(": ")
                .collect::<Vec<&str>>()[1]
                .split(", ")
                .collect::<Vec<&str>>()
        ).collect::<Vec<_>>(),
        filter(|x| x.starts_with("Button B"), &instructions)
    ).collect::<Vec<Vec<f128>>>();
    let prizes = map(
        |line| map(
            |item| 10000000000000.0 + int::<u64>(item.split("=").collect::<Vec<&str>>()[1]) as f128,
            line.split(": ")
                .collect::<Vec<&str>>()[1]
                .split(", ")
                .collect::<Vec<&str>>()
        ).collect::<Vec<_>>(),
        filter(|x| x.starts_with("Prize"), &instructions)
    ).collect::<Vec<Vec<f128>>>();
    //let btns_b = map(|line| map(|item| int(item.split("+").collect::<Vec<_>>()[1]), line.split(": ").collect::<Vec<_>>()[1].split(", ").collect::<Vec<_>>()), instructions.iter().filter(|x| x.starts_with("Button B")).collect::<Vec<_>>()).collect::<Vec<Vec<f128>>>();
    //let prizes = map(|line| map(|item| int(item.split("=").collect::<Vec<_>>()[1]), line.split(": ").collect::<Vec<_>>()[1].split(", ").collect::<Vec<_>>()), instructions.iter().filter(|x| x.starts_with("Prize")).collect::<Vec<_>>()).collect::<Vec<Vec<f128>>>();

    let mut total = 0.0;
    for (count, prize) in enumerate(&prizes) {
        let a = &btns_a[count];
        let b = &btns_b[count];
        // eq1 = prize[0] = a[0] * c1 + b[0] * c2;
        // eq2 = prize[1] = a[1] * c1 + b[1] * c2;
        // c1 = (prize[0] - b[0] * c2) / a[0];
        // prize[1] = a[1] * (prize[0] - b[0] * c2) / a[0] + b[1] * c2;
        // prize[1] = a[1] / a[0] * prize[0] - a[1] / a[0] * b[0] * c2 + b[1] * c2;
        // prize[1] = a[1] / a[0] * prize[0] - (a[1] / a[0] * b[0] - b[1]) * c2;
        let c2 = (a[1] * prize[0] / a[0] - prize[1]) / (a[1] * b[0] / a[0] - b[1]);
        //if (c2.round() - c2).abs() > 0.0000000000000000000000000000000000000000000000000000000000001 {
        //    continue;
        //}
        let c1 = (prize[0] - b[0] * c2) / a[0];
        if c1 < 0.0 || c2 < 0.0 {
            continue;
        }

        let c1f64 = c1 as f64;
        //.round(), .fract() etc. does not work on f128 so far
        //if (c1f64.round() - c1f64).abs() > 0.0000000000000000001 {  // has to be between 0.001 and 0.0001 for f64
        //    continue;
        //}
        if c1f64.round() != c1f64 {
            continue;
        }
        print(&c1f64);
        //print(&(c1 as f64));
        //print(&((c1 - c1.fract()) as f64));
        //if prize[0] != a[0] * c1 + b[0] * c2 {
        //    continue;
        //}
        total += c1 * 3.0 + c2;
    }

    total as usize
}
    */


pub fn main_2() -> usize {  // 92827349540204
    let instructions = open("day13/input.txt", "r").readlines();
    let btns_a = map(
        |line| map(
            |item| int::<f64>(item.split("+").collect::<Vec<&str>>()[1]),
            line.split(": ")
                .collect::<Vec<&str>>()[1]
                .split(", ")
                .collect::<Vec<&str>>()
        ).collect::<Vec<_>>(),
        filter(|x| x.starts_with("Button A"), &instructions)
    ).collect::<Vec<Vec<f64>>>();
    let btns_b = map(
        |line| map(
            |item| int::<f64>(item.split("+").collect::<Vec<&str>>()[1]),
            line.split(": ")
                .collect::<Vec<&str>>()[1]
                .split(", ")
                .collect::<Vec<&str>>()
        ).collect::<Vec<_>>(),
        filter(|x| x.starts_with("Button B"), &instructions)
    ).collect::<Vec<Vec<f64>>>();
    let prizes = map(
        |line| map(
            |item| 10000000000000.0 + int::<f64>(item.split("=").collect::<Vec<&str>>()[1]),
            line.split(": ")
                .collect::<Vec<&str>>()[1]
                .split(", ")
                .collect::<Vec<&str>>()
        ).collect::<Vec<_>>(),
        filter(|x| x.starts_with("Prize"), &instructions)
    ).collect::<Vec<Vec<f64>>>();

    let mut total = 0.0;
    for (count, prize) in enumerate(&prizes) {
        let a = &btns_a[count];
        let b = &btns_b[count];

        let det = a[0] * b[1] - a[1] * b[0];
        let numerator1 = prize[0] * b[1] - prize[1] * b[0];
        let numerator2 = prize[1] * a[0] - prize[0] * a[1];
        if numerator1 % det != 0.0 || numerator2 % det != 0.0 {
            continue;
        }
        let c2 = (a[1] * prize[0] / a[0] - prize[1]) / (a[1] * b[0] / a[0] - b[1]);
        let c1 = (prize[0] - b[0] * c2) / a[0];

        total += c1 * 3.0 + c2;
    }

    total as usize
}
