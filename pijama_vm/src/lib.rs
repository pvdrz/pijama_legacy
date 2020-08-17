mod arg_stack;
mod call_stack;
mod closure;
mod heap;
mod instruction;

use arg_stack::ArgStack;
use call_stack::CallStack;

pub use closure::{Closure, CodeBuf, CodeSlice, FuncPtr};
pub use heap::Heap;

pub const EXIT: u8 = 93;

macro_rules! use_instructions {
    ($($instruction:ident), *) => {
        use std::io;
        pub use instruction::{Instruction, $($instruction, )* };

        impl<'code> Machine<'code> {
            pub fn run(&mut self) {
                loop {
                    // println!("{:?}", self.arg_stack);
                    match self.read_u8() {
                        $($instruction::CODE => {
                            // println!("{}", stringify!($instruction));
                            $instruction::run(self)
                        }, )*
                        EXIT => break,
                        byte => panic!("Invalid opcode {}", byte),
                    }
                }
            }
        }

        fn disassemble<W: io::Write>(code: &CodeSlice, w: &mut W) -> io::Result<()> {
            let mut index = 0;
            while let Some(byte) = code.read_u8(index) {
                write!(w, "{:04}: ", index)?;
                index += 1;
                index = match byte {
                    $($instruction::CODE => $instruction::disassemble(code, index, w)?, )*
                    EXIT => {
                        writeln!(w, "EXIT")?;
                        break;
                    }
                    byte => panic!("Invalid opcode {}", byte),
                };
            }
            writeln!(w, "")
        }
    }
}

use_instructions!(
    PrintInt,
    PrintBool,
    PrintUnit,
    PrintFunc,
    Not,
    Neg,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    BitAnd,
    BitOr,
    BitXor,
    Shr,
    Shl,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
    Push,
    Pop,
    PushLocal,
    Jump,
    JumpIfZero,
    JumpNonZero,
    PushClosure,
    Return,
    Call
);

#[derive(Clone)]
pub struct Machine<'code> {
    code: &'code [CodeBuf],
    arg_stack: ArgStack,
    call_stack: CallStack<'code>,
}

impl<'code> Machine<'code> {
    pub fn new(main: *const Closure, code: &'code [CodeBuf]) -> Self {
        let mut arg_stack = ArgStack::default();
        arg_stack.push(main as i64);

        Self {
            code,
            arg_stack,
            call_stack: CallStack::new(main, &code),
        }
    }

    unsafe fn read_i64(&mut self) -> i64 {
        let frame = self.call_stack.last_mut();
        let ins_ptr = frame.ins_ptr();
        let byte = frame.code().read_i64(ins_ptr).unwrap();
        *frame.ins_ptr_mut() += 8;
        byte
    }

    fn read_u8(&mut self) -> u8 {
        let frame = self.call_stack.last_mut();
        let ins_ptr = frame.ins_ptr();
        let byte = frame.code().read_u8(ins_ptr).unwrap();
        *frame.ins_ptr_mut() += 1;
        byte
    }
}
