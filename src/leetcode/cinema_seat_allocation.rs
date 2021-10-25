pub struct Solution;

struct PossibleRowGroups {
    aisle_1: bool,
    middle: bool,
    aisle_2: bool,
}

impl PossibleRowGroups {
    fn new() -> Self {
        Self {
            aisle_1: true,
            middle: true,
            aisle_2: true,
        }
    }

    fn reserve(&mut self, seat_num: i32) {
        if 2 <= seat_num && seat_num <= 5 {
            self.aisle_1 = false;
        }
        if 4 <= seat_num && seat_num <= 7 {
            self.middle = false;
        }
        if 6 <= seat_num && seat_num <= 9 {
            self.aisle_2 = false;
        }
    }

    fn result(&self) -> i32 {
        if self.aisle_1 && self.aisle_2 {
            0
        } else if self.aisle_1 || self.middle || self.aisle_2 {
            1
        } else {
            2
        }
    }
}

impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        let mut theatre_rows = std::collections::BTreeMap::new();
        for (row_num, seat_num) in reserved_seats.into_iter().map(|seat| (seat[0], seat[1])) {
            theatre_rows
                .entry(row_num)
                .or_insert(PossibleRowGroups::new())
                .reserve(seat_num);
        }

        2 * n
            - theatre_rows
                .into_iter()
                .map(|(_, possible_row_groups)| possible_row_groups.result())
                .sum::<i32>()
    }
}
