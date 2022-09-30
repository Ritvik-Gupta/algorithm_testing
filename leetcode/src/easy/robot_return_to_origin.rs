crate::solution!();

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let (mut x, mut y) = (0, 0);
        for token in moves.chars() {
            match token {
                'U' => y += 1,
                'D' => y -= 1,
                'L' => x -= 1,
                'R' => x += 1,
                _ => panic!(),
            }
        }
        x == 0 && y == 0
    }
}
