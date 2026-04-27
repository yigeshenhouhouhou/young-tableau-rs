use num_rational::Rational64;
use crate::YoungDiagram;

pub fn su_dimension(diag: &YoungDiagram, n: usize) -> Rational64 {
    let mut dim = Rational64::new(1, 1);

    for cell in diag.cells() {
        let hook = cell.hook_length(diag) as i64;
        let content = cell.content();
        let arm = (n as i64) + content;

        // 关键优化：边乘边约分。
        // 将单个格子的贡献构造成分数，与累加器相乘。
        // Rational64 内部会自动消除公约数，防止数字在连乘时撑爆 i64
        dim *= Rational64::new(arm, hook);
    }

    dim
}

pub fn count_standard_tableaux(diag: &YoungDiagram) -> Rational64 {
    let mut count = Rational64::new(1, 1);
    let hooks: Vec<i64> = diag.hook_lengths().map(|h| h as i64).collect();
    
    // 同理，n! 如果超过 20 也会溢出 i64。
    // 杨图的 size 等于格子的数量，所以 1..=n 刚好和 hooks 数组长度一样。
    // 我们把它们一对一组合成分数相乘，边乘边化简。
    for i in 1..=diag.size() {
        let num = i as i64;
        let den = hooks[i - 1];
        count *= Rational64::new(num, den);
    }
    
    count
}