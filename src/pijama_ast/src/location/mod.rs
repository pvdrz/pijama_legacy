use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

pub type Span<'a> = nom_locate::LocatedSpan<&'a str>;

impl<'a> From<Span<'a>> for Location {
    fn from(span: Span<'a>) -> Self {
        let start = span.location_offset();
        let end = start + 1;
        Location { start, end }
    }
}

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
    /// Creates a new `Located` consuming this `Location`.
    pub fn with_content<T: Debug>(self, content: T) -> Located<T> {
        Located::new(content, self)
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
    /// Creates a new `Located`.
    pub fn new(content: T, loc: impl Into<Location>) -> Self {
        Located {
            content,
            loc: loc.into(),
        }
    }
    /// Maps the content of the `Located` leaving its location untouched.
    pub fn map<U: Debug, F: FnOnce(T) -> U>(self, f: F) -> Located<U> {
        Located {
            content: f(self.content),
            loc: self.loc,
        }
    }
    /// Like `map` but the closure is allowed to return a `Result`.
    pub fn map_res<U: Debug, E, F: FnOnce(T) -> Result<U, E>>(self, f: F) -> Result<Located<U>, E> {
        Ok(Located {
            content: f(self.content)?,
            loc: self.loc,
        })
    }
    /// Joins two `Located`s by adding their locations and joining their contents using a closure.
    pub fn zip_with<U: Debug, V: Debug, F: FnOnce(T, U) -> V>(
        self,
        other: Located<U>,
        f: F,
    ) -> Located<V> {
        Located {
            content: f(self.content, other.content),
            loc: self.loc + other.loc,
        }
    }
}

impl<T: Display + Debug> Display for Located<T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.content)
    }
}

/// `Located`s are compared by their content only.
impl<T: Eq + Debug> Eq for Located<T> {}
/// `Located`s are compared by their content only.
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
