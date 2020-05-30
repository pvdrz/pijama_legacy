use std::convert::TryFrom;

use pijama_ast::{BinOp, BinOp::*, UnOp, UnOp::*};

/// Trait determining how arithmetic operations should be handled.
pub trait Arithmetic {
    fn binary_operation(op: BinOp, n1: i64, n2: i64) -> i64;
    fn unary_operation(op: UnOp, n: i64) -> i64;
}

/// Regular arithmetic that is allowed to overflow or panic when dividing by zero.
pub struct OverflowArithmetic;

impl Arithmetic for OverflowArithmetic {
    fn binary_operation(op: BinOp, n1: i64, n2: i64) -> i64 {
        match op {
            Add => n1 + n2,
            Sub => n1 - n2,
            Mul => n1 * n2,
            Div => n1 / n2,
            Rem => n1 % n2,
            Lt => (n1 < n2).into(),
            Lte => (n1 <= n2).into(),
            Gt => (n1 > n2).into(),
            Gte => (n1 >= n2).into(),
            Eq => (n1 == n2).into(),
            Neq => (n1 != n2).into(),
            BitAnd | And => n1 & n2,
            BitOr | Or => n1 | n2,
            BitXor => n1 ^ n2,
            Shr => n1 >> n2,
            Shl => n1 << n2,
        }
    }

    fn unary_operation(op: UnOp, n: i64) -> i64 {
        match op {
            Neg => -n,
            Not => !n,
        }
    }
}

/// Checked arithmetic that panics when overflowing or dividing by zero.
pub struct CheckedArithmetic;

impl Arithmetic for CheckedArithmetic {
    fn binary_operation(op: BinOp, n1: i64, n2: i64) -> i64 {
        let (result, overflowed) = match op {
            Add => n1.overflowing_add(n2),
            Sub => n1.overflowing_sub(n2),
            Mul => n1.overflowing_mul(n2),
            Div => n1.overflowing_div(n2),
            Rem => n1.overflowing_rem(n2),
            Lt => ((n1 < n2).into(), false),
            Lte => ((n1 <= n2).into(), false),
            Gt => ((n1 > n2).into(), false),
            Gte => ((n1 >= n2).into(), false),
            Eq => ((n1 == n2).into(), false),
            Neq => ((n1 != n2).into(), false),
            BitAnd | And => (n1 & n2, false),
            BitOr | Or => (n1 | n2, false),
            BitXor => (n1 ^ n2, false),
            Shr => n1.overflowing_shr(try_into_u32_or_panic(n2)),
            Shl => n1.overflowing_shl(try_into_u32_or_panic(n2)),
        };

        if overflowed {
            panic!(
                "Binary operation `{}` overflowed with operands `{}` and `{}`",
                op, n1, n2
            )
        }

        result
    }

    fn unary_operation(op: UnOp, n: i64) -> i64 {
        let (result, overflowed) = match op {
            Neg => n.overflowing_neg(),
            Not => (!n, false),
        };

        if overflowed {
            panic!("Unary operation `{}` overflowed with operand `{}`", op, n)
        }

        result
    }
}

fn try_into_u32_or_panic(n: i64) -> u32 {
    match u32::try_from(n) {
        Ok(n) => n,
        _ => panic!("Operand `{}` is negative", n),
    }
}
