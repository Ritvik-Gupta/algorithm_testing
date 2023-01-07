crate::solution!();

use std::collections::HashSet;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut unique_emails = HashSet::new();
        emails.iter().for_each(|email| {
            let (local, domain) = email.split_once('@').unwrap();
            unique_emails.insert(
                local
                    .chars()
                    .take_while(|&ch| ch != '+')
                    .filter(|&ch| ch != '.')
                    .chain(std::iter::once('@'))
                    .chain(domain.chars())
                    .collect::<String>(),
            );
        });
        unique_emails.len() as _
    }
}
