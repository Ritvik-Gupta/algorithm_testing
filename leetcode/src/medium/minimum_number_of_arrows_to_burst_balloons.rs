crate::solution!();

impl Solution {
    pub fn find_min_arrow_shots(mut segments: Vec<Vec<i32>>) -> i32 {
        segments.sort_by_key(|balloons| balloons[1]);

        let (mut num_arrows, mut arrow_loc) = (1, segments[0][1]);

        segments[1..]
            .into_iter()
            .map(|balloon| (balloon[0], balloon[1]))
            .for_each(|(start, end)| {
                if start > arrow_loc {
                    arrow_loc = end;
                    num_arrows += 1;
                }
            });
        num_arrows
    }
}
