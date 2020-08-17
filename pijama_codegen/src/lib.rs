mod compiler;

use pijama_ctx::{Context, LocalId};
use pijama_mir::Term;
use pijama_vm::{EXIT, Heap, CodeBuf, FuncPtr, Closure, Machine};

use compiler::Compiler;

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
