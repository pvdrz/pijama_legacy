use pijama_ast::{
    location::Located,
    node::{BinOp, Literal, Name, Primitive, UnOp},
};
use pijama_mir::{LetKind, Term};

pub fn codegen(term: Located<Term>) -> (Vec<u8>, Vec<i64>) {
    let mut generator = Generator::default();
    generator.transpile(term);
    (generator.code, generator.values)
}

#[derive(Debug)]
pub enum Op {
    Ret,
    Lit,
    Neg,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    BitAnd,
    BitOr,
    BitXor,
    True,
    False,
    Unit,
    GetLocal,
    Pop,
    Print,
}

impl Op {
    pub fn into_byte(self) -> u8 {
        match self {
            Op::Ret => 0,
            Op::Lit => 1,
            Op::Neg => 2,
            Op::Add => 3,
            Op::Sub => 4,
            Op::Mul => 5,
            Op::Div => 6,
            Op::Rem => 7,
            Op::BitAnd => 8,
            Op::BitOr => 9,
            Op::BitXor => 10,
            Op::True => 11,
            Op::False => 12,
            Op::Unit => 13,
            Op::GetLocal => 14,
            Op::Pop => 15,
            Op::Print => 16,
        }
    }

    pub fn from_byte(byte: u8) -> Self {
        match byte {
            0 => Op::Ret,
            1 => Op::Lit,
            2 => Op::Neg,
            3 => Op::Add,
            4 => Op::Sub,
            5 => Op::Mul,
            6 => Op::Div,
            7 => Op::Rem,
            8 => Op::BitAnd,
            9 => Op::BitOr,
            10 => Op::BitXor,
            11 => Op::True,
            12 => Op::False,
            13 => Op::Unit,
            14 => Op::GetLocal,
            15 => Op::Pop,
            16 => Op::Print,
            _ => panic!("Invalid opcode {}", byte),
        }
    }
}

#[derive(Default)]
struct Generator<'a> {
    code: Vec<u8>,
    values: Vec<i64>,
    locals: Vec<Name<'a>>,
}

impl<'a> Generator<'a> {
    fn write_byte(&mut self, byte: u8) {
        self.code.push(byte);
    }

    fn write_index(&mut self, index: usize) {
        for &byte in index.to_be_bytes().iter() {
            self.write_byte(byte);
        }
    }

    fn store_value(&mut self, value: i64) -> usize {
        let index = self.values.len();
        self.values.push(value);
        index
    }

    fn transpile(&mut self, term: Located<Term<'a>>) {
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
                Literal::Number(uint) => {
                    let index = self.store_value(uint);
                    self.write_byte(Op::Lit.into_byte());
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
                self.transpile(*term);
                self.write_byte(Op::Neg.into_byte());
            }
            Term::BinaryOp(op, term1, term2) => {
                self.transpile(*term2);
                self.transpile(*term1);
                let byte = match op {
                    BinOp::Add => Op::Add,
                    BinOp::Sub => Op::Sub,
                    BinOp::Mul => Op::Mul,
                    BinOp::Div => Op::Div,
                    BinOp::Rem => Op::Rem,
                    BinOp::BitAnd => Op::BitAnd,
                    BinOp::BitOr => Op::BitOr,
                    BinOp::BitXor => Op::BitXor,
                    _ => todo!("unsupported binary operator {}", op),
                }
                .into_byte();
                self.write_byte(byte);
            }
            Term::Let(LetKind::NonRec(_), lhs, rhs, tail) => {
                self.transpile(*rhs);
                self.locals.push(lhs.content);
                self.transpile(*tail);
                self.write_byte(Op::Pop.into_byte());
                self.locals.pop().unwrap();
            }
            Term::App(t1, t2) => {
                self.transpile(*t2);
                self.transpile(*t1);
            }
            _ => todo!("unsupported term `{}`", term),
        }
    }
}
