// https://adventofcode.com/2022/day/7

use indextree::{Arena, NodeId};
use std::fmt;

use crate::NodeType::{Directory, File};

#[derive(PartialEq)]
enum CommandType {
    Ls,
    Cd,
}

impl CommandType {
    pub(crate) fn new(input: &str) -> Self {
        if input == "cd" {
            CommandType::Cd
        } else {
            CommandType::Ls
        }
    }
}

impl fmt::Display for CommandType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandType::Ls => {
                write!(f, "ls")
            }
            CommandType::Cd => {
                write!(f, "cd")
            }
        }
    }
}

enum Input {
    Command {
        command_type: CommandType,
        argument: Option<String>,
    },
    LsDirectory {
        name: String,
    },
    LsFile {
        name: String,
        size: usize,
    },
}

impl fmt::Display for Input {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Input::Command {
                command_type,
                argument,
            } => {
                write!(
                    f,
                    "$ {} {}",
                    command_type,
                    match argument {
                        None => {
                            "".to_string()
                        }
                        Some(argument) => {
                            argument.to_string()
                        }
                    }
                )
            }
            Input::LsDirectory { name } => {
                write!(f, "dir {}", name)
            }
            Input::LsFile { name, size } => {
                write!(f, "{} {}", size, name)
            }
        }
    }
}

impl Input {
    pub(crate) fn new(input: &str) -> Self {
        if input.starts_with("$ ") {
            Input::Command {
                command_type: CommandType::new(input.split(" ").nth(1).unwrap()),
                argument: input.split(" ").nth(2).map(|x| x.to_string()),
            }
        } else if input.starts_with("dir ") {
            Input::LsDirectory {
                name: input.split(" ").nth(1).unwrap().to_string(),
            }
        } else {
            Input::LsFile {
                name: input.split(" ").nth(1).unwrap().to_string(),
                size: input.split(" ").nth(0).unwrap().parse::<usize>().unwrap(),
            }
        }
    }
}

#[derive(PartialEq)]
enum NodeType {
    Directory,
    File,
}

struct Node {
    node_type: NodeType,
    name: String,
    size: usize,
}

impl Node {
    pub(crate) fn new(node_type: NodeType) -> Self {
        Node {
            node_type,
            name: "/".to_string(),
            size: 0,
        }
    }
}

fn main() {
    let arena = &mut Arena::new();
    let root = arena.new_node(Node::new(Directory));
    let mut current_node = root;
    let mut directories: Vec<NodeId> = vec![];
    include_str!("../input.txt")
        .lines()
        .into_iter()
        .for_each(|x| {
            let input = Input::new(x);
            match input {
                Input::Command {
                    command_type,
                    argument,
                } => {
                    if command_type == CommandType::Cd
                        && argument.is_some()
                        && argument.clone().unwrap() == "/"
                    {
                        current_node = root;
                    } else if command_type == CommandType::Cd {
                        let argument = argument.unwrap();
                        if argument == ".." {
                            current_node = arena[current_node].parent().unwrap();
                        } else {
                            current_node = current_node
                                .children(arena)
                                .find(|&child| {
                                    let x = arena[child].get();
                                    x.node_type == Directory && x.name == argument
                                })
                                .unwrap();
                        }
                    }
                }
                Input::LsDirectory { name } => {
                    let directory = arena.new_node(Node {
                        node_type: Directory,
                        name,
                        size: 0,
                    });
                    directories.push(directory);
                    current_node.append(directory, arena);
                }
                Input::LsFile { name, size } => {
                    let file = arena.new_node(Node {
                        node_type: File,
                        name,
                        size,
                    });
                    current_node.append(file, arena);
                }
            }
        });
    let dir_sizes: Vec<usize> = directories.iter().map(|dir| dir_size(dir, arena)).collect();
    let part_1 = dir_sizes
        .iter()
        .filter(|x| x <= &&(100000 as usize))
        .sum::<usize>();
    println!("Part 1 : {}", part_1);
    let root_files_space: usize = root
        .children(arena)
        .map(|x| match arena[x].get().node_type {
            Directory => 0,
            File => arena[x].get().size,
        })
        .sum();
    let space_available = 70000000;
    let space_needed = 30000000;
    let total_used_space: usize = dir_sizes.iter().sum::<usize>() + root_files_space;
    let free_space = space_available - total_used_space;
    let free_up = space_needed - free_space;
    println!(
        "Part 2 : {}",
        &dir_sizes.iter().filter(|&&x| x >= free_up).min().unwrap()
    );
}

fn dir_size(node: &NodeId, arena: &Arena<Node>) -> usize {
    match arena[*node].get() {
        Node {
            node_type,
            name: _,
            size,
        } => match node_type {
            Directory => node
                .children(arena)
                .map(|x| match arena[x].get().node_type {
                    Directory => dir_size(&x, arena),
                    File => arena[x].get().size.clone(),
                })
                .sum(),
            File => size.clone(),
        },
    }
}
