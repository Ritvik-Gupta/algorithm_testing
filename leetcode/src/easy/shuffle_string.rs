crate::solution!();

impl Solution {
    pub fn restore_string(mut s: String, mut indices: Vec<i32>) -> String {
        let bytes = unsafe { s.as_bytes_mut() };

        let mut i = 0;
        while i != indices.len() {
            let j = indices[i] as usize;
            if j == i {
                i += 1;
            } else {
                bytes.swap(i, j);
                indices.swap(i, j);
            }
        }
        s
    }
}
