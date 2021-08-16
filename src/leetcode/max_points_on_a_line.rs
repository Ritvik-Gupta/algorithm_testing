use std::collections::HashSet;

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from(vec: &Vec<i32>) -> Point {
        assert_eq!(vec.len(), 2);
        Point {
            x: vec[0],
            y: vec[1],
        }
    }
}

enum Line {
    WithParams { slope: f64, y_intercept: f64 },
    YAxisParallel(i32),
}

impl Line {
    fn lies_on(&self, point: Point) -> bool {
        match self {
            Line::YAxisParallel(with_x) => point.x == *with_x,
            Line::WithParams { slope, y_intercept } => {
                f64::from(point.y) == slope * f64::from(point.x) + y_intercept
            }
        }
    }

    fn new(point_a: Point, point_b: Point) -> Line {
        if point_a.x == point_a.y {
            return Line::YAxisParallel(point_a.x);
        }
        let slope = f64::from(point_a.y - point_b.y) / f64::from(point_a.x - point_b.x);
        Line::WithParams {
            slope,
            y_intercept: f64::from(point_a.y) - slope * f64::from(point_a.x),
        }
    }
}

pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
    if points.len() == 1 {
        return 1;
    }

    let recorded_lines: HashSet<Line> = HashSet::new();

    panic!();
}

// y-y1 = (x-x1) * (y2-y1) / (x2-x1)
// del_x * (y-y_a) = (x-x_a) * del_y

// y = (x-x1)*m +y1
// y = m*x + (y1 - m*x1)
