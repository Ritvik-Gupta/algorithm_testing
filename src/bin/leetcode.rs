fn main() {
    println!(
        "{}",
        algorithms::leetcode::time_needed_to_inform_all_employees::Solution::num_of_minutes(
            8,
            0,
            vec![-1, 5, 0, 6, 7, 0, 0, 0],
            vec![89, 0, 0, 0, 0, 523, 241, 519],
        )
    );
}
