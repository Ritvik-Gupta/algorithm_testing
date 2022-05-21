crate::leetcode::solution!();

fn optional_min(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    match (a, b) {
        (Some(a), Some(b)) => Some(i32::min(a, b)),
        (Some(_), _) => a,
        (_, Some(_)) => b,
        (_, _) => None,
    }
}

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;

        let mut possibles = vec![None; amount + 1];
        possibles[0] = Some(0);

        coins.iter().map(|&coin| coin as usize).for_each(|coin| {
            (0..=amount).filter(|&j| j >= coin).for_each(|j| {
                possibles[j] = optional_min(possibles[j], possibles[j - coin].map(|x| x + 1))
            });
        });

        possibles[amount].unwrap_or(-1)
    }
}
