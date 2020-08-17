use pijama_common::{BinOp, Literal, UnOp};
use pijama_ctx::{Context, ContextExt, LocalId};
use pijama_mir::{Lambda, PrimFn, Term, TermKind};
use pijama_ty::Ty;
use pijama_vm::*;

pub fn run(ctx: &Context, term: &Term) {
    let heap = Heap::new();
    let mut code = vec![CodeBuf::default()];

    let mut compiler = Compiler::new(ctx, &mut code, &heap, LocalId::main());
    compiler.compile(term);
    code[0].write_u8(EXIT);

    println!("main:");
    code[0].disassemble();

    let main_ptr = FuncPtr::new(0);

    let main = heap.insert(Closure::new(main_ptr));

    Machine::new(main, &code).run();
}

pub fn compile<'code>(
    ctx: &Context,
    code: &'code mut Vec<CodeBuf>,
    heap: &'code Heap,
    term: &Term,
) -> Machine<'code> {
    code.push(CodeBuf::default());

    let mut compiler = Compiler::new(ctx, code, &heap, LocalId::main());
    compiler.compile(term);
    code[0].write_u8(EXIT);

    let main_ptr = FuncPtr::new(0);

    let main = heap.insert(Closure::new(main_ptr));

    Machine::new(main, code)
}

struct Compiler<'ast, 'ctx, 'code> {
    ctx: &'ctx Context<'ast>,
    locals: Vec<LocalId>,
    code: &'code mut Vec<CodeBuf>,
    index: usize,
    heap: &'code Heap,
}

impl<'ast, 'ctx, 'code> Compiler<'ast, 'ctx, 'code> {
    fn new(
        ctx: &'ctx Context<'ast>,
        code: &'code mut Vec<CodeBuf>,
        heap: &'code Heap,
        local_id: LocalId,
    ) -> Self {
        let index = code.len() - 1;
        Self {
            ctx,
            locals: vec![local_id],
            code,
            index,
            heap,
        }
    }

    fn code(&mut self) -> &mut CodeBuf {
        &mut self.code[self.index]
    }

    fn compile_lambda(&mut self, local_id: LocalId, Lambda(_, args, body): &Lambda) {
        let code_ptr = self.index + 1;

        self.code.push(CodeBuf::default());

        let mut compiler = Compiler::new(self.ctx, self.code, self.heap, local_id);
        for arg in args {
            compiler.locals.push(*arg);
        }
        compiler.compile(body);
        compiler.code().write_u8(Return::CODE);
        for _ in args {
            compiler.locals.pop().unwrap();
        }

        let func_ptr = FuncPtr::new(code_ptr);

        let ptr = self.heap.insert(Closure::new(func_ptr));

        println!("{}:", self.ctx.get_local(local_id).unwrap());
        compiler.code().disassemble();

        self.code().write_u8(PushClosure::CODE);
        self.code().write_i64(ptr as i64);
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
                self.code().write_u8(Push::CODE);
                self.code().write_i64(int);
            }
            TermKind::Var(id) => {
                for (index, id2) in self.locals.iter().enumerate().rev() {
                    if id2 == id {
                        self.code().write_u8(PushLocal::CODE);
                        self.code().write_i64(index as i64);
                        return;
                    }
                }
                panic!("could not find {:?}", id)
            }
            TermKind::PrimApp(PrimFn::BinOp(BinOp::And), args) => {
                self.compile(&args[0]);
                self.code().write_u8(JumpIfZero::CODE);
                self.code().write_i64(i64::max_value());

                let start_arg_1 = self.code().len();
                self.code().write_u8(Pop::CODE);
                self.compile(&args[1]);
                let end_arg_1 = self.code().len();

                self.code()
                    .overwrite_i64(start_arg_1 - 8, (end_arg_1 - start_arg_1) as i64)
                    .unwrap();
            }
            TermKind::PrimApp(PrimFn::BinOp(BinOp::Or), args) => {
                self.compile(&args[0]);
                self.code().write_u8(JumpNonZero::CODE);
                self.code().write_i64(i64::max_value());

                let start_arg_1 = self.code().len();
                self.code().write_u8(Pop::CODE);
                self.compile(&args[1]);
                let end_arg_1 = self.code().len();

                self.code()
                    .overwrite_i64(start_arg_1 - 8, (end_arg_1 - start_arg_1) as i64);
            }
            TermKind::PrimApp(prim, args) => {
                for arg in args.iter().take(prim.arity()) {
                    self.compile(arg);
                }
                let opcode = match prim {
                    PrimFn::Print => match self.ctx.get_type_info(args[0].id).unwrap().ty {
                        Ty::Int => PrintInt::CODE,
                        Ty::Bool => PrintBool::CODE,
                        Ty::Unit => PrintUnit::CODE,
                        Ty::Arrow(_, _) => PrintFunc::CODE,
                        Ty::Var(_) => unreachable!(),
                    },
                    PrimFn::UnOp(UnOp::Neg) => Neg::CODE,
                    PrimFn::UnOp(UnOp::Not) => Not::CODE,
                    PrimFn::BinOp(BinOp::Add) => Add::CODE,
                    PrimFn::BinOp(BinOp::Sub) => Sub::CODE,
                    PrimFn::BinOp(BinOp::Mul) => Mul::CODE,
                    PrimFn::BinOp(BinOp::Div) => Div::CODE,
                    PrimFn::BinOp(BinOp::Rem) => Rem::CODE,
                    PrimFn::BinOp(BinOp::BitAnd) => BitAnd::CODE,
                    PrimFn::BinOp(BinOp::BitOr) => BitOr::CODE,
                    PrimFn::BinOp(BinOp::BitXor) => BitXor::CODE,
                    PrimFn::BinOp(BinOp::Shr) => Shr::CODE,
                    PrimFn::BinOp(BinOp::Shl) => Shl::CODE,
                    PrimFn::BinOp(BinOp::Eq) => Eq::CODE,
                    PrimFn::BinOp(BinOp::Neq) => Neq::CODE,
                    PrimFn::BinOp(BinOp::Lt) => Lt::CODE,
                    PrimFn::BinOp(BinOp::Gt) => Gt::CODE,
                    PrimFn::BinOp(BinOp::Lte) => Lte::CODE,
                    PrimFn::BinOp(BinOp::Gte) => Gte::CODE,
                    PrimFn::BinOp(BinOp::And) | PrimFn::BinOp(BinOp::Or) => unreachable!(),
                };
                self.code().write_u8(opcode);
            }
            TermKind::Let(lhs_id, rhs, tail) => {
                self.locals.push(*lhs_id);
                if rhs.1.len() > 0 {
                    self.compile_lambda(*lhs_id, rhs);
                } else {
                    self.compile(&rhs.2);
                }
                self.compile(tail);
                self.locals.pop().unwrap();
            }
            TermKind::App(func, args) => {
                self.compile(func);
                for arg in args {
                    self.compile(arg);
                }
                self.code().write_u8(Call::CODE);
                self.code().write_i64(args.len() as i64);
            }
            TermKind::Cond(if_term, do_term, else_term) => {
                self.compile(if_term.as_ref());
                self.code().write_u8(JumpIfZero::CODE);
                self.code().write_i64(i64::max_value());

                let start_do = self.code().len();
                self.code().write_u8(Pop::CODE);
                self.compile(do_term.as_ref());
                self.code().write_u8(Jump::CODE);
                self.code().write_i64(i64::max_value());
                let end_do = self.code().len();
                self.code().write_u8(Pop::CODE);

                self.code()
                    .overwrite_i64(start_do - 8, (end_do - start_do) as i64);

                let start_else = end_do;
                self.compile(else_term.as_ref());
                let end_else = self.code().len();

                self.code()
                    .overwrite_i64(start_else - 8, (end_else - start_else) as i64);
            }
        }
    }
}
