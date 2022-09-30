crate::solution!();

struct Rectangle(u64, u64);

impl std::cmp::PartialEq for Rectangle {
    fn eq(&self, other: &Self) -> bool {
        self.0 * other.1 == self.1 * other.0
    }
}

impl std::cmp::Eq for Rectangle {}

impl std::cmp::PartialOrd for Rectangle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((self.0 * other.1).cmp(&(self.1 * other.0)))
    }
}

impl std::cmp::Ord for Rectangle {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution {
    pub fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
        use std::collections::BTreeMap;

        let mut similar_rectangles_record = BTreeMap::new();
        for rectangle in rectangles
            .into_iter()
            .map(|rect| Rectangle(rect[0] as u64, rect[1] as u64))
        {
            similar_rectangles_record
                .entry(rectangle)
                .and_modify(|(x, y)| {
                    *y += *x;
                    *x += 1;
                })
                .or_insert((1, 0));
        }

        similar_rectangles_record
            .values()
            .fold(0, |acc, (_, num_similar)| acc + num_similar)
    }
}
