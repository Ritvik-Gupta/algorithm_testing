crate::solution!();
crate::linked_list_definition!();

struct LinkedCriticalPointer<'a> {
    prev_node: &'a Box<ListNode>,
    curr_node: &'a Box<ListNode>,
    next_node: Option<&'a Box<ListNode>>,
    idx: i32,
}

impl<'a> LinkedCriticalPointer<'a> {
    fn new(head: &'a Box<ListNode>) -> Self {
        let prev_node = head;
        let curr_node = prev_node.next.as_ref().unwrap();
        let next_node = curr_node.next.as_ref();

        Self {
            prev_node,
            curr_node,
            next_node,
            idx: 1,
        }
    }

    fn advance_pointer(&mut self) {
        let next_node = self.next_node.unwrap();
        self.prev_node = self.curr_node;
        self.curr_node = next_node;
        self.next_node = next_node.next.as_ref();
        self.idx += 1;
    }
}

use std::cmp::Ordering::Equal;

impl<'a> Iterator for LinkedCriticalPointer<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next_node) = self.next_node {
            let cmp_prev = self.curr_node.val.cmp(&self.prev_node.val);
            let cmp_next = self.curr_node.val.cmp(&next_node.val);

            if cmp_prev == cmp_next && cmp_next != Equal {
                let crit_point_found = self.idx;
                self.advance_pointer();
                return Some(crit_point_found);
            }
            self.advance_pointer();
        }
        None
    }
}

impl Solution {
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        let crit_points = LinkedCriticalPointer::new(head.as_ref().unwrap()).collect::<Vec<_>>();

        if crit_points.len() < 2 {
            return vec![-1, -1];
        }

        vec![
            crit_points
                .windows(2)
                .map(|crit_window| crit_window[1] - crit_window[0])
                .min()
                .unwrap(),
            *crit_points.last().unwrap() - *crit_points.first().unwrap(),
        ]
    }
}
