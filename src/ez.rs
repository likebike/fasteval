use crate::parser::Parser;
use crate::slab::Slab;
use crate::evalns::EvalNS;
use crate::evaler::Evaler;

use kerr::KErr;

pub struct EZ<'a> {
    pub parser:Parser<'a>,
    pub slab:  Slab,
    pub ns:    EvalNS<'a>,
}

impl EZ<'_> {
    #[inline]
    pub fn new() -> Self {
        Self {
            parser:Parser::new(None,None),
            slab:  Slab::new(),
            ns:    EvalNS::new(|_| None),
        }
    }

    #[inline]
    pub fn eval(&mut self, s:&str) -> Result<f64,KErr> {
        self.parser.parse(&mut self.slab, s)?.get(&self.slab).eval(&self.slab, &mut self.ns)
    }
}

