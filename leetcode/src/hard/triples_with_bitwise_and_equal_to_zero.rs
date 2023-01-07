crate::solution!();

fn bit_flag_of(num: usize, offset: i32) -> usize {
    (num >> offset) & 1
}

struct Trie {
    count: i32,
    child: [Option<Box<Trie>>; 2],
}

impl Trie {
    fn new() -> Box<Self> {
        Box::new(Self {
            count: 0,
            child: [None, None],
        })
    }

    fn add(&mut self, num: usize) {
        let mut node = self;
        for i in (0..=15).rev() {
            let b = bit_flag_of(num, i);
            if node.child[b].is_none() {
                node.child[b] = Some(Trie::new());
            }
            node = node.child[b].as_mut().unwrap();
        }
        node.count += 1;
    }

    fn count_zero_ands(self: &Box<Self>, num: usize) -> i32 {
        dfs_find_zero_ands(Some(self), num, 15)
    }
}

fn dfs_find_zero_ands(node: Option<&Box<Trie>>, num: usize, i: i32) -> i32 {
    match node {
        Some(node) if i == -1 => node.count,
        Some(node) => {
            dfs_find_zero_ands(node.child[0].as_ref(), num, i - 1)
                + (bit_flag_of(num, i) == 0)
                    .then(|| dfs_find_zero_ands(node.child[1].as_ref(), num, i - 1))
                    .unwrap_or(0)
        }
        None => 0,
    }
}

impl Solution {
    pub fn count_triplets(nums: Vec<i32>) -> i32 {
        let mut trie = Trie::new();
        for &a in nums.iter() {
            for &b in nums.iter() {
                trie.add((a & b) as usize);
            }
        }

        nums.into_iter()
            .map(|num| trie.count_zero_ands(num as usize))
            .sum()
    }
}
