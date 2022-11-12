crate::solution!();

use std::collections::HashSet;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let hub_cities = paths.iter().map(|path| &path[0]).collect::<HashSet<_>>();

        paths
            .iter()
            .find_map(|path| match hub_cities.contains(&path[1]) {
                true => None,
                false => Some(path[1].clone()),
            })
            .unwrap()
    }
}
