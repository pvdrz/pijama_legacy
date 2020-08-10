use pijama_common::{BinOp, Literal, UnOp};
use pijama_ctx::{Context, ContextExt, LocalId};
use pijama_mir::{BindKind, PrimFn, Term, TermKind};
use pijama_ty::Ty;

pub fn run(ctx: &Context, term: &Term) {
    let main = Function::new(0);
    let mut heap = Heap::new();
    heap.push(main);
    let mut compiler = Compiler::new(ctx, &mut heap, 0);
    compiler.compile(term);
    *heap.get_mut(0).unwrap() = compiler.func;
    let mut interpreter = Interpreter::new(0, heap);
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

#[derive(Debug, Clone)]
enum OpCode {
    PrintInt,
    PrintBool,
    PrintUnit,
    PrintFunc,
    Add,
    Eq,
    Neg,
    Local(usize),
    Call(usize),
    Push(Value),
    Return,
}

#[derive(Debug, Clone)]
struct Function {
    arity: usize,
    chunk: Chunk,
}

impl Function {
    fn new(arity: usize) -> Self {
        Self {
            arity,
            chunk: Chunk::default(),
        }
    }

    fn write(&mut self, opcode: OpCode) {
        self.chunk.write(opcode)
    }

    fn read(&mut self, index: usize) -> Option<&OpCode> {
        self.chunk.read(index)
    }
}

type Heap = Vec<Function>;

struct Compiler<'ast, 'ctx, 'heap> {
    ctx: &'ctx Context<'ast>,
    func: Function,
    locals: Vec<LocalId>,
    heap: &'heap mut Heap,
}

impl<'ast, 'ctx, 'heap> Compiler<'ast, 'ctx, 'heap> {
    fn new(ctx: &'ctx Context<'ast>, heap: &'heap mut Heap, ptr: usize) -> Self {
        let func = heap.get(ptr).unwrap().clone();
        Self {
            ctx,
            func,
            locals: vec![],
            heap,
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
                self.func.write(OpCode::Push(Value::Int(int)));
            }
            TermKind::Var(id) => {
                for (index, id2) in self.locals.iter().enumerate().rev() {
                    if id2 == id {
                        self.func.write(OpCode::Local(index));
                        return;
                    }
                }
                panic!()
            }
            TermKind::PrimApp(prim, args) => {
                for arg in args.iter().take(prim.arity()) {
                    self.compile(arg);
                }
                let opcode = match prim {
                    PrimFn::Print => match self.ctx.get_type_info(args[0].id).unwrap().ty {
                        Ty::Int => OpCode::PrintInt,
                        Ty::Bool => OpCode::PrintBool,
                        Ty::Unit => OpCode::PrintUnit,
                        Ty::Arrow(_, _) => OpCode::PrintFunc,
                        Ty::Var(_) => unreachable!(),
                    },
                    PrimFn::BinOp(BinOp::Add) => OpCode::Add,
                    PrimFn::BinOp(BinOp::Eq) => OpCode::Eq,
                    PrimFn::UnOp(UnOp::Neg) => OpCode::Neg,
                    _ => todo!(),
                };
                self.func.write(opcode);
            }
            TermKind::Let(BindKind::NonRec, lhs_id, rhs, tail) => {
                self.compile(rhs);
                self.locals.push(*lhs_id);
                self.compile(tail);
                self.locals.pop().unwrap();
            }
            TermKind::Abs(args, body) => {
                let function = Function::new(args.len());
                let ptr = self.heap.len();
                self.heap.push(function);
                let mut compiler = Compiler::new(self.ctx, self.heap, ptr);
                for arg in args {
                    compiler.locals.push(*arg);
                }
                compiler.compile(body);
                compiler.func.write(OpCode::Return);
                for _ in args {
                    compiler.locals.pop().unwrap();
                }
                let func = compiler.func;
                *self.heap.get_mut(ptr).unwrap() = func;

                self.func.write(OpCode::Push(Value::Ptr(ptr)));
            }
            TermKind::App(func, args) => {
                self.compile(func);
                for arg in args {
                    self.compile(arg);
                }
                self.func.write(OpCode::Call(args.len()));
            }

            _ => todo!(),
        }
    }
}

#[derive(Debug, Default, Clone)]
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

struct CallFrame {
    function: Function,
    ins_ptr: usize,
    base_ptr: usize,
}

struct CallStack {
    head: CallFrame,
    stack: Vec<CallFrame>,
}

impl CallStack {
    fn new(head: CallFrame) -> Self {
        Self {
            head,
            stack: vec![],
        }
    }

    fn head_mut(&mut self) -> &mut CallFrame {
        &mut self.head
    }

    fn head(&self) -> &CallFrame {
        &self.head
    }

    fn push(&mut self, head: CallFrame) {
        let old_head = std::mem::replace(&mut self.head, head);
        self.stack.push(old_head);
    }

    fn pop(&mut self) -> Option<CallFrame> {
        let new_head = self.stack.pop()?;
        Some(std::mem::replace(&mut self.head, new_head))
    }
}

#[derive(Default, Debug)]
struct ArgStack {
    base_ptr: usize,
    stack: Vec<Value>,
}

impl ArgStack {
    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Option<Value> {
        if self.stack.len() > self.base_ptr {
            Some(self.stack.pop().unwrap())
        } else {
            None
        }
    }

    fn get(&self, index: usize) -> Option<&Value> {
        self.stack.get(index + self.base_ptr)
    }

    fn len(&self) -> usize {
        self.stack.len() - self.base_ptr
    }

    fn inc_base(&mut self, base_ptr: usize) {
        self.base_ptr += base_ptr;
    }

    fn truncate(&mut self, base_ptr: usize) {
        self.base_ptr -= base_ptr;
        self.stack.truncate(self.len());
    }
}

struct Interpreter {
    arg_stack: ArgStack,
    call_stack: CallStack,
    heap: Heap,
}

impl Interpreter {
    fn new(main: usize, heap: Heap) -> Self {
        let main = heap.get(main).cloned().unwrap();
        Self {
            call_stack: CallStack::new(CallFrame {
                function: main,
                ins_ptr: 0,
                base_ptr: 0,
            }),
            arg_stack: ArgStack::default(),
            heap,
        }
    }

    fn read_op(&mut self) -> Option<OpCode> {
        let frame = self.call_stack.head_mut();
        let op = frame.function.read(frame.ins_ptr)?.clone();
        frame.ins_ptr += 1;
        Some(op)
    }

    fn run(&mut self) {
        // println!("{:?}", self.arg_stack);
        while let Some(op) = self.read_op() {
            // println!("{:?}", op);
            // println!("{:?}", self.arg_stack);
            match op {
                OpCode::PrintInt => {
                    let int = self.arg_stack.pop().unwrap().assert_int();
                    println!("{}", int);
                    self.arg_stack.push(Value::Int(0));
                }
                OpCode::PrintBool => {
                    let int = self.arg_stack.pop().unwrap().assert_int();
                    println!("{}", int != 0);
                    self.arg_stack.push(Value::Int(0));
                }
                OpCode::PrintUnit => {
                    let int = self.arg_stack.pop().unwrap().assert_int();
                    assert_eq!(int, 0);
                    println!("unit");
                    self.arg_stack.push(Value::Int(0));
                }
                OpCode::PrintFunc => {
                    let ptr = self.arg_stack.pop().unwrap().assert_ptr();
                    println!("<function at 0x{:x}>", ptr);
                    self.arg_stack.push(Value::Int(0));
                }
                OpCode::Add => {
                    let int2 = self.arg_stack.pop().unwrap().assert_int();
                    let int1 = self.arg_stack.pop().unwrap().assert_int();
                    self.arg_stack.push(Value::Int(int1 + int2));
                }
                OpCode::Eq => {
                    let int2 = self.arg_stack.pop().unwrap().assert_int();
                    let int1 = self.arg_stack.pop().unwrap().assert_int();
                    self.arg_stack.push(Value::Int((int1 == int2).into()));
                }
                OpCode::Neg => {
                    let int = self.arg_stack.pop().unwrap().assert_int();
                    self.arg_stack.push(Value::Int(-int));
                }
                OpCode::Push(value) => {
                    let value = value.clone();
                    self.arg_stack.push(value);
                }
                OpCode::Local(index) => {
                    let value = self.arg_stack.get(index).unwrap().clone();
                    self.arg_stack.push(value);
                }
                OpCode::Call(arity) => {
                    let base_ptr = self.arg_stack.len() - arity;
                    let ptr = self
                        .arg_stack
                        .get(base_ptr - 1)
                        .unwrap()
                        .clone()
                        .assert_ptr();
                    self.arg_stack.inc_base(base_ptr);
                    self.call_stack.push(CallFrame {
                        function: self.heap.get(ptr).cloned().unwrap(),
                        base_ptr,
                        ins_ptr: 0,
                    })
                }
                OpCode::Return => {
                    let ret_value = self.arg_stack.pop().unwrap();
                    let frame = self.call_stack.pop().unwrap();
                    self.arg_stack.truncate(frame.base_ptr);
                    self.arg_stack.push(ret_value);
                }
            }
        }
    }
}
