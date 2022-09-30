crate::solution!();

struct Vector(usize, usize);

impl Vector {
    fn neighbours(&self) -> impl Iterator<Item = Self> {
        vec![
            Vector(self.0 + 1, self.1),
            Vector(self.0 - 1, self.1),
            Vector(self.0, self.1 + 1),
            Vector(self.0, self.1 - 1),
        ]
        .into_iter()
    }

    fn contains(&self, other: &Self) -> bool {
        self.0 > other.0 && self.1 > other.1
    }
}

const WATER: i32 = 0;
const LAND: i32 = 1;
const VISITED_LAND: i32 = 2;

struct WorldMap {
    grid: Vec<Vec<i32>>,
    dimensions: Vector,
}

impl WorldMap {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        let dimensions = Vector(grid.len(), grid[0].len());
        Self { grid, dimensions }
    }

    fn find_island(&mut self, from_pos: Vector) -> i32 {
        let mut island_size = 0;
        let mut island_queue = std::collections::LinkedList::new();
        island_queue.push_back(from_pos);

        while let Some(land_pos) = island_queue.pop_front() {
            if self[&land_pos] == LAND {
                island_size += 1;
                self[&land_pos] = VISITED_LAND;
                land_pos
                    .neighbours()
                    .for_each(|neighbour| island_queue.push_back(neighbour));
            }
        }
        island_size
    }
}

impl std::ops::Index<&Vector> for WorldMap {
    type Output = i32;

    fn index(&self, pos: &Vector) -> &Self::Output {
        if !self.dimensions.contains(&pos) {
            return &WATER;
        }
        &self.grid[pos.0][pos.1]
    }
}

impl std::ops::IndexMut<&Vector> for WorldMap {
    fn index_mut(&mut self, pos: &Vector) -> &mut Self::Output {
        &mut self.grid[pos.0][pos.1]
    }
}

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let (mut world_map, mut max_area) = (WorldMap::new(grid), 0);

        for i in 0..world_map.dimensions.0 {
            for pos in (0..world_map.dimensions.1).map(|j| Vector(i, j)) {
                if world_map[&pos] == LAND {
                    let island_size = world_map.find_island(pos);
                    if island_size > max_area {
                        max_area = island_size;
                    }
                }
            }
        }
        max_area
    }
}
