use crate::parser::Parser;
use crate::slab::Slab;
use crate::evalns::EvalNS;
use crate::evaler::Evaler;

use kerr::KErr;

use std::collections::HashMap;

pub fn ez_eval(expr_str:&str, vars:&HashMap<String,f64>) -> Result<f64,KErr> {
    let mut parser = Parser::new();
    let mut slab = Slab::new();           // A big block of memory, so we don't need to perform many tiny (and slow!) allocations.
    let mut ns = EvalNS::new(|name, _args| vars.get(name).map(|f| *f));   // An evaluation namespace, with a default closure that reads variables from the given HashMap.

    // Here is a one-liner that performs the entire parse-and-eval process:
    // parser.parse(&mut slab.ps, expr_str)?.from(&slab).eval(&slab, &mut ns)

    // Here is the same process, broken into steps:

    // First, parse the string:
    let expr_i = parser.parse(&mut slab.ps, expr_str)?;

    // 'expr_i' is an index into the Slab.  You can extract the Expression object with either of these:
    //     slab.get_expr(expr_i)  ...OR...  expr_i.from(&slab)
    // The first is more direct.  The second is a convenience built on top of the first.
    let expr_ref = slab.ps.get_expr(expr_i);

    // Use the reference to the Expression object to perform the evaluation:
    expr_ref.eval(&slab, &mut ns)
}

