use crate::grammar::Variable;

use std::fmt;

impl fmt::Display for Variable {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}",self.0)
    }
}

impl fmt::Debug for Variable {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(),fmt::Error> {
        write!(f, "Variable(`{}`)",self.0)
    }
}

