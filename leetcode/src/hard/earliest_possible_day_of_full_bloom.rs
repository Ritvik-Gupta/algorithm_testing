crate::solution!();

impl Solution {
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        let mut ordered_table: Vec<_> = plant_time.into_iter().zip(grow_time.into_iter()).collect();
        ordered_table.sort_by_key(|(_, grow_time)| -grow_time);

        ordered_table
            .iter()
            .scan(0, |total_plant_time, &(plant_time, grow_time)| {
                *total_plant_time += plant_time;
                Some(*total_plant_time + grow_time)
            })
            .max()
            .unwrap()
    }
}
