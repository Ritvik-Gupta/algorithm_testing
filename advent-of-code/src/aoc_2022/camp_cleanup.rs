pub struct CampCleanup;

#[derive(PartialEq, Eq)]
pub struct SectionRange {
    start: u16,
    end: u16,
}

impl crate::AdventDayProblem for CampCleanup {
    type Arg = Vec<(SectionRange, SectionRange)>;
    type Ret = i128;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg {
        dataset
            .map(|line| {
                let (elf1, elf2) = line.split_once(',').unwrap();
                let elf1 = elf1.split_once('-').unwrap();
                let elf2 = elf2.split_once('-').unwrap();

                (
                    SectionRange {
                        start: elf1.0.parse().unwrap(),
                        end: elf1.1.parse().unwrap(),
                    },
                    SectionRange {
                        start: elf2.0.parse().unwrap(),
                        end: elf2.1.parse().unwrap(),
                    },
                )
            })
            .collect()
    }

    fn part_1(cleanup_tasks: Self::Arg) -> Self::Ret {
        cleanup_tasks
            .iter()
            .filter(|(elf1, elf2)| {
                let shared_work = SectionRange {
                    start: u16::max(elf1.start, elf2.start),
                    end: u16::min(elf1.end, elf2.end),
                };
                &shared_work == elf1 || &shared_work == elf2
            })
            .count() as i128
    }

    fn part_2(cleanup_tasks: Self::Arg) -> Self::Ret {
        cleanup_tasks
            .iter()
            .filter(|(elf1, elf2)| {
                (elf1.start <= elf2.start && elf2.start <= elf1.end)
                    || (elf2.start <= elf1.start && elf1.start <= elf2.end)
            })
            .count() as i128
    }
}
