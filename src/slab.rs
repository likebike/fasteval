use crate::parser::{ExpressionI, ValueI,
                    Expression,  Value};
use crate::compiler::{Instruction::{self, IConst}, InstructionI};

use kerr::KErr;
use std::fmt;
use std::mem;

impl ExpressionI {
    #[inline]
    pub fn from(self, ps:&ParseSlab) -> &Expression {
        ps.get_expr(self)
    }
}
impl ValueI {
    #[inline]
    pub fn from(self, ps:&ParseSlab) -> &Value {
        ps.get_val(self)
    }
}

pub struct Slab {
    pub ps: ParseSlab,
    pub cs: CompileSlab,
}
pub struct ParseSlab {
    exprs:Vec<Expression>,
    vals: Vec<Value>,
}
pub struct CompileSlab {
    instrs:Vec<Instruction>,
}
impl ParseSlab {
    #[inline]
    pub fn get_expr(&self, expr_i:ExpressionI) -> &Expression {
        &self.exprs[expr_i.0]
    }
    #[inline]
    pub fn get_val(&self, val_i:ValueI) -> &Value {
        &self.vals[val_i.0]
    }
    #[inline]
    pub fn push_expr(&mut self, expr:Expression) -> Result<ExpressionI,KErr> {
        let i = self.exprs.len();
        if i>=self.exprs.capacity() { return Err(KErr::new("slab expr overflow")); }
        self.exprs.push(expr);
        Ok(ExpressionI(i))
    }
    #[inline]
    pub fn push_val(&mut self, val:Value) -> Result<ValueI,KErr> {
        let i = self.vals.len();
        if i>=self.vals.capacity() { return Err(KErr::new("slab val overflow")); }
        self.vals.push(val);
        Ok(ValueI(i))
    }
}
impl CompileSlab {
    #[inline]
    pub fn get_instr(&self, i:InstructionI) -> &Instruction {
        &self.instrs[i.0]
    }
    #[inline]
    pub fn push_instr(&mut self, instr:Instruction) -> InstructionI {
        if self.instrs.capacity()==0 { self.instrs.reserve(32); }
        let i = self.instrs.len();
        self.instrs.push(instr);
        InstructionI(i)
    }
    #[inline]
    pub fn take_instr(&mut self, i:InstructionI) -> Instruction {
        if i.0==self.instrs.len()-1 {
            self.instrs.pop().unwrap()
        } else {
            mem::replace(&mut self.instrs[i.0], IConst(std::f64::NAN))  // Conspicuous Value
        }
    }
}
impl Slab {
    #[inline]
    pub fn new() -> Self { Self::with_capacity(64) }
    #[inline]
    pub fn with_capacity(cap:usize) -> Self {
        Self{
            ps:ParseSlab{
                exprs:Vec::with_capacity(cap),
                vals: Vec::with_capacity(cap),
            },
            cs:CompileSlab{instrs:Vec::new()},  // Don't pre-allocation for compilation.
        }
    }

    pub fn clear(&mut self) {
        self.ps.exprs.clear();
        self.ps.vals.clear();
        self.cs.instrs.clear();
    }
}


fn write_indexed_list<T>(f:&mut fmt::Formatter, lst:&[T]) -> Result<(), fmt::Error> where T:fmt::Debug {
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
impl fmt::Debug for Slab {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Slab{{ exprs:")?;
        write_indexed_list(f, &self.ps.exprs)?;
        write!(f, ", vals:")?;
        write_indexed_list(f, &self.ps.vals)?;
        write!(f, ", instrs:")?;
        write_indexed_list(f, &self.cs.instrs)?;
        write!(f, " }}")?;
        Ok(())
    }
}
impl fmt::Debug for ParseSlab {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "ParseSlab{{ exprs:")?;
        write_indexed_list(f, &self.exprs)?;
        write!(f, ", vals:")?;
        write_indexed_list(f, &self.vals)?;
        write!(f, " }}")?;
        Ok(())
    }
}
impl fmt::Debug for CompileSlab {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "CompileSlab{{ instrs:")?;
        write_indexed_list(f, &self.instrs)?;
        write!(f, " }}")?;
        Ok(())
    }
}

impl Default for Slab {
    fn default() -> Self { Self::new() }
}

