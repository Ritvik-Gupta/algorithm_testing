crate::solution!();

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut res = Vec::new();

        (1..4)
            .flat_map(|a| {
                (1..4)
                    .flat_map(move |b| (1..4).flat_map(move |c| (1..4).map(move |d| (a, b, c, d))))
            })
            .for_each(|(a, b, c, d)| {
                if a + b + c + d == s.len() {
                    let w = s[0..a].parse::<u16>().unwrap();
                    let x = s[a..a + b].parse::<u16>().unwrap();
                    let y = s[a + b..a + b + c].parse::<u16>().unwrap();
                    let z = s[a + b + c..a + b + c + d].parse::<u16>().unwrap();

                    if w <= 255 && x <= 255 && y <= 255 && z <= 255 {
                        let ip_address = format!("{w}.{x}.{y}.{z}");
                        if ip_address.len() == s.len() + 3 {
                            res.push(ip_address);
                        }
                    }
                }
            });

        res
    }
}
