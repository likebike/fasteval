// TODO:
//     * Readme
//     * Documentation
//     * Profile, boost critical sections.
//
//     * optimize after parse
//     * optimize the peek/read process -- be able to read N bytes if we peek successfully.
//     * sprintf
//     * e() pi() ... or should i prefer variables?  Provide a default layer of variables?  Vars don't work well with TV symbols.

mod error;
pub mod grammar;
pub mod parser;
mod evaler;
pub mod evalns;
pub mod display;

#[cfg(test)]
mod experiments;

