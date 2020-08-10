use pijama_common::{BinOp, Literal, UnOp};
use pijama_ctx::{Context, ContextExt};
use pijama_mir::{BindKind, PrimFn, Term, TermKind};
use pijama_ty::Ty;

pub fn run(ctx: &Context, term: &Term) {
    let mut compiler = Compiler::new(ctx);
    compiler.compile(term);
    let mut interpreter = Interpreter::new(compiler.chunk);
    interpreter.run();
}

#[derive(Debug, Clone)]
enum Value {
    Int(i64),
    Ptr(usize),
}

impl Value {
    fn assert_int(self) -> i64 {
        if let Self::Int(int) = self {
            int
        } else {
            panic!("expected integer");
        }
    }

    fn assert_ptr(self) -> usize {
        if let Self::Ptr(ptr) = self {
            ptr
        } else {
            panic!("expected pointer");
        }
    }
}

enum OpCode {
    PrintInt,
    Add,
    Eq,
    Neg,
    Local(usize),
    Call(usize),
    Push(Value),
    Return,
}

struct Compiler<'ast, 'ctx> {
    ctx: &'ctx Context<'ast>,
    chunk: Chunk,
}

impl<'ast, 'ctx> Compiler<'ast, 'ctx> {
    fn new(ctx: &'ctx Context<'ast>) -> Self {
        Self {
            ctx,
            chunk: Chunk::default(),
        }
    }
    fn compile(&mut self, term: &Term) {
        match &term.kind {
            TermKind::Lit(lit) => {
                let int = match lit {
                    Literal::Bool(true) => 1,
                    Literal::Bool(false) => 0,
                    Literal::Number(int) => *int,
                    Literal::Unit => 0,
                };
                self.chunk.write(OpCode::Push(Value::Int(int)));
            }
            TermKind::PrimApp(prim, args) => {
                for arg in args.iter().take(prim.arity()) {
                    self.compile(arg);
                }
                let opcode = match prim {
                    PrimFn::Print => match self.ctx.get_type_info(args[0].id).unwrap().ty {
                        Ty::Int => OpCode::PrintInt,
                        _ => todo!(),
                    },
                    PrimFn::BinOp(BinOp::Add) => OpCode::Add,
                    PrimFn::BinOp(BinOp::Eq) => OpCode::Eq,
                    PrimFn::UnOp(UnOp::Neg) => OpCode::Neg,
                    _ => todo!(),
                };
                self.chunk.write(opcode);
            }
            _ => todo!(),
        }
    }
}

#[derive(Default)]
struct Chunk {
    code: Vec<OpCode>,
}

impl Chunk {
    fn write(&mut self, opcode: OpCode) {
        self.code.push(opcode);
    }

    fn read(&self, index: usize) -> Option<&OpCode> {
        self.code.get(index)
    }
}

struct Interpreter {
    stack: Vec<Value>,
    chunk: Chunk,
    ins_ptr: usize,
}

impl Interpreter {
    fn new(chunk: Chunk) -> Self {
        Self {
            chunk,
            stack: vec![],
            ins_ptr: 0,
        }
    }
    fn read_op(&mut self) -> Option<&OpCode> {
        let op = self.chunk.read(self.ins_ptr)?;
        self.ins_ptr += 1;
        Some(op)
    }

    fn run(&mut self) {
        while let Some(op) = self.read_op() {
            match op {
                OpCode::PrintInt => {
                    let int = self.stack.pop().unwrap().assert_int();
                    println!("{}", int);
                }
                OpCode::Add => {
                    let int1 = self.stack.pop().unwrap().assert_int();
                    let int2 = self.stack.pop().unwrap().assert_int();
                    self.stack.push(Value::Int(int1 + int2));
                }
                OpCode::Eq => {
                    let int1 = self.stack.pop().unwrap().assert_int();
                    let int2 = self.stack.pop().unwrap().assert_int();
                    self.stack.push(Value::Int((int1 == int2).into()));
                }
                OpCode::Neg => {
                    let int = self.stack.pop().unwrap().assert_int();
                    self.stack.push(Value::Int(-int));
                }
                OpCode::Push(value) => {
                    let value = value.clone();
                    self.stack.push(value);
                }
                _ => todo!(),
            }
        }
    }
}
