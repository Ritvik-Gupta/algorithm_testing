const MAX_HEIGHT: u8 = 10;

pub fn solve(hieghtmap: Vec<Vec<u8>>) -> u64 {
    let (m, n) = (hieghtmap.len(), hieghtmap[0].len());

    let get_height_if_present =
        |i: usize, j: usize| -> Option<u8> { Some(*hieghtmap.get(i)?.get(j)?) };
    let get_height_at =
        |i: usize, j: usize| -> u8 { get_height_if_present(i, j).unwrap_or(MAX_HEIGHT) };

    let mut risk_level_sum = 0;

    for i in 0..m {
        let mut j = 0;
        while j < n {
            let height = get_height_at(i, j);
            if height < get_height_at(i.wrapping_sub(1), j)
                && height < get_height_at(i, j.wrapping_sub(1))
                && height < get_height_at(i + 1, j)
                && height < get_height_at(i, j + 1)
            {
                risk_level_sum += height as u64 + 1;
                j += 1;
            }
            j += 1;
        }
    }

    risk_level_sum
}
