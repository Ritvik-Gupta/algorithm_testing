pub enum Fold {
    X(usize),
    Y(usize),
}

pub struct TransparentOrigami;

impl super::AdventDayProblem for TransparentOrigami {
    type Arg = (Vec<Vec<bool>>, Vec<Fold>);

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
        let mut paper = Vec::new();

        while let Some(line) = dataset.next() {
            if line.is_empty() {
                break;
            }
        }

        let mut instructions = Vec::new();

        (paper, instructions)
    }

    fn part_1(arg: Self::Arg) -> i128 {
        todo!()
    }

    fn part_2(arg: Self::Arg) -> i128 {
        todo!()
    }
}
