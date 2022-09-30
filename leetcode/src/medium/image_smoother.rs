crate::solution!();

struct Vector(usize, usize);

impl Vector {
    fn neighbour_group(&self) -> impl Iterator<Item = Vector> {
        vec![
            Vector(self.0 - 1, self.1 - 1),
            Vector(self.0 - 1, self.1 + 0),
            Vector(self.0 - 1, self.1 + 1),
            Vector(self.0 + 0, self.1 - 1),
            Vector(self.0 + 0, self.1 + 0),
            Vector(self.0 + 0, self.1 + 1),
            Vector(self.0 + 1, self.1 - 1),
            Vector(self.0 + 1, self.1 + 0),
            Vector(self.0 + 1, self.1 + 1),
        ]
        .into_iter()
    }
}

const PIXEL_COLOR_SIZE: i32 = 256;
const PIXEL_SMOOTHER_OFFSET: u8 = 9;

struct ImageSmoother(Vec<Vec<i32>>);

impl ImageSmoother {
    fn num_rows(&self) -> usize {
        self.0.len()
    }

    fn num_cols(&self) -> usize {
        self.0[0].len()
    }

    fn get(&self, pos: &Vector) -> Option<&i32> {
        Some(self.0.get(pos.0)?.get(pos.1)?)
    }

    fn get_mut(&mut self, pos: &Vector) -> Option<&mut i32> {
        Some(self.0.get_mut(pos.0)?.get_mut(pos.1)?)
    }
}

impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut img = ImageSmoother(img);
        for i in 0..img.num_rows() {
            for pos in (0..img.num_cols()).map(|j| Vector(i, j)) {
                let (sum, count) = pos
                    .neighbour_group()
                    .filter_map(|pos| img.get(&pos))
                    .map(|pixel_color| pixel_color % PIXEL_COLOR_SIZE)
                    .fold((0, 0), |(sum, count), pixel_color| {
                        (sum + pixel_color, count + 1)
                    });
                *img.get_mut(&pos).unwrap() += (sum / count) << PIXEL_SMOOTHER_OFFSET;
            }
        }
        for i in 0..img.num_rows() {
            for pos in (0..img.num_cols()).map(|j| Vector(i, j)) {
                *img.get_mut(&pos).unwrap() >>= PIXEL_SMOOTHER_OFFSET;
            }
        }

        img.0
    }
}
