crate::leetcode::solution!();

use std::collections::{HashSet, VecDeque};

const MAX_JUMP_VALUE: i32 = 6000;

impl Solution {
    pub fn minimum_jumps(
        forbidden: Vec<i32>,
        forward_jump: i32,
        backward_jump: i32,
        final_state: i32,
    ) -> i32 {
        let mut visited_states: HashSet<_> = forbidden.into_iter().collect();

        let mut total_jumps = 0;
        let mut state_queue = VecDeque::new();
        state_queue.push_back((0, false));
        visited_states.insert(0);

        while !state_queue.is_empty() {
            let level_states = state_queue.len();

            for _ in 0..level_states {
                let (state, had_back_jump) = state_queue.pop_front().unwrap();
                if state == final_state {
                    return total_jumps;
                }

                let backward_state = state - backward_jump;
                if !had_back_jump
                    && backward_state >= 0
                    && !visited_states.contains(&backward_state)
                {
                    state_queue.push_back((backward_state, true));
                    visited_states.insert(backward_state);
                }

                let forward_state = state + forward_jump;
                if forward_state <= MAX_JUMP_VALUE && !visited_states.contains(&forward_state) {
                    state_queue.push_back((forward_state, false));
                    visited_states.insert(forward_state);
                }
            }
            total_jumps += 1;
        }

        -1
    }
}
