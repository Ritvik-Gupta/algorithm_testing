crate::leetcode::solution!();

struct Combinations {
    upper_limit: i32,
    combination: Vec<i32>,
    result: Vec<Vec<i32>>,
}

impl Combinations {
    fn generate_for(upper_limit: i32, combination_size: usize) -> Self {
        let mut combination_gen = Self {
            upper_limit,
            combination: Vec::with_capacity(combination_size),
            result: Vec::with_capacity({
                let upper_limit = upper_limit as usize;
                let mut numerator = 1;
                let mut denominator = 1;

                for i in 1..=combination_size {
                    numerator *= upper_limit - i + 1;
                    denominator *= i;
                }
                numerator / denominator
            }),
        };
        combination_gen.generate(1);
        combination_gen
    }

    fn generate(&mut self, current_num: i32) {
        if self.combination.len() == self.combination.capacity() {
            self.result.push(self.combination.clone());
            return;
        }
        if current_num > self.upper_limit {
            return;
        }

        self.combination.push(current_num);
        self.generate(current_num + 1);
        self.combination.pop();
        self.generate(current_num + 1);
    }
}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        Combinations::generate_for(n, k as usize).result
    }
}
