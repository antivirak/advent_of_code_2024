#![allow(clippy::unused_unit)]

use std::sync::mpsc;

use rayon::Scope;

use crate::*;

// https://adventofcode.com/2024/day/11
// 8102 / 62322


fn process_item(
    rx: mpsc::Receiver<String>,
    result_tx: mpsc::Sender<String>,
    level: u8,
    processing_thread: &Scope,
) {
    while let Ok(stone_string) = rx.recv() {
        let stone = &stone_string;
        let mut to_push = vec![];
        if stone == "0" {
            to_push.push("1".to_string());
        } else if len(&stone.chars().collect::<Vec<_>>()) % 2 == 0 {
            let stone_chars = stone.chars().collect::<Vec<_>>();
            let len_stone = len(&stone_chars);
            to_push.push(format!("{}", int::<u32>(&stone_chars[..len_stone / 2  ].iter().join(""))));
            to_push.push(format!("{}", int::<u32>(&stone_chars[(len_stone / 2)..].iter().join(""))));
        } else {
            to_push.push(format!("{}", int::<u64>(stone) * 2024));
        }

        // Recursively spawn a new processing thread
        if level < 24 {  // blink - 1
            let (new_tx, new_rx) = mpsc::channel::<String>();
            let result_tx_clone = result_tx.clone(); // Clone the transmitter

            processing_thread.spawn(move |child| {
                process_item(new_rx, result_tx_clone, level + 1, child);
            });

            // Send the processed item to the new thread
            for item in to_push {
                new_tx.send(item).expect("Failed to send item to new thread");
            }
        } else {
            // Send the processed item to the result channel
            for item in to_push {
                result_tx.send(item).expect("Failed to send processed item");
            }
        }
    }
}


pub fn main_1() -> usize {  // 190865
    // Something like FFT algo. Flexing with parallel processing without needing one for part1
    // producer-consumer pattern
    let stones = map(
        |x| x.to_string(),
        pstr::rstrip(&open("day11/input.txt", "r").read()).split(" "),
    ).collect::<Vec<_>>();

    let (tx, rx) = mpsc::channel::<String>();
    let (result_tx, result_rx) = mpsc::channel();
    // Spawn the first processing thread
    rayon::scope(|processing_thread| {
        processing_thread.spawn(move |child| {
            process_item(rx, result_tx, 1, child);
        });

        for stone in &stones {
            if stone == "0" {
                tx.send("1".to_string()).expect("Failed to send item");
            } else if len(&stone.chars().collect::<Vec<_>>()) % 2 == 0 {
                let len_stone = len(&stone.chars().collect::<Vec<_>>());
                let to_push = format!("{}", int::<u32>(&stone.chars().collect::<Vec<_>>()[..len_stone / 2  ].iter().join("")));
                tx.send(to_push.clone()).expect("Failed to send item");
                let to_push = format!("{}", int::<u32>(&stone.chars().collect::<Vec<_>>()[(len_stone / 2)..].iter().join("")));
                tx.send(to_push.clone()).expect("Failed to send item");
            } else {
                // actual math
                let to_push = format!("{}", int::<u64>(stone) * 2024);
                tx.send(to_push.clone()).expect("Failed to send item");
            }
        }
        // Close the channel by dropping the sender
        drop(tx);
    });

    let mut count = 0;
    while result_rx.recv().is_ok() {
        count += 1;
    }

    count
}


/// traverse the tree
#[allow(dead_code)]
fn step(tree: Dict<String, Vec<String>>, mut nodes: Vec<String>, key: &String) -> Vec<String> {
    if nodes.contains(key) {
        // skip node which we already processed
        //print(&key);
        //print(&"quit");
        return nodes;
    }
    nodes.push(key.clone());

    let children = tree.get(key).unwrap();
    for child in children {
        nodes = step(tree.clone(), nodes, child);
    }

    nodes
}


fn process_stone_recursive(stone: &str, blink: usize, cache: &mut Dict<(String, usize), u64>) -> u64 {
    if blink == 0 {
        return 1;  // end recursion
    }
    if let Some(entry) = cache.get(&(stone.to_string(), blink)) {
        return *entry;
    }

    let mut res = vec![];
    if stone == "0" {
        res.push("1".to_string());
    } else if len(&stone.chars().collect::<Vec<_>>()) % 2 == 0 {
        let len_stone = len(&stone.chars().collect::<Vec<_>>());
        let to_push = format!("{}", int::<u32>(&stone.chars().collect::<Vec<_>>()[..len_stone / 2  ].iter().join("")));
        res.push(to_push);
        let to_push = format!("{}", int::<u32>(&stone.chars().collect::<Vec<_>>()[(len_stone / 2)..].iter().join("")));
        res.push(to_push);
    } else {
        // actual math
        let to_push = format!("{}", int::<u64>(stone) * 2024);
        res.push(to_push);
    }
    let mut total = 0u64;
    for new_stone in &res {
        total += process_stone_recursive(new_stone, blink - 1, cache);
    }

    cache.insert((stone.to_string(), blink), total);

    total
}


pub fn main_2() -> u64 {  // 225404711855335
    let stones = map(
        |x| x.to_string(),
        pstr::rstrip(&open("day11/input.txt", "r").read(),
    ).split(" ")).collect::<Vec<_>>();
    let blink = 75;

    let mut cache = dict();
    let mut total = 0u64;
    // TODO caching decorator
    for stone in &stones {
        total += process_stone_recursive(stone, blink, &mut cache);
    }

    total
}

/*
pub fn main_2() -> usize {  // bruteforcing - it is also valid single threaded part1 solution
    let mut stones = map(
        |x| x.to_string(),
        pstr::rstrip(&open("day11/input.txt", "r").read(),
    ).split(" ")).collect::<Vec<_>>();
    let blink = 25;

    for count in 0..blink {
        let mut res = vec![];
        for stone in &stones {
            if stone == "0" {
                res.push("1".to_string());
            } else if len(&stone.chars().collect::<Vec<_>>()) % 2 == 0 {
                let len_stone = len(&stone.chars().collect::<Vec<_>>());
                let to_push = format!("{}", int::<u32>(&stone.chars().collect::<Vec<_>>()[..len_stone / 2  ].iter().join("")));
                res.push(to_push);
                let to_push = format!("{}", int::<u32>(&stone.chars().collect::<Vec<_>>()[(len_stone / 2)..].iter().join("")));
                res.push(to_push);
            } else {
                // actual math
                let to_push = format!("{}", int::<u64>(stone) * 2024);
                res.push(to_push);
            }
        }
        stones = res.clone();
        print(&count);
    }

    len(&stones)
}
*/
