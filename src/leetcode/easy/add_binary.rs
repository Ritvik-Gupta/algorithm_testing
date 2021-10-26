pub struct Solution;

const ZERO: u8 = b'0';

impl Solution {
    pub fn add_binary(mut a: String, mut b: String) -> String {
        use std::collections::LinkedList;

        if a.len() < b.len() {
            std::mem::swap(&mut a, &mut b);
        }

        let mut result = LinkedList::<char>::new();
        let mut carry = 0;
        let mut x = b
            .bytes()
            .map(|byte| byte - ZERO)
            .rev()
            .chain(std::iter::repeat(0).take(a.len() - b.len()))
            .zip(a.bytes().map(|byte| byte - ZERO).rev());
        while let Some((ch_a, ch_b)) = x.next() {
            let sum = ch_a + ch_b + carry;
            carry = sum / 2;
            result.push_front((sum % 2 + ZERO) as char);
        }
        if carry == 1 {
            result.push_front('1');
        }
        result.into_iter().collect()
    }
}
