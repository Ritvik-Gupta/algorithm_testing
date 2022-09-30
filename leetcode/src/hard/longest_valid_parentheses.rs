crate::solution!();

const UNDEFINED_CHAR_TOKEN: u8 = b'*';

struct StrPtr {
    ptr: *const u8,
    size: usize,
}

impl StrPtr {
    fn build(str: &String) -> Self {
        Self {
            ptr: str.as_ptr(),
            size: str.len(),
        }
    }

    fn at(&self, pos: usize) -> u8 {
        if pos >= self.size {
            return UNDEFINED_CHAR_TOKEN;
        }
        unsafe { *self.ptr.wrapping_offset(pos as isize) }
    }
}

impl Solution {
    pub fn longest_valid_parentheses(str: String) -> i32 {
        let mut max_result = 0;
        let mut dp = vec![0; str.len()];

        let str_ptr = StrPtr::build(&str);
        for i in 1..str.len() {
            if str_ptr.at(i) == b')' {
                if str_ptr.at(i - 1) == b'(' {
                    dp[i] = 2 + *dp.get(i - 2).unwrap_or(&0);
                } else if str_ptr.at(i - dp[i - 1] - 1) == b'(' {
                    dp[i] = 2 + dp[i - 1] + *dp.get(i - dp[i - 1] - 2).unwrap_or(&0);
                }
            }
            max_result = i32::max(max_result, dp[i] as i32);
        }
        max_result
    }
}
