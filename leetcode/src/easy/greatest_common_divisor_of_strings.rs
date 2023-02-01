crate::solution!();

fn gcd(a: usize, b: usize) -> usize {
    match a {
        0 => b,
        _ => gcd(b % a, a),
    }
}

impl Solution {
    pub fn gcd_of_strings(s1: String, s2: String) -> String {
        (format!("{s1}{s2}") == format!("{s2}{s1}"))
            .then(|| s1[0..gcd(s1.len(), s2.len())].to_string())
            .unwrap_or_default()
    }
}
