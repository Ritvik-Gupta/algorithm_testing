pub struct Solution;

impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut result = vec![0; n];

        bookings
            .into_iter()
            .map(|booking| (booking[0] as usize, booking[1] as usize, booking[2]))
            .for_each(|(first, last, seats)| {
                result[first - 1] += seats;
                if last < n {
                    result[last] -= seats;
                }
            });

        (1..n).for_each(|idx| result[idx] += result[idx - 1]);
        result
    }
}
