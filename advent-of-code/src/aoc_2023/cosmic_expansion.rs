use crate::utils::Vector;
use itertools::Itertools;

fn transpose<T>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Default + Clone + std::fmt::Display,
{
    let (m, n) = (matrix.len(), matrix[0].len());
    let mut transposed = vec![vec![T::default(); m]; n];

    for i in 0..m {
        for j in 0..n {
            transposed[j][i] = matrix[i][j].clone();
        }
    }

    transposed
}

fn find_expansion_locations(mut universe: Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let (m, n) = (universe.len(), universe[0].len());

    let expansion_rows = (0..m)
        .filter(|&i| universe[i].iter().all(|&x| x == '.'))
        .collect();

    universe = transpose(universe);

    let expansion_cols = (0..n)
        .filter(|&j| universe[j].iter().all(|&x| x == '.'))
        .collect();

    (expansion_rows, expansion_cols)
}

fn compute_galaxy_separations(
    universe: Vec<Vec<char>>,
    expanded_rows: Vec<usize>,
    expanded_cols: Vec<usize>,
    expansion_factor: u64,
) -> u64 {
    let galaxies = universe
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, x)| (*x == '#').then(|| Vector(i, j)))
        })
        .collect_vec();

    let mut total_distance = 0;
    for p in 0..galaxies.len() {
        for q in p + 1..galaxies.len() {
            let (x1, x2) = (
                std::cmp::min(galaxies[p].0, galaxies[q].0),
                std::cmp::max(galaxies[p].0, galaxies[q].0),
            );
            let (y1, y2) = (
                std::cmp::min(galaxies[p].1, galaxies[q].1),
                std::cmp::max(galaxies[p].1, galaxies[q].1),
            );

            let mut delta_x = (x2 - x1) as u64;
            delta_x += expanded_rows
                .iter()
                .filter(|e| (x1..x2).contains(&e))
                .count() as u64
                * (expansion_factor - 1);

            let mut delta_y = (y2 - y1) as u64;
            delta_y += expanded_cols
                .iter()
                .filter(|e| (y1..y2).contains(&e))
                .count() as u64
                * (expansion_factor - 1);

            total_distance += delta_x + delta_y;
        }
    }
    total_distance
}

pub struct CosmicExpansion;

impl crate::AdventDayProblem for CosmicExpansion {
    type Arg = Vec<Vec<char>>;

    type Ret = u64;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg {
        dataset.map(|line| line.chars().collect()).collect()
    }

    fn part_1(universe: Self::Arg) -> Self::Ret {
        let (expansion_rows, expansion_cols) = find_expansion_locations(universe.clone());
        compute_galaxy_separations(universe, expansion_rows, expansion_cols, 2)
    }

    fn part_2(universe: Self::Arg) -> Self::Ret {
        let (expansion_rows, expansion_cols) = find_expansion_locations(universe.clone());
        compute_galaxy_separations(universe, expansion_rows, expansion_cols, 1000000)
    }
}
