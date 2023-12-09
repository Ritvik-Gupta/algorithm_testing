use itertools::Itertools;

pub struct WaitForIt;

impl crate::AdventDayProblem for WaitForIt {
    type Arg = Vec<(u128, u128)>;

    type Ret = u128;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(mut dataset: impl Iterator<Item = String>) -> Self::Arg {
        let time_line = dataset.next().unwrap().replace("Time:", "");
        let dist_line = dataset.next().unwrap().replace("Distance:", "");

        let race_times = time_line.split(' ').filter_map(|x| x.parse().ok());
        let race_dists = dist_line.split(' ').filter_map(|x| x.parse().ok());

        race_times.zip(race_dists).collect()
    }

    fn part_1(races: Self::Arg) -> Self::Ret {
        races
            .iter()
            .map(|&(time, distance)| {
                (0..=time).filter(|&x| x * (time - x) > distance).count() as u128
            })
            .product()
    }

    fn part_2(races: Self::Arg) -> Self::Ret {
        let time: u128 = races.iter().map(|x| x.0).join("").parse().unwrap();
        let distance: u128 = races.iter().map(|x| x.1).join("").parse().unwrap();

        (0..=time).filter(|&x| x * (time - x) > distance).count() as u128
    }
}

/*
x + y = t
x * y > d

x * ( t - x ) > d
d + x2 < tx

x2 - tx + d < 0

x = ( t +- (t2 - 4d)^1/2 ) / 2
*/
