crate::solution!();

use std::cmp::Ordering::*;
use std::collections::HashMap;

impl Solution {
    pub fn find_restaurant(list: Vec<String>, lookup_list: Vec<String>) -> Vec<String> {
        let lookup_table = lookup_list
            .into_iter()
            .enumerate()
            .map(|(idx, restaurant)| (restaurant, idx))
            .collect::<HashMap<_, _>>();

        let mut least_idx_sum = list.len() + lookup_table.len();
        let mut restaurants = Vec::new();

        list.into_iter().enumerate().for_each(|(i, restaurant)| {
            if let Some(&j) = lookup_table.get(&restaurant) {
                match (i + j).cmp(&least_idx_sum) {
                    Less => {
                        least_idx_sum = i + j;
                        restaurants.clear();
                        restaurants.push(restaurant);
                    }
                    Equal => restaurants.push(restaurant),
                    _ => {}
                }
            }
        });

        restaurants
    }
}
