use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{anychar, char as character, i64 as integer},
    combinator::map,
    sequence::{separated_pair, tuple},
    IResult, Parser,
};
use std::collections::HashMap;
use Operator::*;

structstruck::strike! {
    pub enum MonkeyOperations {
        Number(i64),
        DependsOn {
            monkey_a: String,
            operator:#[derive(Clone)] enum {
                PLUS,
                MINUS,
                TIMES,
                DIVIDE
            },
            monkey_b: String,
        }
    }
}

impl Operator {
    fn solve(&self, a: i64, b: i64) -> i64 {
        match self {
            PLUS => a + b,
            MINUS => a - b,
            TIMES => a * b,
            DIVIDE => a / b,
        }
    }

    fn reverse(&self) -> Self {
        match self {
            PLUS => MINUS,
            MINUS => PLUS,
            TIMES => DIVIDE,
            DIVIDE => TIMES,
        }
    }
}

impl From<char> for Operator {
    fn from(opr: char) -> Self {
        match opr {
            '+' => PLUS,
            '-' => MINUS,
            '*' => TIMES,
            '/' => DIVIDE,
            _ => unreachable!(),
        }
    }
}

fn parse_monkey_id(input: &str) -> IResult<&str, &str> {
    take(4usize)(input)
}

impl MonkeyOperations {
    fn parse(input: &str) -> IResult<&str, (String, Self)> {
        separated_pair(
            parse_monkey_id.map(|monkey_id| monkey_id.to_string()),
            tag(": "),
            alt((
                map(integer, |num| MonkeyOperations::Number(num)),
                map(
                    tuple((
                        parse_monkey_id,
                        character(' '),
                        anychar,
                        character(' '),
                        parse_monkey_id,
                    )),
                    |(monkey_a, _, operation, _, monkey_b)| MonkeyOperations::DependsOn {
                        monkey_a: monkey_a.to_string(),
                        operator: operation.into(),
                        monkey_b: monkey_b.to_string(),
                    },
                ),
            )),
        )(input)
    }
}

fn solve_for_monkey(monkey_id: &String, monkeys: &HashMap<String, MonkeyOperations>) -> i64 {
    match &monkeys[monkey_id] {
        MonkeyOperations::Number(num) => *num,
        MonkeyOperations::DependsOn {
            monkey_a,
            operator,
            monkey_b,
        } => operator.solve(
            solve_for_monkey(monkey_a, monkeys),
            solve_for_monkey(monkey_b, monkeys),
        ),
    }
}

#[derive(Default)]
enum Expression {
    Solved(i64),
    Unsolved {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
    #[default]
    Variable,
}

fn expression_for_monkey(
    monkey_id: &String,
    monkeys: &HashMap<String, MonkeyOperations>,
) -> Expression {
    if monkey_id == "humn" {
        return Expression::Variable;
    }

    match &monkeys[monkey_id] {
        MonkeyOperations::Number(num) => Expression::Solved(*num),
        MonkeyOperations::DependsOn {
            monkey_a,
            operator,
            monkey_b,
        } => {
            let left = expression_for_monkey(monkey_a, monkeys);
            let right = expression_for_monkey(monkey_b, monkeys);

            match (&left, &right) {
                (Expression::Solved(left_ans), Expression::Solved(right_ans)) => {
                    Expression::Solved(operator.solve(*left_ans, *right_ans))
                }
                _ => Expression::Unsolved {
                    left: Box::new(left),
                    operator: operator.clone(),
                    right: Box::new(right),
                },
            }
        }
    }
}

pub struct MonkeyMath;

impl crate::AdventDayProblem for MonkeyMath {
    type Arg = HashMap<String, MonkeyOperations>;
    type Ret = i64;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg {
        dataset
            .map(|line| MonkeyOperations::parse(&line).unwrap().1)
            .collect()
    }

    fn part_1(monkeys: Self::Arg) -> Self::Ret {
        solve_for_monkey(&"root".to_string(), &monkeys)
    }

    fn part_2(monkeys: Self::Arg) -> Self::Ret {
        let Expression::Unsolved {
            left: mut unsolved_expr,
            operator: _,
            right: mut solved_expr,
        } = expression_for_monkey(&"root".to_string(), &monkeys) else { unreachable!() };

        if let Expression::Solved(_) = unsolved_expr.as_ref() {
            std::mem::swap(&mut unsolved_expr, &mut solved_expr);
        }

        let Expression::Solved(mut variable_value) = *solved_expr else { unreachable!() };
        loop {
            if let Expression::Variable = unsolved_expr.as_ref() {
                break;
            }

            let Expression::Unsolved {
                left,
                operator,
                right,
            } = std::mem::take(&mut *unsolved_expr) else { unreachable!() };

            if let Expression::Solved(val) = right.as_ref() {
                variable_value = operator.reverse().solve(variable_value, *val);
                unsolved_expr = left;
            } else if let Expression::Solved(val) = left.as_ref() {
                variable_value = match operator {
                    MINUS | DIVIDE => operator.solve(*val, variable_value),
                    _ => operator.reverse().solve(variable_value, *val),
                };
                unsolved_expr = right;
            }
        }
        variable_value
    }
}
