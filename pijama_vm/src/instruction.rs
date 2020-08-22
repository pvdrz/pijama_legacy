use std::io;

use crate::{Closure, CodeSlice, Machine};

pub trait Instruction {
    const CODE: u8;

    fn run(machine: &mut Machine);

    fn disassemble<W: io::Write>(
        code: &CodeSlice,
        index: usize,
        buffer: &mut W,
    ) -> io::Result<usize>;
}

// ----------------
// Print Operations
// ----------------

pub struct PrintInt;

impl Instruction for PrintInt {
    const CODE: u8 = 94;

    fn run(machine: &mut Machine) {
        let value = machine.arg_stack.pop().unwrap();
        println!("{}", value);
    }

    fn disassemble<W: io::Write>(
        _code: &CodeSlice,
        index: usize,
        buffer: &mut W,
    ) -> io::Result<usize> {
        writeln!(buffer, "PrintInt")?;
        Ok(index)
    }
}

pub struct PrintBool;

impl Instruction for PrintBool {
    const CODE: u8 = 95;

    fn run(machine: &mut Machine) {
        let value = machine.arg_stack.pop().unwrap();
        println!("{}", value == 0);
    }

    fn disassemble<W: io::Write>(
        _code: &CodeSlice,
        index: usize,
        buffer: &mut W,
    ) -> io::Result<usize> {
        writeln!(buffer, "PrintBool")?;
        Ok(index)
    }
}

pub struct PrintUnit;

impl Instruction for PrintUnit {
    const CODE: u8 = 96;

    fn run(machine: &mut Machine) {
        let _value = machine.arg_stack.pop().unwrap();
        println!("unit");
    }

    fn disassemble<W: io::Write>(
        _code: &CodeSlice,
        index: usize,
        buffer: &mut W,
    ) -> io::Result<usize> {
        writeln!(buffer, "PrintUnit")?;
        Ok(index)
    }
}

pub struct PrintFunc;

impl Instruction for PrintFunc {
    const CODE: u8 = 97;

    fn run(machine: &mut Machine) {
        let value = machine.arg_stack.pop().unwrap();
        println!("<function at 0x{:x}", value);
    }

    fn disassemble<W: io::Write>(
        _code: &CodeSlice,
        index: usize,
        buffer: &mut W,
    ) -> io::Result<usize> {
        writeln!(buffer, "PrintFunc")?;
        Ok(index)
    }
}

// -----------------
// Unary Operations
// -----------------

pub struct Not;

impl Instruction for Not {
    const CODE: u8 = 98;

    fn run(machine: &mut Machine) {
        let value = machine.arg_stack.pop().unwrap();
        machine.arg_stack.push(value ^ 0);
    }

    fn disassemble<W: io::Write>(
        _code: &CodeSlice,
        index: usize,
        buffer: &mut W,
    ) -> io::Result<usize> {
        writeln!(buffer, "Not")?;
        Ok(index)
    }
}

pub struct Neg;

impl Instruction for Neg {
    const CODE: u8 = 99;

    fn run(machine: &mut Machine) {
        let value = machine.arg_stack.pop().unwrap();
        machine.arg_stack.push(!value);
    }

    fn disassemble<W: io::Write>(
        _code: &CodeSlice,
        index: usize,
        buffer: &mut W,
    ) -> io::Result<usize> {
        writeln!(buffer, "Neg")?;
        Ok(index)
    }
}

// -----------------
// Binary Operations
// -----------------

macro_rules! bin_op {
    ($name:ident, $op:tt, $code:tt) => {
        pub struct $name;

        impl Instruction for $name {
            const CODE: u8 = $code;

            fn run(machine: &mut Machine) {
                let right_value = machine.arg_stack.pop().unwrap();
                let left_value = machine.arg_stack.pop().unwrap();

                machine.arg_stack.push(i64::from(left_value $op right_value));
            }

            fn disassemble<W: io::Write>(
                _code: &CodeSlice,
                index: usize,
                buffer: &mut W,
            ) -> io::Result<usize> {
                writeln!(buffer, stringify!($name))?;
                Ok(index)
            }
        }
    }
}

bin_op!(Add, +, 100);
bin_op!(Sub, -, 101);
bin_op!(Mul, *, 102);
bin_op!(Div, /, 103);
bin_op!(Rem, %, 104);
bin_op!(BitAnd, &, 105);
bin_op!(BitOr, |, 106);
bin_op!(BitXor, ^, 107);
bin_op!(Shr, >>, 108);
bin_op!(Shl, <<, 109);
bin_op!(Eq, ==, 110);
bin_op!(Neq, !=, 111);
bin_op!(Lt, <, 112);
bin_op!(Gt, >, 113);
bin_op!(Lte, <=, 114);
bin_op!(Gte, >=, 115);

// ---------------------------
// Argument Stack Instructions
// ---------------------------

pub struct Push;

impl Instruction for Push {
    const CODE: u8 = 116;

    fn run(machine: &mut Machine) {
        let value = unsafe { machine.read_i64() };

        machine.arg_stack.push(value);
    }

    fn disassemble<W: io::Write>(
        code: &CodeSlice,
        index: usize,
        buffer: &mut W,
    ) -> io::Result<usize> {
        let value = code.read_i64(index).unwrap();

        writeln!(buffer, "Push {}", value)?;
        Ok(index + 8)
    }
}

pub struct Pop;

impl Instruction for Pop {
    const CODE: u8 = 117;

    fn run(machine: &mut Machine) {
        machine.arg_stack.pop().unwrap();
    }

    fn disassemble<W: io::Write>(
        _code: &CodeSlice,
        index: usize,
        buffer: &mut W,
    ) -> io::Result<usize> {
        writeln!(buffer, "Pop")?;
        Ok(index)
    }
}

pub struct PushLocal;

impl Instruction for PushLocal {
    const CODE: u8 = 118;

    fn run(machine: &mut Machine) {
        let index = unsafe { machine.read_i64() } as usize;

        let local = machine.arg_stack[index];
        machine.arg_stack.push(local);
    }

    fn disassemble<W: io::Write>(
        code: &CodeSlice,
        index: usize,
        buffer: &mut W,
    ) -> io::Result<usize> {
        let offset = code.read_i64(index).unwrap() as usize;

        writeln!(buffer, "PushLocal {}", offset)?;
        Ok(index + 8)
    }
}

pub struct PushUpvalue;

impl Instruction for PushUpvalue {
    const CODE: u8 = 150;

    fn run(machine: &mut Machine) {
        let index = unsafe { machine.read_i64() } as usize;
        let upvalue = unsafe { &*(machine.call_stack.last_mut().closure()) }.get_upvalue(index);
        machine.arg_stack.push(upvalue);
    }

    fn disassemble<W: io::Write>(
        code: &CodeSlice,
        index: usize,
        buffer: &mut W,
    ) -> io::Result<usize> {
        let offset = code.read_i64(index).unwrap() as usize;

        writeln!(buffer, "PushUpvalue {}", offset)?;
        Ok(index + 8)
    }
}
// -------------------------
// Control Flow Instructions
// -------------------------

pub struct Jump;

impl Instruction for Jump {
    const CODE: u8 = 119;

    fn run(machine: &mut Machine) {
        let offset = unsafe { machine.read_i64() } as usize;

        *machine.call_stack.last_mut().ins_ptr_mut() += offset;
    }

    fn disassemble<W: io::Write>(
        code: &CodeSlice,
        index: usize,
        buffer: &mut W,
    ) -> io::Result<usize> {
        let offset = code.read_i64(index).unwrap() as usize;

        writeln!(buffer, "Jump {}", offset)?;
        Ok(index + 8)
    }
}

pub struct JumpIfZero;

impl Instruction for JumpIfZero {
    const CODE: u8 = 120;

    fn run(machine: &mut Machine) {
        let offset = unsafe { machine.read_i64() } as usize;

        if machine.arg_stack.last().unwrap() == 0 {
            *machine.call_stack.last_mut().ins_ptr_mut() += offset;
        }
    }

    fn disassemble<W: io::Write>(
        code: &CodeSlice,
        index: usize,
        buffer: &mut W,
    ) -> io::Result<usize> {
        let offset = code.read_i64(index).unwrap() as usize;

        writeln!(buffer, "JumpIfZero {}", offset)?;
        Ok(index + 8)
    }
}

pub struct JumpNonZero;

impl Instruction for JumpNonZero {
    const CODE: u8 = 121;

    fn run(machine: &mut Machine) {
        let offset = unsafe { machine.read_i64() } as usize;

        if machine.arg_stack.last().unwrap() != 0 {
            *machine.call_stack.last_mut().ins_ptr_mut() += offset;
        }
    }

    fn disassemble<W: io::Write>(
        code: &CodeSlice,
        index: usize,
        buffer: &mut W,
    ) -> io::Result<usize> {
        let offset = code.read_i64(index).unwrap() as usize;

        writeln!(buffer, "JumpNonZero {}", offset)?;
        Ok(index + 8)
    }
}

// -----------------------
// Call Stack Instructions
// -----------------------

pub struct PushClosure;

impl Instruction for PushClosure {
    const CODE: u8 = 122;

    fn run(machine: &mut Machine) {
        let value = unsafe { machine.read_i64() };
        let count = unsafe { machine.read_i64() } as usize;

        let closure = unsafe { &mut *(value as *mut Closure) };
        for i in 0..count {
            let is_local = unsafe { machine.read_u8() } != 0;
            let index = unsafe { machine.read_i64() } as usize;

            if is_local {
                closure.push_upvalue(machine.arg_stack[index]);
            } else {
                let peek_closure = unsafe { &*(machine.call_stack.peek().closure()) };
                closure.push_upvalue(peek_closure.get_upvalue(index));
            }
        }

        machine.arg_stack.push(value);
    }

    fn disassemble<W: io::Write>(
        code: &CodeSlice,
        mut index: usize,
        buffer: &mut W,
    ) -> io::Result<usize> {
        let value = code.read_i64(index).unwrap();
        index += 8;
        let count = code.read_i64(index).unwrap();
        index += 8;

        writeln!(buffer, "PushClosure 0x{:x} {}", value, count)?;

        for _ in 0..count {
            let is_local = code.read_u8(index).unwrap() != 0;
            index += 1;
            let upvalue_index = code.read_i64(index).unwrap();
            index += 8;

            writeln!(buffer, "      Upvalue {} {}", is_local, upvalue_index)?;
        }
        Ok(index)
    }
}

pub struct Return;

impl Instruction for Return {
    const CODE: u8 = 123;

    fn run(machine: &mut Machine) {
        let return_value = machine.arg_stack.pop().unwrap();
        let base_ptr = machine.call_stack.pop().unwrap().base_ptr();

        machine.arg_stack.clear();
        machine.arg_stack.decrease_base(base_ptr);

        machine.arg_stack.push(return_value);
    }

    fn disassemble<W: io::Write>(
        _code: &CodeSlice,
        index: usize,
        buffer: &mut W,
    ) -> io::Result<usize> {
        writeln!(buffer, "Return")?;
        Ok(index)
    }
}

pub struct Call;

impl Instruction for Call {
    const CODE: u8 = 124;

    fn run(machine: &mut Machine) {
        let arity = unsafe { machine.read_i64() } as usize;

        let base_ptr = machine.arg_stack.len() - (arity + 1);
        let closure = machine.arg_stack[base_ptr] as usize as *const Closure;

        machine.arg_stack.increase_base(base_ptr);
        machine
            .call_stack
            .push_frame(closure, &machine.code, base_ptr);
    }

    fn disassemble<W: io::Write>(
        code: &CodeSlice,
        index: usize,
        buffer: &mut W,
    ) -> io::Result<usize> {
        let arity = code.read_i64(index).unwrap() as usize;

        writeln!(buffer, "Call {}", arity)?;
        Ok(index + 8)
    }
}
