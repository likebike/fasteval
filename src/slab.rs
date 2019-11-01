use crate::grammar::{ExpressionI, ValueI,
                     Expression,  Value};
use crate::error::Error;
use crate::stackvec::{StackVec, StackVec64};

pub struct Slab {
    expressions: StackVec64<Expression>,
    values:      StackVec64<Value>,
}
impl Slab {
    pub fn new() -> Self {
        Self{
            expressions:StackVec64::new(),
            values:     StackVec64::new(),
        }
    }
    pub fn push_expr(&self, expr:Expression) -> Result<ExpressionI,Error> {
        self.expressions.push(expr).map(|i| ExpressionI(i))
    }
    pub fn push_val(&self, val:Value) -> Result<ValueI,Error> {
        self.values.push(val).map(|i| ValueI(i))
    }
}

