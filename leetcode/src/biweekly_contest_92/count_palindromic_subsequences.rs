crate::solution!();

fn ch_to_idx(ch: char) -> usize {
    (ch as u8 - b'0') as usize
}

fn combination(n: u128, r: u128) -> u128 {
    let (mut nume, mut deno) = (1, 1);

    for x in 1..=r {
        nume *= n - r + x;
        deno *= x;
    }

    nume / deno
}

const MOD: u128 = 1000000007;

impl Solution {
    pub fn count_palindromes(word: String) -> i32 {
        let (mut lft_tbl, mut rgt_tbl) = (vec![0; 10], vec![0; 10]);
        word.chars().for_each(|ch| rgt_tbl[ch_to_idx(ch)] += 1);

        let mut total_combinations = 0;

        word.chars().for_each(|ch| {
            rgt_tbl[ch_to_idx(ch)] -= 1;

            for n in 0..10 {
                for m in n + 1..10 {
                    if lft_tbl[n] >= 1 && rgt_tbl[n] >= 1 && lft_tbl[m] >= 1 && rgt_tbl[m] >= 1 {
                        total_combinations = (total_combinations
                            + combination(lft_tbl[n], 1)
                                * combination(rgt_tbl[n], 1)
                                * combination(lft_tbl[m], 1)
                                * combination(rgt_tbl[m], 1))
                            % MOD;
                    }
                }

                if lft_tbl[n] >= 2 && rgt_tbl[n] >= 2 {
                    total_combinations = (total_combinations
                        + combination(lft_tbl[n], 2) * combination(rgt_tbl[n], 2))
                        % MOD;
                }
            }

            lft_tbl[ch_to_idx(ch)] += 1;
        });

        (total_combinations % MOD) as i32
    }
}
