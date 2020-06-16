use std::convert::TryInto;

use pijama_codegen::Op;

pub fn run(code: Vec<u8>, values: Vec<i64>) {
    let mut machine = Machine::new(code, values);
    machine.run();
}

struct Machine {
    ins_ptr: usize,
    code: Vec<u8>,
    values: Vec<i64>,
    stack: Vec<i64>,
}

impl Machine {
    fn new(code: Vec<u8>, values: Vec<i64>) -> Self {
        Machine {
            ins_ptr: 0,
            code,
            values,
            stack: Vec::default(),
        }
    }

    fn read_byte(&self) -> Option<u8> {
        self.code.get(self.ins_ptr).copied()
    }

    fn read_bytes(&mut self, n: usize) -> Option<&[u8]> {
        self.code.get(self.ins_ptr..self.ins_ptr + n)
    }

    fn read_op(&mut self) -> Option<Op> {
        let op = Op::from_byte(self.read_byte()?);
        self.ins_ptr += 1;
        Some(op)
    }

    fn read_index(&mut self) -> Option<usize> {
        const N: usize = std::mem::size_of::<usize>() / std::mem::size_of::<u8>();
        let bytes: [u8; N] = self.read_bytes(N)?.try_into().ok()?;
        self.ins_ptr += N;
        Some(usize::from_be_bytes(bytes))
    }

    fn run(&mut self) {
        while let Some(op) = self.read_op() {
            match op {
                Op::Ret => self.eval_ret(),
                Op::Lit => self.eval_lit(),
                Op::Neg => self.eval_neg(),
                Op::Add => self.eval_add(),
                Op::Sub => self.eval_sub(),
                Op::Mul => self.eval_mul(),
                Op::Div => self.eval_div(),
                Op::Rem => self.eval_rem(),
                Op::BitAnd => self.eval_bitand(),
                Op::BitOr => self.eval_bitor(),
                Op::BitXor => self.eval_bitxor(),
            }
        }
    }

    fn eval_ret(&mut self) {
        let value = self.stack.pop().expect("Empty stack in ret");
        println!("{}", value);
    }

    fn eval_lit(&mut self) {
        let index = self.read_index().unwrap();
        let value = *self.values.get(index).expect("Invalid value index in lit");
        self.stack.push(value);
    }

    fn eval_neg(&mut self) {
        let value = self.stack.pop().expect("Empty stack in neg");
        self.stack.push(-value);
    }

    fn eval_add(&mut self) {
        let v1 = self.stack.pop().expect("Empty stack in add");
        let v2 = self.stack.pop().expect("Empty stack in add");
        self.stack.push(v1 + v2);
    }

    fn eval_sub(&mut self) {
        let v1 = self.stack.pop().expect("Empty stack in add");
        let v2 = self.stack.pop().expect("Empty stack in add");
        self.stack.push(v1 - v2);
    }

    fn eval_mul(&mut self) {
        let v1 = self.stack.pop().expect("Empty stack in add");
        let v2 = self.stack.pop().expect("Empty stack in add");
        self.stack.push(v1 * v2);
    }

    fn eval_div(&mut self) {
        let v1 = self.stack.pop().expect("Empty stack in add");
        let v2 = self.stack.pop().expect("Empty stack in add");
        self.stack.push(v1 / v2);
    }

    fn eval_rem(&mut self) {
        let v1 = self.stack.pop().expect("Empty stack in add");
        let v2 = self.stack.pop().expect("Empty stack in add");
        self.stack.push(v1 % v2);
    }

    fn eval_bitand(&mut self) {
        let v1 = self.stack.pop().expect("Empty stack in add");
        let v2 = self.stack.pop().expect("Empty stack in add");
        self.stack.push(v1 & v2);
    }

    fn eval_bitor(&mut self) {
        let v1 = self.stack.pop().expect("Empty stack in add");
        let v2 = self.stack.pop().expect("Empty stack in add");
        self.stack.push(v1 | v2);
    }

    fn eval_bitxor(&mut self) {
        let v1 = self.stack.pop().expect("Empty stack in add");
        let v2 = self.stack.pop().expect("Empty stack in add");
        self.stack.push(v1 ^ v2);
    }
}

#[test]
fn it_works() {}
