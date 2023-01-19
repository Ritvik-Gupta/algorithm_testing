crate::solution!();

impl Solution {
    pub fn can_change(start: String, targt: String) -> bool {
        let start = start
            .chars()
            .enumerate()
            .filter(|&(_, ch)| ch != '_')
            .collect::<Vec<_>>();

        let targt = targt
            .chars()
            .enumerate()
            .filter(|&(_, ch)| ch != '_')
            .collect::<Vec<_>>();

        start.len() == targt.len()
            && start
                .into_iter()
                .zip(targt.into_iter())
                .all(|((i, a), (j, b))| {
                    if a != b {
                        return false;
                    }

                    match a {
                        'L' => i >= j,
                        'R' => i <= j,
                        _ => unreachable!(),
                    }
                })
    }
}
