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
    let mut max_score = 0;
    for i in 0..trees.len() {
        let mut column: Vec<i32> = vec![];
        for j in 0..trees[i].len() {
            column.push(trees[j][i]);
        }
        for j in 0..trees[i].len() {
            let tree_height = trees[i][j];
            let left_slice = &trees[i][..j];
            let right_slice = &trees[i][j + 1..width];
            let mut top_slice: Vec<i32> = vec![];
            let mut bottom_slice: Vec<i32> = vec![];
            for k in 0..trees[i].len() {
                if k > i {
                    bottom_slice.push(trees[k][j]);
                } else if k < i {
                    top_slice.push(trees[k][j]);
                }
            }
            let mut top = 0;
            for k in (0..top_slice.len()).rev() {
                if top_slice[k] < tree_height {
                    top += 1;
                } else if top_slice[k] >= tree_height {
                    top += 1;
                    break;
                }
            }
            let mut bottom = 0;
            for k in 0..bottom_slice.len() {
                if bottom_slice[k] < tree_height {
                    bottom += 1;
                } else if bottom_slice[k] >= tree_height {
                    bottom += 1;
                    break;
                }
            }
            let mut left = 0;
            for k in (0..left_slice.len()).rev() {
                if left_slice[k] < tree_height {
                    left += 1;
                } else if left_slice[k] >= tree_height {
                    left += 1;
                    break;
                }
            }
            let mut right = 0;
            for k in 0..right_slice.len() {
                if right_slice[k] < tree_height {
                    right += 1;
                } else if right_slice[k] >= tree_height {
                    right += 1;
                    break;
                }
            }
            let score = top * bottom * right * left;
            if score > max_score {
                max_score = score;
            }
        }
    }
    println!("{}", max_score);
}