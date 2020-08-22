use std::mem::replace;

use crate::{Closure, CodeSlice, CodeBuf};

#[derive(Clone)]
pub(crate) struct CallFrame<'code> {
    closure: *const Closure,
    code: &'code CodeSlice,
    ins_ptr: usize,
    base_ptr: usize,
}

impl<'code> CallFrame<'code> {
    fn new(closure: *const Closure, code: &'code [CodeBuf], base_ptr: usize) -> Self {
        let code = unsafe { (&*closure).func_ptr() }.get_code(code);
        Self {
            closure,
            code,
            ins_ptr: 0,
            base_ptr,
        }
    }

    pub(crate) fn code(&self) -> &CodeSlice {
        self.code
    }

    pub(crate) fn ins_ptr(&self) -> usize {
        self.ins_ptr
    }

    pub(crate) fn ins_ptr_mut(&mut self) -> &mut usize {
        &mut self.ins_ptr
    }

    pub(crate) fn base_ptr(self) -> usize {
        self.base_ptr
    }

    pub(crate) fn closure(&self) -> *const Closure {
        self.closure
    }
}

#[derive(Clone)]
pub(crate) struct CallStack<'code> {
    last: CallFrame<'code>,
    stack: Vec<CallFrame<'code>>,
}

impl<'code> CallStack<'code> {
    pub(crate) fn new(main: *const Closure, code: &'code [CodeBuf]) -> Self {
        Self {
            last: CallFrame::new(main, code, 0),
            stack: vec![],
        }
    }

    pub(crate) fn last_mut(&mut self) -> &mut CallFrame<'code> {
        &mut self.last
    }

    pub(crate) fn peek(&self) -> &CallFrame<'code> {
        self.stack.last().unwrap()
    }

    pub(crate) fn push_frame(
        &mut self,
        closure: *const Closure,
        code: &'code [CodeBuf],
        base_ptr: usize,
    ) {
        let old_last = replace(&mut self.last, CallFrame::new(closure, code, base_ptr));
        self.stack.push(old_last);
    }

    pub(crate) fn pop(&mut self) -> Option<CallFrame> {
        // FIXME: Figure out How to pop last frame.
        let new_last = self.stack.pop()?;
        Some(replace(&mut self.last, new_last))
    }
}
