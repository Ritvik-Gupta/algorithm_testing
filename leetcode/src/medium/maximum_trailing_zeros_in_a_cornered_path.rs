crate::solution!();

#[derive(Default, Clone, Copy)]
struct TenFactor(i32, i32);

impl TenFactor {
    fn of(mut num: i32) -> Self {
        let (mut twos, mut fives) = (0, 0);

        while num % 2 == 0 {
            twos += 1;
            num /= 2;
        }
        while num % 5 == 0 {
            fives += 1;
            num /= 5;
        }

        Self(twos, fives)
    }

    fn min(self) -> i32 {
        i32::min(self.0, self.1)
    }
}

impl std::ops::Add for TenFactor {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl std::ops::Sub for TenFactor {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

#[derive(Default, Clone, Copy)]
struct CornerCut {
    left: TenFactor,
    top: TenFactor,
}

impl CornerCut {
    fn get_in(table: &Vec<Vec<Self>>, i: usize, j: usize) -> Option<Self> {
        Some(*table.get(i)?.get(j)?)
    }
}

fn grid_iterator(m: usize, n: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..m).flat_map(move |i| (0..n).map(move |j| (i, j)))
}

impl Solution {
    pub fn max_trailing_zeros(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut table = vec![vec![CornerCut::default(); n]; m];

        grid_iterator(m, n).for_each(|(i, j)| {
            table[i][j].top = TenFactor::of(grid[i][j])
                + CornerCut::get_in(&table, i - 1, j).unwrap_or_default().top;
            table[i][j].left = TenFactor::of(grid[i][j])
                + CornerCut::get_in(&table, i, j - 1).unwrap_or_default().left;
        });

        grid_iterator(m, n)
            .map(|(i, j)| {
                let top_bound = table[m - 1][j].top;
                let left_bound = table[i][n - 1].left;
                let factor = TenFactor::of(grid[i][j]);

                let top_factor = table[i][j].top;
                let left_factor = table[i][j].left;

                vec![
                    top_factor + left_factor - factor,
                    left_bound - left_factor + top_factor,
                    top_bound - top_factor + left_factor,
                    top_bound + left_bound - top_factor - left_factor + factor,
                ]
                .into_iter()
                .map(TenFactor::min)
                .max()
                .unwrap()
            })
            .max()
            .unwrap_or(0)
    }
}
