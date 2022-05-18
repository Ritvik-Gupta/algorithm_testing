crate::leetcode::solution!();

struct Graph {
    adjacency_list: Vec<Vec<i32>>,
    time: i32,
}

impl Graph {
    fn build(num_nodes: usize, connections: Vec<Vec<i32>>) -> Self {
        let mut graph = Self {
            adjacency_list: vec![Vec::new(); num_nodes],
            time: 0,
        };

        for conn in connections {
            graph.adjacency_list[conn[0] as usize].push(conn[1]);
            graph.adjacency_list[conn[1] as usize].push(conn[0]);
        }

        graph
    }

    fn bridge(&mut self) -> Vec<Vec<i32>> {
        let num_nodes = self.adjacency_list.len();

        let mut visited = vec![false; num_nodes];
        let mut disc = vec![0; num_nodes];
        let mut low = vec![0; num_nodes];
        let mut parent = vec![-1; num_nodes];

        let mut bridges = Vec::new();

        for i in 0..num_nodes {
            if !visited[i] {
                self.bridge_util(
                    i,
                    &mut visited,
                    &mut disc,
                    &mut low,
                    &mut parent,
                    &mut bridges,
                );
            }
        }

        bridges
    }

    fn bridge_util(
        &mut self,
        u: usize,
        visited: &mut Vec<bool>,
        disc: &mut Vec<i32>,
        low: &mut Vec<i32>,
        parent: &mut Vec<i32>,
        bridges: &mut Vec<Vec<i32>>,
    ) {
        visited[u] = true;

        self.time += 1;
        low[u] = self.time;
        disc[u] = self.time;

        for &v in self.adjacency_list[u].clone().iter() {
            let v = v as usize;

            if !visited[v] {
                parent[v] = u as i32;
                self.bridge_util(v, visited, disc, low, parent, bridges);

                low[u] = i32::min(low[u], low[v]);

                if low[v] > disc[u] {
                    bridges.push(vec![u as i32, v as i32]);
                }
            } else if v != parent[u] as usize {
                low[u] = i32::min(low[u], disc[v]);
            }
        }
    }
}

impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let num_nodes = n as usize;

        let mut graph = Graph::build(num_nodes, connections);
        graph.bridge()
    }
}
