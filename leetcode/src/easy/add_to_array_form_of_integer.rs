crate::solution!();

struct NumWrapper(Vec<i32>);

impl NumWrapper {
    fn at(&self, index: isize) -> i32 {
        if index < 0 {
            return 0;
        }
        self.0[index as usize]
    }
}

impl Solution {
    pub fn add_to_array_form(num: Vec<i32>, mut k: i32) -> Vec<i32> {
        let (mut carry, mut result) = (0, std::collections::LinkedList::new());
        let (mut num_idx, wrapper) = ((num.len() - 1) as isize, NumWrapper(num));

        while num_idx >= 0 || k > 0 || carry == 1 {
            let sum = k % 10 + wrapper.at(num_idx) + carry;
            carry = sum / 10;
            result.push_front(sum % 10);
            k /= 10;
            num_idx -= 1;
        }
        result.into_iter().collect()
    }
}
