crate::solution!();

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        if num_rows == 1 {
            return s;
        }

        let s = s.as_bytes();
        let next_edge_jump = (num_rows - 1) * 2;
        let mut builder = String::with_capacity(s.len());

        for i in 0..num_rows {
            for j in (i..s.len()).step_by(next_edge_jump) {
                builder.push(char::from(s[j]));
                let zig_zag_jump = next_edge_jump - i * 2;
                if i != 0 && i != num_rows - 1 && j + zig_zag_jump < s.len() {
                    builder.push(char::from(s[j + zig_zag_jump]));
                }
            }
        }
        builder
    }
}
