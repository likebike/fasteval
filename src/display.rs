use crate::grammar::Variable;

use std::fmt;

impl fmt::Display for Variable {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Variable(`{}`)",self.0.as_str().map_err(|_| fmt::Error)?)
    }
}

impl fmt::Debug for Variable {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(),fmt::Error> {
        <Self as fmt::Display>::fmt(&self, f)
    }
}

