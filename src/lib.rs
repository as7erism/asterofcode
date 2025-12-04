use std::cmp::min;

pub trait NumDigits {
    fn num_digits(&self) -> u8;
}

impl NumDigits for u64 {
    fn num_digits(&self) -> u8 {
        (self.ilog10() + 1) as u8
    }
}

pub fn num_adjacent<T, P>(row: usize, col: usize, predicate: P, grid: &[Vec<T>]) -> usize
where
    P: Fn(&T) -> bool,
{
    grid[row.saturating_sub(1)..=min(row + 1, grid.len() - 1)]
        .iter()
        .map(|tiles| {
            tiles[col.saturating_sub(1)..=min(col + 1, grid.len() - 1)]
                .iter()
                .filter(|&t| predicate(t))
                .count()
        })
        .sum::<usize>()
        - if predicate(&grid[row][col]) { 1 } else { 0 }
}

pub struct IterAdjacent {
    init_row: usize,
    row: usize,
    row_bound: usize,
    init_col: usize,
    col: usize,
    col_bound: usize,
}

impl Iterator for IterAdjacent {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row == self.row_bound {
            None
        } else if self.col == self.col_bound {
            self.col = self.init_row.saturating_sub(1);
            self.row += 1;
            self.next()
        } else if self.row == self.init_row && self.col == self.init_col {
            self.col += 1;
            self.next()
        } else {
            self.col += 1;
            Some((self.row, self.col - 1))
        }
    }

    // TODO make this exact size
}

impl IterAdjacent {
    pub fn new(init_row: usize, init_col: usize, row_bound: usize, col_bound: usize) -> Self {
        IterAdjacent {
            init_row,
            row: init_row.saturating_sub(1),
            row_bound,
            init_col,
            col: init_col.saturating_sub(1),
            col_bound,
        }
    }
}
