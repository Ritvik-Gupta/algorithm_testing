crate::solution!();

use std::collections::HashMap;

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq)]
struct TriangleBase(u8, u8);

impl From<&[u8]> for TriangleBase {
    fn from(base: &[u8]) -> Self {
        Self(base[0], base[1])
    }
}

struct PyramidBuilder {
    allowed_triangles: HashMap<TriangleBase, Vec<u8>>,
}

impl PyramidBuilder {
    fn build(allowed: Vec<String>) -> Self {
        let mut allowed_triangles: HashMap<TriangleBase, Vec<u8>> = HashMap::new();

        for traingle in allowed {
            let colors = traingle.as_bytes();
            allowed_triangles
                .entry(colors.into())
                .or_insert_with(Vec::new)
                .push(colors[2]);
        }

        Self { allowed_triangles }
    }

    fn has_level_validity(&self, base_level: &[u8]) -> bool {
        self.compute_next_level(base_level, &mut Vec::with_capacity(base_level.len() - 1))
    }

    fn compute_next_level(&self, base_level: &[u8], next_level: &mut Vec<u8>) -> bool {
        if base_level.len() == 1 {
            return true;
        }
        if base_level.len() == next_level.len() + 1 {
            return self.has_level_validity(next_level);
        }

        let idx = next_level.len();
        for &triangle_top in self
            .allowed_triangles
            .get(&base_level[idx..].into())
            .unwrap_or(&Vec::new())
            .iter()
        {
            next_level.push(triangle_top);
            if self.compute_next_level(base_level, next_level) {
                return true;
            }
            next_level.pop();
        }
        false
    }
}

impl Solution {
    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        PyramidBuilder::build(allowed).has_level_validity(bottom.as_bytes())
    }
}
