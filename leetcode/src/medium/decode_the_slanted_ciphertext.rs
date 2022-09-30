crate::solution!();

impl Solution {
    pub fn decode_ciphertext(encoded_text: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        let encoded_size = encoded_text.len() as isize;
        let num_cols = (encoded_text.len() / num_rows) as isize;

        let mut result = String::with_capacity(encoded_text.len());
        let encoded_ptr = encoded_text.as_ptr();
        let increment = (num_cols + 1) as usize;

        for begin_pos in 0..num_cols {
            result.extend(
                (begin_pos..encoded_size)
                    .step_by(increment)
                    .map(|idx| unsafe { *encoded_ptr.wrapping_offset(idx) } as char),
            );
        }

        result.trim_end().to_string()
    }
}
