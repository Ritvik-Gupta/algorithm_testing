crate::leetcode::solution!();

impl Solution {
    pub fn asteroids_destroyed(mass: i32, asteroids: Vec<i32>) -> bool {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut asteroids_for_later_collisions = BinaryHeap::new();
        let mut mass = mass as u64;
        let mut asteroid_idx = 0;

        while asteroid_idx < asteroids.len() {
            match asteroids_for_later_collisions.peek() {
                Some(&Reverse(asteroid_mass)) if asteroid_mass <= mass => {
                    mass += asteroid_mass;
                    asteroids_for_later_collisions.pop();
                }
                _ => {
                    let asteroid_mass = asteroids[asteroid_idx] as u64;
                    if asteroid_mass <= mass {
                        mass += asteroid_mass;
                    } else {
                        asteroids_for_later_collisions.push(Reverse(asteroid_mass));
                    }
                    asteroid_idx += 1;
                }
            }
        }

        while let Some(Reverse(asteroid_mass)) = asteroids_for_later_collisions.pop() {
            if asteroid_mass > mass {
                return false;
            }
            mass += asteroid_mass;
        }
        true
    }
}
