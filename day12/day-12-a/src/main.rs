// https://adventofcode.com/2022/day/12

use std::collections::{BinaryHeap, HashSet};

type Point = (i32, i32);
type PointWithElevation = (Point, i32);

struct Map {
    grid: Vec<Vec<i32>>,
    position: Point,
    goal: Point,
}

impl Map {
    pub fn get_neighbours(&self, position: Point) -> Vec<PointWithElevation> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .map(|(x, y)| (position.0 + x, position.1 + y))
            .filter(|(x, y)| {
                *x >= 0 && (*x as usize) < self.grid.first().unwrap().len() && *y >= 0 && (*y as usize) < self.grid.len()
            })
            .map(|x| ((x), self.get_elevation(x)))
            .collect()
    }

    pub fn get_elevation(&self, position: Point) -> i32 {
        self.grid[position.1 as usize][position.0 as usize]
    }
}

fn main() {
    let mut grid: Vec<Vec<i32>> = vec![];
    let mut position: Point = (0, 0);
    let mut goal: Point = (0, 0);
    let mut row: Vec<i32> = vec![];
    include_str!("../input.txt")
        .lines()
        .enumerate()
        .for_each(|x| {
            x.1.chars()
                .enumerate()
                .for_each(|c| {
                    match c.1 {
                        'E' => {
                            goal = (x.0 as i32, c.0 as i32);
                            row.push('z' as i32 - 48)
                        }
                        'S' => {
                            position = (x.0 as i32, c.0 as i32);
                            row.push('a' as i32 - 48)
                        }
                        _ => row.push(c.1 as i32 - 48),
                    }
                }
                );
            grid.push(row.clone());
            row.clear();
        });
    let map = Map {
        grid,
        position,
        goal,
    };
    let mut current_position = map.position;
    let mut pq = BinaryHeap::new();
    let mut visited: HashSet<Point> = HashSet::from([current_position.clone()]);
    let mut steps = 0;
    pq.push((current_position, 0));
    while let Some((position, cost)) = pq.pop() {
        current_position = position;
        if current_position == map.goal {
            steps = cost;
            break;
        }
        let current_elevation = map.get_elevation(current_position);
        let candidates: Vec<PointWithElevation> = map.get_neighbours(current_position)
            .into_iter()
            .filter(|x| {
                let height = map.get_elevation(x.0);
                height >= current_elevation || height == current_elevation - 1
            })
            .collect();
        for candidate in candidates {
            if !visited.contains(&candidate.0) {
                visited.insert(candidate.0);
                println!("{:?}", candidate.0);
                pq.push((candidate.0, cost));
            }
        }
    }
    println!("{}", steps);
}