crate::solution!();

fn sorted_repr(n: &i32) -> Vec<u8> {
    let mut repr = n.to_string().as_bytes().to_vec();
    repr.sort();
    repr
}

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let repr = sorted_repr(&n);
        (0..32).any(|i| sorted_repr(&(1 << i)) == repr)
    }
}
