crate::leetcode::solution!();

use std::collections::HashSet;

fn dfs_visit_room(room_id: i32, rooms: &Vec<Vec<i32>>, visited_rooms: &mut HashSet<i32>) {
    if visited_rooms.insert(room_id) {
        rooms[room_id as usize]
            .iter()
            .for_each(|&room_key| dfs_visit_room(room_key, rooms, visited_rooms));
    }
}

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut visited_rooms = HashSet::with_capacity(rooms.len() / 5);
        dfs_visit_room(0, &rooms, &mut visited_rooms);
        visited_rooms.len() == rooms.len()
    }
}
