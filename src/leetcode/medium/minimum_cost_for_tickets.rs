crate::leetcode::solution!();

static HOLIDAY_PASSES: [usize; 3] = [1, 7, 30];

fn compute_tickets(
    days: &Vec<i32>,
    costs: &Vec<i32>,
    k: usize,
    i: usize,
    dp: &mut Vec<i32>,
) -> i32 {
    if i > (days.len() - 1) {
        return 0;
    }

    if dp[k] == -1 {
        dp[k] = if k <= days[i] as usize {
            HOLIDAY_PASSES
                .iter()
                .enumerate()
                .map(|(j, &holiday_pass)| {
                    costs[j]
                        + compute_tickets(days, costs, days[i] as usize + holiday_pass, i + 1, dp)
                })
                .min()
                .unwrap()
        } else {
            compute_tickets(days, costs, k, i + 1, dp)
        };
    }
    return dp[k];
}

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        compute_tickets(&days, &costs, 0, 0, &mut vec![-1; 500])
    }
}
