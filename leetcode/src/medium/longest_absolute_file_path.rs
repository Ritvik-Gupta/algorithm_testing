crate::solution!();

impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        let mut path_stack = Vec::new();
        let mut path_length = 0;

        input.split('\n').fold(0, |max_path_length, path_unit| {
            let unit_depth = path_unit.chars().take_while(|&ch| ch == '\t').count();
            let (_, path_unit) = path_unit.split_at(unit_depth);

            while path_stack.len() > unit_depth {
                path_length -= path_stack.pop().unwrap();
            }

            path_stack.push(path_unit.len());
            path_length += path_unit.len();

            usize::max(
                max_path_length,
                match path_unit.contains('.') {
                    true => path_length + path_stack.len() - 1,
                    _ => 0,
                },
            )
        }) as i32
    }
}
