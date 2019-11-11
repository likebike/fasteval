use crate::grammar::{ExpressionI, ValueI,
                     Expression,  Value};

use kerr::KErr;
use stacked::{SVec, SVec32};
use std::fmt;

pub struct Slab {
    exprs: SVec32<Expression>,
    vals:  SVec32<Value>,
}
impl Slab {
    pub fn new() -> Self {
        Self{
            exprs:SVec32::new(),
            vals: SVec32::new(),
        }
    }
    #[inline]
    pub fn push_expr(&self, expr:Expression) -> Result<ExpressionI,KErr> {
        self.exprs.push(expr).map(|i| ExpressionI(i))
    }
    #[inline]
    pub fn push_val(&self, val:Value) -> Result<ValueI,KErr> {
        self.vals.push(val).map(|i| ValueI(i))
    }
    #[inline]
    pub fn get_expr(&self, expr_i:ExpressionI) -> &Expression {
        &self.exprs[expr_i.0]
    }
    #[inline]
    pub fn get_val(&self, val_i:ValueI) -> &Value {
        &self.vals[val_i.0]
    }

    pub fn clear(&mut self) {
        self.exprs.clear();
        self.vals.clear();
    }
}
impl fmt::Debug for Slab {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        fn write_indexed_list<V,T>(f:&mut fmt::Formatter, lst:&V) -> Result<(), fmt::Error> where T:fmt::Debug, V:SVec<Item=T, Output=T> {
            write!(f, "{{")?;
            let mut nonempty = false;
            for (i,x) in lst.iter().enumerate() {
                if nonempty { write!(f, ",")?; }
                nonempty = true;
                write!(f, " {}:{:?}",i,x)?;
            }
            if nonempty { write!(f, " ")?; }
            write!(f, "}}")?;
            Ok(())
        }
        write!(f, "Slab{{ exprs:")?;
        write_indexed_list(f, &self.exprs)?;
        write!(f, ", vals:")?;
        write_indexed_list(f, &self.vals)?;
        write!(f, " }}")?;
        Ok(())
    }
}

