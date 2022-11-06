use rand::{rngs::ThreadRng, Rng};

struct Solution {
    accumulated_weights: Vec<i32>,
    rng: ThreadRng,
}

impl Solution {
    #[allow(dead_code)]
    fn new(weights: Vec<i32>) -> Self {
        Self {
            accumulated_weights: weights
                .into_iter()
                .scan(0, |acc, weight| {
                    *acc += weight;
                    Some(*acc)
                })
                .collect(),
            rng: rand::thread_rng(),
        }
    }

    #[allow(dead_code)]
    fn pick_index(&mut self) -> i32 {
        let random_idx = 1 + self
            .rng
            .gen_range(0..*self.accumulated_weights.last().unwrap());

        self.accumulated_weights
            .binary_search(&random_idx)
            .map_or_else(|i| i, |i| i) as i32
    }
}
