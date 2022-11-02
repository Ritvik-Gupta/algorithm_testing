crate::solution!();

use std::collections::{HashMap, HashSet, VecDeque};

fn mutation_score(start: &str, end: &str) -> usize {
    start
        .chars()
        .zip(end.chars())
        .filter(|(st, en)| st != en)
        .count()
}

impl Solution {
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        if bank.iter().all(|gene| gene != &end) {
            return -1;
        }

        let mut adjacency_list = HashMap::new();

        bank.iter()
            .chain(std::iter::once_with(|| &start))
            .for_each(|from_gene| {
                adjacency_list.insert(
                    from_gene.clone(),
                    bank.iter()
                        .filter(|to_gene| mutation_score(from_gene, to_gene) == 1)
                        .collect::<Vec<_>>(),
                );
            });

        let mut gene_queue = VecDeque::new();
        let mut visited_genes = HashSet::new();
        gene_queue.push_back((start, 0));

        let mut min_mutation = i32::MAX;

        while let Some((gene, total_mutations)) = gene_queue.pop_front() {
            if gene == end {
                min_mutation = i32::min(min_mutation, total_mutations);
                continue;
            }
            visited_genes.insert(gene.clone());

            adjacency_list[&gene]
                .iter()
                .filter(|&&mutated_gene| !visited_genes.contains(mutated_gene))
                .for_each(|&mutated_gene| {
                    gene_queue.push_back((mutated_gene.clone(), total_mutations + 1));
                });
        }

        if min_mutation == i32::MAX {
            -1
        } else {
            min_mutation
        }
    }
}
