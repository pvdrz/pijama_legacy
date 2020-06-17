use pijama_ast::{
    location::Located,
    node::{BinOp, Literal, Name, Primitive, UnOp},
};
use pijama_mir::{LetKind, Term};

pub fn codegen(term: Located<Term>) -> (Vec<u8>, Vec<i64>) {
    let mut compiler = Compiler::default();
    compiler.compile(term);
    println!("{:?}", compiler.values);
    (compiler.code, compiler.values)
}

macro_rules! opcodes {
    ($($op:ident => $byte:tt), *) => {
        #[derive(Debug)]
        pub enum Op {
            $($op,)*
        }

        impl Op {
            pub fn into_byte(self) -> u8 {
                match self {
                    $(Op::$op => $byte,)*
                }
            }

            pub fn from_byte(byte: u8) -> Self {
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
    fn write_byte(&mut self, byte: u8) {
        self.code.push(byte);
    }

    fn write_index(&mut self, index: usize) {
        self.code.extend_from_slice(&index.to_be_bytes());
    }

    fn overwrite_index(&mut self, pos: usize, index: usize) {
        let new_bytes = index.to_be_bytes();
        for (offset, &byte) in new_bytes.iter().enumerate() {
            self.code[pos + offset] = byte;
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

    fn compile(&mut self, term: Located<Term<'a>>) {
        match term.content {
            Term::Var(name) => {
                for (index, &local) in self.locals.iter().enumerate().rev() {
                    if local == name {
                        self.write_byte(Op::GetLocal.into_byte());
                        self.write_index(index);
                        return;
                    }
                }
                panic!("Unbounded name {}", name);
            }
            Term::PrimFn(Primitive::Print) => {
                self.write_byte(Op::Print.into_byte());
            }
            Term::Lit(lit) => match lit {
                Literal::Number(int) => {
                    let index = self.store_value(int);
                    self.write_byte(Op::Int.into_byte());
                    self.write_index(index);
                }
                Literal::Bool(boolean) => {
                    if boolean {
                        self.write_byte(Op::True.into_byte());
                    } else {
                        self.write_byte(Op::False.into_byte());
                    }
                }
                Literal::Unit => {
                    self.write_byte(Op::Unit.into_byte());
                }
            },
            Term::UnaryOp(UnOp::Neg, term) => {
                self.compile(*term);
                self.write_byte(Op::Neg.into_byte());
            }
            Term::BinaryOp(BinOp::And, t1, t2) => {
                self.compile(term.loc.with_content(Term::Cond(
                    t1,
                    t2,
                    Box::new(term.loc.with_content(Term::Lit(Literal::Bool(false)))),
                )));
            }
            Term::BinaryOp(BinOp::Or, t1, t2) => {
                self.compile(term.loc.with_content(Term::Cond(
                    t1,
                    Box::new(term.loc.with_content(Term::Lit(Literal::Bool(true)))),
                    t2,
                )));
            }
            Term::BinaryOp(op, term1, term2) => {
                self.compile(*term2);
                self.compile(*term1);
                let byte = match op {
                    BinOp::Add => Op::Add,
                    BinOp::Sub => Op::Sub,
                    BinOp::Mul => Op::Mul,
                    BinOp::Div => Op::Div,
                    BinOp::Rem => Op::Rem,
                    BinOp::BitAnd => Op::BitAnd,
                    BinOp::BitOr => Op::BitOr,
                    BinOp::BitXor => Op::BitXor,
                    BinOp::And | BinOp::Or => unreachable!(),
                    _ => todo!("unsupported binary operator {}", op),
                }
                .into_byte();
                self.write_byte(byte);
            }
            Term::Let(LetKind::NonRec(_), lhs, rhs, tail) => {
                self.compile(*rhs);
                self.locals.push(lhs.content);
                self.compile(*tail);
                self.write_byte(Op::Pop.into_byte());
                self.locals.pop().unwrap();
            }
            Term::App(t1, t2) => {
                self.compile(*t2);
                self.compile(*t1);
            }
            Term::Cond(t1, t2, t3) => {
                self.compile(*t1);
                self.write_byte(Op::Jump.into_byte());
                let jump_offset_pos = self.code.len();
                self.write_index(usize::max_value());
                let t2_start = self.code.len();
                self.compile(*t2);
                self.write_byte(Op::Skip.into_byte());
                let skip_offset_pos = self.code.len();
                self.write_index(usize::max_value());
                let t3_start = self.code.len();
                self.compile(*t3);
                let t3_end = self.code.len();
                self.overwrite_index(jump_offset_pos, t3_start - t2_start);
                self.overwrite_index(skip_offset_pos, t3_end - t3_start);
            }
            _ => todo!("unsupported term `{}`", term),
        }
    }
}
