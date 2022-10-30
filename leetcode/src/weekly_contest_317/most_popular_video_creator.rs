crate::solution!();

use std::cmp::Ordering::*;
use std::collections::HashMap;

#[derive(Default)]
struct CreatorRating {
    total_views: u128,
    highest_viewed_id: String,
    highest_views: i32,
}

impl Solution {
    pub fn most_popular_creator(
        creators: Vec<String>,
        ids: Vec<String>,
        views: Vec<i32>,
    ) -> Vec<Vec<String>> {
        let mut creators_table = HashMap::new();

        let mut max_overall_views = 0;

        for (creator, (id, views)) in creators
            .into_iter()
            .zip(ids.into_iter().zip(views.into_iter()))
        {
            let CreatorRating {
                total_views,
                highest_viewed_id,
                highest_views,
            } = creators_table.entry(creator).or_default();

            *total_views += views as u128;
            max_overall_views = u128::max(max_overall_views, *total_views);

            match views.cmp(highest_views) {
                Less => continue,
                Equal if !highest_viewed_id.is_empty() && *highest_viewed_id < id => continue,
                _ => {}
            }

            *highest_views = views;
            *highest_viewed_id = id;
        }

        creators_table
            .into_iter()
            .filter(|(_, rating)| rating.total_views == max_overall_views)
            .map(|(creator, rating)| vec![creator, rating.highest_viewed_id])
            .collect()
    }
}
