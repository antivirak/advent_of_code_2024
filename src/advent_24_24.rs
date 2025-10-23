#![allow(clippy::unused_unit)]
#![allow(clippy::needless_borrow)]

use crate::*;

// https://adventofcode.com/2024/day/24
// 3108 / 11244


fn ops(x: u64, y: u64, op: &str) -> u64 {
    match op {
        "AND" => x & y,
        "OR" => x | y,
        "XOR" => x ^ y,  // x != y
        _ => panic!("Unknown operation"),
    }
}

pub fn main_1() -> u64 {
    let binding = open("day24/input.txt", "r").read();
    let split = pstr::strip(&binding).split("\n\n").collect::<Vec<_>>();
    let (inits, gates) = (split[0], split[1]);

    let mut init_map = dict();
    for line in inits.split("\n") {
        let split = line.split(": ").collect::<Vec<_>>();
        let (node, init) = (split[0], split[1]);
        init_map.insert(node, int(init));
    }

    let mut graph = dict();
    let mut output_layer = vec![];
    for line in gates.split("\n") {
        let split = line.split(" -> ").collect::<Vec<_>>();
        let (nodes, target) = (split[0], split[1]);
        let split = nodes.split(" ").collect::<Vec<_>>();
        let (left, op, right) = (split[0], split[1], split[2]);
        graph.insert(target, (left, op, right));
        if target.starts_with("z") {
            output_layer.push(target);
        }
    }

    // end of process input
    let mut stack = output_layer.clone();
    while len(&stack) > 0 {
        let node = stack.pop().unwrap();  // safe to unwrap
        let (left, fcn, right) = graph[&node];
        if !init_map.contains_key(left) || !init_map.contains_key(right) {
            stack.push(node);
            if !init_map.contains_key(left) {
                stack.push(left);
            }
            if !init_map.contains_key(right) {
                stack.push(right);
            }
            continue;
        }

        let predec1 = init_map[left];
        let predec2 = init_map[right];
        init_map.insert(node, ops(predec1, predec2, fcn));
    }

    sum(&comp![
        init_map[&z] * pow(2, power) as u64 for (power, z) in enumerate(&output_layer.into_iter().sorted())
    ])
}


pub fn main_2() -> String {
    let binding = open("day24/input.txt", "r").read();
    let split = pstr::strip(&binding).split("\n\n").collect::<Vec<_>>();
    let (inits, gates) = (split[0], split[1]);

    let mut init_map: Dict<&str, u64> = dict();
    for line in inits.split("\n") {
        let split = line.split(": ").collect::<Vec<_>>();
        let (node, init) = (split[0], split[1]);
        init_map.insert(node, int(init));
    }

    let mut graph = dict();
    let mut nodes = set();
    let mut select = set();
    for line in gates.split("\n") {
        let split = line.split(" -> ").collect::<Vec<_>>();
        let (nodes_, target) = (split[0], split[1]);
        let split = nodes_.split(" ").collect::<Vec<_>>();
        let (left, op, right) = (split[0], split[1], split[2]);
        graph.insert(target, (left, op, right));
        nodes.add((left, op));
        nodes.add((right, op));
    }

    for (key, (predec1, op, predec2)) in graph.items() {
        if op == &"AND" {
            if predec1 != &"x00" && predec2 != &"x00" && !nodes.contains(&(key, "OR")) {
                select.add(key);
            }
        }

        else if op == &"OR" {
            if key.starts_with("z") && key != &"z45" {
                select.add(&key);
            }
            if nodes.contains(&(key, "OR")) {
                select.add(&key);
            }
        }

        else if op == &"XOR" {
            if predec1.starts_with("x") || predec2.starts_with("x") {
                if predec1 != &"x00" && predec2 != &"x00" && !nodes.contains(&(key, "XOR")) {
                    select.add(&key);
                }
            }
            else if !key.starts_with("z") {
                select.add(&key);
            }
        } else {
            panic!("Unknown operation");
        }
    }

    //','.join(sorted(&select, |x| x))
    select.iter().sorted().join(",")
}

/*
use rayon::prelude::*;
pub fn main_2() -> String {  // bruteforce try
    let binding = open("day24/input.txt", "r").read();  // make borowchecker happy
    let input = pstr::rstrip(&binding).split("\n\n").collect::<Vec<_>>();
    let inits = input[0];
    let gates = input[1];

    let mut init_map = dict();
    for line in inits.split("\n") {
        let line_split = line.split(": ").collect::<Vec<_>>();
        let node = line_split[0];
        let init = line_split[1];
        init_map.insert(node, int(init));
    }

    let mut graph = dict();
    let mut output_layer = vec![];
    let mut x = vec![];
    let mut y = vec![];
    for line in gates.split("\n") {
        let line_split = line.split(" -> ").collect::<Vec<_>>();
        let nodes = line_split[0];
        let target = line_split[1];
        let nodes_split = nodes.split(" ").collect::<Vec<_>>();
        let left = nodes_split[0];
        let op = nodes_split[1];
        let right = nodes_split[2];
        graph.insert(target, (left, op, right));
        if target.starts_with("z") {
            output_layer.push(target);
        }
        if left.starts_with("x") {
            x.push(left);
        }
        if right.starts_with("x") {
            x.push(right);
        }
        if left.starts_with("y") {
            y.push(left);
        }
        if right.starts_with("y") {
            y.push(right);
        }
    }

    let x_sorted = x.clone().into_iter().sorted();
    //for idx in 64..90 {
    //    init_map.insert(x[idx], 0);
    //}
    let x_num = sum(&comp![{ init_map[power_z.1] * 2i128.pow(power_z.0 as u32) } for power_z in enumerate(&x_sorted)]);
    let y_num = sum(&comp![{ init_map[z] * 2i128.pow(power as u32) } for (power, z) in enumerate(&y.into_iter().sorted())]);
    //let x_num = enumerate(&x.clone().into_iter().sorted()).into_iter().map(|(power, z)| {print(&power)});
    //let x_num = enumerate(&x.into_iter().sorted()).into_iter().map(|power_z| init_map[power_z.1] * u64::pow(2, power_z.0 as u32)).sum::<u64>();
    //let y_num = enumerate(&y.into_iter().sorted()).into_iter().map(|(power, z)| init_map[z] * 2u64.pow(power as u32)).sum::<u64>();
    let expected = x_num + y_num;
    print(&expected);
    let graph_orig = graph.clone();
    let init_map_orig = init_map.clone();
    // end of process input
    // ~1.5 E 12 iterations
    // TODO does not finish
    // TODO no need to go through all pairs - only the already connected ones
    // that is actually why the stack grows infinitelly for the unreachable combs
    itertools::combinations(&itertools::combinations(&graph_orig.keys(), 2), 4).enumerate().par_bridge().for_each(|(c, select)| {  // enumerate() to use .progress() ?
        if c % 100000 == 0 {
            print(&c);
        }
        let mut init_map = init_map_orig.clone();
        let mut graph = graph_orig.clone();

        let mut intermed = graph[select[0][0]];
        graph.insert(select[0][0], graph[select[0][1]]);
        graph.insert(select[0][1], intermed);
        intermed = graph[select[1][0]];
        graph.insert(select[1][0], graph[select[1][1]]);
        graph.insert(select[1][1], intermed);

        intermed = graph[select[2][0]];
        graph.insert(select[2][0], graph[select[2][1]]);
        graph.insert(select[2][1], intermed);
        intermed = graph[select[3][0]];
        graph.insert(select[3][0], graph[select[3][1]]);
        graph.insert(select[3][1], intermed);

        let mut stack = output_layer.clone();
        while let Some(node) = stack.pop() {
            let (left, op, right) = graph[node];
            if !init_map.contains_key(left) || !init_map.contains_key(right) {
                stack.push(node);
                if !init_map.contains_key(left) {
                    stack.push(left);
                }
                if !init_map.contains_key(right) {
                    stack.push(right);
                }
                //if len(&stack) >= 500 {
                //    return;
                //}
                continue;
            }

            let predec1 = init_map[left] == 1;
            let predec2 = init_map[right] == 1;
            let res = match op {
                "AND" => if predec1 && predec2 { 1 } else { 0 },
                "OR" => if predec1 || predec2 { 1 } else { 0 },
                "XOR" => if predec1 ^ predec2 { 1 } else { 0 },  // or != instead of ^
                _ => unreachable!(),
            };
            init_map.insert(node, res);
        }

        //if enumerate(&output_layer.clone().into_iter().sorted()).into_iter().map(|(power, z)| init_map[z] * 2u64.pow(power as u32)).sum::<u64>() == expected {
        if sum(&comp![{ init_map[z] * 2i128.pow(power as u32) } for (power, z) in enumerate(&output_layer.clone().into_iter().sorted())]) == expected {
            //return select.into_iter().flatten().sorted().join(",");
            print(&select.into_iter().flatten().sorted().join(","));
            panic!();
        }
    });

    "".to_string()  // should not happen
}
*/
