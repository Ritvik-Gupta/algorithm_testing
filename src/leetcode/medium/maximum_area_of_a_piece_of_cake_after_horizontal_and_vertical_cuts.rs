crate::leetcode::solution!();

const MODULO: i64 = 1e9 as i64 + 7;

fn get_max(size: i32, mut cuts: Vec<i32>) -> i32 {
    cuts.sort();
    cuts.push(size);

    let mut max_cut_size = cuts[0];
    for cut_idx in 0..cuts.len() - 1 {
        max_cut_size = i32::max(max_cut_size, cuts[cut_idx + 1] - cuts[cut_idx]);
    }

    max_cut_size
}

impl Solution {
    pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
        ((get_max(h, horizontal_cuts) as i64 * get_max(w, vertical_cuts) as i64) % MODULO) as i32
    }
}
