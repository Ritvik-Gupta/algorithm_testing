use std::collections::VecDeque;

const FISH_INTERNAL_COUNTER: usize = 6;
const INFANT_OFFSET: usize = 2;

pub fn simulate_growth_for_days<const SIMULATION_DAYS: usize>(
    lanternfishes: impl Iterator<Item = usize>,
) -> u128 {
    let mut fish_states: VecDeque<_> = [0usize; FISH_INTERNAL_COUNTER + INFANT_OFFSET + 1].into();
    lanternfishes.for_each(|state| fish_states[state] += 1);

    for _ in 1..=SIMULATION_DAYS {
        let fishes_giving_birth = fish_states[0];
        fish_states.pop_front();
        fish_states.push_back(fishes_giving_birth);
        fish_states[FISH_INTERNAL_COUNTER] += fishes_giving_birth;
    }

    fish_states
        .iter()
        .map(|&fishes_in_state| fishes_in_state as u128)
        .sum()
}
