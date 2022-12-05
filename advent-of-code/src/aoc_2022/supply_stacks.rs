use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char as character, u32 as parse_into_u32},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Clone, Debug)]
pub struct CargoStack(Vec<char>);

#[derive(Debug)]
pub struct MoveOperation {
    quantity: usize,
    from: usize,
    to: usize,
}

fn parse_stack_size(input: &str) -> IResult<&str, usize> {
    let (input, stack) = separated_list1(
        character(' '),
        delimited(character(' '), anychar, character(' ')),
    )(input)?;
    Ok((input, stack.len()))
}

fn parse_cargo_on_stacks(input: &str) -> IResult<&str, Vec<char>> {
    let (input, cargo_added) = separated_list1(
        character(' '),
        alt((
            delimited(character('['), anychar, character(']')),
            preceded(tag("  "), character(' ')),
        )),
    )(input)?;

    Ok((input, cargo_added))
}

fn parse_move_operation(input: &str) -> IResult<&str, MoveOperation> {
    let (input, (_, quantity, _, from, _, to)) = tuple((
        tag("move "),
        parse_into_u32,
        tag(" from "),
        parse_into_u32,
        tag(" to "),
        parse_into_u32,
    ))(input)?;

    Ok((
        input,
        MoveOperation {
            quantity: quantity as usize,
            from: from as usize,
            to: to as usize,
        },
    ))
}

pub struct SupplyStacks;

impl crate::AdventDayProblem for SupplyStacks {
    type Arg = (Vec<CargoStack>, Vec<MoveOperation>);
    type Ret = String;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(mut dataset: impl Iterator<Item = String>) -> Self::Arg {
        let stack_dataset = dataset
            .by_ref()
            .take_while(|line| !line.is_empty())
            .collect::<Vec<_>>();
        let mut stack_dataset = stack_dataset.iter().rev();

        let mut stacks = vec![
            CargoStack(Vec::new());
            parse_stack_size(&stack_dataset.next().unwrap()).unwrap().1
        ];

        for line in stack_dataset {
            let (_, cargo_added) = parse_cargo_on_stacks(line).unwrap();

            cargo_added
                .into_iter()
                .enumerate()
                .filter(|(_, c)| !c.is_ascii_whitespace())
                .for_each(|(idx, c)| stacks[idx].0.push(c));
        }

        let move_oprs = dataset
            .map(|line| parse_move_operation(line.as_str()).unwrap().1)
            .collect();

        (stacks, move_oprs)
    }

    fn part_1((mut stacks, move_oprs): Self::Arg) -> Self::Ret {
        move_oprs.iter().for_each(|move_operation| {
            (0..move_operation.quantity).for_each(|_| {
                let cargo = stacks[move_operation.from - 1].0.pop().unwrap();
                stacks[move_operation.to - 1].0.push(cargo);
            });
        });

        stacks
            .iter()
            .map(|cargo_stack| *cargo_stack.0.last().unwrap())
            .collect()
    }

    fn part_2((mut stacks, move_oprs): Self::Arg) -> Self::Ret {
        move_oprs.iter().for_each(|move_operation| {
            let to_stack: *mut _ = &mut stacks[move_operation.to - 1];
            let from_stack = &mut stacks[move_operation.from - 1];

            unsafe { &mut *to_stack }
                .0
                .extend_from_slice(&from_stack.0[from_stack.0.len() - move_operation.quantity..]);
            from_stack
                .0
                .truncate(from_stack.0.len() - move_operation.quantity);
        });

        stacks
            .iter()
            .map(|cargo_stack| *cargo_stack.0.last().unwrap())
            .collect()
    }
}
