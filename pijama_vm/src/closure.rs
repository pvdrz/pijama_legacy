use crate::disassemble;
use std::io::stdout;

#[derive(Default, Debug)]
pub struct CodeBuf {
    code: Vec<u8>,
}

impl CodeBuf {
    pub fn write_u8(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn write_i64(&mut self, int: i64) {
        self.code.extend_from_slice(&int.to_be_bytes())
    }

    pub fn overwrite_i64(&mut self, index: usize, int: i64) -> Option<()> {
        self.code
            .get_mut(index..index + 8)?
            .copy_from_slice(&int.to_be_bytes());
        Some(())
    }

    pub fn extend_from_slice(&mut self, slice: &CodeSlice) {
        self.code.extend_from_slice(&slice.code);
    }
}

impl std::ops::Deref for CodeBuf {
    type Target = CodeSlice;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.code.deref() as *const [u8] as *const CodeSlice) }
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct CodeSlice {
    code: [u8],
}

impl CodeSlice {
    pub fn len(&self) -> usize {
        self.code.len()
    }

    pub unsafe fn read_u8_unchecked(&self, index: usize) -> u8 {
        *self.code.get_unchecked(index)
    }

    pub fn read_u8(&self, index: usize) -> Option<u8> {
        if index < self.code.len() {
            Some(unsafe { self.read_u8_unchecked(index) })
        } else {
            None
        }
    }

    pub unsafe fn read_i64_unchecked(&self, index: usize) -> i64 {
        let ptr = self.code.get_unchecked(index..index + 8) as *const [u8] as *const [u8; 8];
        i64::from_be_bytes(*ptr)
    }

    pub fn read_i64(&self, index: usize) -> Option<i64> {
        if index + 8 < self.code.len() {
            Some(unsafe { self.read_i64_unchecked(index) })
        } else {
            None
        }
    }

    pub fn disassemble(&self) {
        disassemble(self, &mut stdout()).unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FuncPtr {
    code_ptr: usize,
}

impl FuncPtr {
    pub fn new(code_ptr: usize) -> Self {
        Self { code_ptr }
    }

    pub fn get_code(self, code: &[CodeBuf]) -> &CodeSlice {
        &code[self.code_ptr]
    }
}

pub struct Closure {
    func_ptr: FuncPtr,
    upvalues: Vec<i64>,
}

impl Closure {
    pub fn new(func_ptr: FuncPtr) -> Self {
        Self { func_ptr, upvalues: Vec::new() }
    }

    pub fn func_ptr(&self) -> FuncPtr {
        self.func_ptr
    }

    pub fn push_upvalue(&mut self, value: i64) {
        self.upvalues.push(value);
    }

    pub fn get_upvalue(&self, i: usize) -> i64{
        self.upvalues[i]
    }
}
