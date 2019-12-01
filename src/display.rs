use crate::parser::VarName;

use std::fmt;

impl fmt::Display for VarName {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}",self.0)
    }
}

impl fmt::Debug for VarName {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(),fmt::Error> {
        write!(f, "VarName(`{}`)",self.0)
    }
}

