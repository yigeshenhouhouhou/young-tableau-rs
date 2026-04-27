use std::collections::HashMap;
use crate::{YoungDiagram, Cell};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkewShape {
    outer: YoungDiagram,
    inner: YoungDiagram,
    cells: Vec<Cell>,
}

impl SkewShape {
    pub fn new(outer: YoungDiagram, inner: YoungDiagram) -> Result<Self, &'static str> {
        if !outer.partition().contains(inner.partition()) {
            return Err("外分拆必须包含内分拆");
        }
        let cells: Vec<Cell> = outer.cells()
            .filter(|c| !c.is_in(&inner))
            .collect();
        Ok(Self { outer, inner, cells })
    }

    pub fn has_right(&self, cell: &Cell) -> bool {
        self.contains(&Cell::new(cell.row, cell.col + 1))
    }
    pub fn has_down(&self, cell: &Cell) -> bool {
        self.contains(&Cell::new(cell.row + 1, cell.col))
    }

    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn size(&self) -> usize {
        self.cells.len()
    }

    pub fn outer(&self) -> &YoungDiagram {
        &self.outer
    }

    pub fn inner(&self) -> &YoungDiagram {
        &self.inner
    }

    pub fn contains(&self, cell: &Cell) -> bool {
        cell.is_in(&self.outer) && !cell.is_in(&self.inner)
    }

    pub fn has_left(&self, cell: &Cell) -> bool {
        cell.col > 1 && self.contains(&Cell::new(cell.row, cell.col - 1))
    }

    pub fn has_up(&self, cell: &Cell) -> bool {
        cell.row > 1 && self.contains(&Cell::new(cell.row - 1, cell.col))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemiStandardSkewTableau {
    shape: SkewShape,
    filling: HashMap<Cell, usize>,
}

impl SemiStandardSkewTableau {
    pub fn from_trusted(shape: SkewShape, filling: HashMap<Cell, usize>) -> Self {
        Self { shape, filling }
    }

    pub fn shape(&self) -> &SkewShape {
        &self.shape
    }

    pub fn get(&self, cell: &Cell) -> Option<&usize> {
        self.filling.get(cell)
    }

    pub fn reading_word(&self) -> Vec<usize> {
        let mut word = Vec::with_capacity(self.shape.size());
        let max_row = self.shape.outer().num_rows();
        
        // 修复：去掉 .rev()
        for r in 1..=max_row { 
            let row_cells: Vec<Cell> = self.shape.cells()
                .iter()
                .filter(|c| c.row == r)
                .copied()
                .collect();
            let mut sorted = row_cells;
            sorted.sort_by(|a, b| b.col.cmp(&a.col));
            for cell in sorted {
                if let Some(&val) = self.filling.get(&cell) {
                    word.push(val);
                }
            }
        }
        word
    }

    pub fn is_lattice_word(&self) -> bool {
        let word = self.reading_word();
        let max_val = word.iter().copied().max().unwrap_or(0);
        
        for prefix_len in 1..=word.len() {
            let prefix = &word[..prefix_len];
            for k in 1..max_val {
                let count_k = prefix.iter().filter(|&&x| x == k).count();
                let count_k1 = prefix.iter().filter(|&&x| x == k + 1).count();
                if count_k < count_k1 {
                    return false;
                }
            }
        }
        true
    }
}