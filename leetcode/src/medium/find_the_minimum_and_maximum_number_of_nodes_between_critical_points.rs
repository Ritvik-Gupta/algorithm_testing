crate::solution!();
crate::linked_list_definition!();

struct LinkedCriticalPointer<'a> {
    prev_node: &'a Box<ListNode>,
    curr_node: &'a Box<ListNode>,
    idx: i32,
}

impl<'a> LinkedCriticalPointer<'a> {
    fn new(head: &'a Box<ListNode>) -> Self {
        let prev_node = head;
        let curr_node = prev_node.next.as_ref().unwrap();

        Self {
            prev_node,
            curr_node,
            idx: 1,
        }
    }

    fn advance_pointer(&mut self, next_node: &'a Box<ListNode>) {
        self.prev_node = self.curr_node;
        self.curr_node = next_node;
        self.idx += 1;
    }
}

use std::cmp::Ordering::Equal;

impl<'a> Iterator for LinkedCriticalPointer<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next_node) = &self.curr_node.next {
            let cmp_prev = self.curr_node.val.cmp(&self.prev_node.val);
            let cmp_next = self.curr_node.val.cmp(&next_node.val);

            if cmp_prev == cmp_next && cmp_next != Equal {
                let crit_point_found = self.idx;
                self.advance_pointer(next_node);
                return Some(crit_point_found);
            }
            self.advance_pointer(next_node);
        }
        None
    }
}

fn find_crit_dist_pair(crit_pointer: &mut LinkedCriticalPointer) -> Option<Vec<i32>> {
    let first_point = crit_pointer.next()?;
    let mut curr_point = crit_pointer.next()?;
    let mut min_dist = curr_point - first_point;

    for point in crit_pointer {
        min_dist = i32::min(min_dist, point - curr_point);
        curr_point = point;
    }

    Some(vec![min_dist, curr_point - first_point])
}

impl Solution {
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        find_crit_dist_pair(&mut LinkedCriticalPointer::new(head.as_ref().unwrap()))
            .unwrap_or(vec![-1, -1])
    }
}
