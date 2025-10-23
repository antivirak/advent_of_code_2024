#![allow(clippy::unused_unit)]

use crate::*;

// https://adventofcode.com/2024/day/9
// 8611 / 9817


pub fn main_1() -> usize {  // 6378826667552
    let mut line = pstr::rstrip(
        &open("day09/input.txt", "r").read()
    ).chars().map(|c| int::<u8>(&c.to_string())).collect::<Vec<_>>();
    let len_line = len(&line);
    let mut q = vec![];
    if len_line % 2 == 0 { panic!(); }

    let mut back = 0;
    let mut max_id = len_line / 2 + 1;
    let mut total = 0;
    'outer: for (count, rot) in enumerate(&line.clone()) {
        if count % 2 == 0 {
            if total > len_line - count {
                for _ in 0..back {
                    q.push(count / 2);
                }
                break 'outer;
            }
            for _ in 0..rot {
                q.push(count / 2);
            }
        } else {
            for _ in 0..rot {
                while back == 0 {
                    back = line.pop().unwrap();
                    line.pop().unwrap();
                    max_id -= 1;
                    total += 2;
                }
                q.push(max_id);
                back -= 1;
            }
        }
    }

    enumerate(&q).map(|(i, x)| x * i).sum()
}


pub fn main_2() -> usize {  // 6_413_328_569_890
    // could use heap::BinaryHeap
    // could use double-linked list, but queue is ok
    // the algo is actually quite slow given it is in Rust - space for optimization
    // inserting into the middle of the queue by rotating left and right - because that is the way in python
    let mut line = pstr::rstrip(
        &open("day09/input.txt", "r").read()
    ).chars().map(|c| int::<i32>(&c.to_string())).collect::<Vec<_>>();
    let len_line = len(&line);
    let mut q = collections::deque(vec![]);
    if len_line % 2 == 0 { panic!(); }  // assert odd size

    for (count, rot) in enumerate(&line.clone()) {
        // replace space sizes with negative ints
        if rot == 0 {
            // zero-sized space
            continue;
        }
        if count % 2 == 0 {
            line[count] = rot;
            for _ in 0..rot {
                q.push_back((count / 2) as i32);
            }
        } else {
            q.push_back(-rot);
            line[count] = -rot;
        }
    }
    if len(&q) < 50 {
        // print parsed input in case of testing
        println!("{q:?}");
    }

    let mut max_id = (len_line / 2) as i32;  // kep track of file id
    let mut count = 0;
    for current in line.into_iter().rev() {
        // iterate line in reverse - defragment rightmost files first
        if current <= 0 {
            continue;
        }
        if let Some(hole_size) = q.clone().into_iter().find(|x| -*x >= current) {
            while *q.front().unwrap() != hole_size {
                count += 1;
                q.rotate_left(1);
            }
            q.pop_front().unwrap();
            if hole_size + current < 0 {
                q.push_front(hole_size + current);
            }
            for _ in 0..current {
                q.push_front(max_id);
            }
        }

        q.rotate_right(count);
        let mut count_back = 0;
        if count > 0 {
            while *q.back().unwrap() != max_id {
                q.rotate_right(1);
                count_back += 1;
            }
            for _ in 0..current {
                q.pop_back();
            }
            // create new empty space that was left after moving file
            q.push_back(-current);

            q.rotate_left(count_back);
            count = 0;
        }

        max_id -= 1;
        // println!("{:?}", q);
    }

    // expand units of negative spaces to one-sized spaces of zeros
    let mut res = vec![];
    for rot in q {
        if rot < 0 {
            res.resize(len(&res) + (-rot) as usize, 0)
        } else {
            res.push(rot as usize)
        }
    }

    enumerate(&res).map(|(i, x)| x * i).sum()
}
