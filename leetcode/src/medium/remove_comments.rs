crate::solution!();

impl Solution {
    pub fn remove_comments(source: Vec<String>) -> Vec<String> {
        let mut line_builder = String::new();
        let mut is_in_multiline_comment = false;

        source
            .into_iter()
            .flat_map(|line| {
                let mut i = 0;

                while i < line.len() {
                    match (
                        is_in_multiline_comment,
                        line[i..usize::min(i + 2, line.len())].as_bytes(),
                    ) {
                        (true, b"*/") => {
                            is_in_multiline_comment = false;
                            i += 1;
                        }
                        (false, b"//") => break,
                        (false, b"/*") => {
                            is_in_multiline_comment = true;
                            i += 1;
                        }
                        (false, bytes) => line_builder.push(char::from(bytes[0])),
                        _ => {}
                    }
                    i += 1;
                }

                if !is_in_multiline_comment && !line_builder.is_empty() {
                    Some(line_builder.drain(..).collect())
                } else {
                    None
                }
            })
            .collect()
    }
}
