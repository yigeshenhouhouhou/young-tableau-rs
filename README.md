***Young Tableau in Rust (young-tableau-rs)***




这是一个用 Rust 编写的高性能组合数学与代数物理工具库。主要用于处理
**_整数分拆 (Integer Partitions)_**、**_杨图 (Young Diagrams)_**、**_杨表 (Young Tableaux)_**，以及计算群表示论中的 
**_SU(N) 维数_** 和 **_张量积分解(Littlewood-Richardson Rule)_** 。

核心特性

- 基础组合学: 整数分拆的生成、校验与包含关系判断。

- 可视化: 在终端中美观地渲染杨图与杨表（支持 ASCII 边框绘制）。

- 钩长公式 (Hook Length Formula): 精确计算标准杨表 (SYT) 的数量，使用有理数 (num-rational) 避免大数溢出。

- 物理与群论: 计算任意杨图对应在 SU(N) 群中的物理表示维数。

- 标准杨表枚举: 使用回溯算法枚举指定形状的所有标准杨表。

- 极速张量积分解:实现了 Littlewood-Richardson 规则的构造型算法 (广义 Pieri 规则)。

- 支持利用 rayon 进行多线程并发计算。

依赖配置
在使用本库之前，请确保你的 Cargo.toml 中包含了以下依赖：

```rust
[dependencies]
num-rational = "0.4"
num-traits = "0.2"
rayon = "1.8"
```
快速开始
1. 分拆与杨图可视化
```rust
use young_tableau_rs::{Partition, YoungDiagram};

fn main() {
    let p = Partition::new(vec![4, 3, 2, 1]).expect("不合法分拆");
    let diag = YoungDiagram::from_partition(p);
    println!("杨图渲染:\n{}", diag);
}
```


2. SU(N) 维数计算
```rust
use young_tableau_rs::{Partition, YoungDiagram, su_dimension};

fn main() {
    let p = Partition::new(vec![2, 1]).unwrap();
    let diag = YoungDiagram::from_partition(p);
    let dim = su_dimension(&diag, 3);
    println!("SU(3) 维数: {}", dim); // 输出 8
}
```
3. 张量积分解
```rust
use young_tableau_rs::{Partition};
use young_tableau_rs::lr_rule::pieri_decompose;

fn main() {
    let lambda = Partition::new(vec![2, 1]).unwrap();
    let mu = Partition::new(vec![2]).unwrap();

    let results = pieri_decompose(&lambda, &mu);
    for (nu, mult) in results {
        println!("生成表示: {} (多重度: {})", nu, mult);
    }
}
```
模块指南

```partition.rs```: 核心基础，定义整数分拆及其校验逻辑。

```diagram.rs```: 处理杨图的几何属性及 ASCII 渲染。

```cell.rs```: 抽象单个“格子”，封装臂长、腿长及钩长计算。

```tableau.rs```: 杨表数据结构，支持标准性校验 (行/列严格递增)。

```dimension.rs```: 包含 $SU(N)$ 维数公式及标准杨表计数逻辑。

```enumeration.rs```: 生成指定形状的所有合法标准杨表填充的回溯算法。

```lr_rule.rs```: 核心算法库，包含 LR 规则和 Pieri 规则的并行实现。
