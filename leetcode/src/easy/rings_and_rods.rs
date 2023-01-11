crate::solution!();

fn ring_power(ring: u8) -> u8 {
    match ring {
        b'R' => 0b001,
        b'G' => 0b010,
        b'B' => 0b100,
        _ => unreachable!(),
    }
}

impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let mut rods = [0; 10];
        let rings = rings.as_bytes();

        for i in (0..rings.len()).step_by(2) {
            let (ring, rod_idx) = (ring_power(rings[i]), rings[i + 1] - b'0');
            rods[rod_idx as usize] |= ring;
        }

        rods.iter().filter(|&&rod_power| rod_power == 0b111).count() as _
    }
}
