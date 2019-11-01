use crate::grammar::{Expression, ExprPair, Value::EConstant, Constant, BinaryOp::EPlus};

impl Default for Expression {
    fn default() -> Self {
        Expression{ first:EConstant(Constant(0.0)),
                    pairs:Box::new([]) }
    }
}

impl Default for ExprPair {
    fn default() -> Self {
        ExprPair(EPlus, EConstant(Constant(0.0)))
    }
}

