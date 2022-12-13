use once_cell::sync::Lazy;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::iter::Peekable;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Packet {
    List(Vec<Packet>),
    Integer(u8),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => a.partial_cmp(b),
            (Self::List(a), Self::List(b)) => a.partial_cmp(b),
            (Self::Integer(a), Self::List(b)) => vec![Self::Integer(*a)].partial_cmp(b),
            (Self::List(a), Self::Integer(b)) => a.partial_cmp(&vec![Self::Integer(*b)]),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

fn parse_packet_integer(data: &mut Peekable<impl Iterator<Item = char>>) -> u8 {
    let mut buffer = 0;
    while data.peek().map_or(false, |ch| ('0'..='9').contains(ch)) {
        buffer = buffer * 10 + (data.next().unwrap() as u8 - b'0');
    }
    buffer
}

fn parse_packet_list(data: &mut Peekable<impl Iterator<Item = char>>) -> Vec<Packet> {
    let mut list = Vec::new();
    while let Some(&ch) = data.peek() {
        match ch {
            ']' => {
                data.next();
                break;
            }
            ',' => {
                data.next();
            }
            '[' => {
                data.next();
                list.push(Packet::List(parse_packet_list(data)));
            }
            '0'..='9' => {
                list.push(Packet::Integer(parse_packet_integer(data)));
            }
            _ => unreachable!(),
        }
    }
    list
}

macro_rules! pack {
    ($int: literal) => {
        Packet::Integer($int)
    };
    ([ $($val: tt),*]) => {{
        let mut packet = Vec::new();
        $(
            packet.push(pack!($val));
        )*
        Packet::List(packet)
    }};
}

// static DIVIDER_MARKER_A: Packet = Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]);
// static DIVIDER_MARKER_B: Packet = Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]);

const DIVIDER_MARKER_A: Lazy<Packet> = Lazy::new(|| pack!([[2]]));
const DIVIDER_MARKER_B: Lazy<Packet> = Lazy::new(|| pack!([[6]]));

pub struct DistressSignal;

impl crate::AdventDayProblem for DistressSignal {
    type Arg = Vec<Packet>;
    type Ret = usize;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg {
        dataset
            .filter(|line| !line.is_empty())
            .map(|data| {
                let mut data = data.chars().peekable();
                data.next();
                Packet::List(parse_packet_list(&mut data))
            })
            .collect()
    }

    fn part_1(packets: Self::Arg) -> Self::Ret {
        packets
            .chunks(2)
            .zip(1..)
            .map(|(packet_pairs, idx)| {
                (packet_pairs[0].partial_cmp(&packet_pairs[1]).unwrap() == std::cmp::Ordering::Less)
                    .then_some(idx)
                    .unwrap_or(0)
            })
            .sum()
    }

    fn part_2(mut packets: Self::Arg) -> Self::Ret {
        packets.push(DIVIDER_MARKER_A.clone());
        packets.push(DIVIDER_MARKER_B.clone());

        packets.sort();

        packets
            .iter()
            .zip(1..)
            .filter_map(|(packet, idx)| {
                (packet == &DIVIDER_MARKER_A.to_owned() || packet == &DIVIDER_MARKER_B.to_owned())
                    .then_some(idx)
            })
            .product()
    }
}
