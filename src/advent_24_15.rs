#![allow(clippy::unused_unit)]

use crate::*;

// https://adventofcode.com/2024/day/15
// 6666 / 30397


pub fn main_1() -> usize {  // 1559280
    let binding = open("day15/input.txt", "r").read();
    let input = binding.split("\n\n")
        .collect::<Vec<_>>();
    let mut input_map = map(
        |x| pstr::rstrip(x).chars().collect::<Vec<_>>(),
        input[0].lines(),
    ).collect::<Vec<_>>();
    let binding = input[1].replace("\n", "");  // for borrow checker to be happy; could've been oneliner
    let directions = binding.chars();

    // find init position
    let mut count = 0;
    let (mut idx1, mut idx2) = loop {
        let line = &input_map[count];
        let idx_none = line.iter().position(|&c| c == '@');
        match idx_none {
            Some(idx) => { break (count, idx); },
            None => {
                count += 1;
                continue;
            },
        }
    };
    input_map[idx1][idx2] = '.';  // clear that position

    for direction in directions {
        let to_check;
        let vector: (isize, isize);
        match direction {
            '^' => {
                to_check = (idx1 - 1, idx2);
                vector = (-1, 0);
            },
            'v' => {
                to_check = (idx1 + 1, idx2);
                vector = (1, 0);
            },
            '<' => {
                to_check = (idx1, idx2 - 1);
                vector = (0, -1);
            },
            '>' => {
                to_check = (idx1, idx2 + 1);
                vector = (0, 1);
            },
            _ => unreachable!(),
        }
        if input_map[to_check.0][to_check.1] == '#' {
            continue;
        }
        if input_map[to_check.0][to_check.1] == '.' {
            (idx1, idx2) = to_check;
            continue;
        }

        let mut count = 1;
        let mut to_move = 1;
        loop {
            let to_check = ((idx1 as isize + vector.0 * count) as usize, (idx2 as isize + vector.1 * count) as usize);
            if input_map[to_check.0][to_check.1] == '#' {
                if to_move > 1 {
                    count = 1;
                }
                break;
            }
            if input_map[to_check.0][to_check.1] == '.' {
                count += 1;
                if to_move > 1 {
                    break;
                }
                continue;
            }
            to_move += 1;
            count += 1;
        }
        if count <= 1 {
            continue
        }
        // moving box(es)
        input_map[(idx1 as isize + vector.0 * (count - 1)) as usize][(idx2 as isize + vector.1 * (count - 1)) as usize] = 'O';

        // moving the fish
        idx1 = (idx1 as isize + vector.0) as usize;
        idx2 = (idx2 as isize + vector.1) as usize;
        input_map[idx1][idx2] = '.';
    }

    // find GPS boxes
    let mut total = 0;
    for (i, line) in enumerate(&input_map) {
        for (j, character) in enumerate(&line) {
            if character == 'O' {
                total += 100 * i + j;
            }
        }
    }

    total
}


fn push_block(
    input_map: &mut Vec<Vec<char>>, idx1: (usize, usize), vector: (isize, isize)
) -> bool {
    let left_part_pos;   // Current position of the left part
    let right_part_pos;  // Current position of the right part
    if input_map[idx1.0][idx1.1] == '[' {
        left_part_pos = idx1;
        right_part_pos = (idx1.0, idx1.1 + 1);
    } else if input_map[idx1.0][idx1.1] == ']' {
        left_part_pos = (idx1.0, idx1.1 - 1);
        right_part_pos = idx1;
    } else if input_map[idx1.0][idx1.1] == '#' {
        //print(&"current field is not block, nothing to push");
        return false;
    } else {
        //print(&"current field is not block, nothing to push");
        return true;
    }

    // New positions after moving
    let new_left_part_pos = (
        (left_part_pos.0 as isize + vector.0) as usize,
        (left_part_pos.1 as isize + vector.1) as usize,
    );
    let new_right_part_pos = (
        (right_part_pos.0 as isize + vector.0) as usize,
        (right_part_pos.1 as isize + vector.1) as usize,
    );

    // ## .# #.
    // [] [] []
    if
      input_map[new_left_part_pos.0][new_left_part_pos.1] == '#'
    ||
      input_map[new_right_part_pos.0][new_right_part_pos.1] == '#'
    {
        //print(&"cannot move, wall");
        return false;  // Can't push into a wall
    }

    // Check for pushing another block
    // []
    // []
    if input_map[new_left_part_pos.0][new_left_part_pos.1] == '[' {
        if
          push_block(&mut input_map.clone(), new_left_part_pos, vector)
        &&
          push_block(input_map, new_right_part_pos, vector)
        {
            push_block(input_map, new_left_part_pos, vector);
            input_map[new_left_part_pos.0][new_left_part_pos.1] = '[';    // Move left part
            input_map[new_right_part_pos.0][new_right_part_pos.1] = ']';  // Move right part
            input_map[left_part_pos.0][left_part_pos.1] = '.';            // Clear old left position
            input_map[right_part_pos.0][right_part_pos.1] = '.';          // Clear old right position
            return true;
        }
        return false;
    }
    // ][
    // []
    if
      input_map[new_left_part_pos.0][new_left_part_pos.1] == ']'
    &&
      input_map[new_right_part_pos.0][new_right_part_pos.1] == '['
    {
        if
          push_block(&mut input_map.clone(), new_left_part_pos, vector)
        &&
          push_block(input_map, new_right_part_pos, vector)
        {
            push_block(input_map, new_left_part_pos, vector);
            input_map[new_left_part_pos.0][new_left_part_pos.1] = '[';
            input_map[new_right_part_pos.0][new_right_part_pos.1] = ']';
            input_map[left_part_pos.0][left_part_pos.1] = '.';
            input_map[right_part_pos.0][right_part_pos.1] = '.';
            return true;
        }
        return false;
    }
    // [].
    // .[]
    if
      input_map[new_left_part_pos.0][new_left_part_pos.1] == ']'
    &&
      input_map[new_right_part_pos.0][new_right_part_pos.1] != '#'
    {
        if push_block(input_map, new_left_part_pos, vector) {
            input_map[new_left_part_pos.0][new_left_part_pos.1] = '[';
            input_map[new_right_part_pos.0][new_right_part_pos.1] = ']';
            input_map[left_part_pos.0][left_part_pos.1] = '.';
            input_map[right_part_pos.0][right_part_pos.1] = '.';
            return true;
        }
        return false;
    }
    // .[]
    // []
    if
      input_map[new_right_part_pos.0][new_right_part_pos.1] == '['
    &&
      input_map[new_left_part_pos.0][new_left_part_pos.1] != '#'
    &&
      push_block(input_map, new_right_part_pos, vector)
    {
        input_map[new_left_part_pos.0][new_left_part_pos.1] = '[';
        input_map[new_right_part_pos.0][new_right_part_pos.1] = ']';
        input_map[left_part_pos.0][left_part_pos.1] = '.';
        input_map[right_part_pos.0][right_part_pos.1] = '.';
        return true;
    }
    // If the left and right part is empty, move both parts to the new locations
    // ..
    // []
    if
      input_map[new_left_part_pos.0][new_left_part_pos.1] == '.'
    &&
      input_map[new_right_part_pos.0][new_right_part_pos.1] == '.'
    {
        //print(&"empty, moving");
        input_map[new_left_part_pos.0][new_left_part_pos.1] = '[';
        input_map[new_right_part_pos.0][new_right_part_pos.1] = ']';
        input_map[left_part_pos.0][left_part_pos.1] = '.';
        input_map[right_part_pos.0][right_part_pos.1] = '.';
        return true;
    }

    false
}


pub fn main_2() -> usize {  // 1576353
    let binding = open("day15/input.txt", "r").read();
    let input = binding.split("\n\n")
        .collect::<Vec<_>>();
    let mut input_map = map(
        |x| pstr::rstrip(x).chars().collect::<Vec<_>>(),
        input[0].lines(),
    ).collect::<Vec<_>>();
    let binding = input[1].replace("\n", "");
    let directions = binding.chars();

    // should be possible to do this in one interator
    for (count, line) in enumerate(&input_map.clone()) {
        let mut new_line = vec![];
        for c in line {
            match c {
                'O' => new_line.append(&mut "[]".chars().collect()),
                '.' => new_line.append(&mut "..".chars().collect()),
                '#' => new_line.append(&mut "##".chars().collect()),
                '@' => new_line.append(&mut "@.".chars().collect()),
                _ => unreachable!(),
            }
        }
        input_map[count] = new_line;
    }

    // for line in &input_map {
    //     println!("{:?}", line);
    // }

    // find init position
    let mut count = 0;
    let (mut idx1, mut idx2) = loop {
        let line = &input_map[count];
        let idx_none = line.iter().position(|&c| c == '@');
        match idx_none {
            Some(idx) => { break (count, idx); },
            None => {
                count += 1;
                continue;
            },
        }
    };
    input_map[idx1][idx2] = '.';  // clear that "@" position

    for direction in directions {
        // print(&direction);
        let to_check;
        let vector: (isize, isize);
        match direction {
            '^' => {
                to_check = (idx1 - 1, idx2);
                vector = (-1, 0);
            },
            'v' => {
                to_check = (idx1 + 1, idx2);
                vector = (1, 0);
            },
            '<' => {
                to_check = (idx1, idx2 - 1);
                vector = (0, -1);
            },
            '>' => {
                to_check = (idx1, idx2 + 1);
                vector = (0, 1);
            },
            _ => unreachable!(),
        }

        if input_map[to_check.0][to_check.1] == '#' {
            continue;
        }
        if input_map[to_check.0][to_check.1] == '.' {
            (idx1, idx2) = to_check;
            continue;
        }

        let mut count = 1;
        let mut to_move = 1;
        loop {
            let to_check = ((idx1 as isize + vector.0 * count) as usize, (idx2 as isize + vector.1 * count) as usize);
            if input_map[to_check.0][to_check.1] == '#' {
                if to_move > 1 {
                    count = 1;
                }
                break;
            }
            if input_map[to_check.0][to_check.1] == '.' {
                count += 1;
                if to_move > 1 {
                    break;
                }
                continue;
            }
            to_move += 1;
            count += 1;
        }
        if count <= 1 {
            continue
        }

        match direction {
            '<' => {
                for tm in 0..to_move {
                    let ch = if tm % 2 == 0 { '[' } else { ']' };
                    input_map[
                        (idx1 as isize + vector.0 * (tm + 1)) as usize
                    ][  (idx2 as isize + vector.1 * (tm + 1)) as usize] = ch;
                }
                input_map[to_check.0][to_check.1] = '.';
            },
            '>' => {
                for tm in 0..to_move {
                    let ch = if tm % 2 == 0 { ']' } else { '[' };
                    input_map[
                        (idx1 as isize + vector.0 * (tm + 1)) as usize
                    ][  (idx2 as isize + vector.1 * (tm + 1)) as usize] = ch;
                }
                input_map[to_check.0][to_check.1] = '.';
            },
            '^' | 'v' => {
                // vertically they do not have to alternate unlike the horizontal case
                // Pass clone of input_map first to check that the root can move. If it is the case, pass the real map.
                if !push_block(&mut input_map.clone(), ((idx1 as isize + vector.0) as usize, (idx2 as isize + vector.1) as usize), vector) {
                    continue;  // we cannot move, so skip the update of @ position
                }
                push_block(&mut input_map, ((idx1 as isize + vector.0) as usize, (idx2 as isize + vector.1) as usize), vector);
            },
            _ => unreachable!(),
        }

        idx1 = to_check.0;
        idx2 = to_check.1;

        // let mut map_copy = input_map.clone();
        // map_copy[idx1][idx2] = '@';
        // for line in &map_copy {
        //     println!("{:?}", line);
        // }
    }

    // find GPS boxes
    let mut total = 0;
    for (i, line) in enumerate(&input_map) {
        for (j, character) in enumerate(&line) {
            if character == '[' {
                total += 100 * i + j;
            }
        }
    }

    total
}
