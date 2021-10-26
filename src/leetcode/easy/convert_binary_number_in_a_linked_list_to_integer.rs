pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

const BINARY_BASE: i32 = 2;

struct BinaryLink {
    sum: i32,
    power_position: u32,
}

impl BinaryLink {
    fn last_link(val: i32) -> Self {
        Self {
            sum: val,
            power_position: 0,
        }
    }

    fn next_link(&self, val: i32) -> Self {
        let power_position = self.power_position + 1;
        Self {
            sum: self.sum + val * BINARY_BASE.pow(power_position),
            power_position,
        }
    }
}

impl Solution {
    fn get_prev_binary_link(head: &Box<ListNode>) -> BinaryLink {
        match &head.next {
            Some(next) => Solution::get_prev_binary_link(next).next_link(head.val),
            None => BinaryLink::last_link(head.val),
        }
    }

    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        Solution::get_prev_binary_link(&head.unwrap()).sum
    }
}
