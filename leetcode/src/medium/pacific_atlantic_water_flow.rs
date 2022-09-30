crate::solution!();

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Vector(i32, i32);

impl Vector {
    fn new(x: usize, y: usize) -> Self {
        Self(x as i32, y as i32)
    }

    fn into_vec(&self) -> Vec<i32> {
        vec![self.0, self.1]
    }
}

impl std::ops::Add for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Vector(self.0 + other.0, self.1 + other.1)
    }
}

const NEIGHBOUR_OFFSETS: [Vector; 4] = [Vector(-1, 0), Vector(1, 0), Vector(0, -1), Vector(0, 1)];

struct Island {
    heights: Vec<Vec<i32>>,
    num_rows: usize,
    num_cols: usize,
}

impl Island {
    fn new(heights: Vec<Vec<i32>>) -> Self {
        let (num_rows, num_cols) = (heights.len(), heights[0].len());
        Island {
            heights,
            num_rows,
            num_cols,
        }
    }

    fn get_height(&self, Vector(x, y): Vector) -> Option<i32> {
        use std::convert::TryFrom;
        Some(
            *self
                .heights
                .get(usize::try_from(x).ok()?)?
                .get(usize::try_from(y).ok()?)?,
        )
    }

    fn higher_neighbours(&self, location: Vector) -> impl Iterator<Item = Vector> {
        let location_height = self.get_height(location).unwrap();
        let mut result = Vec::new();
        NEIGHBOUR_OFFSETS
            .iter()
            .map(|&offset| location + offset)
            .for_each(
                |neighbour_location| match self.get_height(neighbour_location) {
                    Some(neighbour) if location_height <= neighbour => {
                        result.push(neighbour_location)
                    }
                    _ => {}
                },
            );
        result.into_iter()
    }
}

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::{BTreeSet, LinkedList};
        use std::thread;

        let island = Island::new(heights);
        let (mut pacific_queue, mut atlantic_queue) = (LinkedList::new(), LinkedList::new());

        for i in 0..island.num_rows {
            pacific_queue.push_back(Vector::new(i, 0));
            atlantic_queue.push_back(Vector::new(i, island.num_cols - 1));
        }
        for j in 0..island.num_cols {
            pacific_queue.push_back(Vector::new(0, j));
            atlantic_queue.push_back(Vector::new(island.num_rows - 1, j));
        }

        let (pacific, atlantic) = thread::scope(|scope| {
            let pacific = scope.spawn(|| {
                let mut flow_into_pacific = BTreeSet::new();

                while let Some(location) = pacific_queue.pop_front() {
                    if flow_into_pacific.insert(location) {
                        island
                            .higher_neighbours(location)
                            .filter(|neighbour| !flow_into_pacific.contains(neighbour))
                            .for_each(|neighbour| pacific_queue.push_back(neighbour));
                    }
                }
                flow_into_pacific
            });

            let atlantic = scope.spawn(|| {
                let mut flow_into_atlantic = BTreeSet::new();

                while let Some(location) = atlantic_queue.pop_front() {
                    if flow_into_atlantic.insert(location) {
                        island
                            .higher_neighbours(location)
                            .filter(|neighbour| !flow_into_atlantic.contains(neighbour))
                            .for_each(|neighbour| atlantic_queue.push_back(neighbour));
                    }
                }
                flow_into_atlantic
            });

            (pacific.join().unwrap(), atlantic.join().unwrap())
        });

        BTreeSet::intersection(&pacific, &atlantic)
            .into_iter()
            .map(Vector::into_vec)
            .collect()
    }
}
