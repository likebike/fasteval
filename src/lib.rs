// TODO:
//   [x] Port all tests
//   [x] NaN, inf, -inf are valid.  problem?  no because my parser thinks they're vars.
//   [x] e() pi() ... or should i prefer variables?  Provide a default layer of variables?  Vars don't work well with TV symbols.
//   [ ] Profile, boost critical sections.
//   [ ] Readme
//   [ ] Documentation
//
//   [ ] sprintf
//   [ ] optimize after parse
//   [ ] optimize the peek/read process -- be able to read N bytes if we peek successfully.

#![feature(test)]

pub mod slab;
pub mod grammar;
pub mod parser;
pub mod evaler;
pub mod evalns;
pub mod display;
//pub mod defaults;

pub use self::grammar::{Expression, ExpressionI, Value, ValueI};
pub use self::slab::Slab;
pub use self::parser::Parser;
pub use self::evalns::EvalNS;
pub use self::evaler::Evaler;
