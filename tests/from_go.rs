use al::{ExpressionI, Parser, Slab, EvalNS, Evaler};

use kerr::KErr;

use std::collections::HashMap;
use std::collections::BTreeSet;

fn parse_raw<'a>(slab:&'a mut Slab, s:&str) -> Result<ExpressionI, KErr> {
    Parser::new(None,None).parse(&mut slab.ps,s)
}
fn parse<'a>(slab:&'a mut Slab, s:&str) -> ExpressionI { parse_raw(slab,s).unwrap() }

fn ez_eval(s:&str) -> f64 {
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(|_| None);
    parse(&mut slab, s).from(&slab.ps).eval(&slab, &mut ns).unwrap()
}

fn capture_stderr(f:&dyn Fn()) -> String {
    f();
    "".to_string()
}

#[test]
fn aaa_test_a() {
    let mut slab = Slab::new();

    parse({slab.clear(); &mut slab}, "3");
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(3.0)), pairs: [] } }, vals:{}, instrs:{} }");
    parse({slab.clear(); &mut slab}, "3.14");
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(3.14)), pairs: [] } }, vals:{}, instrs:{} }");
    parse({slab.clear(); &mut slab}, "3+5");
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(3.0)), pairs: [ExprPair(EPlus, EConstant(Constant(5.0)))] } }, vals:{}, instrs:{} }");
    parse({slab.clear(); &mut slab}, "3-5");
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(3.0)), pairs: [ExprPair(EMinus, EConstant(Constant(5.0)))] } }, vals:{}, instrs:{} }");
    parse({slab.clear(); &mut slab}, "3*5");
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(3.0)), pairs: [ExprPair(EMul, EConstant(Constant(5.0)))] } }, vals:{}, instrs:{} }");
    parse({slab.clear(); &mut slab}, "3/5");
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(3.0)), pairs: [ExprPair(EDiv, EConstant(Constant(5.0)))] } }, vals:{}, instrs:{} }");
    parse({slab.clear(); &mut slab}, "3^5");
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(3.0)), pairs: [ExprPair(EExp, EConstant(Constant(5.0)))] } }, vals:{}, instrs:{} }");
}

#[test]
fn aaa_test_b0() {
    let mut slab = Slab::new();

    parse({slab.clear(); &mut slab}, "3.14 + 4.99999999999999");
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(3.14)), pairs: [ExprPair(EPlus, EConstant(Constant(4.99999999999999)))] } }, vals:{}, instrs:{} }");
    parse({slab.clear(); &mut slab}, "3.14 + 4.99999999999999999999999999999999999999999999999999999");
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(3.14)), pairs: [ExprPair(EPlus, EConstant(Constant(5.0)))] } }, vals:{}, instrs:{} }");
    // Go can parse this, but not Rust:
    assert_eq!(parse_raw({slab.clear(); &mut slab}, "3.14 + 4.999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999"),
Err(KErr::new("parse<f64> error")));
    parse({slab.clear(); &mut slab}, "3.14 + 0.9999");
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(3.14)), pairs: [ExprPair(EPlus, EConstant(Constant(0.9999)))] } }, vals:{}, instrs:{} }");
    parse({slab.clear(); &mut slab}, "3.14 + .9999");
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(3.14)), pairs: [ExprPair(EPlus, EConstant(Constant(0.9999)))] } }, vals:{}, instrs:{} }");
    parse({slab.clear(); &mut slab}, "3.14 + 0.");
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(3.14)), pairs: [ExprPair(EPlus, EConstant(Constant(0.0)))] } }, vals:{}, instrs:{} }");
}

#[test]
fn aaa_test_b1() {
    let mut slab = Slab::new();

    assert_eq!(parse_raw({slab.clear(); &mut slab}, "3.14 + 4.99999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999.9999"),
Err(KErr::new("parse<f64> error")));
    assert_eq!(parse_raw({slab.clear(); &mut slab}, "3.14 + 4.9999.9999"),
Err(KErr::new("parse<f64> error")));
}

#[test]
fn aaa_test_b2() {
    let mut slab = Slab::new();

    assert_eq!(parse_raw({slab.clear(); &mut slab}, "3.14 + ."),
Err(KErr::new("parse<f64> error")));
}

#[test]
fn aaa_test_c0() {
    let mut slab = Slab::new();

    parse({slab.clear(); &mut slab}, "3+5-xyz");
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(3.0)), pairs: [ExprPair(EPlus, EConstant(Constant(5.0))), ExprPair(EMinus, EVariable(Variable(`xyz`)))] } }, vals:{}, instrs:{} }");
    parse({slab.clear(); &mut slab}, "3+5-xyz_abc_def123");
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(3.0)), pairs: [ExprPair(EPlus, EConstant(Constant(5.0))), ExprPair(EMinus, EVariable(Variable(`xyz_abc_def123`)))] } }, vals:{}, instrs:{} }");
    parse({slab.clear(); &mut slab}, "3+5-XYZ_abc_def123");
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(3.0)), pairs: [ExprPair(EPlus, EConstant(Constant(5.0))), ExprPair(EMinus, EVariable(Variable(`XYZ_abc_def123`)))] } }, vals:{}, instrs:{} }");
    parse({slab.clear(); &mut slab}, "3+5-XYZ_ab*c_def123");
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(3.0)), pairs: [ExprPair(EPlus, EConstant(Constant(5.0))), ExprPair(EMinus, EVariable(Variable(`XYZ_ab`))), ExprPair(EMul, EVariable(Variable(`c_def123`)))] } }, vals:{}, instrs:{} }");
}

#[test]
fn aaa_test_c1() {
    let mut slab = Slab::new();

    assert_eq!(parse_raw({slab.clear(); &mut slab}, "3+5-XYZ_ab~c_def123"),
Err(KErr::new("unparsed tokens remaining")));
}

#[test]
fn aaa_test_d0() {
    let mut slab = Slab::new();

    parse({slab.clear(); &mut slab}, "3+(-5)");
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EUnaryOp(ENeg(ValueI(0))), pairs: [] }, 1:Expression { first: EConstant(Constant(3.0)), pairs: [ExprPair(EPlus, EUnaryOp(EParens(ExpressionI(0))))] } }, vals:{ 0:EConstant(Constant(5.0)) }, instrs:{} }");
    parse({slab.clear(); &mut slab}, "3+-5");
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(3.0)), pairs: [ExprPair(EPlus, EUnaryOp(ENeg(ValueI(0))))] } }, vals:{ 0:EConstant(Constant(5.0)) }, instrs:{} }");
    parse({slab.clear(); &mut slab}, "3++5");
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(3.0)), pairs: [ExprPair(EPlus, EUnaryOp(EPos(ValueI(0))))] } }, vals:{ 0:EConstant(Constant(5.0)) }, instrs:{} }");
    parse({slab.clear(); &mut slab}, " 3 + ( -x + y ) ");
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EUnaryOp(ENeg(ValueI(0))), pairs: [ExprPair(EPlus, EVariable(Variable(`y`)))] }, 1:Expression { first: EConstant(Constant(3.0)), pairs: [ExprPair(EPlus, EUnaryOp(EParens(ExpressionI(0))))] } }, vals:{ 0:EVariable(Variable(`x`)) }, instrs:{} }");
}

#[test]
fn aaa_test_d1() {
    let mut slab = Slab::new();

    assert_eq!(parse_raw({slab.clear(); &mut slab}, " 3 + ( -x + y  "),
Err(KErr::new("EOF")));
}

#[test]
fn aaa_test_e() {
    assert_eq!(ez_eval("3"), 3.0);
    assert_eq!(ez_eval("3+4+5+6"), 18.0);
    assert_eq!(ez_eval("3-4-5-6"), -12.0);
    assert_eq!(ez_eval("3*4*5*6"), 360.0);
    assert_eq!(ez_eval("3/4/5/6"), 0.024999999999999998);   // Fragile!
    assert_eq!(ez_eval("2^3^4"), 2417851639229258349412352.0);
    assert_eq!(ez_eval("3*3-3/3"), 8.0);
    assert_eq!(ez_eval("(1+1)^3"), 8.0);
    assert_eq!(ez_eval("(1+(-1)^4)^3"), 8.0);
    assert_eq!(ez_eval("(1+1)^(1+1+1)"), 8.0);
    assert_eq!(ez_eval("(8^(1/3))^(3)"), 8.0);  // Fragile!  Go is 7.999999999999997
    assert_eq!(ez_eval("round( (8^(1/3))^(3) )"), 8.0);

    assert_eq!(ez_eval("5%2"), 1.0);
    assert_eq!(ez_eval("5%3"), 2.0);
    assert_eq!(ez_eval("5.1%3.2"), 1.8999999999999995);
    assert_eq!(ez_eval("5.1%2.5"), 0.09999999999999964);
    assert_eq!(ez_eval("5.1%2.499999999"), 0.10000000199999981);
    assert_eq!(ez_eval("-5%2"), -1.0);
    assert_eq!(ez_eval("-5%3"), -2.0);
    assert_eq!(ez_eval("-5.1%3.2"), -1.8999999999999995);
    assert_eq!(ez_eval("-5.1%2.5"), -0.09999999999999964);
    assert_eq!(ez_eval("-5.1%2.499999999"), -0.10000000199999981);
    assert_eq!(ez_eval("5%-2"), 1.0);
    assert_eq!(ez_eval("5%-3"), 2.0);
    assert_eq!(ez_eval("5.1%-3.2"), 1.8999999999999995);
    assert_eq!(ez_eval("5.1%-2.5"), 0.09999999999999964);
    assert_eq!(ez_eval("5.1%-2.499999999"), 0.10000000199999981);
    assert_eq!(ez_eval("int(5)%int(2)"), 1.0);
    assert_eq!(ez_eval("int(5)%int(3)"), 2.0);
    assert_eq!(ez_eval("int(5.1)%round(3.2)"), 2.0);
    assert_eq!(ez_eval("int(5.1)%round(2.5)"), 2.0);
    assert_eq!(ez_eval("int(5.1)%round(2.499999999)"), 1.0);
    assert_eq!(ez_eval("int(5)%int(-2)"), 1.0);
    assert_eq!(ez_eval("int(5)%int(-3)"), 2.0);
    assert_eq!(ez_eval("int(5.1)%round(-3.2)"), 2.0);
    assert_eq!(ez_eval("int(5.1)%round(-2.5)"), 2.0);
    assert_eq!(ez_eval("int(5.1)%round(-2.499999999)"), 1.0);

    assert_eq!(ez_eval("int(123.456/78)*78 + 123.456%78"), 123.456);
    assert_eq!(ez_eval("int(-123.456/78)*78 + -123.456%78"), -123.456);
    assert_eq!(ez_eval("int(-123.456/-78)*-78 + -123.456%-78"), -123.456);
}

#[test]
fn aaa_test_f() {
    let mut slab = Slab::new();

    assert_eq!(parse({slab.clear(); &mut slab}, "(x)^(3)").from(&slab.ps).eval(&slab, &mut EvalNS::new(|n| { [("x",2.0)].iter().cloned().collect::<HashMap<&str,f64>>().get(n).cloned() })).unwrap(), 8.0);
    assert_eq!(parse({slab.clear(); &mut slab}, "(x)^(y)").from(&slab.ps).eval(&slab, &mut EvalNS::new(|n| { [("x",2.0),("y",3.0)].iter().cloned().collect::<HashMap<&str,f64>>().get(n).cloned() })).unwrap(), 8.0);
    assert_eq!(parse({slab.clear(); &mut slab}, "(x)^(y)").from(&slab.ps).var_names(&slab).unwrap().len(), 2);
    assert_eq!(parse({slab.clear(); &mut slab}, "1+(x*y/2)^(z)").from(&slab.ps).var_names(&slab).unwrap().len(), 3);
    assert_eq!(format!("{:?}",parse({slab.clear(); &mut slab}, "1+(x*y/2)^(z)").from(&slab.ps).var_names(&slab).unwrap().iter().collect::<BTreeSet<&String>>()), r#"{"x", "y", "z"}"#);
    assert_eq!(format!("{:?}",parse({slab.clear(); &mut slab}, "1+(x/y/2)^(z)").from(&slab.ps).var_names(&slab).unwrap().iter().collect::<BTreeSet<&String>>()), r#"{"x", "y", "z"}"#);  // Test a division-by-0 during VariableNames()
    assert_eq!(format!("{}",ez_eval("1/0")), "inf");  // Test an explicit division-by-0.  Go says "+Inf".
}

#[test]
fn aaa_test_g() {
    assert_eq!(ez_eval("2k"), 2000.0);
    assert_eq!(ez_eval("2K"), 2000.0);
    assert_eq!(ez_eval("2.10M"), 2.1e+06);
    assert_eq!(ez_eval("2.10G"), 2.1e+09);
    assert_eq!(ez_eval("2.10T"), 2.1e+12);
}

#[test]
fn aaa_test_h() {
    assert_eq!(ez_eval("!100"), 0.0);
    assert_eq!(ez_eval("!0"), 1.0);
    assert_eq!(ez_eval("!(1-1)"), 1.0);
}

#[test]
fn aaa_test_i() {
    assert_eq!(ez_eval("1<2"), 1.0);
    assert_eq!(ez_eval("(1+2)<2"), 0.0);
    assert_eq!(ez_eval("2<=2"), 1.0);
    assert_eq!(ez_eval("2<=(2-0.1)"), 0.0);
    assert_eq!(ez_eval("2k==2K"), 1.0);
    assert_eq!(ez_eval("2k==2000"), 1.0);
    assert_eq!(ez_eval("2k==2000.0000001"), 0.0);
    assert_eq!(ez_eval("2k!=2000.0000001"), 1.0);
    assert_eq!(ez_eval("2k!=3G"), 1.0);
    assert_eq!(ez_eval("1000*2k!=2M"), 0.0);
    assert_eq!(ez_eval("3>=2"), 1.0);
    assert_eq!(ez_eval("3>=2^2"), 0.0);
    assert_eq!(ez_eval("3>2"), 1.0);
    assert_eq!(ez_eval("3>2^2"), 0.0);
    assert_eq!(ez_eval("1 or 1"), 1.0);
    assert_eq!(ez_eval("1 or 0"), 1.0);
    assert_eq!(ez_eval("0 or 1"), 1.0);
    assert_eq!(ez_eval("0 or 0"), 0.0);
    assert_eq!(ez_eval("0 and 0"), 0.0);
    assert_eq!(ez_eval("0 and 1"), 0.0);
    assert_eq!(ez_eval("1 and 0"), 0.0);
    assert_eq!(ez_eval("1 and 1"), 1.0);
    assert_eq!(ez_eval("(2k*1k==2M and 3/2<2 or 0^2) and !(1-1)"), 1.0);

    // Ternary ability:
    assert_eq!(ez_eval("2 and 3"), 3.0);
    assert_eq!(ez_eval("2 or 3"), 2.0);
    assert_eq!(ez_eval("2 and 3 or 4"), 3.0);
    assert_eq!(ez_eval("0 and 3 or 4"), 4.0);
    assert_eq!(ez_eval("2 and 0 or 4"), 4.0);
    assert_eq!(ez_eval("0 and 3 or 0 and 5 or 6"), 6.0);
}

#[test]
fn aaa_test_j() {
    assert_eq!(ez_eval("2/3*3/2"), 1.0);
    assert_eq!(ez_eval("2%3*3/2"), 3.0);
    assert_eq!(ez_eval("3^2%2^2*2^2/3^2"), 0.4444444444444444);
    assert_eq!(ez_eval("1+2-3+4"), 4.0);
}

#[test]
fn aaa_test_k() {
    let mut slab = Slab::new();

    assert_eq!(ez_eval("eval(one, one=1)"), 1.0);
    assert_eq!(ez_eval("eval(one+one, one=1)"), 2.0);
    assert_eq!(ez_eval("eval(one+one*one/one, one=1)"), 2.0);
    assert_eq!(format!("{:?}",parse({slab.clear(); &mut slab}, "eval(one+two, one=1)").from(&slab.ps).var_names(&slab).unwrap()), r#"{"two"}"#);

    assert_eq!(parse_raw({slab.clear(); &mut slab}, "eval(one, one=1, one=2)"), Err(KErr::new("already defined: one")));

    assert_eq!(ez_eval("eval(eval(eval(a*2,a=3),a=2),a=1)"), 2.0);  // 'eval()' processes from-outside-to-in (opposite of normal convention).

    ez_eval(r#"print("a",print("b",print("c",5,"C"),"B"),"A")"#);

// TODO: Capture -- i bet i can re-use rust's existing test-output capturing feature:
//    stderr:=CaptureStderr(func(){
//        ez_eval(r#"print("a",print("b",print("c",5,"C"),"B"),"A")"#);    // Other stuff process from-inside-to-out.
//    })
//    stderr=strings.TrimSpace(stderr)
//    Assert(stderr==`c 5 C
//b 5 B
//a 5 A`)
}

