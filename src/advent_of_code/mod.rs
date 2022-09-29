pub mod binary_diagnostic;
pub mod dive;
pub mod dumbo_octopus;
pub mod giant_squid;
pub mod lanternfish;
pub mod smoke_basin;
pub mod sonar_sweep;
pub mod syntax_scoring;

trait AdventDayProblem {
    type Arg;

    fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg;
    fn part_1(arg: Self::Arg) -> i128;
    fn part_2(arg: Self::Arg) -> i128;

    fn get_problem_name() -> &'static str {
        file!()
            .split('\\')
            .last()
            .expect("has a file path")
            .split('.')
            .next()
            .expect("has a file name")
    }
}
