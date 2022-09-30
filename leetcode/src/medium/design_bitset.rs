use std::collections::HashSet;

#[allow(unused)]
struct Bitset {
    size: i32,
    ones: HashSet<i32>,
    zeroes: HashSet<i32>,
}

impl Bitset {
    #[allow(unused)]
    fn new(size: i32) -> Self {
        Bitset {
            size,
            ones: HashSet::with_capacity(size as usize),
            zeroes: (0..size).collect(),
        }
    }

    #[allow(unused)]
    fn fix(&mut self, idx: i32) {
        self.zeroes.remove(&idx);
        self.ones.insert(idx);
    }

    #[allow(unused)]
    fn unfix(&mut self, idx: i32) {
        self.ones.remove(&idx);
        self.zeroes.insert(idx);
    }

    #[allow(unused)]
    fn flip(&mut self) {
        std::mem::swap(&mut self.zeroes, &mut self.ones);
    }

    #[allow(unused)]
    fn all(&self) -> bool {
        self.count() == self.size
    }

    #[allow(unused)]
    fn one(&self) -> bool {
        self.count() > 0
    }

    #[allow(unused)]
    fn count(&self) -> i32 {
        self.ones.len() as i32
    }

    #[allow(unused)]
    fn to_string(&self) -> String {
        let mut str_rep = "".to_string();
        for i in 0..self.size {
            str_rep.push(if self.zeroes.contains(&i) { '0' } else { '1' });
        }
        str_rep
    }
}
