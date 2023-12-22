use std::collections::VecDeque;

fn create_prediction_report(sensor_history: VecDeque<i64>) -> Vec<VecDeque<i64>> {
    let mut report = vec![sensor_history];
    let mut idx = 0;

    loop {
        if report[idx].iter().all(|&x| x == 0) {
            return report;
        }

        let mut difference = VecDeque::new();
        for i in 0..report[idx].len() - 1 {
            difference.push_back(report[idx][i + 1] - report[idx][i]);
        }
        report.push(difference);
        idx += 1;
    }
}

pub struct MirageMaintenance;

impl crate::AdventDayProblem for MirageMaintenance {
    type Arg = Vec<VecDeque<i64>>;

    type Ret = i64;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg {
        dataset
            .map(|line| line.split(' ').filter_map(|x| x.parse().ok()).collect())
            .collect()
    }

    fn part_1(sensor_histories: Self::Arg) -> Self::Ret {
        sensor_histories
            .into_iter()
            .map(|sensor_history| {
                let mut report = create_prediction_report(sensor_history);
                let depth = report.len();

                report[depth - 1].push_back(0);
                for j in (0..depth - 1).rev() {
                    let predict = report[j].back().unwrap() + report[j + 1].back().unwrap();
                    report[j].push_back(predict);
                }

                *report[0].back().unwrap()
            })
            .sum()
    }

    fn part_2(sensor_histories: Self::Arg) -> Self::Ret {
        sensor_histories
            .into_iter()
            .map(|sensor_history| {
                let mut report = create_prediction_report(sensor_history);
                let depth = report.len();

                report[depth - 1].push_front(0);
                for j in (0..depth - 1).rev() {
                    let predict = report[j].front().unwrap() - report[j + 1].front().unwrap();
                    report[j].push_front(predict);
                }

                *report[0].front().unwrap()
            })
            .sum()
    }
}
