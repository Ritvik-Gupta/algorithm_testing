pub enum Fold {
    X,
    Y,
}

pub struct Paper(Vec<Vec<bool>>);

impl Paper {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn add_mark(&mut self, (x, y): (usize, usize)) {
        if x >= self.num_rows() {
            while x >= self.num_rows() {
                self.0.push(vec![false; self.num_cols()]);
            }
        }
        if y >= self.num_cols() {
            self.0.iter_mut().for_each(|row| {
                while y >= row.len() {
                    row.push(false);
                }
            });
        }

        self.0[x][y] = true;
    }

    fn num_rows(&self) -> usize {
        self.0.len()
    }

    fn num_cols(&self) -> usize {
        self.0.get(0).map(|row| row.len()).unwrap_or(0)
    }
}

pub struct TransparentOrigami;

impl crate::AdventDayProblem for TransparentOrigami {
    type Arg = (Paper, Vec<Fold>);
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

    fn construct_arg(mut dataset: impl Iterator<Item = String>) -> Self::Arg {
        let mut paper = Paper::new();

        while let Some(line) = dataset.next() {
            if line.is_empty() {
                break;
            }

            let (x, y) = line.split_once(',').expect("is a 2d coordinate");
            paper.add_mark((
                x.parse().expect("is a number"),
                y.parse().expect("is a number"),
            ));
        }

        (
            paper,
            dataset
                .map(|line| {
                    let (dir, _) = line.split_once('=').expect("is a fold instruction");

                    match dir.chars().rev().next().expect("has a direction index") {
                        'x' => Fold::X,
                        'y' => Fold::Y,
                        _ => unreachable!(),
                    }
                })
                .collect(),
        )
    }

    fn part_1((mut paper, instructions): Self::Arg) -> Self::Ret {
        let fold_inst = instructions
            .into_iter()
            .next()
            .expect("has atleast one fold instruction");

        match fold_inst {
            Fold::X => {
                for i in 0..paper.num_rows() / 2 {
                    for j in 0..paper.num_cols() {
                        paper.0[i][j] |= paper.0[paper.num_rows() - 1 - i][j];
                    }
                }
                paper.0.truncate(paper.num_rows() / 2);
            }
            Fold::Y => {
                for i in 0..paper.num_rows() {
                    for j in 0..paper.num_cols() / 2 {
                        paper.0[i][j] |= paper.0[i][paper.num_cols() - 1 - j];
                    }
                }
                let num_cols = paper.num_cols();
                paper
                    .0
                    .iter_mut()
                    .for_each(|row| row.truncate(num_cols / 2));
            }
        }

        paper
            .0
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&has_mark| if has_mark { 1 } else { 0 })
                    .sum::<i128>()
            })
            .sum()
    }

    fn part_2((mut paper, instructions): Self::Arg) -> Self::Ret {
        instructions
            .into_iter()
            .for_each(|fold_inst| match fold_inst {
                Fold::X => {
                    for i in 0..paper.num_rows() / 2 {
                        for j in 0..paper.num_cols() {
                            paper.0[i][j] |= paper.0[paper.num_rows() - 1 - i][j];
                        }
                    }
                    paper.0.truncate(paper.num_rows() / 2);
                }
                Fold::Y => {
                    for i in 0..paper.num_rows() {
                        for j in 0..paper.num_cols() / 2 {
                            paper.0[i][j] |= paper.0[i][paper.num_cols() - 1 - j];
                        }
                    }

                    let num_cols = paper.num_cols();
                    paper
                        .0
                        .iter_mut()
                        .for_each(|row| row.truncate(num_cols / 2));
                }
            });

        println!();
        for j in 0..paper.num_cols() {
            for i in 0..paper.num_rows() {
                print!("{}", if paper.0[i][j] { '*' } else { ' ' });
            }
            println!();
        }

        paper
            .0
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&has_mark| if has_mark { 1 } else { 0 })
                    .sum::<i128>()
            })
            .sum()
    }
}
