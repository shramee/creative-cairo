use std::fmt::Display;

#[cfg(test)]
#[path = "operand_test.rs"]
mod operand_test;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Register {
    AP,
    FP,
}
impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::AP => write!(f, "ap"),
            Register::FP => write!(f, "fp"),
        }
    }
}

// Represents the rhs operand of an assert equal instruction.
pub enum ResOperand {
    Deref(DerefOperand),
    Immediate(ImmediateOperand),
    BinOp(BinOpOperand),
}
impl Display for ResOperand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResOperand::Deref(operand) => write!(f, "{}", operand),
            ResOperand::Immediate(operand) => write!(f, "{}", operand),
            ResOperand::BinOp(operand) => write!(f, "{}", operand),
        }
    }
}

// Represents an operand of the form [reg + offset].
pub struct DerefOperand {
    pub register: Register,
    pub offset: i16,
}
impl Display for DerefOperand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} + {}]", self.register, self.offset)
    }
}

pub struct ImmediateOperand {
    // TODO(ilya, 10/10/2022): What type do we want to use here.
    pub value: i128,
}
impl Display for ImmediateOperand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub enum DerefOrImmediate {
    Deref(DerefOperand),
    Immediate(ImmediateOperand),
}
impl Display for DerefOrImmediate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DerefOrImmediate::Deref(operand) => write!(f, "{}", operand),
            DerefOrImmediate::Immediate(operand) => write!(f, "{}", operand),
        }
    }
}

pub enum Operation {
    Add,
    Mul,
}
impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Add => write!(f, "+"),
            Operation::Mul => write!(f, "*"),
        }
    }
}

pub struct BinOpOperand {
    pub op: Operation,
    pub a: DerefOperand,
    pub b: DerefOrImmediate,
}
impl Display for BinOpOperand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.a, self.op, self.b)
    }
}