use crate::grammar::{ExpressionI, ValueI,
                     Expression,  Value};

use kerr::KErr;
use std::fmt;

impl ExpressionI {
    #[inline]
    pub fn from<'a>(self, slab:&'a Slab) -> &'a Expression {
        slab.get_expr(self)
    }
}
impl ValueI {
    #[inline]
    pub fn from<'a>(self, slab:&'a Slab) -> &'a Value {
        slab.get_val(self)
    }
}

pub struct Slab {
    exprs:Vec<Expression>,
    vals: Vec<Value>,
}
impl Slab {
    pub fn new() -> Self {
        Self{
            exprs:Vec::with_capacity(32),
            vals: Vec::with_capacity(32),
        }
    }
    #[inline]
    pub fn push_expr(&mut self, expr:Expression) -> Result<ExpressionI,KErr> {
        let i = self.exprs.len();
        if i>=self.exprs.capacity() { return Err(KErr::new("overflow")); }
        self.exprs.push(expr);
        Ok(ExpressionI(i))
    }
    #[inline]
    pub fn push_val(&mut self, val:Value) -> Result<ValueI,KErr> {
        let i = self.vals.len();
        if i>=self.vals.capacity() { return Err(KErr::new("overflow")); }
        self.vals.push(val);
        Ok(ValueI(i))
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
        fn write_indexed_list<T>(f:&mut fmt::Formatter, lst:&Vec<T>) -> Result<(), fmt::Error> where T:fmt::Debug {
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

