use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct PolymerPair(char, char);

impl std::ops::Shl<char> for PolymerPair {
    type Output = (PolymerPair, PolymerPair);

    fn shl(self, token: char) -> Self::Output {
        (PolymerPair(self.0, token), PolymerPair(token, self.1))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PolymerTemplate(HashMap<PolymerPair, i128>);

impl PolymerTemplate {
    fn empty() -> Self {
        Self(HashMap::new())
    }

    fn new(raw_template: String) -> Self {
        let mut template = HashMap::new();

        raw_template
            .as_bytes()
            .windows(2)
            .map(|window| PolymerPair(window[0] as char, window[1] as char))
            .for_each(|pair| *template.entry(pair).or_insert(0) += 1);

        Self(template)
    }
}

fn build_polymer_template_for<const NUM_STEPS: usize>(
    mut template: PolymerTemplate,
    extreme_ends_pair: PolymerPair,
    manual: HashMap<PolymerPair, char>,
) -> i128 {
    let mut next_template = PolymerTemplate::empty();

    for _ in 0..NUM_STEPS {
        template.0.iter().for_each(|(pair, &freq)| {
            let (pair_1, pair_2) = *pair << manual[pair];

            *next_template.0.entry(pair_1).or_insert(0) += freq;
            *next_template.0.entry(pair_2).or_insert(0) += freq;
        });

        std::mem::swap(&mut template, &mut next_template);
        next_template.0.clear();
    }

    let mut atom_freqs = HashMap::new();
    template.0.iter().for_each(|(pair, &freq)| {
        *atom_freqs.entry(pair.0).or_insert(0) += freq;
        *atom_freqs.entry(pair.1).or_insert(0) += freq;
    });

    *atom_freqs.entry(extreme_ends_pair.0).or_insert(0) += 1;
    *atom_freqs.entry(extreme_ends_pair.1).or_insert(0) += 1;

    (atom_freqs.values().max().expect("has a max frequency atom")
        - atom_freqs.values().min().expect("has a min frequency atom"))
        / 2
}

pub struct ExtendedPolymerization;

impl super::AdventDayProblem for ExtendedPolymerization {
    type Arg = (PolymerTemplate, PolymerPair, HashMap<PolymerPair, char>);

    fn get_problem_name() -> &'static str {
        file!()
            .split('\\')
            .last()
            .expect("has a file path")
            .split('.')
            .next()
            .expect("has a file name")
    }

    fn construct_arg(mut dataset: impl Iterator<Item = String>) -> Self::Arg {
        let raw_polymer = dataset.next().expect("has the initial polymer template");

        let extreme_ends_pair = PolymerPair(
            raw_polymer.chars().next().unwrap(),
            raw_polymer.chars().last().unwrap(),
        );

        let initial_polymer = PolymerTemplate::new(raw_polymer);
        dataset.next();

        (
            initial_polymer,
            extreme_ends_pair,
            dataset
                .map(|line| {
                    let line = line.as_bytes();
                    (
                        PolymerPair(line[0] as char, line[1] as char),
                        line[line.len() - 1] as char,
                    )
                })
                .collect(),
        )
    }

    fn part_1((template, extreme_ends_pair, manual): Self::Arg) -> i128 {
        build_polymer_template_for::<10>(template, extreme_ends_pair, manual)
    }

    fn part_2((template, extreme_ends_pair, manual): Self::Arg) -> i128 {
        build_polymer_template_for::<40>(template, extreme_ends_pair, manual)
    }
}
