pub struct Solution;

const IMPOSSIBLE_ARR_ELEMENT: i32 = 20001;

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let abs_factor = |pos| match arr.get(pos) {
            Some(elm) => (elm - x).abs(),
            None => IMPOSSIBLE_ARR_ELEMENT,
        };

        let smallest_elm_pos = (0..arr.len()).min_by_key(|&pos| abs_factor(pos)).unwrap();
        let (mut lower, mut upper) = (smallest_elm_pos, smallest_elm_pos);

        (1..k).for_each(|_| match abs_factor(lower - 1) <= abs_factor(upper + 1) {
            true => lower -= 1,
            false => upper += 1,
        });
        arr[lower..=upper].into()
    }
}
