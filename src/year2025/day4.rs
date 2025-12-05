use aoc::{NeighborCoords, iter_neighbors};

const MAX_ADJACENT: usize = 4;

pub struct Solution;

impl crate::Solution for Solution {
    fn part_one(input: &str) {
        let grid = parse_grid(input);
        println!(
            "{}",
            grid.iter()
                .enumerate()
                .map(|(row, tiles)| tiles
                    .iter()
                    .enumerate()
                    .filter(|&(col, is_roll)| {
                        *is_roll
                            && iter_neighbors(&grid, row, col).filter(|&t| *t).count()
                                < MAX_ADJACENT
                    })
                    .count())
                .sum::<usize>()
        );
    }

    fn part_two(input: &str) {
        let mut grid = parse_grid(input);
        let rows = grid.len();
        let cols = grid[0].len();
        let mut sum = 0;

        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col]
                    && iter_neighbors(&grid, row, col).filter(|&t| *t).count() < MAX_ADJACENT
                {
                    sum += chain_removals(row, col, &mut grid)
                }
            }
        }

        //let mut grid = parse_grid(input);
        //let rows = grid.len();
        //let cols = grid[0].len();
        //let mut sum = 0;
        //
        //loop {
        //    let mut removed = 0;
        //    for row in 0..rows {
        //        for col in 0..cols {
        //            if grid[row][col] && num_adjacent_where(row, col, |t| *t, &grid) < MAX_ADJACENT {
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

        println!("{sum}");
    }
}

fn chain_removals(row: usize, col: usize, grid: &mut [Vec<bool>]) -> usize {
    if !grid[row][col] {
        0
    } else {
        grid[row][col] = false;
        1 + NeighborCoords::new(row, col, grid.len(), grid[0].len())
            .filter_map(|(r, c)| {
                (grid[r][c] && iter_neighbors(&grid, r, c).filter(|&t| *t).count() < MAX_ADJACENT)
                    .then(|| chain_removals(r, c, grid))
            })
            .sum::<usize>()
    }
}

fn parse_grid(grid: &str) -> Vec<Vec<bool>> {
    grid.lines()
        .map(|row| row.chars().map(|ch| ch == '@').collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>()
}
