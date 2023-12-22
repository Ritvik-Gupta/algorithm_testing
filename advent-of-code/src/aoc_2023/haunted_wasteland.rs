use derive_more::Deref;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

const CONNECTION_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?<node>...) = \((?<left>...), (?<right>...)\)").unwrap());

#[derive(Hash, PartialEq, Eq, Clone, Copy, Deref)]
pub struct Node([char; 3]);

impl Node {
    fn construct(name: &str) -> Self {
        let mut name = name.chars();
        Self([
            name.next().unwrap(),
            name.next().unwrap(),
            name.next().unwrap(),
        ])
    }
}

pub struct TerrainNavigator {
    commands: String,
    network: HashMap<Node, [Node; 2]>,
}

impl TerrainNavigator {
    fn traverse_network_to_end(&self, mut curr_node: Node) -> usize {
        1 + self
            .commands
            .chars()
            .cycle()
            .take_while(|&c| {
                let direction = (c == 'R') as usize;
                curr_node = self.network[&curr_node][direction];
                curr_node[2] != 'Z'
            })
            .count()
    }
}

pub struct HauntedWasteland;

impl crate::AdventDayProblem for HauntedWasteland {
    type Arg = TerrainNavigator;

    type Ret = usize;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(mut dataset: impl Iterator<Item = String>) -> Self::Arg {
        let commands = dataset.next().unwrap();
        dataset.next().unwrap();

        let network = dataset
            .map(|line| {
                let caps = CONNECTION_REGEX.captures(&line).unwrap();
                (
                    Node::construct(&caps["node"]),
                    [
                        Node::construct(&caps["left"]),
                        Node::construct(&caps["right"]),
                    ],
                )
            })
            .collect();

        TerrainNavigator { commands, network }
    }

    fn part_1(terrain: Self::Arg) -> Self::Ret {
        terrain.traverse_network_to_end(Node::construct("AAA"))
    }

    fn part_2(terrain: Self::Arg) -> Self::Ret {
        let initial_nodes = terrain
            .network
            .keys()
            .filter(|node| node[2] == 'A')
            .cloned()
            .collect_vec();

        // This specifically works because all 'initial nodes' have circular paths,
        // and are disjointed so would follow the same circular path.
        let cycle_lengths = initial_nodes
            .into_iter()
            .map(|node| terrain.traverse_network_to_end(node));

        cycle_lengths.reduce(num::integer::lcm).unwrap()
    }
}
