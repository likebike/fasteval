use al::{Parser, Compiler, Evaler, Slab, EvalNS};
use kerr::KErr;

use std::str::from_utf8;

fn evalns_cb(name:&str, args:Vec<f64>) -> Option<f64> {
    match name {
        "w" => Some(0.0),
        "x" => Some(1.0),
        "y" => Some(2.0),
        "y7" => Some(2.7),
        "z" => Some(3.0),
        "foo" => Some(args[0]*10.0),
        "bar" => Some(args[0]+args[1]),
        _ => None,
    }
}

fn chk_ok(expr_str:&str, expect_compile_str:&str, expect_slab_str:&str, expect_eval:f64) {
    let mut parser = Parser::new();
    let mut slab = Slab::new();
    let expr = parser.parse(&mut slab.ps, expr_str).unwrap().from(&slab.ps);
    let instr = expr.compile(&slab.ps, &mut slab.cs);

    assert_eq!(format!("{:?}",instr), expect_compile_str);
    assert_eq!(format!("{:?}",slab), expect_slab_str);

    let mut ns = EvalNS::new(evalns_cb);
    assert_eq!(instr.eval(&slab, &mut ns).unwrap(), expect_eval);
    
    // Make sure Instruction eval matches normal eval:
    assert_eq!(instr.eval(&slab, &mut ns).unwrap(), expr.eval(&slab, &mut ns).unwrap());
}

fn chk_perr(expr_str:&str, expect_err:&str) {
    let mut parser = Parser::new();
    let mut slab = Slab::new();
    let res = parser.parse(&mut slab.ps, expr_str);
    assert_eq!(res, Err(KErr::new(expect_err)));
}

fn chk_eerr(expr_str:&str, expect_err:&str) {
    let mut parser = Parser::new();
    let mut slab = Slab::new();
    let expr = parser.parse(&mut slab.ps, expr_str).unwrap().from(&slab.ps);
    let instr = expr.compile(&slab.ps, &mut slab.cs);
    let mut ns = EvalNS::new(evalns_cb);
    assert_eq!(instr.eval(&slab, &mut ns), Err(KErr::new(expect_err)));
}

#[test]
fn meval() {
    chk_perr("", "invalid value");
    chk_perr("(", "invalid value");
    chk_perr("0(", "unparsed tokens remaining");
    chk_eerr("e", "variable undefined: e");
    chk_perr("1E", "parse<f64> error");
    chk_perr("1e+", "parse<f64> error");
    chk_perr("()", "invalid value");
    chk_perr("2)", "unparsed tokens remaining");
    chk_perr("2^", "invalid value");
    chk_perr("(((2)", "EOF");
    chk_perr("f(2,)", "invalid value");
    chk_perr("f(,2)", "invalid value");

    chk_ok("round(sin (pi()) * cos(0))",
"IConst(0.0)",
"Slab{ exprs:{ 0:Expression { first: ECallable(EStdFunc(EFuncPi)), pairs: [] }, 1:Expression { first: EConstant(Constant(0.0)), pairs: [] }, 2:Expression { first: ECallable(EStdFunc(EFuncSin(ExpressionI(0)))), pairs: [ExprPair(EMul, ECallable(EStdFunc(EFuncCos(ExpressionI(1)))))] }, 3:Expression { first: ECallable(EStdFunc(EFuncRound { modulus: None, expr: ExpressionI(2) })), pairs: [] } }, vals:{}, instrs:{} }",
0.0);

    chk_ok("max(1.)",
"IConst(1.0)",
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(1.0)), pairs: [] }, 1:Expression { first: ECallable(EStdFunc(EFuncMax { first: ExpressionI(0), rest: [] })), pairs: [] } }, vals:{}, instrs:{} }",
1.0);

    chk_ok("max(1., 2., -1)",
"IConst(2.0)",
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(1.0)), pairs: [] }, 1:Expression { first: EConstant(Constant(2.0)), pairs: [] }, 2:Expression { first: EConstant(Constant(-1.0)), pairs: [] }, 3:Expression { first: ECallable(EStdFunc(EFuncMax { first: ExpressionI(0), rest: [ExpressionI(1), ExpressionI(2)] })), pairs: [] } }, vals:{}, instrs:{} }",
2.0);

    chk_ok("sin(1.) + cos(2.)",
"IConst(0.4253241482607541)",
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(1.0)), pairs: [] }, 1:Expression { first: EConstant(Constant(2.0)), pairs: [] }, 2:Expression { first: ECallable(EStdFunc(EFuncSin(ExpressionI(0)))), pairs: [ExprPair(EAdd, ECallable(EStdFunc(EFuncCos(ExpressionI(1)))))] } }, vals:{}, instrs:{} }",
(1f64).sin() + (2f64).cos());





}

#[test]
fn overflow_stack() {
    chk_perr(from_utf8(&[b'('; 1]).unwrap(), "invalid value");
    chk_perr(from_utf8(&[b'('; 2]).unwrap(), "invalid value");
    chk_perr(from_utf8(&[b'('; 4]).unwrap(), "invalid value");
    chk_perr(from_utf8(&[b'('; 8]).unwrap(), "invalid value");
    chk_perr(from_utf8(&[b'('; 16]).unwrap(), "invalid value");
    chk_perr(from_utf8(&[b'('; 32]).unwrap(), "too deep");
    chk_perr(from_utf8(&[b'('; 64]).unwrap(), "too deep");
    chk_perr(from_utf8(&[b'('; 128]).unwrap(), "too deep");
    chk_perr(from_utf8(&[b'('; 256]).unwrap(), "too deep");
    chk_perr(from_utf8(&[b'('; 512]).unwrap(), "too deep");
    chk_perr(from_utf8(&[b'('; 1024]).unwrap(), "too deep");
    chk_perr(from_utf8(&[b'('; 2048]).unwrap(), "too deep");
    chk_perr(from_utf8(&[b'('; 4096]).unwrap(), "too deep");
    chk_perr(from_utf8(&[b'('; 8192]).unwrap(), "expression string is too long");
}

