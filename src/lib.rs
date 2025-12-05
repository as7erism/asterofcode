use std::cmp::min;

pub trait NumDigits {
    fn num_digits(&self, radix: u32) -> u8;
}

impl NumDigits for u64 {
    fn num_digits(&self, radix: u32) -> u8 {
        (self.ilog(radix as u64) + 1) as u8
    }
}

pub fn num_adjacent_where<T, P>(row: usize, col: usize, predicate: P, grid: &[Vec<T>]) -> usize
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
            self.col = self.col_lower_bound();
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        let total = self.rows() * self.cols();
        let visited = (self.row - self.row_lower_bound()) * self.rows()
            + (self.col - self.col_lower_bound());
        if (self.init_row - self.row_lower_bound()) * self.rows() + (self.init_col - self.col_lower_bound()) {};
        (0, None)
    }
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

    fn row_lower_bound(&self) -> usize {
        self.init_row.saturating_sub(1)
    }

    fn col_lower_bound(&self) -> usize {
        self.init_col.saturating_sub(1)
    }

    fn rows(&self) -> usize {
        self.row_bound - self.init_row.saturating_sub(1) - 1
    }

    fn cols(&self) -> usize {
        self.col_bound - self.init_col.saturating_sub(1) - 1
    }
}
