pub struct CalorieCounting;

impl crate::AdventDayProblem for CalorieCounting {
    type Arg = Vec<i128>;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg {
        let mut calories_table = Vec::new();
        let mut calories = 0;

        dataset
            .chain(std::iter::once("".to_string()))
            .for_each(|line| match line.is_empty() {
                true => {
                    calories_table.push(calories);
                    calories = 0;
                }
                _ => calories += line.parse::<i128>().unwrap(),
            });
        calories_table
    }

    fn part_1(calories_table: Self::Arg) -> i128 {
        *calories_table.iter().max().unwrap()
    }

    fn part_2(mut calories_table: Self::Arg) -> i128 {
        calories_table.sort();
        calories_table[calories_table.len() - 3..].iter().sum()
    }
}
