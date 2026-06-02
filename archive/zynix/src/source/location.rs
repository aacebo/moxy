/// 0 indexed char based location
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Location {
    index: usize,
    line: usize,
    column: usize,
}

impl Location {
    pub const fn new(index: usize, line: usize, column: usize) -> Self {
        Self { index, line, column }
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub const fn line(&self) -> usize {
        self.line
    }

    pub const fn column(&self) -> usize {
        self.column
    }
}
