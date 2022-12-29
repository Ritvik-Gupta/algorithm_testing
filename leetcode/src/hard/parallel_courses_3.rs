crate::solution!();

fn recur_to_completion(
    node: usize,
    adjacency_list: &Vec<Vec<usize>>,
    time: &Vec<i32>,
    completion_time: &mut Vec<i32>,
) -> i32 {
    if completion_time[node] != -1 {
        return completion_time[node];
    }

    completion_time[node] = time[node]
        + adjacency_list[node]
            .iter()
            .map(|&related_node| {
                recur_to_completion(related_node, adjacency_list, time, completion_time)
            })
            .max()
            .unwrap_or(0);

    completion_time[node]
}

impl Solution {
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut completion_time = vec![-1; n];

        let mut adjacency_list = vec![Vec::new(); n];
        relations
            .into_iter()
            .for_each(|edge| adjacency_list[edge[1] as usize - 1].push(edge[0] as usize - 1));

        for i in 0..adjacency_list.len() {
            recur_to_completion(i, &adjacency_list, &time, &mut completion_time);
        }

        completion_time.into_iter().max().unwrap()
    }
}
