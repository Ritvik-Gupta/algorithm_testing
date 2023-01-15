crate::solution!();

impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut matrix = vec![vec![0; n]; n];

        queries.iter().for_each(|query| match &query[..] {
            &[start_x, start_y, end_x, end_y] => {
                let (start_y, end_y) = (start_y as usize, end_y as usize);

                (start_x..=end_x).for_each(|x| {
                    let x = x as usize;

                    matrix[x][start_y] += 1;
                    if end_y + 1 != n {
                        matrix[x][end_y + 1] -= 1;
                    }
                });
            }
            _ => unreachable!(),
        });

        for x in 0..n {
            for y in 1..n {
                matrix[x][y] += matrix[x][y - 1];
            }
        }

        matrix
    }
}
