#![allow(clippy::unused_unit)]

use crate::*;

// https://adventofcode.com/2024/day/10
// 9502 / 11407


fn neighbors(x: isize, y: isize) -> [(isize, isize); 4] {
    // Return all 4 direct neighbor indices
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}


fn dp(input_map: &Vec<Vec<u8>>, coords: (isize, isize), next_needed: u8) -> Set<(isize, isize)> {
    // "Dynamic programming"
    // TODO memoization
    let (i, j) = coords;
    if next_needed == 10 {
        let mut to_ret = set();
        to_ret.insert((i, j));
        return to_ret;
    }

    let mut res = set();
    let len_map = len(&input_map) as isize;  // assume square
    for (x, y) in neighbors(i, j) {
        if 0 <= x && x < len_map && 0 <= y && y < len_map && input_map[x as usize][y as usize] == next_needed {
            res.update(dp(input_map, (x, y), next_needed + 1));
        }
    }

    res
}


fn dp_rank(input_map: &Vec<Vec<u8>>, coords: (isize, isize), next_needed: u8, mut res: Set<(isize, isize)>, res_set: &mut FrozenSet<(isize, isize)>) -> Set<(isize, isize)> {
    let (i, j) = coords;
    res.insert((i, j));
    if next_needed == 10 {
        return res;
    }

    let len_map = len(&input_map) as isize;
    let mut res_new = set();
    for (x, y) in neighbors(i, j) {
        if 0 <= x && x < len_map && 0 <= y && y < len_map && input_map[x as usize][y as usize] == next_needed {
            res_new = dp_rank(input_map, (x, y), next_needed + 1, res.clone(), res_set);
            let to_insert = res_new.clone();
            <dyn Frozen<_>>::insert(res_set, to_insert);
            //res_set.add(res_new);
        }
    }

    res_new
}


pub fn main_1() -> usize {  // 496
    let input_map = map(
        |x| map(
            |c| int(&c.to_string()),
            pstr::rstrip(&x).chars().collect::<Vec<_>>(),
        ).collect::<Vec<_>>(),
        open("day10/input.txt", "r").readlines(),
    ).collect();

    let mut res = 0;
    for (i, line) in enumerate(&input_map) {
        for (j, c) in enumerate(&line) {
            if c == 0 {
                res += len(&dp(&input_map, (i as isize, j as isize), 1))
            }
        }
    }

    res
}


pub fn main_2() -> usize {  // 1120
    let input_map = map(
        |x| map(
            |c| int(&c.to_string()), pstr::rstrip(&x).chars().collect::<Vec<_>>()
        ).collect::<Vec<_>>(), open("day10/input.txt", "r").readlines()
    ).collect();

    let mut res = 0;
    let mut res_set = set_of_frozensets();
    for (i, line) in enumerate(&input_map) {
        for (j, c) in enumerate(&line) {
            if c == 0 {
                dp_rank(&input_map, (i as isize, j as isize), 1, set(), &mut res_set);
                <dyn Frozen<_>>::remove(&mut res_set, &set());
                res += len(&res_set);
                res_set.clear();
            }
        }
    }

    res
}
