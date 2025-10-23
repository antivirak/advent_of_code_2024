#![allow(clippy::unused_unit)]

use crate::*;
use indicatif::ProgressIterator;

// https://adventofcode.com/2024/day/23
// 5716 / 6002
// This one was for Bron–Kerbosch algorithm, but my naive approach was good enough


pub fn main_1() -> u64{  // 1411
    let input = map(
        |x| {
            let intermed = pstr::rstrip(&x).split("-").collect::<Vec<_>>();
            (intermed[0].to_string(), intermed[1].to_string())
        },
        open("day23/input.txt", "r").readlines(),
    ).collect::<Vec<_>>();

    let mut sets: Vec<Set<String>> = vec![];
    for pair in &input {
        let mut new = set();
        new.insert(pair.0.clone());
        new.insert(pair.1.clone());
        sets.push(new);
    }

    let mut sets_to_res = vec!();
    for pair in input.iter().progress() {
        for curr_set in &sets {
            let mut new = curr_set.clone();
            let mut set_without_p0 = curr_set.clone();
            set_without_p0.remove(&pair.0);
            let the_other_p0 = set_without_p0.drain().collect::<Vec<_>>()[0].clone();
            let mut set_without_p1 = curr_set.clone();
            set_without_p1.remove(&pair.1);
            let the_other_p1 = set_without_p1.drain().collect::<Vec<_>>()[0].clone();
            #[allow(unused_parens)]
            if (
                (curr_set.contains(&pair.0) && (
                    input.contains(&(the_other_p0.clone(), pair.1.clone())) || input.contains(&(pair.1.clone(), the_other_p0))
                )) || (curr_set.contains(&pair.1) && (
                    input.contains(&(pair.0.clone(), the_other_p1.clone())) || input.contains(&(the_other_p1, pair.0.clone()))
                ))
            ) {
                new.insert(pair.0.clone());
                new.insert(pair.1.clone());
            }
            if !sets_to_res.contains(&new) {
                sets_to_res.push(new);
            }
        }
    }

    let mut res = 0;
    'outer: for set in sets_to_res {
        if len(&set) != 3 {
            continue;
        }
        for item in set {
            if item.chars().collect::<Vec<_>>()[0] == 't' {  // item.starts_with("t")
                res += 1;
                continue 'outer;
            }
        }
    }

    res
}


pub fn main_2() -> usize {  // aq,bn,ch,dt,gu,ow,pk,qy,tv,us,yx,zg,zu
    let input = map(
        |x| {
            let intermed = pstr::rstrip(&x).split("-").collect::<Vec<_>>();
            (intermed[0].to_string(), intermed[1].to_string())
        },
        open("day23/input.txt", "r").readlines(),
    ).collect::<Set<_>>();

    let mut map = dict();
    for pair in &input {
        if map.contains_key(&pair.0) {
            *map.get_mut(&pair.0).unwrap() += 1;
        } else {
            map.insert(pair.0.clone(), 0);
        }
        if map.contains_key(&pair.1) {
            *map.get_mut(&pair.1).unwrap() += 1;
        } else {
            map.insert(pair.1.clone(), 0);
        }
    }

    // Each node has 13 (4 for test) connections; assume that the answer will use all of them but one
    for res_key in map.keys() {
        let mut connections = set();
        for pair in &input {
            if pair.0 == *res_key {
                connections.insert(pair.1.clone());
            } else if pair.1 == *res_key {
                connections.insert(pair.0.clone());
            }
        }
    
        let len_con = len(&connections);
        let mut connections_full = connections.clone();
        connections_full.insert(res_key.clone());
        'outer: for con_rm in &connections {
            let mut connections_full_rm = connections_full.clone();
            connections_full_rm.remove(con_rm);
            let mut connections_rm = connections.clone();
            connections_rm.remove(con_rm);
            let mut res = set();
            for con in &connections_rm {
                let mut connections_inner = set();
                connections_inner.insert(con.clone());
                for pair in &input {
                    if pair.0 == *con && pair.1 != *con_rm && connections_full_rm.contains(&pair.1) {
                        connections_inner.insert(pair.1.clone());
                    } else if pair.1 == *con && pair.0 != *con_rm && connections_full_rm.contains(&pair.0) {
                        connections_inner.insert(pair.0.clone());
                    }
                }
                if connections_full_rm != connections_inner {
                    continue 'outer;
                }
                res = connections_inner;
            }
            println!("{res:?}");  // sorted manually
            return len_con;
        }
    }

    42  // should not happen
}
