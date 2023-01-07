crate::solution!();

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let (n, mut tot_surplus, mut surplus, mut start) = (gas.len(), 0, 0, 0);

        for i in 0..n {
            tot_surplus += gas[i] - cost[i];
            surplus += gas[i] - cost[i];
            if surplus < 0 {
                surplus = 0;
                start = i + 1;
            }
        }

        match tot_surplus < 0 {
            true => -1,
            _ => start as _,
        }
    }
}
