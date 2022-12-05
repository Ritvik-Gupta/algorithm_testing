use std::collections::HashMap;

#[derive(Clone, Hash, PartialEq, Eq)]
struct Cave {
    name: String,
    is_large: bool,
}

impl Into<Cave> for &str {
    fn into(self) -> Cave {
        Cave {
            name: self.to_string(),
            is_large: self
                .chars()
                .next()
                .expect("has a non empty name")
                .is_uppercase(),
        }
    }
}

const CAVE_START: &str = "start";
const CAVE_END: &str = "end";

pub struct ConnectedCaves(HashMap<Cave, Vec<Cave>>);

impl ConnectedCaves {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn add_connection(&mut self, cave_a: &str, cave_b: &str) {
        self.0
            .entry(cave_a.into())
            .or_insert(Vec::new())
            .push(cave_b.into());
        self.0
            .entry(cave_b.into())
            .or_insert(Vec::new())
            .push(cave_a.into());
    }
}

pub struct PassagePathing;

impl crate::AdventDayProblem for PassagePathing {
    type Arg = ConnectedCaves;
    type Ret = i128;

    fn get_problem_name() -> &'static str {
        file!()
            .split('\\')
            .last()
            .expect("has a file path")
            .split('.')
            .next()
            .expect("has a file name")
    }

    fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg {
        let mut connected_caves = ConnectedCaves::new();
        dataset.for_each(|connection| {
            let (cave_a, cave_b) = connection
                .split_once('-')
                .expect("is a valid graph connection");

            connected_caves.add_connection(cave_a, cave_b);
        });
        connected_caves
    }

    fn part_1(connected_caves: Self::Arg) -> Self::Ret {
        let mut path_counter = 0;
        let mut visit_records = connected_caves
            .0
            .keys()
            .map(|cave| (cave.clone(), 0))
            .collect();

        Self::recur_paths(
            &connected_caves,
            CAVE_START.into(),
            &mut visit_records,
            &mut path_counter,
            false,
        );

        path_counter
    }

    fn part_2(connected_caves: Self::Arg) -> Self::Ret {
        let mut path_counter = 0;
        let mut visit_records = connected_caves
            .0
            .keys()
            .map(|cave| (cave.clone(), 0))
            .collect();

        Self::recur_paths(
            &connected_caves,
            CAVE_START.into(),
            &mut visit_records,
            &mut path_counter,
            true,
        );

        path_counter
    }
}

impl PassagePathing {
    fn recur_paths(
        connected_caves: &ConnectedCaves,
        current_cave: Cave,
        visit_records: &mut HashMap<Cave, usize>,
        path_counter: &mut i128,
        can_visit_small_cave_twice: bool,
    ) {
        if current_cave.name == CAVE_END {
            *path_counter += 1;
        }

        *visit_records
            .get_mut(&current_cave)
            .expect("cave was constructed") += 1;

        connected_caves.0[&current_cave]
            .iter()
            .for_each(|neighbor_cave| {
                match (neighbor_cave.is_large, visit_records[neighbor_cave]) {
                    (false, 1..)
                        if can_visit_small_cave_twice
                            && neighbor_cave.name != CAVE_START
                            && neighbor_cave.name != CAVE_END =>
                    {
                        Self::recur_paths(
                            connected_caves,
                            neighbor_cave.clone(),
                            visit_records,
                            path_counter,
                            false,
                        );
                    }
                    (false, 1..) => {}
                    _ => {
                        Self::recur_paths(
                            connected_caves,
                            neighbor_cave.clone(),
                            visit_records,
                            path_counter,
                            can_visit_small_cave_twice,
                        );
                    }
                }
            });

        *visit_records
            .get_mut(&current_cave)
            .expect("cave was constructed") -= 1;
    }
}
