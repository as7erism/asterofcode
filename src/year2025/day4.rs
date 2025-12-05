use aoc::{NeighborCoords, iter_neighbors};

const MAX_ADJACENT: usize = 4;

pub struct Solution;

impl crate::Solution for Solution {
    type OutputOne = usize;

    fn part_one(input: &str) -> Self::OutputOne {
        let grid = parse_grid(input);
        grid.iter()
            .enumerate()
            .map(|(row, tiles)| {
                tiles
                    .iter()
                    .enumerate()
                    .filter(|&(col, is_roll)| {
                        *is_roll
                            && iter_neighbors(&grid, row, col).filter(|&t| *t).count()
                                < MAX_ADJACENT
                    })
                    .count()
            })
            .sum::<usize>()
    }

    fn part_two(input: &str) -> Self::OutputTwo {
        let mut grid = parse_grid(input);
        let rows = grid.len();
        let cols = grid[0].len();
        let mut sum = 0;

        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col]
                    && iter_neighbors(&grid, row, col).filter(|&t| *t).count() < MAX_ADJACENT
                {
                    sum += chain_removals(&mut grid, row, col)
                }
            }
        }

        // multipass solution:
        //let mut grid = parse_grid(input);
        //let rows = grid.len();
        //let cols = grid[0].len();
        //let mut sum = 0;
        //
        //loop {
        //    let mut removed = 0;
        //    for row in 0..rows {
        //        for col in 0..cols {
        //            if grid[row][col]
        //                && iter_neighbors(&grid, row, col).filter(|&t| *t).count() < MAX_ADJACENT
        //            {
        //                grid[row][col] = false;
        //                removed += 1;
        //            }
        //        }
        //    }
        //    if removed > 0 {
        //        sum += removed;
        //    } else {
        //        break;
        //    }
        //}

        sum
    }
}

fn chain_removals(grid: &mut [Vec<bool>], row: usize, col: usize) -> usize {
    if !grid[row][col] {
        0
    } else {
        grid[row][col] = false;
        let neighbors_iter = NeighborCoords::in_grid(&grid, row, col);
        1 + neighbors_iter
            .filter_map(|(r, c)| {
                if grid[r][c] && iter_neighbors(&grid, r, c).filter(|&t| *t).count() < MAX_ADJACENT
                {
                    Some(chain_removals(grid, r, c))
                } else {
                    None
                }
            })
            .sum::<usize>()
    }
}

fn parse_grid(grid: &str) -> Vec<Vec<bool>> {
    grid.lines()
        .map(|row| row.chars().map(|ch| ch == '@').collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>()
}
