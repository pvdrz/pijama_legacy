use std::fmt::{Debug, Display, Formatter, Result};

/// Represents a location in the source code file.
///
/// Both the start and end correspond to locations reported by `nom_locate`.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Location {
    /// Start of the location
    pub start: usize,
    /// End of the location
    pub end: usize,
}

impl Location {
    pub const fn new(start: usize, end: usize) -> Self {
        Location { start, end }
    }
}

/// Adding two locations `l1` and `l2` returns a location starting in `l1.start` and ending in
/// `l2.end`.
impl std::ops::Add for Location {
    type Output = Self;
    fn add(mut self, other: Self) -> Self {
        self.end = other.end;
        self
    }
}

/// Wrapper type with a `Location` field.
///
/// It is used to add a location to elements in the AST and intermediate representations.
#[derive(Debug)]
pub struct Located<T: Debug> {
    /// Content of the wrapper.
    pub content: T,
    /// Location of `content` in the source file.
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

impl<T: Clone + Debug> Clone for Located<T> {
    fn clone(&self) -> Self {
        Located::new(self.content.clone(), self.loc)
    }
}

impl<T: Copy + Debug> Copy for Located<T> {}
