use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use std::collections::HashMap;

structstruck::strike! {
    #[derive(Debug)]
    pub struct Almanac {
        seeds: Vec<u64>,
        farming_guide: Vec<String>,
        farming_maps: HashMap<
            String,
            Vec<
                #[derive(Debug)]
                pub struct CategoryMap {
                    destination_start: u64,
                    source_start: u64,
                    range: u64,
                }
            >
        >
    }
}

impl CategoryMap {
    fn construct(line: String) -> Self {
        let mut tokens = line.split(' ');
        Self {
            destination_start: tokens.next().unwrap().parse().unwrap(),
            source_start: tokens.next().unwrap().parse().unwrap(),
            range: tokens.next().unwrap().parse().unwrap(),
        }
    }
}

trait FarmStageMapper {
    fn map_unit(&self, unit: u64) -> u64;
}

impl FarmStageMapper for Vec<CategoryMap> {
    fn map_unit(&self, unit: u64) -> u64 {
        for map in self.iter() {
            if (map.source_start..map.source_start + map.range).contains(&unit) {
                return map.destination_start + (unit - map.source_start);
            }
        }
        return unit;
    }
}

fn map_almanac_seed_to_location(almanac: &Almanac, seed: u64) -> u64 {
    almanac.farming_guide.iter().fold(seed, |unit, stage| {
        almanac.farming_maps[stage].map_unit(unit)
    })
}

impl Almanac {
    fn compute_lowest_location_seed(
        &self,
        initial_seeds: impl ParallelIterator<Item = u64>,
    ) -> u64 {
        initial_seeds
            .map(|seed| map_almanac_seed_to_location(self, seed))
            .min()
            .unwrap()
    }
}

pub struct IfYouGiveASeedAFertilizer;

impl crate::AdventDayProblem for IfYouGiveASeedAFertilizer {
    type Arg = Almanac;

    type Ret = u64;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(mut dataset: impl Iterator<Item = String>) -> Self::Arg {
        let seeds_line = dataset.next().unwrap();
        dataset.next();
        let seeds = seeds_line.split(": ").last().unwrap();
        let seeds = seeds.split(' ').map(|x| x.parse().unwrap()).collect_vec();

        let mut farming_guide = Vec::new();
        let mut farming_maps = HashMap::new();

        let mut dataset = dataset.peekable();
        while dataset.peek().is_some() {
            let name = dataset.next().unwrap();
            farming_guide.push(name.clone());

            let range_maps = dataset
                .by_ref()
                .take_while(|l| !l.is_empty())
                .map(CategoryMap::construct)
                .collect();

            farming_maps.insert(name, range_maps);
        }

        Almanac {
            seeds,
            farming_guide,
            farming_maps,
        }
    }

    fn part_1(almanac: Self::Arg) -> Self::Ret {
        almanac.compute_lowest_location_seed(almanac.seeds.par_iter().cloned())
    }

    fn part_2(almanac: Self::Arg) -> Self::Ret {
        let initial_seeds = (0..almanac.seeds.len()).step_by(2).flat_map(|i| {
            let seeds_start = almanac.seeds[i];
            let range = almanac.seeds[i + 1];
            seeds_start..seeds_start + range
        });
        almanac.compute_lowest_location_seed(initial_seeds.par_bridge())
    }
}
