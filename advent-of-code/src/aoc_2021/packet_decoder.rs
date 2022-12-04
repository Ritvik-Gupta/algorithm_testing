use std::cmp::Ordering::{self, *};
use Operator::*;
use PacketType::*;

enum Operator {
    Sum,
    Prod,
    Min,
    Max,
}

enum PacketType {
    WithOperator(Operator),
    RequiredOrdering(Ordering),
}

impl PacketType {
    fn solve(&self, children_values: Vec<i128>) -> i128 {
        match self {
            WithOperator(Sum) => children_values.iter().sum(),
            WithOperator(Prod) => children_values.iter().product(),
            WithOperator(Min) => *children_values.iter().min().unwrap(),
            WithOperator(Max) => *children_values.iter().max().unwrap(),
            RequiredOrdering(ordering) => {
                i128::from(children_values[0].cmp(&children_values[1]) == *ordering)
            }
        }
    }
}

pub struct HexReader {
    dataset: Vec<bool>,
    read_idx: usize,
    is_version_sum: bool,
}

impl HexReader {
    fn from_raw_hex(hex_dataset: impl Iterator<Item = char>) -> Self {
        fn hex_to_binary(token: u8) -> [bool; 4] {
            let binary_format = match token {
                b'0'..=b'9' => token - b'0',
                b'A'..=b'F' => token - b'A' + 10,
                _ => unreachable!(),
            };

            [
                binary_format & 1 << 3 != 0,
                binary_format & 1 << 2 != 0,
                binary_format & 1 << 1 != 0,
                binary_format & 1 << 0 != 0,
            ]
        }

        Self {
            dataset: hex_dataset
                .map(|token| token as u8)
                .flat_map(hex_to_binary)
                .collect(),
            read_idx: 0,
            is_version_sum: false,
        }
    }

    fn switch_to_version_sum(&mut self) -> &mut Self {
        self.is_version_sum = true;
        self
    }

    fn read(&mut self) -> usize {
        let bin = self.dataset[self.read_idx];
        self.read_idx += 1;
        bin as usize
    }

    fn read_n(&mut self, num_bits: u8) -> usize {
        let mut bin_packet = 0;
        for _ in 0..num_bits {
            bin_packet = bin_packet << 1 | self.read();
        }
        bin_packet
    }

    fn recur_compute_packet_value(&mut self) -> i128 {
        let version = self.read_n(3) as i128;
        let packet_type = self.read_n(3);

        match packet_type {
            4 => {
                // Literal Value
                let mut literal_value = 0;

                let mut start_bit = 1;
                while start_bit != 0 {
                    start_bit = self.read_n(1);
                    literal_value = literal_value << 4 | self.read_n(4);
                }

                match self.is_version_sum {
                    true => version,
                    _ => literal_value as i128,
                }
            }
            _ => {
                let packet_type = match packet_type {
                    _ if self.is_version_sum => WithOperator(Sum),
                    0 => WithOperator(Sum),
                    1 => WithOperator(Prod),
                    2 => WithOperator(Min),
                    3 => WithOperator(Max),
                    5 => RequiredOrdering(Greater),
                    6 => RequiredOrdering(Less),
                    7 => RequiredOrdering(Equal),
                    _ => unreachable!(),
                };
                let length_id = self.read_n(1);

                let mut children_values = Vec::new();
                match length_id {
                    0 => {
                        let bit_lengths = self.read_n(15);
                        let start_idx = self.read_idx;
                        while self.read_idx - start_idx < bit_lengths {
                            children_values.push(self.recur_compute_packet_value());
                        }
                    }
                    _ => {
                        let num_subpackets = self.read_n(11);
                        for _ in 0..num_subpackets {
                            children_values.push(self.recur_compute_packet_value());
                        }
                    }
                }

                packet_type.solve(children_values) + if self.is_version_sum { version } else { 0 }
            }
        }
    }
}

pub struct PacketDecoder;

impl crate::AdventDayProblem for PacketDecoder {
    type Arg = HexReader;

    fn get_problem_name() -> &'static str {
        file!()
            .split('\\')
            .last()
            .expect("has a file path")
            .split('.')
            .next()
            .expect("has a file name")
    }

    fn construct_arg(mut dataset: impl Iterator<Item = String>) -> Self::Arg {
        HexReader::from_raw_hex(dataset.next().expect("contains the hex string").chars())
    }

    fn part_1(mut hex_reader: Self::Arg) -> i128 {
        hex_reader
            .switch_to_version_sum()
            .recur_compute_packet_value()
    }

    fn part_2(mut hex_reader: Self::Arg) -> i128 {
        hex_reader.recur_compute_packet_value()
    }
}
