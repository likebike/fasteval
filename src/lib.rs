// TODO:
//   [x] Port all tests
//   [x] NaN, inf, -inf are valid.  problem?  no because my parser thinks they're vars.
//   [x] e() pi() ... or should i prefer variables?  Provide a default layer of variables?  Vars don't work well with TV symbols.
//   [x] Profile, boost critical sections.
//   [x] optimize the peek/read process -- be able to read N bytes if we peek successfully.
//   [x] optimize after parse
//   [ ] Readme
//   [ ] Documentation
//
//   [ ] sprintf

#![feature(test)]
//#![feature(backtrace)]

pub mod slab;
pub mod parser;
pub mod compiler;
pub mod evaler;
pub mod evalns;
pub mod display;
pub mod ez;

pub use self::slab::Slab;
pub use self::parser::{Parser, Expression, ExpressionI, Value, ValueI, Variable};
pub use self::compiler::{Compiler, Instruction, InstructionI};
pub use self::evalns::EvalNS;
pub use self::evaler::Evaler;
pub use self::ez::ez_eval;

