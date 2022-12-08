use grid::{grid, Grid};
use itertools::Itertools;

fn to_grid(input: &str) -> Grid<u8> {
    let mut grid: Grid<u8> = grid![];
    let lines = input.lines().map(|x| {
        x.chars()
            .map(|c| c.to_digit(10).unwrap())
            .map(|digit| digit as u8)
            .collect_vec()
    });
    for row in lines {
        grid.push_row(row);
    }
    grid
}

fn is_visible(grid: &Grid<u8>, row: usize, col: usize) -> bool {
    let val = grid.get(row, col).unwrap();
    let grid_row: Vec<&u8> = grid.iter_row(row).collect();
    let grid_col: Vec<&u8> = grid.iter_col(col).collect();
    grid_row[0..col].iter().all(|x| x < &val) // left
        || grid_row[col+1..grid.cols()].iter().all(|x| x < &val) // right
        || grid_col[0..row].iter().all(|x| x < &val) // top
        || grid_col[row+1..grid.rows()].iter().all(|x| x < &val) // bot
}

fn get_scenic_score(grid: &Grid<u8>, row: usize, col: usize) -> u32 {
    let val = grid.get(row, col).unwrap();
    // Could probably be functionized
    // but I'm not gonna let perfect be the enemy of the good
    let mut left_col = 0;
    for i in (0..col).rev() {
        if grid.get(row, i).unwrap() >= val {
            left_col = i;
            break;
        }
    }
    let mut right_col = grid.cols() - 1;
    for i in (col + 1)..grid.cols() {
        if grid.get(row, i).unwrap() >= val {
            right_col = i;
            break;
        }
    }
    let mut top_row = 0;
    for i in (0..row).rev() {
        if grid.get(i, col).unwrap() >= val {
            top_row = i;
            break;
        }
    }
    let mut bottom_row = grid.rows() - 1;
    for i in (row + 1)..grid.rows() {
        if grid.get(i, col).unwrap() >= val {
            bottom_row = i;
            break;
        }
    }
    ((col - left_col) * (right_col - col) * (row - top_row) * (bottom_row - row)) as u32
}

fn part_a(input: &str) -> u32 {
    let grid = to_grid(input);
    let iter = (0..grid.rows()).cartesian_product(0..grid.cols());
    iter.into_iter()
        .filter(|(row, col)| is_visible(&grid, *row, *col))
        .count() as u32
}

fn part_b(input: &str) -> u32 {
    let grid = to_grid(input);
    let iter = (0..grid.rows()).cartesian_product(0..grid.cols());
    iter.into_iter()
        .map(|(row, col)| get_scenic_score(&grid, row, col))
        .max()
        .unwrap()
}

pub use crate::boilerplate;
boilerplate!(8, 21, 8, u32);
