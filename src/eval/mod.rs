use std::convert::TryFrom;
use std::io::Write;

use crate::ast::{BinOp, UnOp};

mod machine;
mod lang_env;

pub use machine::Machine;
pub use lang_env::LangEnv;

pub struct OverflowMachine<W: Write> {
    env: LangEnv<W>,
}

impl<W: Write> Machine<W> for OverflowMachine<W> {
    fn with_env(env: LangEnv<W>) -> Self {
        OverflowMachine { env }
    }

    fn lang_env(&mut self) -> &mut LangEnv<W> {
        &mut self.env
    }
}

pub struct CheckedMachine<W: Write> {
    env: LangEnv<W>,
}

impl<W: Write> Machine<W> for CheckedMachine<W> {
    fn with_env(env: LangEnv<W>) -> Self {
        CheckedMachine { env }
    }

    fn lang_env(&mut self) -> &mut LangEnv<W> {
        &mut self.env
    }

    fn native_bin_op(op: BinOp, n1: i64, n2: i64) -> i64 {
        use BinOp::*;

        let result = match op {
            Add => n1.checked_add(n2),
            Sub => n1.checked_sub(n2),
            Mul => n1.checked_mul(n2),
            Div => n1.checked_div(n2),
            Rem => n1.checked_rem(n2),
            Lt => Some((n1 < n2).into()),
            Lte => Some((n1 <= n2).into()),
            Gt => Some((n1 > n2).into()),
            Gte => Some((n1 >= n2).into()),
            Eq => Some((n1 == n2).into()),
            Neq => Some((n1 != n2).into()),
            BitAnd | And => Some(n1 & n2),
            BitOr | Or => Some(n1 | n2),
            BitXor => Some(n1 ^ n2),
            Shr => u32::try_from(n2).ok().and_then(|n2| n1.checked_shr(n2)),
            Shl => u32::try_from(n2).ok().and_then(|n2| n1.checked_shl(n2)),
        };

        if let Some(result) = result {
            result
        } else {
            panic!(
                "Binary operation `{}` overflowed with operands `{}` and `{}`",
                op, n1, n2
            )
        }
    }

    fn native_un_op(op: UnOp, n: i64) -> i64 {
        use UnOp::*;

        let result = match op {
            Neg => n.checked_neg(),
            Not => Some(!n),
        };

        if let Some(result) = result {
            result
        } else {
            panic!("Unary operation `{}` overflowed with operand `{}`", op, n)
        }
    }
}
