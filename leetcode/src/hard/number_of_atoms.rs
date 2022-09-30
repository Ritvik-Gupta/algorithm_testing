crate::solution!();

use std::{collections::BTreeMap, iter::Peekable, str::Chars};

fn recursive_compute_atomic_tale(formula: &mut Peekable<Chars>) -> BTreeMap<String, u32> {
    let mut atomic_table = BTreeMap::new();

    let mut atom_name = "".to_owned();
    let mut atom_freq = 0;

    while let Some(token) = formula.next() {
        if token == ')' {
            // Current Atomic Table ends here
            break;
        }

        if token.is_uppercase() || token == '(' {
            // Save Old Atom
            *atomic_table.entry(atom_name.clone()).or_insert(0) += u32::max(atom_freq, 1);
            atom_name.clear();
            atom_freq = 0;
        }

        if token.is_uppercase() {
            // New Atom Name starts
            atom_name.push(token);
        } else if token.is_lowercase() {
            // Same Atom Name
            atom_name.push(token);
        } else if token.is_numeric() {
            // Atom Multiplication Factor
            atom_freq = atom_freq * 10 + token.to_digit(10).unwrap();
        } else if token == '(' {
            // Nested Atomic Table Computation
            recursive_compute_atomic_tale(formula)
                .into_iter()
                .for_each(|(atom, freq)| {
                    *atomic_table.entry(atom).or_insert(0) += freq;
                });
        }
    }

    // Save Old Atom
    *atomic_table.entry(atom_name.clone()).or_insert(0) += u32::max(atom_freq, 1);
    atom_name.clear();
    atom_freq = 0;

    // Has Associated Multiplication Factor for whole Atomic Table
    while let Some(&token) = formula.peek() {
        if !token.is_numeric() {
            break;
        }
        formula.next();
        atom_freq = 10 * atom_freq + token.to_digit(10).unwrap();
    }
    if atom_freq > 0 {
        atomic_table
            .iter_mut()
            .for_each(|(_, freq)| *freq *= atom_freq);
    }

    atomic_table
}

impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        recursive_compute_atomic_tale(&mut formula.chars().peekable())
            .iter()
            .filter(|(atom, _)| !atom.is_empty())
            .map(|(atom, &freq)| match freq {
                1 => atom.clone(),
                _ => format!("{}{}", atom, freq),
            })
            .collect()
    }
}
