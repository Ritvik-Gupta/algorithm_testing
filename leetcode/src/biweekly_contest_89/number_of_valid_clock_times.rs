crate::solution!();

const UKN: u8 = b'?';

impl Solution {
    pub fn count_time(time: String) -> i32 {
        let time = time.as_bytes();

        (match (time[0], time[1]) {
            (UKN, UKN) => 24,
            (UKN, b'0'..=b'3') => 3,
            (UKN, _) => 2,
            (b'2', UKN) => 4,
            (_, UKN) => 10,
            (_, _) => 1,
        }) * (time[3] == UKN).then(|| 6).unwrap_or(1)
            * (time[4] == UKN).then(|| 10).unwrap_or(1)
    }
}
