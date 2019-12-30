// This is a battery of unit tests taken directly from my original Go project.
// I know the test names suck, but I'm not going to change them because I want line-for-line compatibility with the Go tests.


use fasteval::{Evaler, ExpressionI, parse, Error, Slab, EmptyNamespace, CachedCallbackNamespace};

use std::collections::BTreeMap;
use std::collections::BTreeSet;

fn parse_raw<'a>(s:&str, slab:&'a mut Slab) -> Result<ExpressionI,Error> {
    parse(s, &mut slab.ps)
}
fn ok_parse<'a>(s:&str, slab:&'a mut Slab) -> ExpressionI { parse_raw(s,slab).unwrap() }

fn do_eval(s:&str) -> f64 {
    let mut slab = Slab::new();
    let mut ns = EmptyNamespace;
    ok_parse(s, &mut slab).from(&slab.ps).eval(&slab, &mut ns).unwrap()
}

//// TODO:
// fn capture_stderr(f:&dyn Fn()) -> String {
//     f();
//     "".to_string()
// }

#[test]
fn aaa_test_a() {
    let mut slab = Slab::new();

    ok_parse("3", &mut slab);
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(3.0), pairs: [] } }, vals:{}, instrs:{} }");
    ok_parse("3.14", &mut slab);
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(3.14), pairs: [] } }, vals:{}, instrs:{} }");
    ok_parse("3+5", &mut slab);
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(3.0), pairs: [ExprPair(EAdd, EConstant(5.0))] } }, vals:{}, instrs:{} }");
    ok_parse("3-5", &mut slab);
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(3.0), pairs: [ExprPair(ESub, EConstant(5.0))] } }, vals:{}, instrs:{} }");
    ok_parse("3*5", &mut slab);
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(3.0), pairs: [ExprPair(EMul, EConstant(5.0))] } }, vals:{}, instrs:{} }");
    ok_parse("3/5", &mut slab);
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(3.0), pairs: [ExprPair(EDiv, EConstant(5.0))] } }, vals:{}, instrs:{} }");
    ok_parse("3^5", &mut slab);
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(3.0), pairs: [ExprPair(EExp, EConstant(5.0))] } }, vals:{}, instrs:{} }");
}

#[test]
fn aaa_test_b0() {
    let mut slab = Slab::new();

    ok_parse("3.14 + 4.99999999999999", &mut slab);
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(3.14), pairs: [ExprPair(EAdd, EConstant(4.99999999999999))] } }, vals:{}, instrs:{} }");
    ok_parse("3.14 + 4.99999999999999999999999999999999999999999999999999999", &mut slab);
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(3.14), pairs: [ExprPair(EAdd, EConstant(5.0))] } }, vals:{}, instrs:{} }");
    // Go can parse this, but not Rust:
    assert_eq!(parse_raw("3.14 + 4.999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999", &mut slab),
Err(Error::ParseF64("4.999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999".to_string())));
    ok_parse("3.14 + 0.9999", &mut slab);
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(3.14), pairs: [ExprPair(EAdd, EConstant(0.9999))] } }, vals:{}, instrs:{} }");
    ok_parse("3.14 + .9999", &mut slab);
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(3.14), pairs: [ExprPair(EAdd, EConstant(0.9999))] } }, vals:{}, instrs:{} }");
    ok_parse("3.14 + 0.", &mut slab);
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(3.14), pairs: [ExprPair(EAdd, EConstant(0.0))] } }, vals:{}, instrs:{} }");
}

#[test]
fn aaa_test_b1() {
    let mut slab = Slab::new();

    assert_eq!(parse_raw("3.14 + 4.99999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999.9999", &mut slab),
Err(Error::ParseF64("4.99999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999.9999".to_string())));
    assert_eq!(parse_raw("3.14 + 4.9999.9999", &mut slab),
Err(Error::ParseF64("4.9999.9999".to_string())));
}

#[test]
fn aaa_test_b2() {
    let mut slab = Slab::new();

    assert_eq!(parse_raw("3.14 + .", &mut slab),
Err(Error::ParseF64(".".to_string())));
}

#[test]
fn aaa_test_c0() {
    let mut slab = Slab::new();

    ok_parse("3+5-xyz", &mut slab);
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(3.0), pairs: [ExprPair(EAdd, EConstant(5.0)), ExprPair(ESub, EStdFunc(EVar(\"xyz\")))] } }, vals:{}, instrs:{} }");
    ok_parse("3+5-xyz_abc_def123", &mut slab);
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(3.0), pairs: [ExprPair(EAdd, EConstant(5.0)), ExprPair(ESub, EStdFunc(EVar(\"xyz_abc_def123\")))] } }, vals:{}, instrs:{} }");
    ok_parse("3+5-XYZ_abc_def123", &mut slab);
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(3.0), pairs: [ExprPair(EAdd, EConstant(5.0)), ExprPair(ESub, EStdFunc(EVar(\"XYZ_abc_def123\")))] } }, vals:{}, instrs:{} }");
    ok_parse("3+5-XYZ_ab*c_def123", &mut slab);
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(3.0), pairs: [ExprPair(EAdd, EConstant(5.0)), ExprPair(ESub, EStdFunc(EVar(\"XYZ_ab\"))), ExprPair(EMul, EStdFunc(EVar(\"c_def123\")))] } }, vals:{}, instrs:{} }");
}

#[test]
fn aaa_test_c1() {
    let mut slab = Slab::new();

    assert_eq!(parse_raw("3+5-XYZ_ab~c_def123", &mut slab),
Err(Error::UnparsedTokensRemaining("~c_def123".to_string())));
}

#[test]
fn aaa_test_d0() {
    let mut slab = Slab::new();

    ok_parse("3+(-5)", &mut slab);
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(-5.0), pairs: [] }, 1:Expression { first: EConstant(3.0), pairs: [ExprPair(EAdd, EUnaryOp(EParentheses(ExpressionI(0))))] } }, vals:{}, instrs:{} }");
    ok_parse("3+-5", &mut slab);
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(3.0), pairs: [ExprPair(EAdd, EConstant(-5.0))] } }, vals:{}, instrs:{} }");
    ok_parse("3++5", &mut slab);
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(3.0), pairs: [ExprPair(EAdd, EConstant(5.0))] } }, vals:{}, instrs:{} }");
    ok_parse(" 3 + ( -x + y ) ", &mut slab);
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EUnaryOp(ENeg(ValueI(0))), pairs: [ExprPair(EAdd, EStdFunc(EVar(\"y\")))] }, 1:Expression { first: EConstant(3.0), pairs: [ExprPair(EAdd, EUnaryOp(EParentheses(ExpressionI(0))))] } }, vals:{ 0:EStdFunc(EVar(\"x\")) }, instrs:{} }");
}

#[test]
fn aaa_test_d1() {
    let mut slab = Slab::new();

    assert_eq!(parse_raw(" 3 + ( -x + y  ", &mut slab),
Err(Error::EofWhileParsing("parentheses".to_string())));
}

#[test]
fn aaa_test_e() {
    assert_eq!(do_eval("3"), 3.0);
    assert_eq!(do_eval("3+4+5+6"), 18.0);
    assert_eq!(do_eval("3-4-5-6"), -12.0);
    assert_eq!(do_eval("3*4*5*6"), 360.0);
    assert_eq!(do_eval("3/4/5/6"), 0.024999999999999998);   // Fragile!
    assert_eq!(do_eval("2^3^4"), 2417851639229258349412352.0);
    assert_eq!(do_eval("3*3-3/3"), 8.0);
    assert_eq!(do_eval("(1+1)^3"), 8.0);
    assert_eq!(do_eval("(1+(-1)^4)^3"), 8.0);
    assert_eq!(do_eval("(1+1)^(1+1+1)"), 8.0);
    assert_eq!(do_eval("(8^(1/3))^(3)"), 8.0);  // Fragile!  Go is 7.999999999999997
    assert_eq!(do_eval("round( (8^(1/3))^(3) )"), 8.0);

    assert_eq!(do_eval("5%2"), 1.0);
    assert_eq!(do_eval("5%3"), 2.0);
    assert_eq!(do_eval("5.1%3.2"), 1.8999999999999995);
    assert_eq!(do_eval("5.1%2.5"), 0.09999999999999964);
    assert_eq!(do_eval("5.1%2.499999999"), 0.10000000199999981);
    assert_eq!(do_eval("-5%2"), -1.0);
    assert_eq!(do_eval("-5%3"), -2.0);
    assert_eq!(do_eval("-5.1%3.2"), -1.8999999999999995);
    assert_eq!(do_eval("-5.1%2.5"), -0.09999999999999964);
    assert_eq!(do_eval("-5.1%2.499999999"), -0.10000000199999981);
    assert_eq!(do_eval("5%-2"), 1.0);
    assert_eq!(do_eval("5%-3"), 2.0);
    assert_eq!(do_eval("5.1%-3.2"), 1.8999999999999995);
    assert_eq!(do_eval("5.1%-2.5"), 0.09999999999999964);
    assert_eq!(do_eval("5.1%-2.499999999"), 0.10000000199999981);
    assert_eq!(do_eval("int(5)%int(2)"), 1.0);
    assert_eq!(do_eval("int(5)%int(3)"), 2.0);
    assert_eq!(do_eval("int(5.1)%round(3.2)"), 2.0);
    assert_eq!(do_eval("int(5.1)%round(2.5)"), 2.0);
    assert_eq!(do_eval("int(5.1)%round(2.499999999)"), 1.0);
    assert_eq!(do_eval("int(5)%int(-2)"), 1.0);
    assert_eq!(do_eval("int(5)%int(-3)"), 2.0);
    assert_eq!(do_eval("int(5.1)%round(-3.2)"), 2.0);
    assert_eq!(do_eval("int(5.1)%round(-2.5)"), 2.0);
    assert_eq!(do_eval("int(5.1)%round(-2.499999999)"), 1.0);

    assert_eq!(do_eval("int(123.456/78)*78 + 123.456%78"), 123.456);
    assert_eq!(do_eval("int(-123.456/78)*78 + -123.456%78"), -123.456);
    assert_eq!(do_eval("int(-123.456/-78)*-78 + -123.456%-78"), -123.456);
}

#[test]
fn aaa_test_f() {
    let mut slab = Slab::new();

    assert_eq!(ok_parse("(x)^(3)", &mut slab).from(&slab.ps).eval(&slab, &mut CachedCallbackNamespace::new(|n,_| { [("x",2.0)].iter().cloned().collect::<BTreeMap<&str,f64>>().get(n).cloned() })).unwrap(), 8.0);
    assert_eq!(ok_parse("(x)^(y)", &mut slab).from(&slab.ps).eval(&slab, &mut CachedCallbackNamespace::new(|n,_| { [("x",2.0),("y",3.0)].iter().cloned().collect::<BTreeMap<&str,f64>>().get(n).cloned() })).unwrap(), 8.0);
    assert_eq!(ok_parse("(x)^(y)", &mut slab).from(&slab.ps).var_names(&slab).len(), 2);
    assert_eq!(ok_parse("1+(x*y/2)^(z)", &mut slab).from(&slab.ps).var_names(&slab).len(), 3);
    assert_eq!(format!("{:?}",ok_parse("1+(x*y/2)^(z)", &mut slab).from(&slab.ps).var_names(&slab).iter().collect::<BTreeSet<&String>>()), r#"{"x", "y", "z"}"#);
    assert_eq!(format!("{:?}",ok_parse("1+(x/y/2)^(z)", &mut slab).from(&slab.ps).var_names(&slab).iter().collect::<BTreeSet<&String>>()), r#"{"x", "y", "z"}"#);  // Test a division-by-0 during VariableNames()
    assert_eq!(format!("{}",do_eval("1/0")), "inf");  // Test an explicit division-by-0.  Go says "+Inf".
}

#[test]
fn aaa_test_g() {
    assert_eq!(do_eval("2k"), 2000.0);
    assert_eq!(do_eval("2K"), 2000.0);
    assert_eq!(do_eval("2.10M"), 2.1e+06);
    assert_eq!(do_eval("2.10G"), 2.1e+09);
    assert_eq!(do_eval("2.10T"), 2.1e+12);
    assert_eq!(do_eval("2.10m"), 2.1e-03);
    assert_eq!(do_eval("2.10u"), 2.1e-06);
    assert_eq!(do_eval("2.10µ"), 2.1e-06);
    assert_eq!(do_eval("2.10n"), 2.1e-09);
    assert_eq!(do_eval("2.10p"), 2.1e-12);
}

#[test]
fn aaa_test_h() {
    assert_eq!(do_eval("!100"), 0.0);
    assert_eq!(do_eval("!0"), 1.0);
    assert_eq!(do_eval("!(1-1)"), 1.0);
}

#[test]
fn aaa_test_i() {
    assert_eq!(do_eval("1<2"), 1.0);
    assert_eq!(do_eval("(1+2)<2"), 0.0);
    assert_eq!(do_eval("2<=2"), 1.0);
    assert_eq!(do_eval("2<=(2-0.1)"), 0.0);
    assert_eq!(do_eval("2k==2K"), 1.0);
    assert_eq!(do_eval("2k==2000"), 1.0);
    assert_eq!(do_eval("2k==2000.0000001"), 0.0);
    assert_eq!(do_eval("2k!=2000.0000001"), 1.0);
    assert_eq!(do_eval("2k!=3G"), 1.0);
    assert_eq!(do_eval("1000*2k!=2M"), 0.0);
    assert_eq!(do_eval("3>=2"), 1.0);
    assert_eq!(do_eval("3>=2^2"), 0.0);
    assert_eq!(do_eval("3>2"), 1.0);
    assert_eq!(do_eval("3>2^2"), 0.0);
    assert_eq!(do_eval("1 or 1"), 1.0); assert_eq!(do_eval("1 || 1"), 1.0);
    assert_eq!(do_eval("1 or 0"), 1.0); assert_eq!(do_eval("1 || 0"), 1.0);
    assert_eq!(do_eval("0 or 1"), 1.0); assert_eq!(do_eval("0 || 1"), 1.0);
    assert_eq!(do_eval("0 or 0"), 0.0); assert_eq!(do_eval("0 || 0"), 0.0);
    assert_eq!(do_eval("0 and 0"), 0.0); assert_eq!(do_eval("0 && 0"), 0.0);
    assert_eq!(do_eval("0 and 1"), 0.0); assert_eq!(do_eval("0 && 1"), 0.0);
    assert_eq!(do_eval("1 and 0"), 0.0); assert_eq!(do_eval("1 && 0"), 0.0);
    assert_eq!(do_eval("1 and 1"), 1.0); assert_eq!(do_eval("1 && 1"), 1.0);
    assert_eq!(do_eval("(2k*1k==2M and 3/2<2 or 0^2) and !(1-1)"), 1.0); assert_eq!(do_eval("(2k*1k==2M && 3/2<2 || 0^2) && !(1-1)"), 1.0);

    // Ternary ability:
    assert_eq!(do_eval("2 and 3"), 3.0); assert_eq!(do_eval("2 && 3"), 3.0);
    assert_eq!(do_eval("2 or 3"), 2.0); assert_eq!(do_eval("2 || 3"), 2.0);
    assert_eq!(do_eval("2 and 3 or 4"), 3.0); assert_eq!(do_eval("2 && 3 || 4"), 3.0);
    assert_eq!(do_eval("0 and 3 or 4"), 4.0); assert_eq!(do_eval("0 && 3 || 4"), 4.0);
    assert_eq!(do_eval("2 and 0 or 4"), 4.0); assert_eq!(do_eval("2 && 0 || 4"), 4.0);
    assert_eq!(do_eval("0 and 3 or 0 and 5 or 6"), 6.0); assert_eq!(do_eval("0 && 3 || 0 && 5 || 6"), 6.0);
}

#[test]
fn aaa_test_j() {
    assert_eq!(do_eval("2/3*3/2"), 1.0);
    assert_eq!(do_eval("2%3*3/2"), 3.0);
    assert_eq!(do_eval("3^2%2^2*2^2/3^2"), 0.4444444444444444);
    assert_eq!(do_eval("1+2-3+4"), 4.0);
}

#[test]
fn aaa_test_k() {
    do_eval(r#"print("a",print("b",print("c",5,"C"),"B"),"A")"#);

// TODO: Capture -- i bet i can re-use rust's existing test-output capturing feature:
//    stderr:=CaptureStderr(func(){
//        do_eval(r#"print("a",print("b",print("c",5,"C"),"B"),"A")"#);    // Other stuff process from-inside-to-out.
//    })
//    stderr=strings.TrimSpace(stderr)
//    Assert(stderr==`c 5 C
//b 5 B
//a 5 A`)
}

