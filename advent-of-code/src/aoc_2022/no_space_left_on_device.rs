use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char as character, u128},
    combinator::map,
    combinator::rest,
    error::ErrorKind as EK,
    sequence::{preceded, separated_pair},
};
use std::collections::HashMap;

pub enum PathComponent {
    Directory {
        name: String,
        children: HashMap<String, PathComponent>,
    },
    File(u128, String),
}

impl PathComponent {
    fn children(&self) -> &HashMap<String, PathComponent> {
        match self {
            PathComponent::Directory { children, .. } => children,
            _ => panic!(),
        }
    }
}

pub enum TerminalResult<'a> {
    CommandCd(&'a str),
    CommandLs,
    OutputFile(u128, &'a str),
    OutputDir(&'a str),
}

use TerminalResult::*;

pub struct NoSpaceLeftOnDevice;

impl crate::AdventDayProblem for NoSpaceLeftOnDevice {
    type Arg = PathComponent;
    type Ret = u128;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(mut dataset: impl Iterator<Item = String>) -> Self::Arg {
        let mut path_stack = Vec::new();

        let root_dir = PathComponent::Directory {
            name: "/".to_string(),
            children: HashMap::new(),
        };
        path_stack.push(&root_dir);
        dataset.next();

        dataset.for_each(|line| {
            let (_, result) = alt((
                map(preceded(tag("$ cd "), rest::<_, (_, EK)>), |location| {
                    CommandCd(location)
                }),
                map(tag("$ ls"), |_| CommandLs),
                map(preceded(tag("dir "), rest::<_, (_, EK)>), |name| {
                    OutputDir(name)
                }),
                map(
                    separated_pair(u128, character(' '), rest::<_, (_, EK)>),
                    |(file_size, file_name)| OutputFile(file_size, file_name),
                ),
            ))(&line)
            .unwrap();

            match result {
                CommandCd(location) if location == ".." => {
                    path_stack.pop();
                }
                CommandCd(location) => {
                    let current_dir = path_stack.last().unwrap();
                    path_stack.push(current_dir.children().get(location).unwrap());
                }
                CommandLs => {}
                OutputFile(file_size, file_name) => {
                    let dir_children: *const _ = path_stack.last().unwrap().children();
                    unsafe { &mut *(dir_children as *mut HashMap<_, _>) }.insert(
                        file_name.to_string(),
                        PathComponent::File(file_size, file_name.to_string()),
                    );
                }
                OutputDir(file_name) => {
                    let dir_children: *const _ = path_stack.last().unwrap().children();
                    unsafe { &mut *(dir_children as *mut HashMap<_, _>) }.insert(
                        file_name.to_string(),
                        PathComponent::Directory {
                            name: file_name.to_string(),
                            children: HashMap::new(),
                        },
                    );
                }
            }
        });

        root_dir
    }

    fn part_1(root: Self::Arg) -> Self::Ret {
        let mut dir_size_table = HashMap::new();
        traverse_compute_directory_sizes(&root, "/".to_string(), &mut dir_size_table);

        dir_size_table
            .values()
            .filter(|&&dir_size| dir_size < 100000)
            .sum()
    }

    fn part_2(root: Self::Arg) -> Self::Ret {
        let mut dir_size_table = HashMap::new();
        traverse_compute_directory_sizes(&root, "/".to_string(), &mut dir_size_table);

        let left_disk_space = 70000000 - dir_size_table[&"/".to_string()];
        let required_disk_space = 30000000 - left_disk_space;

        *dir_size_table
            .values()
            .filter(|&&dir_size| dir_size >= required_disk_space)
            .min()
            .unwrap()
    }
}

fn traverse_compute_directory_sizes(
    root: &PathComponent,
    path: String,
    dir_size_table: &mut HashMap<String, u128>,
) -> u128 {
    match root {
        PathComponent::File(file_size, _) => *file_size,
        PathComponent::Directory { children, .. } => {
            let dir_size = children
                .iter()
                .map(|(dir_name, component)| {
                    traverse_compute_directory_sizes(
                        component,
                        format!("{path}{dir_name}/"),
                        dir_size_table,
                    )
                })
                .sum::<u128>();

            dir_size_table.insert(path, dir_size);
            dir_size
        }
    }
}
