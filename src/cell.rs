use crate::YoungDiagram;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cell {
    pub row: usize,
    pub col: usize,
}

impl Cell {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn is_in(&self, diag: &YoungDiagram) -> bool {
        match diag.row_len(self.row) {
            Some(len) => self.col <= len,
            None => false,
        }
    }

    pub fn arm(&self, diag: &YoungDiagram) -> usize {
        diag.row_len(self.row).unwrap_or(0).saturating_sub(self.col)
    }

    pub fn leg(&self, diag: &YoungDiagram) -> usize {
        diag.col_height(self.col).saturating_sub(self.row)
    }

    pub fn hook_length(&self, diag: &YoungDiagram) -> usize {
        self.arm(diag) + self.leg(diag) + 1
    }

    pub fn content(&self) -> i64 {
        self.col as i64 - self.row as i64
    }

    pub fn hook(&self, diag: &YoungDiagram) -> Hook {
        Hook {
            cell: *self,
            arm_len: self.arm(diag),
            leg_len: self.leg(diag),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Content(pub i64);

#[derive(Debug, Clone)]
pub struct Hook {
    pub cell: Cell,
    pub arm_len: usize,
    pub leg_len: usize,
}

impl Hook {
    pub fn length(&self) -> usize {
        self.arm_len + self.leg_len + 1
    }
}