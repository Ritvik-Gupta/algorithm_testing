crate::solution!();

use std::collections::{HashMap, HashSet};

struct RectangleBoundary {
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
}

impl RectangleBoundary {
    fn starting_at(x: usize, y: usize) -> Self {
        Self {
            top: x,
            bottom: x,
            left: y,
            right: y,
        }
    }

    fn expand_to(&mut self, x: usize, y: usize) {
        self.top = usize::min(self.top, x);
        self.bottom = usize::max(self.bottom, x);
        self.left = usize::min(self.left, y);
        self.right = usize::max(self.right, y);
    }

    fn area_iterator<'itr>(&'itr self) -> impl Iterator<Item = (usize, usize)> + 'itr {
        (self.top..=self.bottom).flat_map(move |x| (self.left..=self.right).map(move |y| (x, y)))
    }
}

fn check_for_cycle_in_dependency_graph(dependency_graph: HashMap<i32, HashSet<i32>>) -> bool {
    use NodeState::*;

    enum NodeState {
        NotVisited,
        DfsVisited,
        Visited,
    }

    fn check_for_cycle(
        node: i32,
        dependency_graph: &HashMap<i32, HashSet<i32>>,
        node_states: &mut HashMap<i32, NodeState>,
    ) -> bool {
        *node_states.get_mut(&node).unwrap() = DfsVisited;

        if dependency_graph[&node].iter().any(|&supporter_node| {
            match node_states[&supporter_node] {
                Visited => false,
                DfsVisited => true,
                NotVisited => check_for_cycle(supporter_node, dependency_graph, node_states),
            }
        }) {
            return true;
        }

        *node_states.get_mut(&node).unwrap() = Visited;
        false
    }

    let mut node_states = dependency_graph
        .keys()
        .map(|&color| (color, NotVisited))
        .collect::<HashMap<_, _>>();

    dependency_graph
        .keys()
        .any(|&color| check_for_cycle(color, &dependency_graph, &mut node_states))
}

impl Solution {
    pub fn is_printable(target_grid: Vec<Vec<i32>>) -> bool {
        let mut color_boundaries_table = HashMap::<i32, RectangleBoundary>::with_capacity(61);

        target_grid.iter().enumerate().for_each(|(x, row)| {
            row.iter().enumerate().for_each(|(y, &color)| {
                color_boundaries_table
                    .entry(color)
                    .and_modify(|boundary| boundary.expand_to(x, y))
                    .or_insert_with(|| RectangleBoundary::starting_at(x, y));
            });
        });

        !check_for_cycle_in_dependency_graph(
            color_boundaries_table
                .into_iter()
                .map(|(color, boundary)| {
                    (
                        color,
                        boundary
                            .area_iterator()
                            .map(|(x, y)| target_grid[x][y])
                            .filter(|&tile_color| color != tile_color)
                            .collect(),
                    )
                })
                .collect(),
        )
    }
}
