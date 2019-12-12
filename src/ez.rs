use crate::error::Error;
use crate::parser::parse;
use crate::evaler::Evaler;
use crate::slab::Slab;
use crate::evalns::EvalNamespace;

pub fn ez_eval(expr_str:&str, ns:&mut impl EvalNamespace) -> Result<f64,Error> {
    let mut slab = Slab::new();   // A big block of memory, so we don't need to perform many tiny (and slow!) allocations.

    // Here is a one-liner that performs the entire parse-and-eval process:
    // parse(&mut slab.ps, expr_str)?.from(&slab).eval(&slab, &mut ns)

    // Here is the same process, broken into steps:

    // First, parse the string:
    let expr_i = parse(&mut slab.ps, expr_str)?;

    // 'expr_i' is an index into the Slab.  You can extract the Expression object with either of these:
    //     slab.get_expr(expr_i)  ...OR...  expr_i.from(&slab)
    // The first is more direct.  The second is a convenience built on top of the first.
    let expr_ref = slab.ps.get_expr(expr_i);

    // Use the reference to the Expression object to perform the evaluation.
    // You can use either of these forms:
    //     expr_ref.eval(&slab, &mut ns)  ...OR...  ns.eval(&slab, expr_ref)
    // The first is more direct.  The second is built on top of the first, and adds some extra error info.
    expr_ref.eval(&slab,ns)
}

