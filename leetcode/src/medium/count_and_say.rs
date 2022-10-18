crate::solution!();

fn say(num: String) -> String {
    let mut said_num = String::with_capacity(num.len());

    let mut chars = num.chars().chain(std::iter::repeat('-').take(1));
    let mut group_ch = chars.next().unwrap();
    let mut count_ch = 1;

    while let Some(ch) = chars.next() {
        if ch != group_ch {
            said_num += &format!("{count_ch}{group_ch}");
            group_ch = ch;
            count_ch = 0;
        }
        count_ch += 1;
    }

    said_num
}

impl Solution {
    pub fn count_and_say(num: i32) -> String {
        match num {
            1 => "1".to_string(),
            _ => say(Solution::count_and_say(num - 1)),
        }
    }
}
