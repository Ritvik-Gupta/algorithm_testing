crate::solution!();

impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        let n = edges.len();
        let mut distance_table = vec![[i32::MIN, i32::MIN]; n];

        for (i, &start_node) in [node1, node2].iter().enumerate() {
            let (mut node, mut distance) = (start_node, 0);
            while node != -1 {
                let bucket = &mut distance_table[node as usize][i];
                if *bucket != i32::MIN {
                    break;
                }
                *bucket = i32::max(*bucket, distance);
                distance += 1;
                node = edges[node as usize];
            }
        }

        distance_table
            .into_iter()
            .zip(0..)
            .filter(|(dist, _)| dist[0] != i32::MIN && dist[1] != i32::MIN)
            .map(|(dist, i)| (i32::max(dist[0], dist[1]), i))
            .min_by_key(|&(dist, _)| dist)
            .unwrap_or((-1, -1))
            .1
    }
}
