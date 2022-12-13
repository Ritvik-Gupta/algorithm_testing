use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char as character,
    character::complete::{anychar, i128 as int128},
    multi::separated_list0,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};
use std::{cmp::Reverse, collections::VecDeque};
use Operation::*;

structstruck::strike! {
    pub struct Monkey {
        holding_items: VecDeque<i128>,
        operation_seq: struct OperationSequence {
            operation: enum {
                PLUS,
                TIMES,
            },
            value: enum {
                Constant(i128),
                This,
            }
        },
        test: struct TestSuite {
            modulo: i128,
            pass_to: usize,
            fail_to: usize,
        },
        num_inspections: u128,
    }
}

impl OperationSequence {
    fn apply_on(&self, num: i128) -> i128 {
        let value = match self.value {
            Value::Constant(value) => value,
            Value::This => num,
        };

        match self.operation {
            PLUS => num + value,
            TIMES => num * value,
        }
    }
}

impl Monkey {
    fn from_args(monkey: &Vec<String>) -> IResult<&str, Self> {
        let (_, holding_items) = preceded(
            tag("  Starting items: "),
            separated_list0(tag(", "), int128),
        )(monkey[1].as_str())?;

        let (_, operation_seq) = preceded(
            tag("  Operation: new = old "),
            separated_pair(
                anychar.map(|opr| match opr {
                    '+' => PLUS,
                    '*' => TIMES,
                    _ => unreachable!(),
                }),
                character(' '),
                alt((
                    tag("old").map(|_| Value::This),
                    int128.map(|value| Value::Constant(value)),
                )),
            ),
        )(monkey[2].as_str())?;

        let (_, modulo) = preceded(tag("  Test: divisible by "), int128)(monkey[3].as_str())?;

        let (_, pass_to) = preceded(
            tag("    If true: throw to monkey "),
            int128.map(|n| n as usize),
        )(monkey[4].as_str())?;

        let (_, fail_to) = preceded(
            tag("    If false: throw to monkey "),
            int128.map(|n| n as usize),
        )(monkey[5].as_str())?;

        Ok((
            "",
            Monkey {
                holding_items: holding_items.into_iter().collect(),
                operation_seq: OperationSequence {
                    operation: operation_seq.0,
                    value: operation_seq.1,
                },
                test: TestSuite {
                    modulo,
                    pass_to,
                    fail_to,
                },
                num_inspections: 0,
            },
        ))
    }

    fn simulate_inspections<const ITERATIONS: usize, const RELIEF: i128>(
        monkeys: &mut Vec<Monkey>,
    ) {
        let total_worry_modulo = monkeys
            .iter()
            .map(|monkey| monkey.test.modulo)
            .product::<i128>();

        for _ in 0..ITERATIONS {
            for i in 0..monkeys.len() {
                let num_items_holding = monkeys[i].holding_items.len();
                for _ in 0..num_items_holding {
                    let mut item = monkeys[i].holding_items.pop_front().unwrap();
                    item = monkeys[i].operation_seq.apply_on(item) % total_worry_modulo;
                    item /= RELIEF;

                    let throw_item_to = match item % monkeys[i].test.modulo {
                        0 => monkeys[i].test.pass_to,
                        _ => monkeys[i].test.fail_to,
                    };

                    monkeys[throw_item_to].holding_items.push_back(item);
                }
                monkeys[i].num_inspections += num_items_holding as u128;
            }
        }
    }
}

pub struct MonkeyInTheMiddle;

impl crate::AdventDayProblem for MonkeyInTheMiddle {
    type Arg = Vec<Monkey>;
    type Ret = u128;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg {
        dataset
            .chunks(7)
            .into_iter()
            .map(|itr| Monkey::from_args(&itr.collect_vec()).unwrap().1)
            .collect()
    }

    fn part_1(mut monkeys: Self::Arg) -> Self::Ret {
        Monkey::simulate_inspections::<20, 3>(&mut monkeys);
        monkeys.sort_by_key(|monkey| Reverse(monkey.num_inspections));
        monkeys[0].num_inspections * monkeys[1].num_inspections
    }

    fn part_2(mut monkeys: Self::Arg) -> Self::Ret {
        Monkey::simulate_inspections::<10000, 1>(&mut monkeys);
        monkeys.sort_by_key(|monkey| Reverse(monkey.num_inspections));
        monkeys[0].num_inspections * monkeys[1].num_inspections
    }
}
