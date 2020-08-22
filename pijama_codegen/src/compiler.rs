use pijama_common::{BinOp, Literal, UnOp};
use pijama_ctx::{Context, ContextExt, LocalId};
use pijama_mir::{Lambda, PrimFn, Term, TermKind};
use pijama_ty::Ty;
use pijama_vm::*;

#[derive(Clone, Eq, PartialEq)]
struct Upvalue {
    index: usize,
    is_local: bool,
}

impl Upvalue {
    fn new(index: usize, is_local: bool) -> Self {
        Self { index, is_local }
    }
}

struct Scope {
    locals: Vec<LocalId>,
    upvalues: Vec<Upvalue>,
}

impl Scope {
    fn new(id: LocalId) -> Self {
        Self {
            locals: vec![id],
            upvalues: vec![],
        }
    }

    fn push_local(&mut self, id: LocalId) {
        self.locals.push(id);
    }

    fn pop_local(&mut self) {
        self.locals.pop().unwrap();
    }

    fn resolve_local(&self, id: LocalId) -> Option<usize> {
        for (index, id2) in self.locals.iter().enumerate().rev() {
            if *id2 == id {
                return Some(index);
            }
        }
        None
    }

    fn add_upvalue(&mut self, new_upvalue: Upvalue) -> usize {
        for (index, upvalue) in self.upvalues.iter().enumerate() {
            if *upvalue == new_upvalue {
                return index;
            }
        }

        let index = self.upvalues.len();
        self.upvalues.push(new_upvalue);

        index
    }
}

pub(crate) struct Compiler<'ast, 'ctx, 'code> {
    ctx: &'ctx Context<'ast>,
    scopes: Vec<Scope>,
    code: &'code mut Vec<CodeBuf>,
    heap: &'code Heap,
}

impl<'ast, 'ctx, 'code> Compiler<'ast, 'ctx, 'code> {
    pub(crate) fn new(
        ctx: &'ctx Context<'ast>,
        code: &'code mut Vec<CodeBuf>,
        heap: &'code Heap,
        local_id: LocalId,
    ) -> Self {
        Self {
            ctx,
            scopes: vec![Scope::new(local_id)],
            code,
            heap,
        }
    }

    fn scope_mut(&mut self, lvl: usize) -> &mut Scope {
        let index = self.scopes.len() - lvl - 1;
        if let Some(scope) = self.scopes.get_mut(index) {
            scope
        } else {
            panic!("Reached root scope")
        }
    }

    fn code(&mut self) -> &mut CodeBuf {
        &mut self.code[self.scopes.len() - 1]
    }

    fn compile_lambda(&mut self, local_id: LocalId, Lambda(_, args, body): &Lambda) {
        self.scopes.push(Scope::new(local_id));
        self.code.push(CodeBuf::default());

        for arg in args {
            self.scope_mut(0).push_local(*arg);
        }
        self.compile(body);
        self.code().write_u8(Return::CODE);
        for _ in args {
            self.scope_mut(0).pop_local();
        }

        let func_ptr = FuncPtr::new(self.scopes.len() - 1);
        let ptr = self.heap.insert(Closure::new(func_ptr));

        println!("{}:", self.ctx.get_local(local_id).unwrap());
        self.code().disassemble();

        let scope = self.scopes.pop().unwrap();

        self.code().write_u8(PushClosure::CODE);
        self.code().write_i64(ptr as i64);
        self.code().write_i64(scope.upvalues.len() as i64);

        for upvalue in scope.upvalues {
            self.code().write_u8(upvalue.is_local.into());
            self.code().write_i64(upvalue.index as i64);
        }
    }

    fn resolve_local(&mut self, id: LocalId, level: usize) -> Option<usize> {
        self.scope_mut(level).resolve_local(id)
    }

    fn resolve_upvalue(&mut self, id: LocalId, level: usize) -> usize {
        let upvalue = if let Some(local) = self.resolve_local(id, level + 1) {
            Upvalue::new(local, true)
        } else {
            let index = self.resolve_upvalue(id, level + 1);
            Upvalue::new(index, false)
        };

        self.scope_mut(level).add_upvalue(upvalue)
    }

    pub(crate) fn compile(&mut self, term: &Term) {
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
            &TermKind::Var(id) => {
                if let Some(index) = self.resolve_local(id, 0) {
                    self.code().write_u8(PushLocal::CODE);
                    self.code().write_i64(index as i64);
                } else {
                    let index = self.resolve_upvalue(id, 0);
                    self.code().write_u8(PushUpvalue::CODE);
                    self.code().write_i64(index as i64);
                }
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
                self.scope_mut(0).push_local(*lhs_id);
                if rhs.1.len() > 0 {
                    self.compile_lambda(*lhs_id, rhs);
                } else {
                    self.compile(&rhs.2);
                }
                self.compile(tail);
                self.scope_mut(0).pop_local();
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
