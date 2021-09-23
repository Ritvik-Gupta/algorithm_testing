use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

struct Conurbation {
    cities: BTreeMap<u32, MetropolitanCity>,
}

impl Conurbation {
    fn new() -> Self {
        Conurbation {
            cities: BTreeMap::new(),
        }
    }

    fn add_city(&mut self, city_id: u32) {
        self.cities.insert(city_id, City::new(city_id));
    }

    fn get_city(&mut self, city_id: u32) -> MetropolitanCity {
        self.cities.get_mut(&city_id).unwrap().clone()
    }
}

struct City {
    id: u32,
    roads_to: BTreeMap<u32, MetropolitanCity>,
}

type MetropolitanCity = Rc<RefCell<City>>;

impl City {
    fn new(id: u32) -> MetropolitanCity {
        Rc::new(RefCell::new(City {
            id,
            roads_to: BTreeMap::new(),
        }))
    }

    fn add_road_to(&mut self, city: MetropolitanCity) {
        self.roads_to.insert(city.borrow().id, city.clone());
    }
}

fn as_connection(connection_format: String) -> (u32, u32) {
    let mut connection = connection_format
        .split(' ')
        .take(2)
        .map(|x| x.parse::<u32>().unwrap());
    (connection.next().unwrap(), connection.next().unwrap())
}

use crate::services::Returns;

pub fn main() -> Returns {
    use std::{
        collections::HashSet,
        convert::TryInto,
        io::{self, BufRead},
    };

    let stdin = io::stdin();
    let mut input_lines = stdin.lock().lines().map(Result::unwrap);

    let num_test_cases: u32 = input_lines.next().unwrap().parse()?;
    for _ in 0..num_test_cases {
        let num_cities: u32 = input_lines.next().unwrap().parse()?;

        let mut conurbation_roads = BTreeMap::<u32, Vec<u32>>::new();
        let mut conurbation = Conurbation::new();
        (1..num_cities + 1).for_each(|city_id| {
            conurbation_roads.insert(city_id, Vec::new());
            conurbation.add_city(city_id);
        });

        for _ in 1..num_cities {
            let (x, y) = as_connection(input_lines.next().unwrap());
            conurbation_roads.get_mut(&x).map(|conn| conn.push(y));
            conurbation_roads.get_mut(&y).map(|conn| conn.push(x));
        }

        let allowed_jumps_for_city: Vec<u32> = input_lines
            .next()
            .unwrap()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect();

        for city_id in 1..num_cities + 1 {
            let city_idx: usize = (city_id - 1).try_into()?;
            let allowed_jumps = allowed_jumps_for_city[city_idx];
            let mut visied_cities = HashSet::new();
            let current_city_id = city_id;

            for _ in 0..allowed_jumps {
                visied_cities.insert(current_city_id);
            }
        }
    }

    Ok(())
}
