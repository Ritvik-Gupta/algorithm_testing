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

pub struct HexReader {
    dataset: Vec<bool>,
    read_idx: usize,
}

impl HexReader {
    fn from_raw_hex(hex_dataset: impl Iterator<Item = char>) -> Self {
        Self {
            dataset: hex_dataset
                .flat_map(|token| hex_to_binary(token as u8))
                .collect(),
            read_idx: 0,
        }
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

    fn recur_compute_packet(&mut self) -> i128 {
        let version = self.read_n(3);
        let packet_type = self.read_n(3);

        match packet_type {
            4 => {
                let mut literal_value = 0;

                let mut start_bit = 1;
                while start_bit != 0 {
                    start_bit = self.read_n(1);
                    literal_value = literal_value << 4 | self.read_n(4);
                }

                version as i128
            }
            _ => {
                let length_id = self.read_n(1);

                let children_version_sum: i128 = match length_id {
                    0 => {
                        let bit_lengths = self.read_n(15);
                        let start_idx = self.read_idx;

                        (0..)
                            .scan(0, |_, _| {
                                if self.read_idx - start_idx < bit_lengths {
                                    let child_result = self.recur_compute_packet();
                                    Some(child_result)
                                } else {
                                    None
                                }
                            })
                            .sum()
                    }
                    _ => {
                        let num_subpackets = self.read_n(11);
                        (0..num_subpackets)
                            .map(|_| self.recur_compute_packet())
                            .sum()
                    }
                };

                version as i128 + children_version_sum
            }
        }
    }
}

pub struct PacketDecoder;

impl super::AdventDayProblem for PacketDecoder {
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
        hex_reader.recur_compute_packet()
    }

    fn part_2(arg: Self::Arg) -> i128 {
        todo!()
    }
}
