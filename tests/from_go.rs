use algebra::grammar::{*, Value::*, BinaryOp::*, UnaryOp::*};
use algebra::parser::Parser;
use algebra::error::Error;
use algebra::evalns::EvalNS;
use algebra::evaler::Evaler;

use std::collections::HashMap;
use std::collections::BTreeSet;

fn parse_raw(s:&str) -> Result<Expression, Error> {
    Parser{is_const_byte:None,
           is_var_byte:None,
           slab:None}.parse(s)
}
fn parse(s:&str) -> Expression { parse_raw(s).unwrap() }

fn eval(expr:Expression) -> f64 { expr.eval(&mut EvalNS::new(|_| None)).unwrap() }

fn capture_stderr(f:&dyn Fn()) -> String {
    f();
    "".to_string()
}

#[test]
fn aaa_test_a() {
    assert_eq!(parse("3"),
Expression{first:EConstant(Constant(3.0)), pairs:Box::new([])});
    assert_eq!(parse("3.14"),
Expression{first:EConstant(Constant(3.14)), pairs:Box::new([])});
    assert_eq!(parse("3+5"),
Expression { first: EConstant(Constant(3.0)), pairs:Box::new([ExprPair(EPlus, EConstant(Constant(5.0)))]) });
    assert_eq!(parse("3-5"),
Expression { first: EConstant(Constant(3.0)), pairs:Box::new([ExprPair(EMinus, EConstant(Constant(5.0)))]) });
    assert_eq!(parse("3*5"),
Expression { first: EConstant(Constant(3.0)), pairs:Box::new([ExprPair(EMul, EConstant(Constant(5.0)))]) });
    assert_eq!(parse("3/5"),
Expression { first: EConstant(Constant(3.0)), pairs:Box::new([ExprPair(EDiv, EConstant(Constant(5.0)))]) });
    assert_eq!(parse("3^5"),
Expression { first: EConstant(Constant(3.0)), pairs:Box::new([ExprPair(EExp, EConstant(Constant(5.0)))]) });
}

#[test]
fn aaa_test_b0() {
    assert_eq!(parse("3.14 + 4.99999999999999"),
Expression { first: EConstant(Constant(3.14)), pairs:Box::new([ExprPair(EPlus, EConstant(Constant(4.99999999999999)))]) });
    assert_eq!(parse("3.14 + 4.99999999999999999999999999999999999999999999999999999"),
Expression { first: EConstant(Constant(3.14)), pairs:Box::new([ExprPair(EPlus, EConstant(Constant(5.0)))]) });
    // Go can parse this, but not Rust:
    assert_eq!(parse_raw("3.14 + 4.999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999"),
Err(Error::new("parse<f64> error")));
    assert_eq!(parse("3.14 + 0.9999"),
Expression { first: EConstant(Constant(3.14)), pairs:Box::new([ExprPair(EPlus, EConstant(Constant(0.9999)))]) });
    assert_eq!(parse("3.14 + .9999"),
Expression { first: EConstant(Constant(3.14)), pairs:Box::new([ExprPair(EPlus, EConstant(Constant(0.9999)))]) });
    assert_eq!(parse("3.14 + 0."),
Expression { first: EConstant(Constant(3.14)), pairs:Box::new([ExprPair(EPlus, EConstant(Constant(0.0)))]) });
}

#[test]
fn aaa_test_b1() {
    assert_eq!(parse_raw("3.14 + 4.99999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999.9999"),
Err(Error::new("parse<f64> error")));
    assert_eq!(parse_raw("3.14 + 4.9999.9999"),
Err(Error::new("parse<f64> error")));
}

#[test]
fn aaa_test_b2() {
    assert_eq!(parse_raw("3.14 + ."),
Err(Error::new("parse<f64> error")));
}

#[test]
fn aaa_test_c0() {
    assert_eq!(parse("3+5-xyz"),
Expression { first: EConstant(Constant(3.0)), pairs:Box::new([ExprPair(EPlus, EConstant(Constant(5.0))), ExprPair(EMinus, EVariable(Variable("xyz".to_string())))]) });
    assert_eq!(parse("3+5-xyz_abc_def123"),
Expression { first: EConstant(Constant(3.0)), pairs:Box::new([ExprPair(EPlus, EConstant(Constant(5.0))), ExprPair(EMinus, EVariable(Variable("xyz_abc_def123".to_string())))]) });
    assert_eq!(parse("3+5-XYZ_abc_def123"),
Expression { first: EConstant(Constant(3.0)), pairs:Box::new([ExprPair(EPlus, EConstant(Constant(5.0))), ExprPair(EMinus, EVariable(Variable("XYZ_abc_def123".to_string())))]) });
    assert_eq!(parse("3+5-XYZ_ab*c_def123"),
Expression { first: EConstant(Constant(3.0)), pairs:Box::new([ExprPair(EPlus, EConstant(Constant(5.0))), ExprPair(EMinus, EVariable(Variable("XYZ_ab".to_string()))), ExprPair(EMul, EVariable(Variable("c_def123".to_string())))]) });
}

#[test]
fn aaa_test_c1() {
    assert_eq!(parse_raw("3+5-XYZ_ab~c_def123"),
Err(Error::new("unparsed tokens remaining")));
}

#[test]
fn aaa_test_d0() {
    assert_eq!(parse("3+(-5)"),
Expression { first: EConstant(Constant(3.0)), pairs:Box::new([ExprPair(EPlus, EUnaryOp(EParens(Box::new(Expression { first: EUnaryOp(ENeg(Box::new(EConstant(Constant(5.0))))), pairs:Box::new([]) }))))]) });
    assert_eq!(parse("3+-5"),
Expression { first: EConstant(Constant(3.0)), pairs:Box::new([ExprPair(EPlus, EUnaryOp(ENeg(Box::new(EConstant(Constant(5.0))))))]) });
    assert_eq!(parse("3++5"),
Expression { first: EConstant(Constant(3.0)), pairs:Box::new([ExprPair(EPlus, EUnaryOp(EPos(Box::new(EConstant(Constant(5.0))))))]) });
    assert_eq!(parse(" 3 + ( -x + y ) "),
Expression { first: EConstant(Constant(3.0)), pairs:Box::new([ExprPair(EPlus, EUnaryOp(EParens(Box::new(Expression { first: EUnaryOp(ENeg(Box::new(EVariable(Variable("x".to_string()))))), pairs:Box::new([ExprPair(EPlus, EVariable(Variable("y".to_string())))]) }))))]) });
}

#[test]
fn aaa_test_d1() {
    assert_eq!(parse_raw(" 3 + ( -x + y  "),
Err(Error::new("EOF")));
}

#[test]
fn aaa_test_e() {
    assert_eq!(eval(parse("3")), 3.0);
    assert_eq!(eval(parse("3+4+5+6")), 18.0);
    assert_eq!(eval(parse("3-4-5-6")), -12.0);
    assert_eq!(eval(parse("3*4*5*6")), 360.0);
    assert_eq!(eval(parse("3/4/5/6")), 0.024999999999999998);   // Fragile!
    assert_eq!(eval(parse("2^3^4")), 2417851639229258349412352.0);
    assert_eq!(eval(parse("3*3-3/3")), 8.0);
    assert_eq!(eval(parse("(1+1)^3")), 8.0);
    assert_eq!(eval(parse("(1+(-1)^4)^3")), 8.0);
    assert_eq!(eval(parse("(1+1)^(1+1+1)")), 8.0);
    assert_eq!(eval(parse("(8^(1/3))^(3)")), 8.0);  // Fragile!  Go is 7.999999999999997
    assert_eq!(eval(parse("round( (8^(1/3))^(3) )")), 8.0);

    assert_eq!(eval(parse("5%2")), 1.0);
    assert_eq!(eval(parse("5%3")), 2.0);
    assert_eq!(eval(parse("5.1%3.2")), 1.8999999999999995);
    assert_eq!(eval(parse("5.1%2.5")), 0.09999999999999964);
    assert_eq!(eval(parse("5.1%2.499999999")), 0.10000000199999981);
    assert_eq!(eval(parse("-5%2")), -1.0);
    assert_eq!(eval(parse("-5%3")), -2.0);
    assert_eq!(eval(parse("-5.1%3.2")), -1.8999999999999995);
    assert_eq!(eval(parse("-5.1%2.5")), -0.09999999999999964);
    assert_eq!(eval(parse("-5.1%2.499999999")), -0.10000000199999981);
    assert_eq!(eval(parse("5%-2")), 1.0);
    assert_eq!(eval(parse("5%-3")), 2.0);
    assert_eq!(eval(parse("5.1%-3.2")), 1.8999999999999995);
    assert_eq!(eval(parse("5.1%-2.5")), 0.09999999999999964);
    assert_eq!(eval(parse("5.1%-2.499999999")), 0.10000000199999981);
    assert_eq!(eval(parse("int(5)%int(2)")), 1.0);
    assert_eq!(eval(parse("int(5)%int(3)")), 2.0);
    assert_eq!(eval(parse("int(5.1)%round(3.2)")), 2.0);
    assert_eq!(eval(parse("int(5.1)%round(2.5)")), 2.0);
    assert_eq!(eval(parse("int(5.1)%round(2.499999999)")), 1.0);
    assert_eq!(eval(parse("int(5)%int(-2)")), 1.0);
    assert_eq!(eval(parse("int(5)%int(-3)")), 2.0);
    assert_eq!(eval(parse("int(5.1)%round(-3.2)")), 2.0);
    assert_eq!(eval(parse("int(5.1)%round(-2.5)")), 2.0);
    assert_eq!(eval(parse("int(5.1)%round(-2.499999999)")), 1.0);

    assert_eq!(eval(parse("int(123.456/78)*78 + 123.456%78")), 123.456);
    assert_eq!(eval(parse("int(-123.456/78)*78 + -123.456%78")), -123.456);
    assert_eq!(eval(parse("int(-123.456/-78)*-78 + -123.456%-78")), -123.456);
}

#[test]
fn aaa_test_f() {
    assert_eq!(parse("(x)^(3)").eval(&mut EvalNS::new(|n| { [("x",2.0)].iter().cloned().collect::<HashMap<&str,f64>>().get(n).cloned() })).unwrap(), 8.0);
    assert_eq!(parse("(x)^(y)").eval(&mut EvalNS::new(|n| { [("x",2.0),("y",3.0)].iter().cloned().collect::<HashMap<&str,f64>>().get(n).cloned() })).unwrap(), 8.0);
    assert_eq!(parse("(x)^(y)").var_names().unwrap().len(), 2);
    assert_eq!(parse("1+(x*y/2)^(z)").var_names().unwrap().len(), 3);
    assert_eq!(format!("{:?}",parse("1+(x*y/2)^(z)").var_names().unwrap().iter().collect::<BTreeSet<&String>>()), r#"{"x", "y", "z"}"#);
    assert_eq!(format!("{:?}",parse("1+(x/y/2)^(z)").var_names().unwrap().iter().collect::<BTreeSet<&String>>()), r#"{"x", "y", "z"}"#);  // Test a division-by-0 during VariableNames()
    assert_eq!(format!("{}",eval(parse("1/0"))), "inf");  // Test an explicit division-by-0.  Go says "+Inf".
}

#[test]
fn aaa_test_g() {
    assert_eq!(eval(parse("2k")), 2000.0);
    assert_eq!(eval(parse("2K")), 2000.0);
    assert_eq!(eval(parse("2.10M")), 2.1e+06);
    assert_eq!(eval(parse("2.10G")), 2.1e+09);
    assert_eq!(eval(parse("2.10T")), 2.1e+12);
}

#[test]
fn aaa_test_h() {
    assert_eq!(eval(parse("!100")), 0.0);
    assert_eq!(eval(parse("!0")), 1.0);
    assert_eq!(eval(parse("!(1-1)")), 1.0);
}

#[test]
fn aaa_test_i() {
    assert_eq!(eval(parse("1<2")), 1.0);
    assert_eq!(eval(parse("(1+2)<2")), 0.0);
    assert_eq!(eval(parse("2<=2")), 1.0);
    assert_eq!(eval(parse("2<=(2-0.1)")), 0.0);
    assert_eq!(eval(parse("2k==2K")), 1.0);
    assert_eq!(eval(parse("2k==2000")), 1.0);
    assert_eq!(eval(parse("2k==2000.0000001")), 0.0);
    assert_eq!(eval(parse("2k!=2000.0000001")), 1.0);
    assert_eq!(eval(parse("2k!=3G")), 1.0);
    assert_eq!(eval(parse("1000*2k!=2M")), 0.0);
    assert_eq!(eval(parse("3>=2")), 1.0);
    assert_eq!(eval(parse("3>=2^2")), 0.0);
    assert_eq!(eval(parse("3>2")), 1.0);
    assert_eq!(eval(parse("3>2^2")), 0.0);
    assert_eq!(eval(parse("1 or 1")), 1.0);
    assert_eq!(eval(parse("1 or 0")), 1.0);
    assert_eq!(eval(parse("0 or 1")), 1.0);
    assert_eq!(eval(parse("0 or 0")), 0.0);
    assert_eq!(eval(parse("0 and 0")), 0.0);
    assert_eq!(eval(parse("0 and 1")), 0.0);
    assert_eq!(eval(parse("1 and 0")), 0.0);
    assert_eq!(eval(parse("1 and 1")), 1.0);
    assert_eq!(eval(parse("(2k*1k==2M and 3/2<2 or 0^2) and !(1-1)")), 1.0);

    // Ternary ability:
    assert_eq!(eval(parse("2 and 3")), 3.0);
    assert_eq!(eval(parse("2 or 3")), 2.0);
    assert_eq!(eval(parse("2 and 3 or 4")), 3.0);
    assert_eq!(eval(parse("0 and 3 or 4")), 4.0);
    assert_eq!(eval(parse("2 and 0 or 4")), 4.0);
    assert_eq!(eval(parse("0 and 3 or 0 and 5 or 6")), 6.0);
}

#[test]
fn aaa_test_j() {
    assert_eq!(eval(parse("2/3*3/2")), 1.0);
    assert_eq!(eval(parse("2%3*3/2")), 3.0);
    assert_eq!(eval(parse("3^2%2^2*2^2/3^2")), 0.4444444444444444);
    assert_eq!(eval(parse("1+2-3+4")), 4.0);
}

#[test]
fn aaa_test_k() {
    assert_eq!(eval(parse("eval(one, one=1)")), 1.0);
    assert_eq!(eval(parse("eval(one+one, one=1)")), 2.0);
    assert_eq!(eval(parse("eval(one+one*one/one, one=1)")), 2.0);
    assert_eq!(format!("{:?}",parse("eval(one+two, one=1)").var_names().unwrap()), r#"{"two"}"#);

    assert_eq!(parse_raw("eval(one, one=1, one=2)"), Err(Error::new("already defined: one")));

    assert_eq!(eval(parse("eval(eval(eval(a*2,a=3),a=2),a=1)")), 2.0);  // 'eval()' processes from-outside-to-in (opposite of normal convention).

    eval(parse(r#"print("a",print("b",print("c",5,"C"),"B"),"A")"#));

// TODO: Capture -- i bet i can re-use rust's existing test-output capturing feature:
//    stderr:=CaptureStderr(func(){
//        Eval(Parse(`print("a",print("b",print("c",5,"C"),"B"),"A")`))    // Other stuff process from-inside-to-out.
//    })
//    stderr=strings.TrimSpace(stderr)
//    Assert(stderr==`c 5 C
//b 5 B
//a 5 A`)
}

