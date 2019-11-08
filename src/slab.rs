use crate::grammar::{ExpressionI, ValueI,
                     Expression,  Value};

use kerr::KErr;
use stacked::{SVec, SVec64};

pub struct Slab {
    expressions: SVec64<Expression>,
    values:      SVec64<Value>,
}
impl Slab {
    pub fn new() -> Self {
        Self{
            expressions:SVec64::new(),
            values:     SVec64::new(),
        }
    }
    #[inline]
    pub fn push_expr(&self, expr:Expression) -> Result<ExpressionI,KErr> {
        self.expressions.push(expr).map(|i| ExpressionI(i))
    }
    #[inline]
    pub fn push_val(&self, val:Value) -> Result<ValueI,KErr> {
        self.values.push(val).map(|i| ValueI(i))
    }
    #[inline]
    pub fn get_expr(&self, expr_i:ExpressionI) -> &Expression {
        &self.expressions[expr_i.0]
    }
    #[inline]
    pub fn get_val(&self, val_i:ValueI) -> &Value {
        &self.values[val_i.0]
    }
}

