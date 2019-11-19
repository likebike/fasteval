use crate::parser::Parser;
use crate::slab::Slab;
use crate::evalns::EvalNS;
use crate::evaler::Evaler;

use kerr::KErr;

pub fn ez_eval(expr_str:&str) -> Result<f64,KErr> {
    let parser = Parser::new(None,None);
    let mut slab = Slab::new();           // A big block of memory, so we don't need to perform many tiny (and slow!) allocations.
    let mut ns = EvalNS::new(|_| None);   // An evaluation namespace, with a default closure that doesn't define any variables.

    // Here is a one-liner that performs the entire parse-and-eval process:
    // parser.parse(&mut slab, expr_str)?.from(&slab).eval(&slab, &mut ns)

    // Here is the same process, broken into steps:

    // First, parse the string:
    let expr_i = parser.parse(&mut slab, expr_str)?;

    // 'expr_i' is an index into the Slab.  You can extract the Expression object with either of these:
    //     slab.get_expr(expr_i)  ...OR...  expr_i.from(&slab)
    // The first is more direct.  The second is a convenience built on top of the first.
    let expr_ref = slab.get_expr(expr_i);

    // Use the reference to the Expression object to perform the evaluation:
    expr_ref.eval(&slab, &mut ns)
}

