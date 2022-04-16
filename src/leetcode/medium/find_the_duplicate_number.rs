crate::leetcode::solution!();

struct RaceTrack(Vec<i32>);

impl RaceTrack {
    fn new_contestant(&self) -> Contestant {
        Contestant {
            race_track: self,
            position: 0,
        }
    }
}

struct Contestant<'a> {
    race_track: &'a RaceTrack,
    position: usize,
}

impl<'a> Contestant<'a> {
    fn next(&mut self) {
        self.position = self.race_track.0[self.position] as usize;
    }
}

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let race_track = RaceTrack(nums);
        let (mut turtle, mut rabbit) = (race_track.new_contestant(), race_track.new_contestant());
        loop {
            turtle.next();
            rabbit.next();
            rabbit.next();
            if turtle.position == rabbit.position {
                let mut snail = race_track.new_contestant();
                while turtle.position != snail.position {
                    turtle.next();
                    snail.next();
                }
                return snail.position as i32;
            }
        }
    }
}
