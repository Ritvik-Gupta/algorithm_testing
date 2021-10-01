pub struct Solution;

#[derive(Hash, PartialEq, Eq)]
struct DpKey(usize, usize);

use std::collections::HashMap;

struct TabulatedSubSeq<'a> {
    text1: &'a [u8],
    text2: &'a [u8],
    memo: HashMap<DpKey, i32>,
}

impl<'a> TabulatedSubSeq<'a> {
    fn new(text1: &'a str, text2: &'a str) -> Self {
        let mut memo = HashMap::new();
        for i in 0..text1.len() + 1 {
            memo.insert(DpKey(i, 0), 0);
        }
        for j in 0..text2.len() + 1 {
            memo.insert(DpKey(0, j), 0);
        }

        TabulatedSubSeq {
            text1: text1.as_bytes(),
            text2: text2.as_bytes(),
            memo,
        }
    }

    fn bottom_up(&mut self, m: usize, n: usize) -> i32 {
        if let Some(&stored) = self.memo.get(&DpKey(m, n)) {
            return stored;
        }

        let store = if self.text1[m - 1] == self.text2[n - 1] {
            1 + self.bottom_up(m - 1, n - 1)
        } else {
            std::cmp::max(self.bottom_up(m - 1, n), self.bottom_up(m, n - 1))
        };
        self.memo.insert(DpKey(m, n), store);
        store
    }
}

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        TabulatedSubSeq::new(&text1, &text2).bottom_up(text1.len(), text2.len())
    }
}
