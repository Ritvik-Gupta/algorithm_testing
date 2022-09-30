crate::solution!();

struct Triplet {
    triplet: [i32; 3],
    size: usize,
}

impl Triplet {
    fn new() -> Self {
        Self {
            triplet: [i32::MAX; 3],
            size: 0,
        }
    }

    fn search_add(&mut self, num: i32) {
        if let Some(found_idx) = self.triplet[0..self.size]
            .iter()
            .position(|&val| val >= num)
        {
            self.triplet[found_idx] = num;
        } else {
            self.triplet[self.size] = num;
            self.size += 1;
        }
    }
}

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut triplet = Triplet::new();

        for &num in nums.iter() {
            triplet.search_add(num);
            if triplet.size == 3 {
                break;
            }
        }

        triplet.size == 3
    }
}
