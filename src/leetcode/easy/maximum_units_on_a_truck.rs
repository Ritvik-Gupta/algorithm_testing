crate::leetcode::solution!();

impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, mut truck_size: i32) -> i32 {
        box_types.sort_by_key(|box_info| -box_info[1]);

        let mut total_units = 0;
        for box_info in box_types {
            if truck_size == 0 {
                break;
            }
            let boxes_taken = i32::min(box_info[0], truck_size);
            total_units += boxes_taken * box_info[1];
            truck_size -= boxes_taken;
        }
        total_units
    }
}
