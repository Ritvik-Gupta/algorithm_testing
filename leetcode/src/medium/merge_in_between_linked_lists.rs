crate::solution!();
crate::linked_list_definition!();

impl Solution {
    pub fn merge_in_between(
        mut list1: Option<Box<ListNode>>,
        a: i32,
        b: i32,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut idx = 0;
        let mut cn1 = list1.as_mut().unwrap();

        while idx + 1 < a {
            cn1 = cn1.next.as_mut().unwrap();
            idx += 1;
        }
        let mut unwanted_list = std::mem::replace(&mut cn1.next, list2);
        idx += 1;

        let mut cn2 = unwanted_list.as_mut().unwrap();
        while idx < b {
            cn2 = cn2.next.as_mut().unwrap();
            idx += 1;
        }
        let unwanted_end_node = std::mem::replace(&mut cn2.next, None);

        let mut end_node = cn1.next.as_mut().unwrap();
        while end_node.next.is_some() {
            end_node = end_node.next.as_mut().unwrap();
        }
        end_node.next = unwanted_end_node;

        list1
    }
}
