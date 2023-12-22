pub struct GearRatios;

fn is_symbol(elm: u8) -> bool {
    !elm.is_ascii_digit() && elm != b'.'
}

pub struct EngineGrid {
    grid: Vec<Vec<u8>>,
    m: usize,
    n: usize,
}

impl EngineGrid {
    fn iterate(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.m).flat_map(|i| (0..self.n).map(move |j| (i, j)))
    }

    fn at(&self, i: usize, j: usize) -> u8 {
        *self.grid.get(i).and_then(|row| row.get(j)).unwrap_or(&b'.')
    }
}

impl crate::AdventDayProblem for GearRatios {
    type Arg = EngineGrid;

    type Ret = u32;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg {
        let grid: Vec<Vec<_>> = dataset.map(|line| line.as_bytes().to_vec()).collect();
        EngineGrid {
            m: grid.len(),
            n: grid[0].len(),
            grid,
        }
    }

    fn part_1(engine: Self::Arg) -> Self::Ret {
        let mut symbol_schematic: Vec<Vec<bool>> = vec![vec![false; engine.n]; engine.m];

        engine.iterate().for_each(|(i, j)| {
            if !is_symbol(engine.at(i, j)) {
                return;
            }
            for x in i.wrapping_sub(1)..=usize::min(engine.m, i + 1) {
                for y in j.wrapping_sub(1)..=usize::min(engine.n, j + 1) {
                    symbol_schematic[x][y] = true;
                }
            }
        });

        let mut engine_cumulative = 0;
        let mut current_number = 0;
        let mut is_marked = false;

        engine.iterate().for_each(|(i, j)| {
            if engine.at(i, j).is_ascii_digit() {
                current_number = current_number * 10 + (engine.at(i, j) - b'0') as u32;
                is_marked |= symbol_schematic[i][j];

                if !engine.at(i, j + 1).is_ascii_digit() {
                    engine_cumulative += current_number * is_marked as u32;

                    current_number = 0;
                    is_marked = false;
                }
            }
        });

        engine_cumulative
    }

    fn part_2(engine: Self::Arg) -> Self::Ret {
        todo!()
    }
}
