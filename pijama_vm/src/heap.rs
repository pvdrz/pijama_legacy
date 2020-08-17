use std::cell::UnsafeCell;

use Node::{Cons, Empty};

use crate::Closure;

pub struct Heap {
    nodes: UnsafeCell<Node>,
}

impl Heap {
    pub fn new() -> Self {
        Self {
            nodes: UnsafeCell::new(Empty),
        }
    }
    pub fn insert(&self, closure: Closure) -> *const Closure {
        unsafe { &mut *self.nodes.get() }.push(closure)
    }
    pub fn len(&self) -> usize {
        unsafe { &*self.nodes.get() }.len()
    }
}

enum Node {
    Empty,
    Cons(UnsafeCell<Closure>, Box<Self>),
}

impl Node {
    fn len(&self) -> usize {
        match self {
            Empty => 0,
            Cons(_, next) => 1 + next.len(),
        }
    }

    fn push(&mut self, closure: Closure) -> *const Closure {
        match self {
            Empty => {
                *self = Cons(UnsafeCell::new(closure), Box::new(Empty));
                match self {
                    Cons(closure, _) => closure.get(),
                    _ => unreachable!(),
                }
            }
            Cons(_, next) => next.push(closure),
        }
    }
}
