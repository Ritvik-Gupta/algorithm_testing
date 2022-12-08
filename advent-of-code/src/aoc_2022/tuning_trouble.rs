use std::collections::HashSet;

pub struct TuningTrouble;

fn find_marker_signal<const PACKETSIZE: usize>(signals: &[char]) -> usize {
    signals
        .windows(PACKETSIZE)
        .enumerate()
        .find_map(|(idx, packet)| {
            (packet.iter().collect::<HashSet<_>>().len() == PACKETSIZE).then(|| idx + PACKETSIZE)
        })
        .unwrap()
}

impl crate::AdventDayProblem for TuningTrouble {
    type Arg = Vec<char>;
    type Ret = usize;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(mut dataset: impl Iterator<Item = String>) -> Self::Arg {
        dataset.next().unwrap().chars().collect()
    }

    fn part_1(signals: Self::Arg) -> Self::Ret {
        find_marker_signal::<4>(&signals)
    }

    fn part_2(signals: Self::Arg) -> Self::Ret {
        find_marker_signal::<14>(&signals)
    }
}
