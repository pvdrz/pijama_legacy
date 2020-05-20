use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug, Clone, Copy)]
pub struct Location {
    pub start: usize,
    pub end: usize,
}

impl Location {
    pub fn new(start: usize, end: usize) -> Self {
        Location { start, end }
    }
}

impl std::ops::Add for Location {
    type Output = Self;
    fn add(mut self, other: Self) -> Self {
        self.end = other.end;
        self
    }
}

#[derive(Debug)]
pub struct Located<T: Debug> {
    pub content: T,
    pub loc: Location,
}

impl<T: Debug> Located<T> {
    pub fn new(content: T, loc: impl Into<Location>) -> Self {
        Located {
            content,
            loc: loc.into(),
        }
    }
}

impl<T: Display + Debug> Display for Located<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.content)
    }
}

impl<T: Eq + Debug> Eq for Located<T> {}

impl<T: PartialEq + Debug> PartialEq for Located<T> {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
    }
}
