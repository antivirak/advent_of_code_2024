#![allow(clippy::unused_unit)]

use crate::*;
use crate::scipy::label;
//use ::itertools::izip;

// https://adventofcode.com/2024/day/12


fn cluster_pixels(
    pixel_indices: Vec<(usize, usize)>, grid_size: usize,
) -> Vec<Vec<(usize, usize)>> {
    //let mut grid = vec![vec![false; grid_size]; grid_size];
    let mut grid = np::zeros((grid_size, grid_size), 1usize);

    for index in &pixel_indices {
        //grid[index.0][index.1] = true;
        grid[index.0][index.1] = 1;
    }

    let structure = scipy::connectivity_4();
    let (labeled_array, _) = label(&grid, Some(&structure));

    let mut clusters: Dict<usize, Vec<(usize, usize)>> = dict();
    for pixel in pixel_indices {
        let label_value = labeled_array[pixel.0][pixel.1];
        clusters.entry(label_value).or_default();
        clusters.get_mut(&label_value).unwrap().push(pixel)
    }

    comp![pixels.clone() for pixels in clusters.values() if !pixels.is_empty()]
}


fn count_corners(mut grid: Vec<Vec<usize>>) -> usize {
    if grid[1][1] == 0 {
        return 0;
    }

    let mut ret = 0;
    for _ in range(4) {
        if (grid[0][1] == 0 && grid[1][2] == 0) || (grid[0][2] == 0 && grid[0][1] == 1 && grid[1][2] == 1) {
            ret += 1;
        }
        grid = np::rot90(&grid);
    }

    ret
}


pub fn main_1() -> usize {  // 
    let lines = open("day12/input.txt", "r").readlines();
    let plan = map(|line| pstr::rstrip(&line).chars().collect::<Vec<_>>(), lines).collect::<Vec<_>>();

    let len_plan = len(&plan);
    let mut unique = set();
    for line in &plan {
        unique.update(line);
    }

    let mut total = 0;
    for item in unique {
        let mut vertices = vec![];
        for (i, row) in enumerate(&plan) {
            for (j, cell) in enumerate(&row) {
                if cell == *item {
                    vertices.push((i, j));
                }
            }
        }

        let clusters = cluster_pixels(vertices, len_plan);
        for cluster in clusters {
            let mut perim = 0;
            for pixel in &cluster {
                for (i, j) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    if !cluster.contains(&((pixel.0 as i16 + i) as usize, (pixel.1 as i16 + j) as usize)) {
                        perim += 1;
                    }
                }
            }
            total += len(&cluster) * perim;
        }
    }

    total
}


pub fn main_2() -> usize {  // 
    let lines = open("day12/input.txt", "r").readlines();
    let plan = map(|line| pstr::rstrip(&line).chars().collect::<Vec<_>>(), lines).collect::<Vec<_>>();

    let len_plan = len(&plan);
    let mut unique = set();
    for line in &plan {
        unique.update(line);
    }

    let mut total = 0;
    for item in unique {
        let mut vertices = vec![];
        for (i, row) in enumerate(&plan) {
            for (j, cell) in enumerate(&row) {
                if cell == *item {
                    vertices.push((i, j));
                }
            }
        }

        let clusters = cluster_pixels(vertices, len_plan);
        for cluster in &clusters {
            // add a border of 0s - no index out of bound checking, yeah
            let mut plan_to_measure_perim = np::zeros((len_plan + 2, len_plan + 2), 1usize);
            for pixel in map(|x| (x.0 + 1, x.1 + 1), cluster) {
                plan_to_measure_perim[pixel.0][pixel.1] = 1;
            }

            let mut sides = 0;
            if len(&cluster) == 1 {
                sides = 4;
            } else {
                let windows = np::sliding_window_view(&plan_to_measure_perim, (3, 3));
                for window in windows {
                    sides += count_corners(window);
                }
                //for offset in range(len(&plan_to_measure_perim) - 2) {
                //    for window in izip!(plan_to_measure_perim[offset].windows(3), plan_to_measure_perim[offset + 1].windows(3), plan_to_measure_perim[offset + 2].windows(3)) {
                //        let tmp = vec![window.0.to_vec(), window.1.to_vec(), window.2.to_vec()];
                //        sides += count_corners(tmp);
                //    }
                //}
            }
            total += len(&cluster) * sides;
        }
    }

    total
}
