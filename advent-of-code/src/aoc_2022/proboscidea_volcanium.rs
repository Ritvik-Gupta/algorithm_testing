use bit_set::BitSet;
use derive_more::Deref;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::anychar,
    character::complete::u32 as integer,
    combinator::map,
    multi::separated_list0,
    sequence::{pair, preceded, separated_pair},
    IResult, Parser,
};
use std::collections::HashMap;

type ValveId = usize;

fn parse_valve_id(input: &str) -> IResult<&str, ValveId> {
    map(pair(anychar, anychar), |valve| {
        (valve.0 as u8 - b'A') as usize * 26 + (valve.1 as u8 - b'A') as usize
    })(input)
}

#[derive(Default, Clone)]
pub struct ValveNode {
    pressure: u32,
    tunnel_to_ids: Vec<ValveId>,
}

impl ValveNode {
    fn parse(input: &str) -> IResult<&str, (ValveId, Self)> {
        preceded(
            tag("Valve "),
            separated_pair(
                parse_valve_id,
                tag(" has flow rate="),
                alt((
                    separated_pair(
                        integer,
                        tag("; tunnels lead to valves "),
                        separated_list0(tag(", "), parse_valve_id),
                    ),
                    separated_pair(
                        integer,
                        tag("; tunnel leads to valve "),
                        parse_valve_id.map(|valve_id| vec![valve_id]),
                    ),
                ))
                .map(|(pressure, tunnel_ids)| Self {
                    pressure,
                    tunnel_to_ids: tunnel_ids,
                }),
            ),
        )(input)
    }
}

#[derive(Deref)]
pub struct ValveGraph(Vec<ValveNode>);

static mut START_VALVE: ValveId = 0;
static mut VOLCANO_TIMER: u32 = 30;

impl ValveGraph {
    fn find_max_pressure<const NUM_PLAYERS: u8>(&self) -> u32 {
        self.max_pressure_in(
            NUM_PLAYERS - 1,
            unsafe { START_VALVE },
            unsafe { VOLCANO_TIMER },
            &mut BitSet::new(),
            &mut HashMap::new(),
        )
    }

    fn max_pressure_in(
        &self,
        player_id: u8,
        valve_id: ValveId,
        timer: u32,
        open_valves: &mut BitSet,
        score_table: &mut HashMap<(ValveId, u32, BitSet, u8), u32>,
    ) -> u32 {
        if timer == 0 {
            return match player_id {
                0 => 0,
                _ => self.max_pressure_in(
                    player_id - 1,
                    unsafe { START_VALVE },
                    unsafe { VOLCANO_TIMER },
                    open_valves,
                    score_table,
                ),
            };
        }

        let dp_key = (valve_id, timer, open_valves.clone(), player_id);
        if let Some(&score) = score_table.get(&dp_key) {
            return score;
        }

        let score = u32::max(
            match !open_valves.contains(valve_id) {
                true if self[valve_id].pressure > 0 => {
                    let timer = timer - 1;
                    open_valves.insert(valve_id);
                    let score = self[valve_id].pressure * timer
                        + self.max_pressure_in(
                            player_id,
                            valve_id,
                            timer,
                            open_valves,
                            score_table,
                        );
                    open_valves.remove(valve_id);
                    score
                }
                _ => 0,
            },
            self[valve_id]
                .tunnel_to_ids
                .iter()
                .map(|&next_valve_id| {
                    self.max_pressure_in(
                        player_id,
                        next_valve_id,
                        timer - 1,
                        open_valves,
                        score_table,
                    )
                })
                .max()
                .unwrap(),
        );

        score_table.insert(dp_key, score);
        score
    }
}

pub struct ProboscideaVolcanium;

impl crate::AdventDayProblem for ProboscideaVolcanium {
    type Arg = ValveGraph;
    type Ret = u32;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg {
        let mut network = HashMap::new();
        {
            let mut alias_id = 0;
            dataset.for_each(|line| {
                let (_, (valve_id, node)) = ValveNode::parse(&line).unwrap();
                network.insert(valve_id, (alias_id, node));
                alias_id += 1;
            });
        }

        let mut graph = vec![ValveNode::default(); network.len()];

        network
            .iter()
            .for_each(|(valve_id, (alias_valve_id, node))| {
                graph[*alias_valve_id] = ValveNode {
                    pressure: node.pressure,
                    tunnel_to_ids: node
                        .tunnel_to_ids
                        .iter()
                        .map(|tunnel_to| network[tunnel_to].0)
                        .collect(),
                };

                if *valve_id == 0 {
                    unsafe { START_VALVE = *alias_valve_id };
                }
            });

        ValveGraph(graph)
    }

    fn part_1(graph: Self::Arg) -> Self::Ret {
        graph.find_max_pressure::<1>()
    }

    fn part_2(graph: Self::Arg) -> Self::Ret {
        unsafe { VOLCANO_TIMER = 26 };

        let timer = std::time::Instant::now();
        let res = graph.find_max_pressure::<2>();
        println!("{:?}", timer.elapsed());
        res
    }
}
