crate::solution!();

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 2 {
            return 0;
        }

        let num = n as usize;
        let mut prime_table = vec![true; num];

        for i in 2..=(num as f64).sqrt() as usize {
            if prime_table[i] == false {
                continue;
            }

            (i * i..num).step_by(i).for_each(|j| prime_table[j] = false);
        }

        prime_table.iter().filter(|&&is_prime| is_prime).count() as i32 - 2
    }
}
