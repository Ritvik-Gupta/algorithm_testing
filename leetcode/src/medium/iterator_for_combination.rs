struct CombinationIterator {
    characters: Vec<char>,
    combination: Vec<usize>,
}

impl CombinationIterator {
    #[allow(dead_code)]
    fn new(characters: String, combination_length: i32) -> Self {
        Self {
            characters: characters.chars().collect(),
            combination: characters
                .char_indices()
                .map(|(idx, _)| idx)
                .take(combination_length as usize)
                .collect(),
        }
    }

    #[allow(dead_code)]
    fn next(&mut self) -> String {
        let result = self
            .combination
            .iter()
            .map(|&idx| self.characters[idx])
            .collect();
        self.update_to_next_combination();
        result
    }

    fn update_to_next_combination(&mut self) -> Option<()> {
        let mut combination_update_idx = self.combination.len() - 1;
        loop {
            self.combination[combination_update_idx] += 1;
            if self.combination_idx_is_valid(combination_update_idx) {
                break;
            }
            combination_update_idx = combination_update_idx.checked_sub(1)?;
        }

        let mut updated_combination_idx = self.combination[combination_update_idx];
        for idx in combination_update_idx + 1..self.combination.len() {
            self.combination[idx] = updated_combination_idx + 1;
            updated_combination_idx += 1;
        }
        Some(())
    }

    #[allow(dead_code)]
    fn has_next(&self) -> bool {
        self.combination_idx_is_valid(self.combination.len() - 1)
    }

    fn combination_idx_is_valid(&self, idx: usize) -> bool {
        self.combination[idx] + self.combination.len() <= self.characters.len() + idx
    }
}
