use std::fmt;

#[derive(Clone, Default)]
pub(crate) struct ArgStack {
    base: usize,
    stack: Vec<i64>,
}

impl ArgStack {
    pub(crate) fn len(&self) -> usize {
        self.stack.len() - self.base
    }

    pub(crate) fn push(&mut self, value: i64) {
        self.stack.push(value);
    }

    pub(crate) fn pop(&mut self) -> Option<i64> {
        if self.len() > 0 {
            // FIXME: check if it is possible can avoid the double length check.
            self.stack.pop()
        } else {
            None
        }
    }

    pub(crate) fn last(&mut self) -> Option<i64> {
        self.stack.last().copied()
    }

    pub(crate) fn increase_base(&mut self, offset: usize) {
        self.base += offset;
    }

    pub(crate) fn decrease_base(&mut self, offset: usize) {
        self.base -= offset;
    }

    pub(crate) fn clear(&mut self) {
        self.stack.truncate(self.base);
    }
}

impl std::ops::Index<usize> for ArgStack {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        self.stack.index(index + self.base)
    }
}

impl fmt::Debug for ArgStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // FIXME: maybe it is better to show the "hidden" part of the stack too.
        self.stack[self.base..].fmt(f)
    }
}
