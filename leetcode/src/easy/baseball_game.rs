crate::solution!();

impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut records = Vec::<i32>::new();
        ops.into_iter()
            .for_each(|operation| match operation.as_ref() {
                "C" => {
                    records.pop();
                }
                "D" => records.push(records[records.len() - 1] * 2),
                "+" => records.push(records[records.len() - 1] + records[records.len() - 2]),
                num => records.push(num.parse().unwrap()),
            });

        records.into_iter().sum()
    }
}
