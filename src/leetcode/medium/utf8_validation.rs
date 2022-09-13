crate::leetcode::solution!();

const BYTE_MSB_OFFSET: i32 = 1 << 7;

fn num_bytes_of_sequence(byte: i32) -> Option<usize> {
    let mut bit_pos = BYTE_MSB_OFFSET;
    let mut num_bits = 0;

    while byte & bit_pos != 0 && num_bits < 5 {
        num_bits += 1;
        bit_pos >>= 1;
    }

    match num_bits {
        1 | 5.. => None,
        _ => Some(num_bits),
    }
}

fn is_part_of_n_byte_sequence(byte: i32) -> bool {
    byte & BYTE_MSB_OFFSET != 0 && byte & (BYTE_MSB_OFFSET >> 1) == 0
}

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut data_itr = data.iter();

        while let Some(&byte) = data_itr.next() {
            match num_bytes_of_sequence(byte) {
                Some(byte_seq_size) => {
                    for _ in 1..byte_seq_size {
                        match data_itr.next() {
                            None => return false,
                            Some(&byte) if !is_part_of_n_byte_sequence(byte) => return false,
                            _ => {}
                        }
                    }
                }
                None => return false,
            }
        }

        true
    }
}
