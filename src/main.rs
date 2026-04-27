use young_tableau_rs::*;
use young_tableau_rs::lr_rule::pieri_decompose;

fn main() {
    println!("--- 1. 分拆与杨图 (Partitions & Diagrams) ---");
    // 创建一个分拆并生成杨图
    let p = Partition::new(vec![4, 3, 2, 1]).expect("合法分拆");
    let diag = YoungDiagram::from_partition(p.clone());
    println!("分拆: {}", p);
    println!("杨图渲染:\n{}", diag);

    println!("\n--- 2. 物理维度计算 (SU(N) Dimensions) ---");
    let n = 4;
    let dim = su_dimension(&diag, n);
    println!("该表示在 SU({}) 中的维数: {}", n, dim);

    println!("\n--- 3. 标准杨表枚举 (Standard Young Tableaux) ---");
    // 计算并枚举所有标准杨表
    let count = count_standard_tableaux(&diag);
    let tableaux = enumerate_standard_tableaux(&diag);
    println!("标准杨表总数 (钩长公式): {}", count);
    println!("枚举前 2 个杨表:");
    for t in tableaux.iter().take(2) {
        println!("{}\n", t.display());
    }

    println!("\n--- 4. 极速张量积分解 (Tensor Product Decomposition) ---");
    let mu = Partition::new(vec![2, 1]).expect("基本表示");
    println!("计算分解: {} ⊗ {}", p, mu);
    
    let results = pieri_decompose(&p, &mu);
    for (nu, mult) in results {
        let nu_diag = YoungDiagram::from_partition(nu.clone());
        println!("生成表示: {} (多重度: {}), SU(4)维数: {}", 
                 nu, mult, su_dimension(&nu_diag, 4));
    }
}