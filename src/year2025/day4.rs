use aoc::{IterAdjacent, num_adjacent};

const MAX_ADJACENT: usize = 4;

pub fn run(part: Option<u8>, input: &str) {
    match part {
        Some(p) => {
            if p == 1 {
                part_one(input);
            } else {
                part_two(input);
            }
        }
        None => {
            part_one(input);
            part_two(input);
        }
    }
}

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
                    *is_roll && num_adjacent(row, col, |t| *t, &grid) < MAX_ADJACENT
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
            if grid[row][col] && num_adjacent(row, col, |t| *t, &grid) < MAX_ADJACENT {
                sum += chain_removals(row, col, &mut grid)
            }
        }
    }

    println!("{sum}");
}

fn chain_removals(row: usize, col: usize, grid: &mut [Vec<bool>]) -> usize {
    if !grid[row][col] {
        0
    } else {
        grid[row][col] = false;
        1 + IterAdjacent::new(row, col, grid.len(), grid[0].len())
            .filter_map(|(r, c)| {
                (grid[r][c] && num_adjacent(r, c, |t| *t, grid) < MAX_ADJACENT)
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
