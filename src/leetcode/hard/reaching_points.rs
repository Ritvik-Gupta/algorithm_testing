pub struct Solution;

impl Solution {
    pub fn reaching_points(mut sx: i32, mut sy: i32, mut tx: i32, mut ty: i32) -> bool {
        if tx > ty {
            std::mem::swap(&mut tx, &mut ty);
            std::mem::swap(&mut sx, &mut sy);
        }
        tx >= sx
            && ((tx == sx && ty >= sy && (ty - sy) % tx == 0)
                || Solution::reaching_points(sy, sx, ty % tx, tx))
    }
}
