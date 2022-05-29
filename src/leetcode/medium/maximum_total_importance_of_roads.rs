crate::leetcode::solution!();

impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        let num_city = n as usize;
        let mut city_degrees = vec![0; num_city];

        for road in roads {
            city_degrees[road[0] as usize] += 1;
            city_degrees[road[1] as usize] += 1;
        }

        city_degrees.sort();

        city_degrees
            .iter()
            .enumerate()
            .map(|(idx, &degree)| degree * (idx as i64 + 1))
            .sum()
    }
}
