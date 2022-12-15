use nom::{
    bytes::complete::tag,
    character::complete::i128 as integer,
    combinator::map,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};
use rayon::prelude::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use std::collections::BTreeSet;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Location(i128, i128);

impl std::ops::BitXor for Location {
    type Output = i128;

    fn bitxor(self, other: Self) -> Self::Output {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

pub struct Sensor {
    location: Location,
    reach: i128,
}

pub type Beacon = Location;

fn parse_location(input: &str) -> IResult<&str, Location> {
    map(
        separated_pair(
            preceded(tag("x="), integer),
            tag(", "),
            preceded(tag("y="), integer),
        ),
        |(x, y)| Location(x, y),
    )(input)
}

fn parse_beacon(input: &str) -> IResult<&str, (Sensor, Beacon)> {
    preceded(
        tag("Sensor at "),
        separated_pair(
            parse_location,
            tag(": closest beacon is at "),
            parse_location,
        )
        .map(|(sensor_loc, beacon_loc)| {
            (
                Sensor {
                    location: sensor_loc,
                    reach: sensor_loc ^ beacon_loc,
                },
                beacon_loc,
            )
        }),
    )(input)
}

const ZONE_MARKER_VAR: i128 = 2000000;

pub struct BeaconExclusionZone;

impl crate::AdventDayProblem for BeaconExclusionZone {
    type Arg = (Vec<Sensor>, BTreeSet<Beacon>);
    type Ret = i128;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg {
        let mut sensors = Vec::new();
        let mut beacons = BTreeSet::new();

        dataset.for_each(|line| {
            let (_, (sensor, beacon)) = parse_beacon(&line).unwrap();
            sensors.push(sensor);
            beacons.insert(beacon);
        });

        (sensors, beacons)
    }

    fn part_1((sensors, beacons): Self::Arg) -> Self::Ret {
        let (min_x, max_x) = (
            i128::min(
                beacons.first().unwrap().0,
                sensors
                    .iter()
                    .min_by_key(|sn| sn.location.0)
                    .map(|sn| sn.location.0 - sn.reach)
                    .unwrap(),
            ),
            i128::max(
                beacons.last().unwrap().0,
                sensors
                    .iter()
                    .max_by_key(|sn| sn.location.0)
                    .map(|sn| sn.location.0 + sn.reach)
                    .unwrap(),
            ),
        );

        let y = ZONE_MARKER_VAR;
        (min_x..=max_x)
            .into_par_iter()
            .map(|x| Location(x, y))
            .filter(|loc| {
                !beacons.contains(&loc)
                    && sensors
                        .iter()
                        .any(|sensor| (sensor.location ^ *loc) <= sensor.reach)
            })
            .count() as i128
    }

    fn part_2((sensors, beacons): Self::Arg) -> Self::Ret {
        let search_space = ZONE_MARKER_VAR * 2;

        let distress_beacon = (0..=search_space)
            .flat_map(|x| (0..=search_space).map(move |y| Location(x, y)))
            .par_bridge()
            .find_any(|loc| {
                !beacons.contains(loc)
                    && sensors
                        .iter()
                        .all(|sensor| (sensor.location ^ *loc) > sensor.reach)
            })
            .unwrap();

        distress_beacon.0 * 4000000 + distress_beacon.1
    }
}
