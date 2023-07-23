crate::solution!();

use std::collections::HashMap;

const KNIGHT_HIGHJUMPS: [(i32, i32); 8] = [
    (-2, -1),
    (-2, 1),
    (2, -1),
    (2, 1),
    (-1, -2),
    (-1, 2),
    (1, -2),
    (1, 2),
];

fn knight_move_probability(
    n: i32,
    moves_left: i32,
    r: i32,
    c: i32,
    memo: &mut HashMap<(i32, i32, i32), f64>,
) -> f64 {
    if moves_left == 0 {
        return 1.0;
    }

    let prob = KNIGHT_HIGHJUMPS
        .iter()
        .map(|(dx, dy)| {
            let (x, y) = (r + dx, c + dy);
            if x < n && x >= 0 && y < n && y >= 0 {
                if let Some(&prob) = memo.get(&(x, y, moves_left - 1)) {
                    return prob;
                }
                knight_move_probability(n, moves_left - 1, x, y, memo)
            } else {
                0.0
            }
        })
        .sum::<f64>()
        / 8.0;

    memo.insert((r, c, moves_left), prob);
    prob
}

impl Solution {
    pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
        knight_move_probability(n, k, row, column, &mut HashMap::new())
    }
}
