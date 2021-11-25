crate::leetcode::solution!();

const MIN_HP: i32 = 1;

fn to_hp(hp: i32) -> i32 {
    std::cmp::max(hp, MIN_HP)
}

struct Room {
    cost: i32,
    min_hp_required: Option<i32>,
}

impl Room {
    fn new(cost: i32) -> Self {
        Room {
            cost,
            min_hp_required: None,
        }
    }

    fn update_by_next_room(&mut self, next_room_min_hp: i32) {
        if self.min_hp_required == None {
            self.min_hp_required = Some(to_hp(next_room_min_hp));
        }
    }
}

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (dungeon.len(), dungeon[0].len());
        let mut rooms: Vec<Vec<Room>> = dungeon
            .iter()
            .map(|row| row.iter().map(|&cost| Room::new(cost)).collect())
            .collect();
        rooms[n - 1][m - 1].min_hp_required = Some(to_hp(MIN_HP - dungeon[n - 1][m - 1]));

        for i in (0..n - 1).rev() {
            for j in (0..m - 1).rev() {}
        }

        panic!();
    }
}
