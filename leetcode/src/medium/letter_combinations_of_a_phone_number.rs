crate::solution!();

static DIGIT_MAPPING: [&[u8]; 8] = [
    b"abc", b"def", b"ghi", b"jkl", b"mno", b"pqrs", b"tuv", b"wxyz",
];

const AVG_MAPPING_SIZE: usize = 3;

struct PhoneNumberBuilder {
    combinations: Vec<String>,
    cumulative_build: Vec<u8>,
}

impl PhoneNumberBuilder {
    fn build_for(phone_num: String) -> Self {
        let mut builder = Self {
            combinations: Vec::with_capacity(AVG_MAPPING_SIZE.pow(phone_num.len() as u32)),
            cumulative_build: Vec::with_capacity(phone_num.len()),
        };
        builder.branch_add_digit_mapping(phone_num.as_bytes());
        builder
    }

    fn branch_add_digit_mapping(&mut self, digits: &[u8]) {
        match digits.split_first() {
            Some((first_digit, rest_digits)) => DIGIT_MAPPING[(first_digit - b'2') as usize]
                .iter()
                .for_each(|&mapped_char| {
                    self.cumulative_build.push(mapped_char);
                    self.branch_add_digit_mapping(rest_digits);
                    self.cumulative_build.pop();
                }),
            None => self
                .combinations
                .push(String::from_utf8(self.cumulative_build.clone()).unwrap()),
        }
    }
}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits == "" {
            return vec![];
        }
        PhoneNumberBuilder::build_for(digits).combinations
    }
}
