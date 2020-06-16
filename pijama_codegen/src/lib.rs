use pijama_ast::{location::Located, node::{UnOp, Literal, BinOp}};
use pijama_mir::Term;

pub fn codegen(term: Located<Term>) -> (Vec<u8>, Vec<i64>) {
    let mut generator = Generator::default();
    generator.transpile(term);
    generator.code.push(Op::Ret.into_byte());
    (generator.code, generator.values)
}

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
            _ => panic!("Invalid opcode {}", byte),
        }
    }
}

#[derive(Default)]
struct Generator {
    code: Vec<u8>,
    values: Vec<i64>,
}

impl Generator {
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

    fn transpile(&mut self, term: Located<Term>) {
        match term.content {
            Term::Lit(Literal::Number(uint)) => {
                let index = self.store_value(uint);
                self.write_byte(Op::Lit.into_byte());
                self.write_index(index);
            }
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
                }.into_byte();
                self.write_byte(byte);
            }
            _ => todo!("unsupported term `{}`", term)
        }
    }
}
