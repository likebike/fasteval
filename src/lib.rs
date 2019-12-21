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
//!   * sin(radians)    * asin(val)
//!   * cos(radians)    * acos(val)
//!   * tan(radians)    * atan(val)
//!   * sinh(val)       * asinh(val)
//!   * cosh(val)       * acosh(val)
//!   * tanh(val)       * atanh(val)
//! ```
//!
//! # Examples
//!
//! ## Easy evaluation of constant expressions
//! The `ez_eval()` function performs the entire allocation-parse-eval process
//! for you.  It is a little bit inefficient because it always allocates a
//! fresh Slab, but it is very simple to use:
//!
//! ```
//! fn main() -> Result<(), al::Error> {
//!     let val = al::ez_eval(
//!         "1+2*3/4^5%6 + log(100) + log(e(),100) + [3*(3-3)/3] + (2<3) && 1.23",    &mut al::EmptyNamespace)?;
//!     //    |            |          |   |          |               |   |
//!     //    |            |          |   |          |               |   boolean logic with ternary support
//!     //    |            |          |   |          |               comparisons
//!     //    |            |          |   |          square-brackets act like parenthesis
//!     //    |            |          |   built-in constants: e(), pi()
//!     //    |            |          'log' can take an optional first 'base' argument, defaults to 10
//!     //    |            many built-in functions: print, int, ceil, floor, abs, sign, log, round, min, max, sin, asin, ...
//!     //    standard binary operators
//!
//!     assert_eq!(val, 1.23);
//!
//!     Ok(())
//! }
//! ```
//!
//!
//! ## Simple variables
//! Several namespace types are supported, each designed for different
//! situations.  ([See the various Namespace types here.](evalns/index.html))  For simple cases, you can define variables with a `BTreeMap`:
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
//! This time, instead of using a map, we will use a namespace with a callback
//! function, which enables us to do advanced things, like define custom
//! functions and array-like objects:
//!
//! ```
//! fn main() -> Result<(), al::Error> {
//!     let mut ns = al::CachedFlatNamespace::new(|name:&str, args:Vec<f64>| -> Option<f64> {
//!         let mydata : [f64; 3] = [11.1, 22.2, 33.3];
//!         match name {
//!             "x" => Some(3.0),
//!             "y" => Some(4.0),
//!             "sum" => Some(args.into_iter().fold(0.0, |s,f| s+f)),
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
//! If we perform the parse and eval ourselves (without relying on the 'ez'
//! interface), then we can re-use the Slab allocation for subsequent parsing
//! and evaluations.  This avoids a significant amount of slow memory
//! operations:
//!
//! ```
//! use std::collections::BTreeMap;
//! use al::Evaler;  // import this trait so we can call eval().
//! fn main() -> Result<(), al::Error> {
//!     let mut slab = al::Slab::new();
//!
//!     let expr_ref = al::parse("x + 1", &mut slab.ps)?.from(&slab.ps);
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
//!     let expr_ref = al::parse("x * 10", &mut slab.ps)?.from(&slab.ps);
//!
//!     let val = expr_ref.eval(&slab, &mut map)?;
//!     assert_eq!(val, 25.0);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Compile to go super fast!
//! If you plan to evaluate an expression just one or two times, then you
//! should parse-eval as shown in previous examples.  But if you expect to
//! evaluate an expression three or more times, you can dramatically improve
//! your performance by compiling.  The compiled form is usually more than 10
//! times faster than the un-compiled form, and for constant expressions it is
//! usually more than 200 times faster.
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
//!     for deg in 0..360 {
//!         map.insert("deg".to_string(), deg as f64);
//!         // When working with compiled constant expressions, you can use the
//!         // eval_compiled*!() macros to save a function call:
//!         let val = al::eval_compiled!(compiled, &slab, &mut map);
//!         eprintln!("sin({}°) = {}", deg, val);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Unsafe Variables
//! If your variables *must* be as fast as possible and you are willing to be
//! very careful, you can build with the `unsafe-vars` feature (`cargo build
//! --features unsafe-vars`), which enables pointer-based variables.  These
//! unsafe variables perform 2x-4x faster than the compiled form above.  This
//! feature is not enabled by default because it slightly slows down other
//! non-variable operations.
//! ```
//! use al::Compiler;  // import this trait so we can call compile().
//! use al::Evaler;    // import this trait so we can call eval().
//! fn main() -> Result<(), al::Error> {
//!     let mut slab = al::Slab::new();
//!     let mut deg : f64 = 0.0;
//!     unsafe { slab.ps.add_unsafe_var("deg".to_string(), &deg); }  // Saves a pointer to 'deg'.
//!
//!     let mut ns = al::EmptyNamespace;  // We only define unsafe variables, not normal variables,
//!                                       // so EmptyNamespace is fine.
//!
//!     let expr_str = "sin(deg/360 * 2*pi())";
//!     let compiled = al::parse(expr_str, &mut slab.ps)?.from(&slab.ps).compile(&slab.ps, &mut slab.cs);
//!     for d in 0..360 {
//!         deg = d as f64;
//!         let val = al::eval_compiled!(compiled, &slab, &mut ns);
//!         eprintln!("sin({}°) = {}", deg, val);
//!     }
//!
//!     Ok(())
//! }
//! ```
//! 
//!
//! # Performance Benchmarks
//!
//! These benchmarks were performed on 2019-12-16.
//!
//! Here are links to all the libraries/tools included in these benchmarks:
//!     * [al (this library)](https://github.com/likebike/al)
//!     * [caldyn](https://github.com/Luthaf/caldyn)
//!     * [rsc](https://github.com/codemessiah/rsc)
//!     * [meval](https://github.com/rekka/meval-rs)
//!     * [calc](https://github.com/redox-os/calc/tree/master/src)
//!     * [tinyexpr (Rust)](https://github.com/kondrak/tinyexpr-rs)
//!     * [tinyexpr (C)](https://github.com/codeplea/tinyexpr)
//!     * [bc](https://www.gnu.org/software/bc/)
//!     * [python3](https://www.python.org/)
//!
//! ## Summary & Analysis
//!
//! ## Benchmark Descriptions
//! ```text
//!     * simple = `3 * 3 - 3 / 3`
//!       This is a simple test with primitive binary operators.
//!       Since the expression is quite simple, it does a good job of showing
//!       the intrinsic performance costs of a library.
//!       Results:
//!           * For compiled expressions, 'al' is 5x as fast as the closest
//!             competitor (caldyn) because the eval_compiled!() macro is able to
//!             eliminate all function calls.
//!           * For interpreted expressions, 'al' is 1.6x as fast as the
//!             tinyexpr C lib, and 2.3x as fast as the tinyexpr Rust lib.
//!             This is because 'al' eliminates redundant work and memory
//!             allocation during the parse phase.
//!
//!     * power = `2 ^ 3 ^ 4`
//!               `2 ^ (3 ^ 4)` for 'tinyexpr' and 'rsc'
//!       This test shows the associativity of the exponent operator.
//!       Most libraries (including 'al') use right-associativity,
//!       but some libraries (particularly tinyexpr and rsc) use
//!       left-associativity.
//!       This test is also interesting because it shows the precision of a
//!       library's number system.  'al' just uses f64 and therefore truncates
//!       the result (2417851639229258300000000), while python, bc, and the
//!       tinyexpr C library produce a higher precision result
//!       (2417851639229258349412352).
//!       Results:
//!           Same as the 'simple' case.
//!
//!     * variable = `x * 2`
//!       This is a simple test of variable support.
//!       Since the expression is quite simple, it shows the intrinsic
//!       performance costs of a library.
//!       Results:
//!           * The tinyexpr Rust library does not currently support variables.
//!           * For safe compiled evaluation, 'al' is 3x as fast as the closest
//!             competitor (caldyn).
//!           * For safe interpretation, 'al' is 2.4x as fast as the closest
//!             competitor (caldyn).
//!           * For unsafe operations, 'al' performance is similar to the
//!             tinyexpr C library.
//!
//!     * trig = `sin(x)`
//!       This is a test of variables, built-in function calls, and trigonometry.
//!       Results:
//!           * The tinyexpr Rust library does not currently support variables.
//!           * The 'calc' library does not support trigonometry.
//!           * For safe compiled evaluation, 'al' is 1.9x as fast as the
//!             closest competitor (caldyn).
//!           * For safe interpretation, 'al' is 1.6x as fast as the closest
//!             competitor (caldyn).
//!           * For unsafe operation, 'al' performance is similar to the
//!             tinyexpr C library.
//!
//!     * quadratic = `(-z + (z^2 - 4*x*y)^0.5) / (2*x)`
//!       This test demonstrates a more complex expression, involving several
//!       variables, some of which are accessed more than once.
//!       Results:
//!           * 
//!
//!     * big = `((((87))) - 73) + (97 + (((15 / 55 * ((31)) + 35))) + (15 - (9)) - (39 / 26) / 20 / 91 + 27 / (33 * 26 + 28 - (7) / 10 + 66 * 6) + 60 / 35 - ((29) - (69) / 44 / (92)) / (89) + 2 + 87 / 47 * ((2)) * 83 / 98 * 42 / (((67)) * ((97))) / (34 / 89 + 77) - 29 + 70 * (20)) + ((((((92))) + 23 * (98) / (95) + (((99) * (41))) + (5 + 41) + 10) - (36) / (6 + 80 * 52 + (90))))`
//! ```
//!
//! ## Charts
//! Note that the following charts use logarithmic scales.  Therefore, tiny
//! visual differences actually represent very significant performance
//! differences.
//!
//! Performance of evaluation of a compiled expression:
//! ![abc](http://hk.likebike.com/code/al/benches/al-compiled-20191214.png)
//!
//! Performance of one-time interpretation (parse and eval):
//! ![abc](http://hk.likebike.com/code/al/benches/al-interp-20191214.png)
//!
//! Performance of Unsafe Variables, compared to the tinyexpr C library (the
//! only other library in our test set that supports this mode):
//! ![abc](http://hk.likebike.com/code/al/benches/al-unsafe-20191214.png)
//!
//! ## Methodology
//! I am benchmarking on an Asus G55V (a 2012 laptop with Intel(R) Core(TM) i7-3610QM CPU @ 2.30GHz).
//!
//! I run 
//!
//! All numeric results can be found in `al/benches/bench.rs`.
//!
//! # How is `al` so fast?
//!
//! A variety of techniques are used to improve performance:
//!   * Minimization of memory allocations/deallocations;
//!     I just pre-allocate a large slab during initialization.
//!   * Elimination of redundant work, especially when parsing.
//!   * Compilation: Constant Folding and Expression Simplification.
//!     Boosts performance up to 1000x.
//!   * Profile-driven application of inlining.  Don't inline too much or too little.
//!   * Use of macros to eliminate call overhead for the most-frequently-used functions.
//!   * Don't panic.
//!   * Localize variables.  Use "--emit asm" as a guide.


#![feature(test)]
//#![warn(missing_docs)]

pub mod error;
#[macro_use]
pub mod slab;
pub mod parser;
#[macro_use]
pub mod compiler;
pub mod evaler;
pub mod evalns;
pub mod ez;

pub use self::error::Error;
pub use self::parser::{parse, Parser, Expression, ExpressionI, Value, ValueI};
pub use self::compiler::{Compiler, Instruction::{self, IConst}, InstructionI};
#[cfg(feature="unsafe-vars")]
pub use self::compiler::Instruction::IUnsafeVar;
pub use self::evaler::Evaler;
pub use self::slab::Slab;
pub use self::evalns::{EvalNamespace, Layered, EmptyNamespace, CachedFlatNamespace, CachedScopedNamespace, Bubble};
pub use self::ez::ez_eval;

