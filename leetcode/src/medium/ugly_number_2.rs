crate::solution!();

impl Solution {
    pub fn nth_ugly_number(nth: i32) -> i32 {
        let factors = [2, 3, 5];
        let (mut factor_powers, mut numbers) = ([0; 3], Vec::with_capacity(nth as usize));
        let mut candidates = [0; 3];

        numbers.push(1);

        for _ in 0..nth - 1 {
            (0..3).for_each(|i| candidates[i] = factors[i] * numbers[factor_powers[i]]);

            let new_num = *candidates.iter().min().unwrap();
            numbers.push(new_num);
            factor_powers
                .iter_mut()
                .enumerate()
                .filter(|&(i, _)| candidates[i] == new_num)
                .for_each(|(_, val)| *val += 1);
        }

        *numbers.last().unwrap()
    }
}
