crate::solution!();

impl Solution {
    pub fn get_xor_sum(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let xors = |arr: Vec<_>| arr.into_iter().fold(0, |xors, num| xors ^ num);
        xors(arr1) & xors(arr2)
    }
}
