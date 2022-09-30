crate::solution!();

static NEIGHBOUR_OFFSETS: [Vector; 9] = [
    Vector(-1, -1),
    Vector(-1, 0),
    Vector(-1, 1),
    Vector(0, -1),
    Vector(0, 0),
    Vector(0, 1),
    Vector(1, -1),
    Vector(1, 0),
    Vector(1, 1),
];

#[derive(PartialEq, Eq, Hash, Clone)]
struct Vector(i32, i32);

impl Vector {
    fn from(vector: Vec<i32>) -> Self {
        Self(vector[0], vector[1])
    }

    fn neighbours(self) -> impl Iterator<Item = Vector> {
        NEIGHBOUR_OFFSETS.iter().map(move |offset| &self + offset)
    }
}

impl std::ops::Add for &Vector {
    type Output = Vector;

    fn add(self, other: &Vector) -> Vector {
        Vector(self.0 + other.0, self.1 + other.1)
    }
}

struct AxisIntensity<F = Box<dyn Fn(&Vector) -> i32>>
where
    F: Fn(&Vector) -> i32,
{
    unit_converter: F,
    store: BTreeMap<i32, usize>,
}

impl<F> AxisIntensity<F>
where
    F: Fn(&Vector) -> i32,
{
    fn new(unit_converter: F) -> Self {
        AxisIntensity {
            unit_converter,
            store: BTreeMap::new(),
        }
    }

    fn has_intensity_at(&self, pos: &Vector) -> bool {
        *self.store.get(&(self.unit_converter)(pos)).unwrap_or(&0) > 0
    }

    fn increment(&mut self, pos: &Vector) {
        *self.store.entry((self.unit_converter)(pos)).or_insert(0) += 1;
    }

    fn decrement(&mut self, pos: &Vector) {
        self.store
            .entry((self.unit_converter)(pos))
            .and_modify(|intensity| *intensity -= 1);
    }
}

use std::collections::{BTreeMap, HashSet};

struct IlluminatedGrid {
    axes: [AxisIntensity; 4],
    active_lamps: HashSet<Vector>,
}

impl IlluminatedGrid {
    fn new(total_lamps: usize) -> Self {
        Self {
            axes: [
                AxisIntensity::new(Box::new(|&Vector(x, _)| x)),
                AxisIntensity::new(Box::new(|&Vector(_, y)| y)),
                AxisIntensity::new(Box::new(|&Vector(x, y)| x + y)),
                AxisIntensity::new(Box::new(|&Vector(x, y)| x - y)),
            ],
            active_lamps: HashSet::with_capacity(total_lamps),
        }
    }

    fn add_lamp(&mut self, pos: &Vector) {
        if self.active_lamps.insert(pos.clone()) {
            self.axes.iter_mut().for_each(|axis| axis.increment(pos));
        }
    }

    fn remove_lamp(&mut self, pos: &Vector) {
        if self.active_lamps.remove(pos) {
            self.axes.iter_mut().for_each(|axis| axis.decrement(pos));
        }
    }

    fn is_illuminated(&self, pos: &Vector) -> bool {
        self.axes.iter().any(|axis| axis.has_intensity_at(pos))
    }
}

impl Solution {
    pub fn grid_illumination(_n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut illuminated_grid = IlluminatedGrid::new(lamps.len());
        lamps
            .into_iter()
            .map(Vector::from)
            .for_each(|ref lamp_pos| illuminated_grid.add_lamp(lamp_pos));

        let mut result = Vec::with_capacity(queries.len());

        queries.into_iter().map(Vector::from).for_each(|query_pos| {
            result.push(illuminated_grid.is_illuminated(&query_pos) as i32);

            query_pos
                .neighbours()
                .for_each(|ref neighbour_pos| illuminated_grid.remove_lamp(neighbour_pos));
        });
        return result;
    }
}
