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

pub fn life_support_rating(binaries: impl Iterator<Item = String>) -> Result<u64, Box<dyn Error>> {
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
