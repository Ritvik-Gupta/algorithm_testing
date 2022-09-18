use std::error::Error;

pub fn power_consumption(binaries: impl Iterator<Item = String>) -> Result<u64, Box<dyn Error>> {
    let mut binaries = binaries.peekable();

    let first_binary = binaries
        .peek()
        .expect("to have atleast one binary diagnostic");
    let mut bit_count_store = vec![0; first_binary.len()];

    binaries.for_each(|binary_diagnostic| {
        binary_diagnostic.char_indices().for_each(|(idx, ch)| {
            bit_count_store[idx] += match ch {
                '1' => 1,
                _ => -1,
            };
        });
    });

    let (mut gamma_rate, mut epsilon_rate) = (0, 0);

    bit_count_store
        .iter()
        .rev()
        .enumerate()
        .for_each(|(idx, &bit_count)| {
            if bit_count < 0 {
                epsilon_rate |= 1 << idx;
            } else if bit_count > 0 {
                gamma_rate |= 1 << idx;
            }
        });

    Ok(gamma_rate * epsilon_rate)
}

pub fn life_support_rating(binaries: Vec<usize>, total_bits: usize) -> Result<u64, Box<dyn Error>> {
    fn has_one_bit_at(binary: usize, bit_offset: usize) -> bool {
        binary & bit_offset != 0
    }

    let mut oxygen_binaries = binaries.clone();
    let mut bit_offset = 1 << (total_bits - 1);

    while oxygen_binaries.len() > 1 {
        let num_ones = oxygen_binaries
            .iter()
            .map(|&binary| has_one_bit_at(binary, bit_offset) as usize)
            .sum::<usize>();
        let num_zeroes = oxygen_binaries.len() - num_ones;
        let collecting_bit_is_one = num_ones >= num_zeroes;

        oxygen_binaries = oxygen_binaries
            .into_iter()
            .filter(|&binary| has_one_bit_at(binary, bit_offset) == collecting_bit_is_one)
            .collect();
        bit_offset >>= 1;
    }
    let oxygen_level = oxygen_binaries[0] as u64;

    let mut carbon_binaries = binaries.clone();
    let mut bit_offset = 1 << (total_bits - 1);

    while carbon_binaries.len() > 1 {
        let num_ones = carbon_binaries
            .iter()
            .map(|&binary| has_one_bit_at(binary, bit_offset) as usize)
            .sum::<usize>();
        let num_zeroes = carbon_binaries.len() - num_ones;
        let collecting_bit_is_one = num_ones < num_zeroes;

        carbon_binaries = carbon_binaries
            .into_iter()
            .filter(|&binary| has_one_bit_at(binary, bit_offset) == collecting_bit_is_one)
            .collect();
        bit_offset >>= 1;
    }
    let carbon_level = carbon_binaries[0] as u64;

    Ok(oxygen_level * carbon_level)
}
