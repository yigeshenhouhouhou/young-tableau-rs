use std::collections::HashMap;
use crate::{YoungDiagram, Cell, StandardYoungTableau};

pub fn enumerate_standard_tableaux(shape: &YoungDiagram) -> Vec<StandardYoungTableau> {
    let cells: Vec<Cell> = shape.cells().collect();
    let n = cells.len();
    if n == 0 {
        return Vec::new();
    }

    let mut result = Vec::new();
    let mut current = HashMap::new();
    let mut used = vec![false; n + 1];

    backtrack(shape, &cells, 0, &mut current, &mut used, &mut result);
    result
}

fn backtrack(
    shape: &YoungDiagram,
    cells: &[Cell],
    pos: usize,
    current: &mut HashMap<Cell, usize>,
    used: &mut [bool],
    result: &mut Vec<StandardYoungTableau>,
) {
    if pos == cells.len() {
        let syt = StandardYoungTableau::from_trusted(shape.clone(), current.clone());
        result.push(syt);
        return;
    }

    let cell = cells[pos];
    let n = cells.len();

    for num in 1..=n {
        if used[num] { continue; }

        if cell.col > 1 {
            let left = Cell::new(cell.row, cell.col - 1);
            if let Some(&lv) = current.get(&left) {
                if lv >= num { continue; }
            }
        }

        if cell.row > 1 {
            let up = Cell::new(cell.row - 1, cell.col);
            if let Some(&uv) = current.get(&up) {
                if uv >= num { continue; }
            }
        }

        used[num] = true;
        current.insert(cell, num);
        backtrack(shape, cells, pos + 1, current, used, result);
        current.remove(&cell);
        used[num] = false;
    }
}