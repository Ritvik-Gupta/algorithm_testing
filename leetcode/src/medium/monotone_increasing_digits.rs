crate::solution!();

impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut num_str = n.to_string();
        let num = unsafe { num_str.as_bytes_mut() };
        let mut invers_idx = usize::MAX;

        for i in (0..num.len() - 1).rev() {
            if num[i] > num[i + 1] || (invers_idx != usize::MAX && num[invers_idx] == num[i]) {
                invers_idx = i;
            }
        }

        match invers_idx {
            usize::MAX => return n,
            _ => {
                (invers_idx + 1..num.len()).for_each(|i| num[i] = b'9');
                num[invers_idx] -= 1;

                num_str.parse().unwrap()
            }
        }
    }
}
