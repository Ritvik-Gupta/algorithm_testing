crate::solution!();

static FIZZ: &str = "Fizz";
static BUZZ: &str = "Buzz";

impl Solution {
    pub fn fizz_buzz(num: i32) -> Vec<String> {
        (1..=num)
            .map(|idx| {
                let mut ans = String::with_capacity(8);
                if idx % 3 == 0 {
                    ans.push_str(FIZZ);
                }
                if idx % 5 == 0 {
                    ans.push_str(BUZZ);
                }
                if ans.len() == 0 {
                    ans.push_str(&idx.to_string());
                }
                ans
            })
            .collect()
    }
}
