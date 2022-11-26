crate::solution!();

struct Job {
    start: i32,
    end: i32,
    profit: i32,
}

use std::cmp::Ordering::*;

fn search_job_schedule(jobs: &Vec<Job>, end_time: i32) -> usize {
    let (mut lower, mut upper) = (0, jobs.len() - 1);

    while lower <= upper {
        let mid = lower + (upper - lower) / 2;

        match jobs[mid].start.cmp(&end_time) {
            Less => lower = mid + 1,
            Greater => upper = mid - 1,
            Equal if mid > 0 && jobs[mid].start == jobs[mid - 1].start => upper = mid - 1,
            Equal => return mid,
        }
    }
    lower
}

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = start_time.len();

        let mut jobs = start_time
            .iter()
            .zip(end_time.iter())
            .zip(profit.iter())
            .map(|((&start, &end), &profit)| Job { start, end, profit })
            .collect::<Vec<_>>();
        jobs.sort_by_key(|job| (job.start, job.end, job.profit));

        let mut dp = vec![0; n + 1];
        for i in (0..n).rev() {
            let k = search_job_schedule(&jobs, jobs[i].end);
            dp[i] = i32::max(jobs[i].profit + dp[k], dp[i + 1]);
        }
        dp[0]
    }
}
