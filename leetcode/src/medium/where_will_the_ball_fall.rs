crate::solution!();

macro_rules! try_get {
    ($arr: tt [$y: tt]) => {{
        if $y < 0 {
            return -1;
        }
        let y = $y as usize;
        if y >= $arr.len() {
            return -1;
        } else {
            $arr[y]
        }
    }};
}

impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid[0].len() as i32;

        (0..n)
            .map(|mut j| {
                for row in grid.iter() {
                    if (try_get!(row[j]) == -1 && try_get!(row[(j - 1)]) == 1)
                        || (try_get!(row[j]) == 1 && try_get!(row[(j + 1)]) == -1)
                    {
                        return -1;
                    }

                    j += try_get!(row[j]);
                    try_get!(row[j]);
                }
                j
            })
            .collect()
    }
}
