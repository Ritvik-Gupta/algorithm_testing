crate::solution!();

const MODULO: u128 = 1000000007;

macro_rules! protected_sum {
    (in $array:tt for indices $only_index:tt) => {
        $array[$only_index]
    };
    (in $array:tt for indices $head_index:tt, $($tail_indices:tt),*) => {
        ($array[$head_index] + protected_sum!(in $array for indices $($tail_indices),*)) % MODULO
    };
}

impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let mut gen_dp = [1u128; 5];

        (1..n).for_each(|_| {
            gen_dp = [
                protected_sum!(in gen_dp for indices 1, 2, 4),
                protected_sum!(in gen_dp for indices 0, 2),
                protected_sum!(in gen_dp for indices 1, 3),
                protected_sum!(in gen_dp for indices 2),
                protected_sum!(in gen_dp for indices 2, 3),
            ];
        });

        protected_sum!(in gen_dp for indices 0, 1, 2, 3, 4) as i32
    }
}
