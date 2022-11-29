crate::solution!();

fn mapping_of_num(mut num: i32, mapping: &Vec<i32>) -> i32 {
    if num == 0 {
        return mapping[0];
    }

    let mut mapped_num = 0;
    let mut base_offset = 1;

    while num > 0 {
        let digit = (num % 10) as usize;
        mapped_num += mapping[digit] * base_offset;

        base_offset *= 10;
        num /= 10;
    }
    mapped_num
}

impl Solution {
    pub fn sort_jumbled(mapping: Vec<i32>, mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_by_key(|&num| mapping_of_num(num, &mapping));
        nums
    }
}
