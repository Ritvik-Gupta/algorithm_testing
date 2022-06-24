crate::leetcode::solution!();

use std::collections::BinaryHeap;

impl Solution {
    pub fn schedule_course(mut courses: Vec<Vec<i32>>) -> i32 {
        courses.sort_by_key(|course| course[1]);

        let mut date = 0;
        let mut courses_duration_queue = BinaryHeap::with_capacity(courses.len() / 10);
        for (duration, end_date) in courses.iter().map(|course| (course[0], course[1])) {
            match courses_duration_queue.peek() {
                _ if date + duration <= end_date => {
                    date += duration;
                    courses_duration_queue.push(duration);
                }
                Some(&prev_long_course) if prev_long_course > duration => {
                    date += duration - courses_duration_queue.pop().unwrap();
                    courses_duration_queue.push(duration);
                }
                _ => {}
            }
        }
        courses_duration_queue.len() as i32
    }
}
