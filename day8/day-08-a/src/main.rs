// https://adventofcode.com/2022/day/8

fn main() {
    let mut trees: Vec<Vec<i32>> = vec![];
    include_str!("../input.txt")
        .lines()
        .into_iter()
        .for_each(|x| trees.push(
            x.chars()
                .map(|x| x as i32 - 48)
                .collect()
        )
        );
    let width = trees[0].len();
    let mut total_visible = trees.len() * 2 + (width - 2) * 2;
    for i in 1..trees.len() - 1 {
        for j in 1..trees[i].len() - 1 {
            let tree_height = trees[i][j];
            let left_slice = &trees[i][0..j];
            if tree_height > *left_slice.iter().max().unwrap() {
                total_visible += 1;
                continue;
            }
            let right_slice = &trees[i][j + 1..width];
            if tree_height > *right_slice.iter().max().unwrap() {
                total_visible += 1;
                continue;
            }
            let mut top_slice: Vec<i32> = vec![];
            let mut bottom_slice: Vec<i32> = vec![];
            for k in 0..trees[i].len() {
                if k > i {
                    bottom_slice.push(trees[k][j]);
                } else if k < i {
                    top_slice.push(trees[k][j]);
                }
            }
            if tree_height > *top_slice.iter().max().unwrap() || tree_height > *bottom_slice.iter().max().unwrap() {
                total_visible += 1;
            }
        }
    }
    println!("{}", total_visible);
}