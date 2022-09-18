use std::collections::HashSet;

#[derive(PartialEq, Eq)]
enum Direction {
    Horizontal,
    Vertical,
    DiagonalPos,
    DiagonalNeg,
}

use Direction::*;

struct Vent {
    start_x: isize,
    start_y: isize,
    end_x: isize,
    end_y: isize,
    direction: Direction,
}

impl From<String> for Vent {
    fn from(vent: String) -> Self {
        let mut coords = vent.split(&[' ', ','][..]);
        let start_x = coords
            .next()
            .expect("has a start x coordinate")
            .parse()
            .expect("is a number");
        let start_y = coords
            .next()
            .expect("has a start y coordinate")
            .parse()
            .expect("is a number");
        coords.next().expect("has an arrow sign");
        let end_x = coords
            .next()
            .expect("has an end x coordinate")
            .parse()
            .expect("is a number");
        let end_y = coords
            .next()
            .expect("has an end y coordinate")
            .parse()
            .expect("is a number");

        Self {
            start_x,
            start_y,
            end_x,
            end_y,
            direction: match (start_x - end_x, start_y - end_y) {
                (_, 0) => Horizontal,
                (0, _) => Vertical,
                (delta_x, delta_y) if delta_x.signum() == delta_y.signum() => DiagonalPos,
                (delta_x, delta_y) if delta_x.signum() != delta_y.signum() => DiagonalNeg,
                _ => unreachable!(),
            },
        }
    }
}

pub fn solve(vents: impl Iterator<Item = String>) -> usize {
    let vents = vents
        .map(Vent::from)
        .filter(|vent| vent.direction == Horizontal || vent.direction == Vertical);

    // let mut unit_grid_locations = HashSet::new();
    // let mut intersect_locations = HashSet::new();
    vents.for_each(|vent| {});

    0
}
