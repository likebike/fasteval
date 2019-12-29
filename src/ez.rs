//! An easy API for single-function-call expression evaluation.

use crate::error::Error;
use crate::parser::parse_noclear;
use crate::evaler::Evaler;
use crate::slab::Slab;
use crate::evalns::EvalNamespace;

/// The `ez_eval()` function provides a very simple way to perform expression evaluation with just one function call.
///
/// If you only need to evaluate an expression one time, then `ez_eval()` will
/// probably be perfectly adequate.  But if you plan to evaluate the same
/// expression many times, or if you plan to evaluate many expressions, you
/// are able to achieve better performance by allocating a single `Slab` and
/// using it to perform multiple parse-compile-eval cycles yourself.
///
/// # Errors
///
/// If there are any [`Error`](../error/enum.Error.html)s during the parse-eval process, they will be returned.
///
/// # Examples
///
/// [See the `fasteval` top-level documentation for examples.](../index.html#easy-evaluation)
pub fn ez_eval(expr_str:&str, ns:&mut impl EvalNamespace) -> Result<f64,Error> {
    let mut slab = Slab::new();  // A big block of memory, so we don't need to perform many tiny (and slow!) allocations.

    // Here is a one-liner that performs the entire parse-and-eval process:
    // parse(expr_str, &mut slab.ps)?.from(&slab.ps).eval(&mut slab, &mut ns)

    // Here is the same process, broken into steps:

    // First, parse the string:
    // We use the 'parse_noclear' function instead of 'parse' because we know the slab is empty.
    let expr_i = parse_noclear(expr_str, &mut slab.ps)?;

    // 'expr_i' is an index into the Slab.  You can extract the Expression object with either of these:
    //     slab.get_expr(expr_i)  ...OR...  expr_i.from(&slab.ps)
    // The first is more direct.  The second is a convenience built on top of the first.
    let expr_ref = slab.ps.get_expr(expr_i);

    // Use the reference to the Expression object to perform the evaluation:
    expr_ref.eval(&slab, ns)
}

