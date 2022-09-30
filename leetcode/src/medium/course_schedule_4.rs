crate::solution!();

use std::collections::{HashMap, HashSet};

struct Course {
    out_links: Vec<i32>,
}

impl Course {
    fn new() -> Self {
        Self {
            out_links: Vec::new(),
        }
    }
}

struct CourseRecord(HashMap<i32, Course>);

impl CourseRecord {
    fn with_cap(num_courses: usize) -> Self {
        Self(HashMap::with_capacity(num_courses))
    }

    fn add_prereq(&mut self, from_id: i32, to_id: i32) {
        self.0.entry(to_id).or_insert_with(Course::new);
        self.0
            .entry(from_id)
            .or_insert_with(Course::new)
            .out_links
            .push(to_id);
    }

    fn check_if_prereq(&self, from_id: i32, to_id: i32) -> bool {
        self.dfs_check_if_prereq(from_id, to_id, &mut HashSet::new())
            .is_some()
    }

    fn dfs_check_if_prereq(
        &self,
        from_id: i32,
        to_id: i32,
        visited_courses: &mut HashSet<i32>,
    ) -> Option<()> {
        if from_id == to_id {
            return Some(());
        }
        if visited_courses.contains(&from_id) {
            return None;
        }

        visited_courses.insert(from_id);
        if self.0.get(&from_id)?.out_links.iter().any(|&link_to_id| {
            self.dfs_check_if_prereq(link_to_id, to_id, visited_courses)
                .is_some()
        }) {
            return Some(());
        }

        None
    }
}

impl Solution {
    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let mut courses = CourseRecord::with_cap(num_courses as usize);
        prerequisites
            .iter()
            .for_each(|prereq| courses.add_prereq(prereq[0], prereq[1]));

        queries
            .iter()
            .map(|query| courses.check_if_prereq(query[0], query[1]))
            .collect()
    }
}
