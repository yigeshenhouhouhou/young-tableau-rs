use std::collections::HashMap;
use crate::{Partition, YoungDiagram, Cell, SkewShape};
use rayon::prelude::*;

pub fn lr_coefficient(lambda: &Partition, mu: &Partition, nu: &Partition) -> usize {
    lr_coefficient_impl(lambda, mu, nu, false)
}

fn lr_coefficient_impl(lambda: &Partition, mu: &Partition, nu: &Partition, is_conj: bool) -> usize {
    if nu.size() != lambda.size() + mu.size() {
        return 0;
    }
    
    let outer = YoungDiagram::from_partition(nu.clone());
    let inner = YoungDiagram::from_partition(lambda.clone());
    
    // 标准情况：ν ⊇ λ，直接计算斜形状 ν/λ
    if outer.partition().contains(inner.partition()) {
        let shape = match SkewShape::new(outer, inner) {
            Ok(s) => s,
            Err(_) => return 0,
        };
        
        let mut available = Vec::new();
        for (idx, &count) in mu.parts().iter().enumerate() {
            let val = idx + 1;
            for _ in 0..count {
                available.push(val);
            }
        }
        
        let cells: Vec<Cell> = shape.cells().to_vec();
        let mut count = 0;
        let mut used = vec![false; available.len()];
        let mut current = HashMap::new();
        
        backtrack(&shape, &cells, 0, &available, &mut used, &mut current, &mut count);
        count
    } else if !is_conj {
        // ν 不包含 λ：尝试共轭对偶 ν'/λ'，但只尝试一次防止无限递归
        let nu_conj = outer.conjugate().partition().clone();
        let lambda_conj = inner.conjugate().partition().clone();
        let mu_conj = YoungDiagram::from_partition(mu.clone()).conjugate().partition().clone();
        
        if nu_conj.contains(&lambda_conj) {
            lr_coefficient_impl(&lambda_conj, &mu_conj, &nu_conj, true)
        } else {
            0
        }
    } else {
        0
    }
}

fn backtrack(
    shape: &SkewShape,
    cells: &[Cell],
    pos: usize,
    available: &[usize],
    used: &mut [bool],
    current: &mut HashMap<Cell, usize>,
    count: &mut usize,
) {
    if pos == cells.len() {
        let word = reading_word(shape, current);
        if is_lattice_word(&word) {
            *count += 1;
        }
        return;
    }
    
    let cell = cells[pos];
    let mut tried = std::collections::HashSet::new();
    
    for i in 0..available.len() {
        if used[i] { continue; }
        let num = available[i];
        if tried.contains(&num) { continue; }
        tried.insert(num);
        
        if cell.col > 1 {
            let left = Cell::new(cell.row, cell.col - 1);
            if shape.contains(&left) {
                if let Some(&lv) = current.get(&left) {
                    if lv > num { continue; }
                }
            }
        }
        
        if cell.row > 1 {
            let up = Cell::new(cell.row - 1, cell.col);
            if shape.contains(&up) {
                if let Some(&uv) = current.get(&up) {
                    if uv >= num { continue; }
                }
            }
        }
        
        used[i] = true;
        current.insert(cell, num);
        backtrack(shape, cells, pos + 1, available, used, current, count);
        current.remove(&cell);
        used[i] = false;
    }
}

fn reading_word(shape: &SkewShape, filling: &HashMap<Cell, usize>) -> Vec<usize> {
    let mut word = Vec::with_capacity(shape.size());
    let max_row = shape.outer().num_rows();
    
    // 修复：去掉 .rev()，改为从上到下（行 1 到 max_row）读取
    for r in 1..=max_row { 
        let mut row_cells: Vec<Cell> = shape.cells()
            .iter()
            .filter(|c| c.row == r)
            .copied()
            .collect();
        // 列从右到左读取，逻辑正确，保持不变
        row_cells.sort_by(|a, b| b.col.cmp(&a.col)); 
        
        for cell in row_cells {
            if let Some(&val) = filling.get(&cell) {
                word.push(val);
            }
        }
    }
    word
}

fn is_lattice_word(word: &[usize]) -> bool {
    if word.is_empty() { return true; }
    let max_val = *word.iter().max().unwrap_or(&0);
    
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

pub fn tensor_product_decompose(lambda: &Partition, mu: &Partition) -> Vec<(Partition, usize)> {
    let total_size = lambda.size() + mu.size();
    let all_nu = Partition::generate_all(total_size);
    
    // 魔法就在这里：把 into_iter() 改成 into_par_iter()，Rust 会自动分配到所有 CPU 核心上运算！
    let mut result: Vec<_> = all_nu.into_par_iter()
        .map(|nu| {
            let c = lr_coefficient(lambda, mu, &nu);
            (nu, c)
        })
        .filter(|(_, c)| *c > 0)
        .collect();
        
    result.sort_by(|a, b| b.0.cmp(&a.0));
    result
}


/// 使用广义 Pieri 规则 (构造型 LR 算法) 计算张量积分解
pub fn pieri_decompose(lambda: &Partition, mu: &Partition) -> Vec<(Partition, usize)> {
    if mu.is_empty() {
        return vec![(lambda.clone(), 1)];
    }

    // 结果的最大行数不会超过 lambda 的行数 + mu 的行数
    let max_rows = lambda.len() + mu.len();
    
    // 初始化当前形状
    let mut shape = vec![0; max_rows];
    for (i, &p) in lambda.parts().iter().enumerate() {
        shape[i] = p;
    }

    let mut results = HashMap::new();
    let mut curr_c = vec![0; max_rows];
    
    // 对于第一步 (加 '1')，没有 Lattice 限制，后缀和设为无限大
    let prev_c_suffix = vec![usize::MAX; max_rows + 1];

    distribute(
        0,
        mu.parts()[0],
        0,
        &mut shape,
        &mut curr_c,
        &prev_c_suffix,
        mu.parts(),
        &mut results,
    );

    // 将 HashMap 转为 Vec 并排序输出
    let mut final_res: Vec<_> = results.into_iter()
        .map(|(parts, c)| (Partition::new(parts).unwrap(), c))
        .collect();
    
    final_res.sort_by(|a, b| b.0.cmp(&a.0));
    final_res
}

/// 核心递归函数：将 mu[step] 个方块分配到各行
fn distribute(
    row: usize,
    boxes_left: usize,
    step: usize,
    shape: &mut [usize],
    curr_c: &mut [usize],
    prev_c_suffix: &[usize],
    mu: &[usize],
    results: &mut HashMap<Vec<usize>, usize>
) {
    // 基础情况 1：这层的方块分发完毕
    if boxes_left == 0 {
        // 清理当前数组的剩余部分
        for i in row..curr_c.len() {
            curr_c[i] = 0;
        }

        let next_step = step + 1;
        // 如果所有 mu 都分配完了，记录结果！
        if next_step == mu.len() {
            let mut final_parts = shape.to_vec();
            while final_parts.last() == Some(&0) {
                final_parts.pop(); // 去除末尾的 0
            }
            *results.entry(final_parts).or_insert(0) += 1;
        } else {
            // 准备进入下一层！计算当前层的后缀和，供下一层做 Lattice 校验
            let mut next_prev_c_suffix = vec![0; curr_c.len() + 1];
            let mut sum = 0;
            for i in (0..curr_c.len()).rev() {
                sum += curr_c[i];
                next_prev_c_suffix[i] = sum;
            }
            
            let mut next_curr_c = vec![0; curr_c.len()];
            distribute(
                0,
                mu[next_step],
                next_step,
                shape,
                &mut next_curr_c,
                &next_prev_c_suffix,
                mu,
                results,
            );
        }
        return;
    }

    // 基础情况 2：行数耗尽，分配失败，直接剪枝
    if row == curr_c.len() {
        return;
    }

    // 🔥 Lattice 剪枝核心：当前及以下行要分配的方块数，不能超过上一层对应行的分配数！
    if row > 0 && boxes_left > prev_c_suffix[row - 1] {
        return;
    }

    // 确定当前行最多能放多少个方块 (同一列不能有多个，且不能超过剩余的方块)
    let max_c = if row == 0 {
        boxes_left
    } else {
        boxes_left.min(shape[row - 1] - shape[row])
    };

    // 确定当前行最少必须放多少个方块 (为了保证后面的行不违反 Lattice 规则)
    let min_c = boxes_left.saturating_sub(prev_c_suffix[row]);

    // 遍历所有合法的放置方案
    for c in min_c..=max_c {
        curr_c[row] = c;
        shape[row] += c; // 放上方块
        
        // 递归进入下一行
        distribute(
            row + 1,
            boxes_left - c,
            step,
            shape,
            curr_c,
            prev_c_suffix,
            mu,
            results,
        );
        
        shape[row] -= c; // 回溯：拿走方块
    }
}