use std::fmt;
use crate::{Partition, Cell};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct YoungDiagram {
    partition: Partition,
    size: usize,
}

impl YoungDiagram {
    pub fn from_partition(partition: Partition) -> Self {
        let size = partition.size();
        Self { partition, size }
    }

    pub fn partition(&self) -> &Partition {
        &self.partition
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn num_rows(&self) -> usize {
        self.partition.len()
    }

    pub fn num_cols(&self) -> usize {
        self.partition.get(0).unwrap_or(0)
    }

    pub fn row_len(&self, i: usize) -> Option<usize> {
        self.partition.get(i - 1)
    }

    pub fn col_height(&self, j: usize) -> usize {
        self.partition.iter().filter(|&&r| r >= j).count()
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn cells(&self) -> impl Iterator<Item = Cell> + '_ {
        let rows = self.num_rows();
        (1..=rows).flat_map(move |r| {
            let len = self.row_len(r).unwrap_or(0);
            (1..=len).map(move |c| Cell::new(r, c))
        })
    }

    pub fn hook_lengths(&self) -> impl Iterator<Item = usize> + '_ {
        self.cells().map(|cell| cell.hook_length(self))
    }

    pub fn conjugate(&self) -> Self {
        let max_col = self.num_cols();
        let mut conj_parts = Vec::with_capacity(max_col);
        for j in 1..=max_col {
            conj_parts.push(self.col_height(j));
        }
        Self::from_partition(Partition::new(conj_parts).unwrap())
    }

    fn exists(&self, r: usize, c: usize) -> bool {
        if r == 0 || c == 0 { return false; }
        self.row_len(r).map_or(false, |len| c <= len)
    }
}

impl fmt::Display for YoungDiagram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cell_w = 2usize;
        let n_rows = self.num_rows();
        let n_cols = self.num_cols();
        if n_rows == 0 { return Ok(()); }

        let exists = |r: usize, c: usize| -> bool { self.exists(r, c) };

        let h_right = |i: usize, j: usize| -> bool {
            exists(i + 1, j + 1) || (i > 0 && exists(i, j + 1))
        };
        let h_left = |i: usize, j: usize| -> bool {
            if j == 0 { return false; }
            exists(i + 1, j) || (i > 0 && exists(i, j))
        };
        let v_down = |i: usize, j: usize| -> bool {
            exists(i + 1, j + 1) || (j > 0 && exists(i + 1, j))
        };
        let v_up = |i: usize, j: usize| -> bool {
            if i == 0 { return false; }
            exists(i, j + 1) || (j > 0 && exists(i, j))
        };

        let cross = |up: bool, down: bool, left: bool, right: bool| -> char {
            match (up, down, left, right) {
                (false, true, false, true) => '┌',
                (false, true, true, false) => '┐',   // ← 原来是 '└'
                (false, true, true, true) => '┬',
                (true, false, false, true) => '└',   // ← 原来是 '┐'
                (true, false, true, false) => '┘',
                (true, false, true, true) => '┴',
                (true, true, false, true) => '├',
                (true, true, true, false) => '┤',
                (true, true, true, true) => '┼',
                (false, false, true, true) => '─',
                (false, false, true, false) => '─',
                (false, false, false, true) => '─',
                (true, true, false, false) => '│',
                (false, true, false, false) => '│',
                (true, false, false, false) => '│',
                _ => ' ',
            }
        };

        let grid_rows = 2 * n_rows + 1;
        let grid_cols = n_cols * (cell_w + 1) + 1;
        let mut grid = vec![vec![' '; grid_cols]; grid_rows];

        for i in 0..=n_rows {
            let r_idx = 2 * i;
            for j in 0..=n_cols {
                let c_idx = j * (cell_w + 1);
                grid[r_idx][c_idx] = cross(v_up(i, j), v_down(i, j), h_left(i, j), h_right(i, j));
            }
            for j in 0..n_cols {
                let start = j * (cell_w + 1) + 1;
                let end = (j + 1) * (cell_w + 1);
                if h_right(i, j) {
                    for col in start..end {
                        grid[r_idx][col] = '─';
                    }
                }
            }
        }

        for i in 0..n_rows {
            let r_idx = 2 * i + 1;
            for j in 0..=n_cols {
                let c_idx = j * (cell_w + 1);
                if v_down(i, j) {
                    grid[r_idx][c_idx] = '│';
                }
            }
        }

        for row in grid {
            let s: String = row.into_iter().collect();
            let trimmed = s.trim_end();
            if !trimmed.is_empty() {
                writeln!(f, "{}", trimmed)?;
            }
        }
        Ok(())
    }
}