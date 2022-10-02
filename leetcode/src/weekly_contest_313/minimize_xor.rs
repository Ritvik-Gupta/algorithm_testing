crate::solution!();

fn count_set_bits(mut num: usize) -> usize {
    let mut num_set_bits = 0;
    while num > 0 {
        num_set_bits += num & 1;
        num >>= 1;
    }
    num_set_bits
}

use std::cmp::Ordering::*;

impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let mut num = num1 as usize;
        let (num_set_bits, available_set_bits) =
            (count_set_bits(num), count_set_bits(num2 as usize));
        let is_less_order = available_set_bits.cmp(&num_set_bits) == Less;

        let mut remaining_set_bits = if is_less_order {
            num_set_bits - available_set_bits
        } else {
            available_set_bits - num_set_bits
        };

        let mut bit_offset = 1;
        let mut result = 0;

        let seeing_bit = if is_less_order { 1 } else { 0 };
        while remaining_set_bits > 0 {
            if num & 1 == seeing_bit {
                result |= bit_offset;
                remaining_set_bits -= 1;
            }
            num >>= 1;
            bit_offset <<= 1;
        }

        if is_less_order {
            num1 ^ result
        } else {
            num1 | result
        }
    }
}
