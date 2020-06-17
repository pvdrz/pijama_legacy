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
        let op = self.read_byte()?.into();
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
                Op::Int => self.eval_int(),
                Op::Neg => self.eval_neg(),
                Op::Not => self.eval_not(),
                Op::Add => self.eval_add(),
                Op::Sub => self.eval_sub(),
                Op::Mul => self.eval_mul(),
                Op::Div => self.eval_div(),
                Op::Rem => self.eval_rem(),
                Op::BitAnd => self.eval_bitand(),
                Op::BitOr => self.eval_bitor(),
                Op::BitXor => self.eval_bitxor(),
                Op::True => self.eval_true(),
                Op::False => self.eval_false(),
                Op::Unit => self.eval_unit(),
                Op::GetLocal => self.eval_get_local(),
                Op::Pop => self.eval_pop(),
                Op::Print => self.eval_print(),
                Op::Jump => self.eval_jump(),
                Op::Skip => self.eval_skip(),
            }
        }
    }

    fn eval_ret(&mut self) {
        self.stack.pop().expect("Empty stack in ret");
    }

    fn eval_int(&mut self) {
        let index = self.read_index().unwrap();
        let value = *self.values.get(index).expect("Invalid value index in int");
        self.stack.push(value);
    }

    fn eval_neg(&mut self) {
        let value = self.stack.pop().expect("Empty stack in neg");
        self.stack.push(-value);
    }

    fn eval_not(&mut self) {
        let value = self.stack.pop().expect("Empty stack in not");
        self.stack.push(value ^ 0);
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

    fn eval_true(&mut self) {
        self.stack.push(1);
    }

    fn eval_false(&mut self) {
        self.stack.push(0);
    }

    fn eval_unit(&mut self) {
        self.stack.push(0);
    }

    fn eval_get_local(&mut self) {
        let index = self.read_index().unwrap();
        self.stack.push(self.stack[index]);
    }

    fn eval_pop(&mut self) {
        self.stack.pop().expect("Empty stack at pop");
    }

    fn eval_print(&mut self) {
        let value = self.stack.pop().expect("Empty stack at print");
        println!("{}", value);
    }

    fn eval_jump(&mut self) {
        let offset = self.read_index().unwrap();
        let cond = self.stack.pop().expect("Empty stack at jump");
        if cond == 0 {
            self.ins_ptr += offset;
        }
    }

    fn eval_skip(&mut self) {
        let offset = self.read_index().unwrap();
        self.ins_ptr += offset;
    }
}

#[test]
fn it_works() {}
