use bit_set::BitSet;

pub struct TreetopTreeHouse;

impl crate::AdventDayProblem for TreetopTreeHouse {
    type Arg = Vec<Vec<i8>>;
    type Ret = usize;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg {
        dataset
            .map(|line| line.chars().map(|ch| ch as i8).collect())
            .collect()
    }

    fn part_1(grid: Self::Arg) -> Self::Ret {
        let (m, n) = (grid.len(), grid[0].len());
        let mut visible_trees = BitSet::with_capacity(m * n);

        for x in 0..m {
            let mut prev_tree_height = -1;
            (0..n).for_each(|y| {
                (grid[x][y] > prev_tree_height).then(|| visible_trees.insert(x * n + y));
                prev_tree_height = i8::max(prev_tree_height, grid[x][y]);
            });

            let mut prev_tree_height = -1;
            (0..n).rev().for_each(|y| {
                (grid[x][y] > prev_tree_height).then(|| visible_trees.insert(x * n + y));
                prev_tree_height = i8::max(prev_tree_height, grid[x][y]);
            });
        }

        for y in 0..n {
            let mut prev_tree_height = -1;
            (0..m).for_each(|x| {
                (grid[x][y] > prev_tree_height).then(|| visible_trees.insert(x * n + y));
                prev_tree_height = i8::max(prev_tree_height, grid[x][y]);
            });

            let mut prev_tree_height = -1;
            (0..m).rev().for_each(|x| {
                (grid[x][y] > prev_tree_height).then(|| visible_trees.insert(x * n + y));
                prev_tree_height = i8::max(prev_tree_height, grid[x][y]);
            });
        }

        visible_trees.len()
    }

    fn part_2(grid: Self::Arg) -> Self::Ret {
        let (m, n) = (grid.len(), grid[0].len());
        let mut tree_scores = vec![1; m * n];
        let mut tree_stack = Vec::new();

        for x in 0..m {
            tree_stack.clear();
            (0..n).for_each(|y| {
                while let Some((tree, _)) = tree_stack.last() {
                    if grid[x][y] <= *tree {
                        break;
                    }
                    tree_stack.pop();
                }

                tree_scores[x * n + y] *= y - tree_stack.last().map_or_else(|| 0, |&(_, i)| i);
                tree_stack.push((grid[x][y], y));
            });

            tree_stack.clear();
            (0..n).rev().for_each(|y| {
                while let Some((tree, _)) = tree_stack.last() {
                    if grid[x][y] <= *tree {
                        break;
                    }
                    tree_stack.pop();
                }

                tree_scores[x * n + y] *= tree_stack.last().map_or_else(|| n - 1, |&(_, i)| i) - y;
                tree_stack.push((grid[x][y], y));
            });
        }

        for y in 0..n {
            tree_stack.clear();
            (0..m).for_each(|x| {
                while let Some((tree, _)) = tree_stack.last() {
                    if grid[x][y] <= *tree {
                        break;
                    }
                    tree_stack.pop();
                }

                tree_scores[x * n + y] *= x - tree_stack.last().map_or_else(|| 0, |&(_, i)| i);
                tree_stack.push((grid[x][y], x));
            });

            tree_stack.clear();
            (0..m).rev().for_each(|x| {
                while let Some((tree, _)) = tree_stack.last() {
                    if grid[x][y] <= *tree {
                        break;
                    }
                    tree_stack.pop();
                }

                tree_scores[x * n + y] *= tree_stack.last().map_or_else(|| n - 1, |&(_, i)| i) - x;
                tree_stack.push((grid[x][y], x));
            });
        }

        *tree_scores.iter().max().unwrap()
    }
}
