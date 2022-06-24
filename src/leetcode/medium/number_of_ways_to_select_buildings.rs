crate::leetcode::solution!();

impl Solution {
    pub fn number_of_ways(buildings: String) -> i64 {
        let (mut num0, mut num1, mut num01, mut num10, mut num010, mut num101) = (0, 0, 0, 0, 0, 0);
        buildings.chars().rev().for_each(|ch| {
            if ch == '0' {
                num0 += 1;
                num01 += num1;
                num010 += num10;
            } else {
                num1 += 1;
                num10 += num0;
                num101 += num01;
            }
        });
        num010 + num101
    }
}
