use derive_more::Add;
use once_cell::sync::Lazy;
use std::collections::VecDeque;
use StreamDirection::*;

#[derive(PartialEq, Eq)]
pub enum StreamDirection {
    LEFT,
    RIGHT,
}

impl StreamDirection {
    fn force(&self) -> i64 {
        match self {
            LEFT => -1,
            RIGHT => 1,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Add)]
struct Location(i64, i64);

trait Tessellations: Sync + Send {
    fn tessellations(&self) -> &[Location];
}

macro_rules! count_exprs {
    () => (0);
    ($head: expr) => (1);
    ($head: expr, $($tail: expr),*) => (1 + count_exprs!($($tail),*));
}

macro_rules! construct_rock {
    ($rock_name: tt with units $(($unit_x: literal, $unit_y: literal)),+) => {
        {
            #[derive(Clone)]
            struct $rock_name([Location; count_exprs!($($unit_x),+)]);

            impl Tessellations for $rock_name {
                fn tessellations(&self) -> &[Location] {
                    &self.0
                }
            }

            Box::new($rock_name([$(Location($unit_x, $unit_y)),+]))
        }
    };
}

static ROCKS: Lazy<[Box<dyn Tessellations>; 5]> = Lazy::new(|| {
    [
        construct_rock!(Vertical with units (0, 0), (1, 0), (2, 0), (3, 0)),
        construct_rock!(Plus with units (0, 1), (1, 1), (2, 1), (1, 2), (1, 0)),
        construct_rock!(Steep with units (0, 0), (1, 0), (2, 0), (2, 1), (2, 2)),
        construct_rock!(Horizontal with units (0, 0), (0, 1), (0, 2), (0, 3)),
        construct_rock!(Square with units (0, 0), (1, 0), (0, 1), (1, 1)),
    ]
});

struct VisibleWindow {
    area: VecDeque<[bool; 8]>,
    height: i64,
}

impl VisibleWindow {
    fn new() -> Self {
        Self {
            area: std::iter::once([false; 8]).collect(),
            height: 0,
        }
    }

    fn shrink(&mut self) {
        if self.area.len() > 5 {
            self.area.pop_front();
            self.height += 1;
        }
    }

    fn insert(&mut self, loc: Location) {
        while (self.area.len() as i64) <= loc.1 - self.height {
            self.area.push_back([false; 8]);
        }
        self.area[(loc.1 - self.height) as usize][loc.0 as usize] = true;
    }

    fn contains(&self, loc: Location) -> bool {
        self.area
            .get((loc.1 - self.height) as usize)
            .map_or(false, |row| row[loc.0 as usize])
    }
}

fn falling_rocks_tower_simulation<const SIMULATION_LENGTH: usize>(
    stream_dirs: Vec<StreamDirection>,
) -> i64 {
    let mut tower_height = 0;
    let mut fallen_units = VisibleWindow::new();
    (1..=7).for_each(|x| fallen_units.insert(Location(x, 0)));
    let mut stream_dirs = stream_dirs.iter().cycle();

    tqdm::tqdm(ROCKS.iter().cycle().take(SIMULATION_LENGTH)).for_each(|rock| {
        let (mut x, mut y) = (3, 4 + tower_height);

        loop {
            let new_x = x + stream_dirs.next().unwrap().force();
            if rock
                .tessellations()
                .iter()
                .map(|&loc| loc + Location(new_x, y))
                .all(|loc| 0 < loc.0 && loc.0 < 8 && !fallen_units.contains(loc))
            {
                x = new_x;
            }

            let new_y = y - 1;
            if rock
                .tessellations()
                .iter()
                .map(|&loc| loc + Location(x, new_y))
                .any(|loc| fallen_units.contains(loc))
            {
                break;
            }
            y = new_y;
        }

        rock.tessellations()
            .iter()
            .map(|&loc| loc + Location(x, y))
            .for_each(|loc| {
                fallen_units.insert(loc);
                tower_height = i64::max(tower_height, loc.1);
            });

        fallen_units.shrink();
    });

    tower_height
}

pub struct PyroclasticFlow;

impl crate::AdventDayProblem for PyroclasticFlow {
    type Arg = Vec<StreamDirection>;
    type Ret = i64;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(mut dataset: impl Iterator<Item = String>) -> Self::Arg {
        dataset
            .next()
            .unwrap()
            .chars()
            .map(|ch| if ch == '<' { LEFT } else { RIGHT })
            .collect()
    }

    fn part_1(stream_dirs: Self::Arg) -> Self::Ret {
        falling_rocks_tower_simulation::<2022>(stream_dirs)
    }

    fn part_2(stream_dirs: Self::Arg) -> Self::Ret {
        falling_rocks_tower_simulation::<1000000000000>(stream_dirs)
    }
}
