use std::collections::HashMap;
use std::fmt;
use crate::{YoungDiagram, Cell};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YoungTableau<T> {
    shape: YoungDiagram,
    filling: HashMap<Cell, T>,
}

impl<T> YoungTableau<T> {
    pub fn new(shape: YoungDiagram) -> Self {
        Self {
            shape,
            filling: HashMap::new(),
        }
    }

    pub fn with_filling(shape: YoungDiagram, filling: HashMap<Cell, T>) -> Self {
        Self { shape, filling }
    }

    pub fn shape(&self) -> &YoungDiagram {
        &self.shape
    }

    pub fn get(&self, cell: &Cell) -> Option<&T> {
        self.filling.get(cell)
    }

    pub fn set(&mut self, cell: Cell, value: T) {
        self.filling.insert(cell, value);
    }

    pub fn filling(&self) -> &HashMap<Cell, T> {
        &self.filling
    }
}

impl<T: fmt::Display> YoungTableau<T> {
    pub fn display(&self) -> String {
        let n_rows = self.shape.num_rows();
        let n_cols = self.shape.num_cols();
        if n_rows == 0 { return String::new(); }

        let cell_w = self.filling.values()
            .map(|v| format!("{}", v).len())
            .max()
            .unwrap_or(1)
            .max(1);

        let exists = |r: usize, c: usize| -> bool {
            if r == 0 || c == 0 { return false; }
            self.shape.row_len(r).map_or(false, |len| c <= len)
        };

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

        for r in 1..=n_rows {
            for c in 1..=self.shape.row_len(r).unwrap_or(0) {
                let cell = Cell::new(r, c);
                let val_str = self.filling.get(&cell)
                    .map(|v| format!("{}", v))
                    .unwrap_or_else(|| " ".repeat(cell_w));
                let val_len = val_str.chars().count();
                let pad_left = (cell_w.saturating_sub(val_len)) / 2;
                let pad_right = cell_w.saturating_sub(val_len).saturating_sub(pad_left);

                let row_idx = 2 * (r - 1) + 1;
                let col_idx = (c - 1) * (cell_w + 1) + 1;

                for k in 0..pad_left {
                    if col_idx + k < grid_cols {
                        grid[row_idx][col_idx + k] = ' ';
                    }
                }
                for (k, ch) in val_str.chars().enumerate() {
                    if col_idx + pad_left + k < grid_cols {
                        grid[row_idx][col_idx + pad_left + k] = ch;
                    }
                }
                for k in 0..pad_right {
                    if col_idx + pad_left + val_len + k < grid_cols {
                        grid[row_idx][col_idx + pad_left + val_len + k] = ' ';
                    }
                }
            }
        }

        let mut lines = Vec::new();
        for row in grid {
            let s: String = row.into_iter().collect();
            let trimmed = s.trim_end();
            if !trimmed.is_empty() {
                lines.push(trimmed.to_string());
            }
        }
        lines.join("\n")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StandardYoungTableau {
    inner: YoungTableau<usize>,
}

impl StandardYoungTableau {
    pub fn try_new(shape: YoungDiagram, filling: HashMap<Cell, usize>) -> Result<Self, &'static str> {
        let n = shape.size();
        let mut used = vec![false; n + 1];

        for r in 1..=shape.num_rows() {
            for c in 1..=shape.row_len(r).unwrap_or(0) {
                let cell = Cell::new(r, c);
                let val = filling.get(&cell).copied()
                    .ok_or("存在未填充的格子")?;

                if val == 0 || val > n {
                    return Err("填值必须在 1..n 范围内");
                }
                if used[val] {
                    return Err("填值重复");
                }
                used[val] = true;
            }
        }

        for r in 1..=shape.num_rows() {
            let mut prev = 0usize;
            for c in 1..=shape.row_len(r).unwrap_or(0) {
                let val = filling[&Cell::new(r, c)];
                if val <= prev {
                    return Err("行必须严格递增");
                }
                prev = val;
            }
        }

        let max_col = shape.num_cols();
        for c in 1..=max_col {
            let mut prev = 0usize;
            for r in 1..=shape.col_height(c) {
                let val = filling[&Cell::new(r, c)];
                if val <= prev {
                    return Err("列必须严格递增");
                }
                prev = val;
            }
        }

        Ok(Self {
            inner: YoungTableau::with_filling(shape, filling),
        })
    }

    pub fn from_trusted(shape: YoungDiagram, filling: HashMap<Cell, usize>) -> Self {
        Self {
            inner: YoungTableau::with_filling(shape, filling),
        }
    }

    pub fn shape(&self) -> &YoungDiagram {
        self.inner.shape()
    }

    pub fn get(&self, cell: &Cell) -> Option<&usize> {
        self.inner.get(cell)
    }

    pub fn display(&self) -> String {
        self.inner.display()
    }
}