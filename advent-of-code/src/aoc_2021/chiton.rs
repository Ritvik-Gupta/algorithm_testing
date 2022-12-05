use std::{cmp::Reverse, collections::BinaryHeap};

fn neighboring_caves((x, y): (usize, usize)) -> [(usize, usize); 4] {
    [
        (x.wrapping_sub(1), y),
        (x.wrapping_add(1), y),
        (x, y.wrapping_sub(1)),
        (x, y.wrapping_add(1)),
    ]
}

fn perform_dijkstras_search(cavern: &Vec<Vec<u8>>) -> i128 {
    let (w, h) = (cavern.len(), cavern[0].len());

    let mut shortest_dist_from_start = vec![vec![i128::MAX; h]; w];
    shortest_dist_from_start[0][0] = 0;

    let mut visited = vec![vec![false; h]; w];

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, (0, 0))));

    while let Some(Reverse((min_val, (x, y)))) = heap.pop() {
        visited[x][y] = true;
        if shortest_dist_from_start[x][y] < min_val {
            continue;
        }

        neighboring_caves((x, y))
            .into_iter()
            .filter(|&(nx, ny)| nx < w && ny < h && !visited[nx][ny])
            .for_each(|(nx, ny)| {
                let new_distance = shortest_dist_from_start[x][y] + cavern[nx][ny] as i128;

                if new_distance < shortest_dist_from_start[nx][ny] {
                    shortest_dist_from_start[nx][ny] = new_distance;
                    heap.push(Reverse((new_distance, (nx, ny))));
                }
            });
    }

    shortest_dist_from_start[w - 1][h - 1]
}

fn expand_cavern(cavern: &mut Vec<Vec<u8>>) {
    let (w, h) = (cavern.len(), cavern[0].len());

    fn add_delta_to_cave(cave: u8) -> u8 {
        if cave == 9 {
            1
        } else {
            cave + 1
        }
    }

    for _ in 1..5 {
        cavern.iter_mut().for_each(|row| {
            let size = row.len();
            row.extend_from_within(size - h..size);

            let size = row.len();
            row[size - h..size]
                .iter_mut()
                .for_each(|cave| *cave = add_delta_to_cave(*cave));
        });
    }

    for _ in 1..5 {
        let size = cavern.len();
        for row_idx in size - w..size {
            cavern.push(
                cavern[row_idx]
                    .iter()
                    .map(|&cave| add_delta_to_cave(cave))
                    .collect(),
            );
        }
    }
}

pub struct Chiton;

impl crate::AdventDayProblem for Chiton {
    type Arg = Vec<Vec<u8>>;
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
        dataset
            .map(|line| line.chars().map(|token| token as u8 - b'0').collect())
            .collect()
    }

    fn part_1(cavern: Self::Arg) -> Self::Ret {
        perform_dijkstras_search(&cavern)
    }

    fn part_2(mut cavern: Self::Arg) -> Self::Ret {
        expand_cavern(&mut cavern);
        perform_dijkstras_search(&cavern)
    }
}
