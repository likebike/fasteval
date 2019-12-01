// TODO:
//   [x] Port all tests
//   [x] NaN, inf, -inf are valid.  problem?  no because my parser thinks they're vars.
//   [x] e() pi() ... or should i prefer variables?  Provide a default layer of variables?  Vars don't work well with TV symbols.
//   [x] Profile, boost critical sections.
//   [x] optimize the peek/read process -- be able to read N bytes if we peek successfully.
//   [x] optimize after parse
//   [ ] REPL Example with Variables
//   [ ] Copy smart tests from other libs.
//   [ ] Readme
//   [ ] Documentation
//
//   [ ] sprintf

#![feature(test)]
//#![feature(backtrace)]


#[macro_export]
macro_rules! eval_instr {
    ($evaler:expr, $slab_ref:expr, $ns_mut:expr) => {
        {
            let evaler = $evaler;
            if let IConst(c) = evaler {
                c
            } else {
                evaler.eval($slab_ref, $ns_mut)?
            }
        }
    }
}

#[macro_export]
macro_rules! eval_instr_ref {
    ($evaler:expr, $slab_ref:expr, $ns_mut:expr) => {
        {
            let evaler = $evaler;
            if let IConst(c) = evaler {
                *c
            } else {
                evaler.eval($slab_ref, $ns_mut)?
            }
        }
    }
}

pub mod parser;
pub mod compiler;
pub mod evaler;
pub mod slab;
pub mod evalns;
pub mod display;
pub mod ez;

pub use self::parser::{Parser, Expression, ExpressionI, Value, ValueI, Variable};
pub use self::compiler::{Compiler, Instruction::{self, IConst}, InstructionI};
pub use self::evaler::Evaler;
pub use self::slab::Slab;
pub use self::evalns::EvalNS;
pub use self::ez::ez_eval;

