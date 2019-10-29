// TODO:
//   [x] Port all tests
//   [x] NaN, inf, -inf are valid.  problem?  no because my parser thinks they're vars.
//   [ ] Profile, boost critical sections.
//   [ ] Readme
//   [ ] Documentation
//
//   [ ] sprintf
//   [ ] optimize after parse
//   [ ] optimize the peek/read process -- be able to read N bytes if we peek successfully.
//   [ ] e() pi() ... or should i prefer variables?  Provide a default layer of variables?  Vars don't work well with TV symbols.

pub mod error;
pub mod grammar;
pub mod parser;
pub mod evaler;
pub mod evalns;
pub mod display;

#[cfg(test)]
mod experiments;

