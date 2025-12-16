use std::{cmp::min, iter::FusedIterator, num::NonZeroUsize};

pub trait NumDigits {
    fn num_digits(&self, radix: u32) -> u8;
}

impl NumDigits for u64 {
    fn num_digits(&self, radix: u32) -> u8 {
        (self.ilog(radix as u64) + 1) as u8
    }
}

pub fn iter_neighbors<'a, Outer, Inner, T: 'a>(
    grid: &'a Outer,
    row: usize,
    col: usize,
) -> impl Iterator<Item = &'a T>
where
    Outer: AsRef<[Inner]>,
    Inner: AsRef<[T]> + 'a,
{
    let rows = grid.as_ref().len();
    let cols = grid.as_ref()[0].as_ref().len();
    assert!(row < rows);
    assert!(col < cols);

    grid.as_ref()[row.saturating_sub(1)..=min(row + 1, rows - 1)]
        .iter()
        .enumerate()
        .flat_map(move |(r, tiles)| {
            tiles.as_ref()[col.saturating_sub(1)..=min(col + 1, cols - 1)]
                .iter()
                .enumerate()
                .filter(move |(c, _)| {
                    r + row.saturating_sub(1) != row || c + col.saturating_sub(1) != col
                })
                .map(|(_, tile)| tile)
        })
}

pub fn iter_neighbors_mut<'a, Outer, Inner, T: 'a>(
    grid: &'a mut Outer,
    row: usize,
    col: usize,
) -> impl Iterator<Item = &'a mut T>
where
    Outer: AsMut<[Inner]>,
    Inner: AsMut<[T]> + 'a,
{
    let rows = grid.as_mut().len();
    let cols = grid.as_mut()[0].as_mut().len();
    assert!(row < rows);
    assert!(col < cols);

    grid.as_mut()[row.saturating_sub(1)..=min(row + 1, rows - 1)]
        .iter_mut()
        .enumerate()
        .flat_map(move |(r, tiles)| {
            tiles.as_mut()[col.saturating_sub(1)..=min(col + 1, cols - 1)]
                .iter_mut()
                .enumerate()
                .filter(move |(c, _)| {
                    r + row.saturating_sub(1) != row || c + col.saturating_sub(1) != col
                })
                .map(|(_, tile)| tile)
        })
}

/// iterator over the coordinates adjacent to a given row and column in a given grid.
///
/// this doesn't retain a reference to the grid on which it operates, so this is useful for indexed
/// mutations. it is also [`ExactSizeIterator`].
#[derive(Debug, Clone)]
pub struct NeighborCoords {
    // TODO space optimize this with i8 offsets
    init_row: usize,
    current_row: usize,
    row_bound: usize,
    init_col: usize,
    current_col: usize,
    col_bound: usize,
}

impl ExactSizeIterator for NeighborCoords {}
impl FusedIterator for NeighborCoords {}

impl NeighborCoords {
    pub fn new(init_row: usize, init_col: usize, row_bound: usize, col_bound: usize) -> Self {
        NeighborCoords {
            init_row,
            current_row: init_row.saturating_sub(1),
            row_bound,
            init_col,
            current_col: init_col.saturating_sub(1),
            col_bound,
        }
    }

    // TODO genericize this :3
    pub fn in_grid<_T>(row: usize, col: usize, grid: &[Vec<_T>]) -> Self {
        assert!(row < grid.len());
        assert!(col < grid[0].len());
        NeighborCoords::new(row, col, grid.len(), grid[0].len())
    }

    fn row_lower_bound(&self) -> usize {
        self.init_row.saturating_sub(1)
    }

    fn col_lower_bound(&self) -> usize {
        self.init_col.saturating_sub(1)
    }

    fn row_upper_bound(&self) -> usize {
        min(self.row_bound, self.init_row + 2)
    }

    fn col_upper_bound(&self) -> usize {
        min(self.col_bound, self.init_col + 2)
    }

    fn rows(&self) -> usize {
        self.row_upper_bound() - self.row_lower_bound()
    }

    fn cols(&self) -> usize {
        self.col_upper_bound() - self.col_lower_bound()
    }
}

impl Iterator for NeighborCoords {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_row == self.row_upper_bound() {
            None
        } else if self.current_col == self.col_upper_bound() {
            self.current_col = self.col_lower_bound();
            self.current_row += 1;
            self.next()
        } else if self.current_row == self.init_row && self.current_col == self.init_col {
            self.current_col += 1;
            self.next()
        } else {
            self.current_col += 1;
            Some((self.current_row, self.current_col - 1))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let total = self.rows() * self.cols();
        let visited = (self.current_row - self.row_lower_bound()) * self.cols()
            + (self.current_col - self.col_lower_bound());

        // subtract 1 if we haven't skipped the middle yet
        if (self.init_row - self.row_lower_bound()) * self.cols()
            + (self.init_col - self.col_lower_bound())
            >= (self.current_row - self.row_lower_bound()) * self.cols()
                + (self.current_col - self.col_lower_bound())
        {
            (total - visited - 1, Some(total - visited - 1))
        } else {
            (total - visited, Some(total - visited))
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct DisjoinSetsNode {
    parent_plus_one: Option<NonZeroUsize>,
    rank: u32,
}

// TODO maybe reimplement with refcell so api isnt all mutating functions
#[derive(Debug, Clone)]
pub struct DisjointSets {
    nodes: Vec<DisjoinSetsNode>,
}

impl DisjointSets {
    pub fn new(size: usize) -> Self {
        DisjointSets {
            nodes: vec![Default::default(); size],
        }
    }

    /// returns the set representative of `key` if `key` is a member of a set.
    pub fn find(&mut self, key: usize) -> usize {
        let mut root = key;
        while let Some(parent_plus_one) = self.nodes[root].parent_plus_one {
            root = parent_plus_one.get() - 1;
        }

        let mut y = key;
        while y != root {
            // this is suspect
            let temp = self.nodes[y];
            self.nodes[y] = DisjoinSetsNode {
                parent_plus_one: Some(NonZeroUsize::new(root + 1).unwrap()),
                rank: temp.rank,
            };
            y = temp.parent_plus_one.unwrap().get() - 1;
        }

        root
    }

    /// combines the sets that contain `k1` and `k2`.
    pub fn union(&mut self, k1: usize, k2: usize) -> bool {
        let r1 = self.find(k1);
        let r2 = self.find(k2);

        if r1 == r2 {
            return false;
        }

        if self.nodes[r1].rank < self.nodes[r2].rank {
            self.nodes[r1].parent_plus_one = Some(NonZeroUsize::new(r2 + 1).unwrap());
            if self.nodes[r2].rank == self.nodes[r2].rank {
                self.nodes[r1].rank += 1;
            }
        } else {
            self.nodes[r2].parent_plus_one = Some(NonZeroUsize::new(r1 + 1).unwrap());
            if self.nodes[r1].rank == self.nodes[r2].rank {
                self.nodes[r2].rank += 1;
            }
        }

        true
    }

    pub fn collect(&mut self) -> Vec<Vec<usize>> {
        let mut sets = vec![vec![]; self.nodes.len()];

        for i in 0..self.nodes.len() {
            sets[self.find(i)].push(i);
        }

        sets.into_iter().filter(|set| !set.is_empty()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yields_expected() {
        let grid = vec![vec![(), (), ()], vec![(), (), ()], vec![(), (), ()]];

        let actual = NeighborCoords::in_grid(1, 0, &grid).collect::<Vec<_>>();
        let expected = vec![(0, 0), (0, 1), (1, 1), (2, 0), (2, 1)];
        assert_eq!(actual, expected);

        let actual = NeighborCoords::in_grid(0, 1, &grid).collect::<Vec<_>>();
        let expected = vec![(0, 0), (0, 2), (1, 0), (1, 1), (1, 2)];
        assert_eq!(actual, expected);

        let actual = NeighborCoords::in_grid(1, 1, &grid).collect::<Vec<_>>();
        let expected = vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2),
        ];
        assert_eq!(actual, expected);

        let actual = NeighborCoords::in_grid(1, 2, &grid).collect::<Vec<_>>();
        let expected = vec![(0, 1), (0, 2), (1, 1), (2, 1), (2, 2)];
        assert_eq!(actual, expected);

        let actual = NeighborCoords::in_grid(2, 1, &grid).collect::<Vec<_>>();
        let expected = vec![(1, 0), (1, 1), (1, 2), (2, 0), (2, 2)];
        assert_eq!(actual, expected);

        let big_grid = vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
        ];

        let actual = NeighborCoords::in_grid(4, 2, &big_grid).collect::<Vec<_>>();
        let expected = vec![
            (3, 1),
            (3, 2),
            (3, 3),
            (4, 1),
            (4, 3),
            (5, 1),
            (5, 2),
            (5, 3),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_exact_size() {
        let grid = vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
        ];

        let mut iter = NeighborCoords::in_grid(1, 0, &grid);
        assert_eq!(iter.len(), 5);
        iter.next();
        assert_eq!(iter.len(), 4);
        iter.next();
        assert_eq!(iter.len(), 3);
        iter.next();
        assert_eq!(iter.len(), 2);
        iter.next();
        assert_eq!(iter.len(), 1);
        iter.next();
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);

        let mut iter = NeighborCoords::in_grid(0, 1, &grid);
        assert_eq!(iter.len(), 5);
        iter.next();
        assert_eq!(iter.len(), 4);
        iter.next();
        assert_eq!(iter.len(), 3);
        iter.next();
        assert_eq!(iter.len(), 2);
        iter.next();
        assert_eq!(iter.len(), 1);
        iter.next();
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);

        let mut iter = NeighborCoords::in_grid(4, 5, &grid);
        assert_eq!(iter.len(), 5);
        iter.next();
        assert_eq!(iter.len(), 4);
        iter.next();
        assert_eq!(iter.len(), 3);
        iter.next();
        assert_eq!(iter.len(), 2);
        iter.next();
        assert_eq!(iter.len(), 1);
        iter.next();
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);

        let mut iter = NeighborCoords::in_grid(5, 4, &grid);
        assert_eq!(iter.len(), 5);
        iter.next();
        assert_eq!(iter.len(), 4);
        iter.next();
        assert_eq!(iter.len(), 3);
        iter.next();
        assert_eq!(iter.len(), 2);
        iter.next();
        assert_eq!(iter.len(), 1);
        iter.next();
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);

        let mut iter = NeighborCoords::in_grid(4, 2, &grid);
        assert_eq!(iter.len(), 8);
        iter.next();
        assert_eq!(iter.len(), 7);
        iter.next();
        assert_eq!(iter.len(), 6);
        iter.next();
        assert_eq!(iter.len(), 5);
        iter.next();
        assert_eq!(iter.len(), 4);
        iter.next();
        assert_eq!(iter.len(), 3);
        iter.next();
        assert_eq!(iter.len(), 2);
        iter.next();
        assert_eq!(iter.len(), 1);
        iter.next();
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
    }
}
