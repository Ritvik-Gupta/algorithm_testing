crate::solution!();

const BIG_MOD: i64 = 1000000007;

fn mod_pow(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}

impl Solution {
    pub fn monkey_move(n: i32) -> i32 {
        ((mod_pow(2, n as i64, BIG_MOD) + BIG_MOD - 2) % BIG_MOD) as i32
    }
}
