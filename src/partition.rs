use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Partition(Vec<usize>);

impl Partition {
    pub fn new(parts: Vec<usize>) -> Result<Self, &'static str> {
        if parts.is_empty() {
            return Ok(Self(parts));
        }
        for w in parts.windows(2) {
            if w[0] < w[1] {
                return Err("分拆各部分必须非递增");
            }
        }
        if parts.last().copied().unwrap_or(0) == 0 {
            return Err("分拆各部分必须为正整数");
        }
        Ok(Self(parts))
    }

    pub fn size(&self) -> usize {
        self.0.iter().sum()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, i: usize) -> Option<usize> {
        self.0.get(i).copied()
    }

    pub fn parts(&self) -> &[usize] {
        &self.0
    }

    pub fn iter(&self) -> impl Iterator<Item = &usize> {
        self.0.iter()
    }

    pub fn contains(&self, other: &Partition) -> bool {
        for i in 0..self.0.len().max(other.0.len()) {
            let a = self.0.get(i).copied().unwrap_or(0);
            let b = other.0.get(i).copied().unwrap_or(0);
            if a < b {
                return false;
            }
        }
        true
    }

    pub fn generate_all(n: usize) -> Vec<Partition> {
        let mut result = Vec::new();
        let mut current = Vec::new();
        Self::gen_helper(n, n, &mut current, &mut result);
        result
    }

    fn gen_helper(remaining: usize, max_part: usize, current: &mut Vec<usize>, result: &mut Vec<Partition>) {
        if remaining == 0 {
            if !current.is_empty() {
                result.push(Partition::new(current.clone()).unwrap());
            }
            return;
        }
        for part in (1..=max_part.min(remaining)).rev() {
            current.push(part);
            Self::gen_helper(remaining - part, part, current, result);
            current.pop();
        }
    }
}

impl fmt::Display for Partition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "))
    }
}