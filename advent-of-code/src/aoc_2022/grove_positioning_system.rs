#[derive(Clone)]
pub struct MixingElm {
    idx: usize,
    steps: i64,
}

fn mixing_rotatory_decryption<const DECRYPTION_KEY: i64, const SALT_ROUNDS: usize>(
    mut list: Vec<MixingElm>,
) -> i64 {
    list.iter_mut().for_each(|elm| elm.steps *= DECRYPTION_KEY);

    let n = list.len();
    let original_list = list.clone();

    tqdm::tqdm(original_list.iter().cycle().take(n * SALT_ROUNDS)).for_each(
        |&MixingElm { idx, steps }| {
            let mut rotr_idx = list.iter().position(|elm| elm.idx == idx).unwrap();
            let delta_update = if steps < 0 { n - 1 } else { 1 };

            for _ in 0..(steps.abs() as usize % (n - 1)) {
                list.swap(rotr_idx, (rotr_idx + delta_update) % n);
                rotr_idx = (rotr_idx + delta_update) % n;
            }
        },
    );

    let idx_of_zero = list.iter().position(|elm| elm.steps == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|offset| list[(idx_of_zero + offset) % list.len()].steps)
        .sum()
}

pub struct GrovePositioningSystem;

impl crate::AdventDayProblem for GrovePositioningSystem {
    type Arg = Vec<MixingElm>;
    type Ret = i64;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg {
        dataset
            .enumerate()
            .map(|(idx, line)| MixingElm {
                idx,
                steps: line.parse().unwrap(),
            })
            .collect()
    }

    fn part_1(list: Self::Arg) -> Self::Ret {
        mixing_rotatory_decryption::<1, 1>(list)
    }

    fn part_2(list: Self::Arg) -> Self::Ret {
        mixing_rotatory_decryption::<811589153, 10>(list)
    }
}
