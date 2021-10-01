pub struct Solution;

const MIN_HP: i32 = 1;

fn to_hp(hp: i32) -> i32 {
    std::cmp::max(hp, MIN_HP)
}

struct Room {
    cost: i32,
    minHpRequired: Option<i32>,
}

impl Room {
    fn new(cost: i32) -> Self {
        Room {
            cost,
            minHpRequired: None,
        }
    }

    fn update_by_next_room(&mut self, next_room_min_hp: i32) {
        if self.minHpRequired == None {
            self.minHpRequired = Some(to_hp(next_room_min_hp));
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
        rooms[n - 1][m - 1].minHpRequired = Some(to_hp(MIN_HP - dungeon[n - 1][m - 1]));

        for i in (0..n - 1).rev() {
            for j in (0..m - 1).rev() {}
        }

        panic!();
    }
}
