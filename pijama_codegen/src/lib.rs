use pijama_ast::{
    location::Located,
    node::{BinOp, Literal, Name, Primitive, UnOp},
};
use pijama_mir::{LetKind, Term};

pub fn codegen(term: Located<Term>) -> (Vec<u8>, Vec<i64>) {
    let mut compiler = Compiler::default();
    compiler.compile(term.content);
    (compiler.code, compiler.values)
}

macro_rules! opcodes {
    ($($op:ident => $byte:tt), *) => {
        #[derive(Debug)]
        pub enum Op {
            $($op,)*
        }


        impl From<Op> for u8 {
            fn from(op: Op) -> u8 {
                match op {
                    $(Op::$op => $byte,)*
                }
            }
        }

        impl From<u8> for Op {
            fn from(byte: u8) -> Op {
                match byte {
                    $($byte => Op::$op,)*
                    _ => panic!("Invalid opcode {}", byte),
                }
            }
        }
    }
}

opcodes! {
    Ret => 0,
    True => 1,
    False => 2,
    Unit => 3,
    Int => 4,
    Neg => 5,
    Not => 19,
    Add => 6,
    Sub => 7,
    Mul => 8,
    Div => 9,
    Rem => 10,
    BitAnd => 11,
    BitOr => 12,
    BitXor => 13,
    GetLocal => 14,
    Pop => 15,
    Jump => 16,
    Skip => 17,
    Print => 18
}

#[derive(Default)]
struct Compiler<'a> {
    code: Vec<u8>,
    values: Vec<i64>,
    locals: Vec<Name<'a>>,
}

impl<'a> Compiler<'a> {
    fn code(&self) -> &[u8] {
        &self.code
    }

    fn code_mut(&mut self) -> &mut Vec<u8> {
        &mut self.code
    }

    fn write_u8(&mut self, byte: impl Into<u8>) {
        self.code_mut().push(byte.into());
    }

    fn write_usize(&mut self, uint: usize) {
        self.code_mut().extend_from_slice(&uint.to_be_bytes());
    }

    fn overwrite_usize(&mut self, pos: usize, uint: usize) {
        let new_bytes = uint.to_be_bytes();
        for (offset, &byte) in new_bytes.iter().enumerate() {
            *self.code_mut().get_mut(pos + offset).unwrap() = byte;
        }
    }

    fn store_value(&mut self, value: i64) -> usize {
        for (index, &val) in self.values.iter().enumerate() {
            if val == value {
                return index;
            }
        }
        let index = self.values.len();
        self.values.push(value);
        index
    }

    fn compile(&mut self, term: Term<'a>) {
        match term {
            Term::Var(name) => self.compile_var(name),
            Term::PrimFn(prim) => self.compile_prim_fn(prim),
            Term::Lit(lit) => self.compile_lit(lit),
            Term::UnaryOp(op, term) => self.compile_unary_op(op, term.content),
            Term::BinaryOp(op, t1, t2) => self.compile_binary_op(op, t1.content, t2.content),
            Term::Let(kind, lhs, rhs, tail) => {
                self.compile_let(kind, lhs.content, rhs.content, tail.content)
            }
            Term::App(t1, t2) => self.compile_app(t1.content, t2.content),
            Term::Cond(t1, t2, t3) => self.compile_cond(t1.content, t2.content, t3.content),
            _ => todo!("unsupported term `{}`", term),
        }
    }

    fn compile_var(&mut self, name: Name<'a>) {
        for (index, &local) in self.locals.iter().enumerate().rev() {
            if local == name {
                self.write_u8(Op::GetLocal);
                self.write_usize(index);
                return;
            }
        }
        // Any previous compilation stage should have captured this error.
        panic!("unbounded name {}", name);
    }

    fn compile_prim_fn(&mut self, prim: Primitive) {
        match prim {
            Primitive::Print => self.write_u8(Op::Print),
        }
    }

    fn compile_lit(&mut self, lit: Literal) {
        match lit {
            Literal::Number(int) => {
                let index = self.store_value(int);
                self.write_u8(Op::Int);
                self.write_usize(index);
            }
            Literal::Bool(boolean) => {
                if boolean {
                    self.write_u8(Op::True);
                } else {
                    self.write_u8(Op::False);
                }
            }
            Literal::Unit => {
                self.write_u8(Op::Unit);
            }
        }
    }

    fn compile_unary_op(&mut self, op: UnOp, term: Term<'a>) {
        let op = match op {
            UnOp::Neg => Op::Neg,
            UnOp::Not => Op::Not,
        };
        self.compile(term);
        self.write_u8(op)
    }

    fn compile_binary_op(&mut self, op: BinOp, t1: Term<'a>, t2: Term<'a>) {
        let op = match op {
            BinOp::Add => Op::Add,
            BinOp::Sub => Op::Sub,
            BinOp::Mul => Op::Mul,
            BinOp::Div => Op::Div,
            BinOp::Rem => Op::Rem,
            BinOp::BitAnd => Op::BitAnd,
            BinOp::BitOr => Op::BitOr,
            BinOp::BitXor => Op::BitXor,
            BinOp::And => {
                return self.compile_cond(t1, t2, Term::Lit(Literal::Bool(false)));
            }
            BinOp::Or => {
                return self.compile_cond(t1, Term::Lit(Literal::Bool(true)), t2);
            }
            _ => todo!("unsupported binary operator {}", op),
        };
        self.compile(t2);
        self.compile(t1);
        self.write_u8(op);
    }

    fn compile_let(&mut self, kind: LetKind, lhs: Name<'a>, rhs: Term<'a>, tail: Term<'a>) {
        match kind {
            LetKind::NonRec(_) => {
                self.compile(rhs);
                self.locals.push(lhs);
                self.compile(tail);
                self.write_u8(Op::Pop);
                self.locals.pop().expect("Could not pop local");
            }
            LetKind::Rec(_) => {
                todo!("cannot compile recursive binding");
            }
        }
    }

    fn compile_app(&mut self, t1: Term<'a>, t2: Term<'a>) {
        self.compile(t2);
        self.compile(t1);
    }

    fn compile_cond(&mut self, t1: Term<'a>, t2: Term<'a>, t3: Term<'a>) {
        // Compile the condition, when the generated code is executed the value of the condition
        // should be in the stack.
        self.compile(t1);
        // Now we generate code for the jump.
        self.write_u8(Op::Jump);
        // Save the position of the jump offset to overwrite it later.
        let jump_offset_pos = self.code().len();
        // Write a dummy value for the offset.
        self.write_usize(usize::max_value());
        // Store the position where the code for the `then` branch starts.
        let t2_start = self.code().len();
        // Compile the `then` branch.
        self.compile(t2);
        // If the VM executes the `then` branch, it must skip over the `else` branch code.
        self.write_u8(Op::Skip);
        // Save the position of the skip offset to overwrite it later.
        let skip_offset_pos = self.code().len();
        // Write a dummy value for the offset.
        self.write_usize(usize::max_value());
        // Store the position where the code for the `else` branch starts.
        let t3_start = self.code().len();
        // Compile the `else` branch.
        self.compile(t3);
        // Store the position where the code for the `else` branch ends.
        let t3_end = self.code().len();
        // Overwrite the offset for the jump. This offset must be the length of the `then` branch
        // code including the skip and its offset.
        self.overwrite_usize(jump_offset_pos, t3_start - t2_start);
        // Overwrite the offset for the skip. This offset must be the length of the `else` branch
        // code.
        self.overwrite_usize(skip_offset_pos, t3_end - t3_start);
    }
}
