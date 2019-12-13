//! A fast algebraic expression evaluation library.
//!
//! # Built-in Functions and Constants
//!
//! ```text
//!   * print(...strings and values...) -- Prints to stderr.  Very useful to 'probe' an expression.
//!                                        Evaluates to the last value.
//!                                        Example: `print("x is", x, "and y is", y)`
//!                                        Example: `x + print("y:",y) + z == x+y+z`
//!
//!   * log(base=10, val) -- Logarithm with optional 'base' as first argument.
//!                          Example: `log(100) + log(e(),100)`
//!
//!   * e()  -- Euler's number (2.718281828459045)
//!   * pi() -- π (3.141592653589793)
//!
//!   * int(val)
//!   * ceil(val)
//!   * floor(val)
//!   * round(modulus=1, val) -- Round with optional 'modulus' as first argument.
//!                              Example: `round(1.23456) == 1  &&  round(0.001, 1.23456) == 1.235`
//!
//!   * abs(val)
//!   * sign(val)
//!
//!   * min(val, ...) -- Example: `min(1,-2,3,-4) == -4`
//!   * max(val, ...) -- Example: `max(1,-2,3,-4) == 3`
//!
//!   * sin(radians)
//!   * cos(radians)
//!   * tan(radians)
//!   * asin(val)
//!   * acos(val)
//!   * atan(val)
//!   * sinh(val)
//!   * cosh(val)
//!   * tanh(val)
//!   * asinh(val)
//!   * acosh(val)
//!   * atanh(val)
//! ```
//!
//! # Examples
//!
//! ## Easy evaluation of constant expressions
//! The `ez_eval()` function performs the entire allocation-parse-eval process for you.  It is a little bit inefficient because it always allocates a fresh Slab, but it is very simple to use:
//!
//! ```
//! fn main() -> Result<(), al::Error> {
//!     let val = al::ez_eval(
//!         "1+2*3/4^5%6 + log(100) + log(e(),100) + [3*(3-3)/3] + (2<3) && 1.23",    &mut al::EmptyNamespace)?;
//!     //   |             |          |   |          |               |   |
//!     //   |             |          |   |          |               |   boolean logic with ternary support
//!     //   |             |          |   |          |               comparisons
//!     //   |             |          |   |          square-brackets act like parenthesis
//!     //   |             |          |   builtin constants: e(), pi()
//!     //   |             |          'log' can take an optional first 'base' argument, defaults to 10
//!     //   |             many builtin functions: print, int, ceil, floor, abs, sign, log, round, min, max, sin, asin, ...
//!     //   standard binary operators
//!
//!     assert_eq!(val, 1.23);
//!
//!     Ok(())
//! }
//! ```
//!
//!
//! ## Simple variables
//! Several namespace types are supported, each designed for different situations.  For simple cases, you can define variables with a `BTreeMap`:
//!
//! ```
//! use std::collections::BTreeMap;
//! fn main() -> Result<(), al::Error> {
//!     let mut map : BTreeMap<String,f64> = BTreeMap::new();
//!     map.insert("x".to_string(), 1.0);
//!     map.insert("y".to_string(), 2.0);
//!     map.insert("z".to_string(), 3.0);
//!
//!     let val = al::ez_eval(r#"x + print("y:",y) + z"#,    &mut map)?;
//!     //                           |
//!     //                           prints "y: 2" to stderr and then evaluates to 2.0
//!
//!     assert_eq!(val, 6.0);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Advanced variables and custom functions
//! This time, instead of using a map, we will use a namespace with a callback function, which enables us to do advanced things, like define custom functions and array-like objects:
//!
//! ```
//! fn main() -> Result<(), al::Error> {
//!     let mut ns = al::FlatNamespace::new(|name:&str, args:Vec<f64>| -> Option<f64> {
//!         let mydata : [f64; 3] = [11.1, 22.2, 33.3];
//!         match name {
//!             "x" => Some(3.0),
//!             "y" => Some(4.0),
//!             "sum" => {
//!                 Some(args.into_iter().fold(0.0, |s,f| s+f))
//!             }
//!             "data" => args.get(0).and_then(|f| mydata.get(*f as usize).copied()),
//!             _ => None,
//!         }
//!     });
//!
//!     let val = al::ez_eval("sum(x^2, y^2)^0.5 + data[0]",    &mut ns)?;
//!     //                     |   |               |
//!     //                     |   |               square-brackets act like parenthesis
//!     //                     |   variables are like custom functions with zero args
//!     //                     custom function
//!
//!     assert_eq!(val, 16.1);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Re-use the Slab to go faster
//! If we perform the parse and eval ourselves (without relying on the 'ez' interface), then we can re-use the Slab allocation for subsequent parsing and evaluations:
//!
//! ```
//! use std::collections::BTreeMap;
//! use al::Evaler;  // import this trait so we can call eval().
//! fn main() -> Result<(), al::Error> {
//!     let mut slab = al::Slab::new();
//!
//!     // Parse an expression string:
//!
//!     let expr_str = "x + 1";
//!     let expr_ref = al::parse(expr_str, &mut slab.ps)?.from(&slab.ps);
//!
//!     // Let's evaluate the expression a couple times with different 'x' values:
//!
//!     let mut map : BTreeMap<String,f64> = BTreeMap::new();
//!     map.insert("x".to_string(), 1.0);
//!     let val = expr_ref.eval(&slab, &mut map)?;
//!     assert_eq!(val, 2.0);
//!
//!     map.insert("x".to_string(), 2.5);
//!     let val = expr_ref.eval(&slab, &mut map)?;
//!     assert_eq!(val, 3.5);
//!
//!     // Now, let's re-use the Slab for a new expression.
//!     // (This is much cheaper than allocating a new Slab.)
//!
//!     slab.clear();
//!     let expr_str = "x * 10";
//!     let expr_ref = al::parse(expr_str, &mut slab.ps)?.from(&slab.ps);
//!
//!     let val = expr_ref.eval(&slab, &mut map)?;
//!     assert_eq!(val, 25.0);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Compile to go super fast!
//! If you plan to evaluate an expression just one or two times, then you should parse-eval as shown in previous examples.  But if you expect to evaluate an expression three or more times, you can dramatically improve your performance by compiling.  The compiled form is usually more than 10 times faster than the un-compiled form, and for constant expressions it is usually more than 200 times faster.
//! ```
//! use std::collections::BTreeMap;
//! use al::Compiler;  // import this trait so we can call compile().
//! use al::Evaler;    // import this trait so we can call eval().
//! fn main() -> Result<(), al::Error> {
//!     let mut slab = al::Slab::new();
//!     let mut map = BTreeMap::new();
//!
//!     let expr_str = "sin(deg/360 * 2*pi())";
//!     let compiled = al::parse(expr_str, &mut slab.ps)?.from(&slab.ps).compile(&slab.ps, &mut slab.cs);
//!     eprintln!("slab: {:?}",slab);
//!     //panic!("halt");
//!     for deg in 0..360 {
//!         map.insert("deg".to_string(), deg as f64);
//!         let val = compiled.eval(&slab, &mut map)?;
//!         eprintln!("{} : {}", deg, val);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! # How is `al` so fast?
//!
//! A variety of techniques are used to improve performance:
//!   * Elimination of redundant work, especially when parsing.
//!   * Minimization of memory allocations/deallocations; I just pre-allocate a large slab during initialization.
//!   * Constant Folding.  Boosts performance of constant expressions 1000x.
//!   * Profile-driven application of inlining.
//!   * Use of macros to eliminate call overhead for the most-frequently-used functions.


//#![warn(missing_docs)]

// TODO: These should be placed in 'evaler.rs':
#[macro_export]
macro_rules! eval_instr {
    ($evaler:ident, $slab_ref:expr, $ns_mut:expr) => {
        if let al::IConst(c) = $evaler {
            c
        } else {
            #[cfg(feature="unsafe-vars")]
            {
                if let al::IUnsafeVar{ptr, ..} = $evaler {
                    unsafe { *ptr }
                } else {
                    $evaler.eval($slab_ref, $ns_mut)?
                }
            }

            #[cfg(not(feature="unsafe-vars"))]
            $evaler.eval($slab_ref, $ns_mut)?
        }
    };
    ($evaler:expr, $slab_ref:expr, $ns_mut:expr) => {
        {
            let evaler = $evaler;
            eval_instr!(evaler, $slab_ref, $ns_mut)
        }
    };
}

#[macro_export]
macro_rules! eval_instr_ref {
    ($evaler:ident, $slab_ref:expr, $ns_mut:expr) => {
        if let al::IConst(c) = $evaler {
            *c
        } else {
            #[cfg(feature="unsafe-vars")]
            {
                if let al::IUnsafeVar{ptr, ..} = $evaler {
                    unsafe { **ptr }
                } else {
                    $evaler.eval($slab_ref, $ns_mut)?
                }
            }

            #[cfg(not(feature="unsafe-vars"))]
            $evaler.eval($slab_ref, $ns_mut)?
        }
    };
    ($evaler:expr, $slab_ref:expr, $ns_mut:expr) => {
        {
            let evaler = $evaler;
            eval_instr_ref!(evaler, $slab_ref, $ns_mut)
        }
    };
}

#[macro_export]
macro_rules! eval_instr_ref_or_panic {
    ($evaler:ident, $slab_ref:expr, $ns_mut:expr) => {
        if let al::IConst(c) = $evaler {
            *c
        } else {
            #[cfg(feature="unsafe-vars")]
            {
                if let al::IUnsafeVar{ptr, ..} = $evaler {
                    unsafe { **ptr }
                } else {
                    $evaler.eval($slab_ref, $ns_mut).unwrap()
                }
            }

            #[cfg(not(feature="unsafe-vars"))]
            $evaler.eval($slab_ref, $ns_mut).unwrap()
        }
    };
    ($evaler:expr, $slab_ref:expr, $ns_mut:expr) => {
        {
            let evaler = $evaler;
            eval_instr_ref_or_panic!(evaler, $slab_ref, $ns_mut)
        }
    };
}

pub mod error;
pub mod parser;
#[macro_use]
pub mod compiler;
pub mod evaler;
pub mod slab;
pub mod evalns;
pub mod ez;

pub use self::error::Error;
pub use self::parser::{parse, Parser, Expression, ExpressionI, Value, ValueI};
pub use self::compiler::{Compiler, Instruction::{self, IConst}, InstructionI};
#[cfg(feature="unsafe-vars")]
pub use self::compiler::Instruction::IUnsafeVar;
pub use self::evaler::Evaler;
pub use self::slab::Slab;
pub use self::evalns::{EvalNamespace, EmptyNamespace, FlatNamespace, ScopedNamespace};
pub use self::ez::ez_eval;

