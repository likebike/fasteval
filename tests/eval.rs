use al::{Slab, Parser, EvalNS, Evaler};
use al::evaler::bool_to_f64;

use kerr::KErr;

use std::mem;
use std::collections::HashSet;

#[test]
fn eval() {
    let mut p = Parser::new();
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(|n,_| match n {
        "x" => Some(1.0),
        "y" => Some(2.0),
        "z" => Some(3.0),
        _ => None,
    });

    // Sanity check:
    assert_eq!(p.parse(&mut slab.ps,"3+3-3/3").unwrap().from(&slab.ps).eval(&slab, &mut ns).unwrap(), 5.0);

    assert_eq!(p.parse(&mut slab.ps,"x+y+z").unwrap().from(&slab.ps).eval(&slab, &mut ns).unwrap(), 6.0);

    assert_eq!(p.parse(&mut slab.ps,"x+y+z+a").unwrap().from(&slab.ps).eval(&slab, &mut ns), Err(KErr::new("variable undefined: a")));

    assert_eq!(p.parse(&mut slab.ps,"x+eval(x)+x").unwrap().from(&slab.ps).eval(&slab, &mut ns).unwrap(), 3.0);
    
    assert_eq!(p.parse(&mut slab.ps,"x+eval(x, x=10)+x").unwrap().from(&slab.ps).eval(&slab, &mut ns).unwrap(), 12.0);

    assert_eq!(p.parse(&mut slab.ps,"x+eval(y, x=10, y=x+1)+x").unwrap().from(&slab.ps).eval(&slab, &mut ns).unwrap(), 13.0);

    assert_eq!(p.parse(&mut slab.ps,"x+eval(y, y=x+1, x=10)+x").unwrap().from(&slab.ps).eval(&slab, &mut ns).unwrap(), 4.0);

    assert_eq!(p.parse(&mut slab.ps,"x+eval(x, x=10)+eval(x, x=20)+x").unwrap().from(&slab.ps).eval(&slab, &mut ns).unwrap(), 32.0);

    assert_eq!(p.parse(&mut slab.ps,"x+eval( eval(x, x=10)+eval(x, x=20), x=30 )+x").unwrap().from(&slab.ps).eval(&slab, &mut ns).unwrap(), 62.0);
}

#[test]
fn aaa_util() {
    assert_eq!(bool_to_f64(true), 1.0);
    assert_eq!(bool_to_f64(false), 0.0);
}

#[test]
fn aaa_aaa_sizes() {
    eprintln!("sizeof(Slab):{}", mem::size_of::<Slab>());
    assert!(mem::size_of::<Slab>()<2usize.pow(18));  // 256kB

}

#[test]
fn aaa_aab_single() {
    let mut p = Parser::new();
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(|_,_| None);
    assert_eq!(p.parse(&mut slab.ps, "123.456").unwrap().from(&slab.ps).eval(&slab, &mut ns).unwrap(), 123.456f64);
}

#[test]
fn aaa_basics() {
    let mut p = Parser::new();
    let mut slab = Slab::new();

    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "12.34 + 43.21 + 11.11").unwrap().from(&slab.ps).var_names(&slab).unwrap(),
        HashSet::new());

    let mut ns = EvalNS::new(|_,_| None);
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "12.34 + 43.21 + 11.11").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(66.66));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "12.34 + 43.21 - 11.11").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(44.44));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "11.11 * 3").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(33.33));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "33.33 / 3").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(11.11));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "33.33 % 3").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(0.3299999999999983));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "1 and 2").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(2.0));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "2 or 0").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(2.0));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "1 > 0").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(1.0));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "1 < 0").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(0.0));

    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "+5.5").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(5.5));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "-5.5").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(-5.5));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "!5.5").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(0.0));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "!0").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(1.0));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "(3 * 3 + 3 / 3)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(10.0));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "(3 * (3 + 3) / 3)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(6.0));

    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "4.4 + -5.5").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(-1.0999999999999996));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "4.4 + +5.5").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(9.9));

    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "x + 1").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Err(KErr::new("variable undefined: x")));

    let mut ns = EvalNS::new(|_,_| Some(3.0));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "x + 1").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(4.0));

    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "1.2 + int(3.4)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(4.2));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "1.2 + ceil(3.4)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(5.2));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "1.2 + floor(3.4)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(4.2));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "1.2 + abs(-3.4)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(4.6));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "1.2 + log(1)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(1.2));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "1.2 + log(10)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(2.2));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "1.2 + log(0)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(std::f64::NEG_INFINITY));
    assert!(p.parse({slab.clear(); &mut slab.ps}, "1.2 + log(-1)").unwrap().from(&slab.ps).eval(&slab, &mut ns).unwrap().is_nan());
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "1.2 + round(3.4)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(4.2));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "1.2 + round(0.5, 3.4)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(4.7));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "1.2 + round(-3.4)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(-1.8));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "1.2 + round(0.5, -3.4)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(-2.3));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "1.2 + min(1,2,0,3.3,-1)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(0.19999999999999996));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "1.2 + min(1)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(2.2));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "1.2 + max(1,2,0,3.3,-1)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(4.5));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "1.2 + max(1)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(2.2));

    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, r#"12.34 + print ( 43.21, "yay" ) + 11.11"#).unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(66.66));

    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, r#"12.34 + eval ( x + 43.21 - y, x=2.5, y = 2.5 ) + 11.11"#).unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(66.66));

    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "e()").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(2.718281828459045));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "pi()").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(3.141592653589793));

    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "sin(pi()/2)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(1.0));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "cos(pi()/2)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(0.00000000000000006123233995736766));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "tan(pi()/4)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(0.9999999999999999));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "asin(1)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(1.5707963267948966));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "acos(0)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(1.5707963267948966));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "atan(1)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(0.7853981633974483));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "sinh(pi()/2)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(2.3012989023072947));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "cosh(pi()/2)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(2.5091784786580567));
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "tanh(pi()/4)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(0.6557942026326724));
}


#[derive(Debug)]
struct TestEvaler;
impl Evaler for TestEvaler {
    fn eval(&self, _slab:&Slab, ns:&mut EvalNS) -> Result<f64,KErr> {
        match ns.get("x",vec![]) {
            Some(v) => Ok(v),
            None => Ok(1.23),
        }
    }
}

#[test]
fn aaa_evalns_basics() {
    let slab = Slab::new();
    let mut ns = EvalNS::new(|_,_| Some(5.4321));
    assert_eq!(ns.eval_bubble(&slab, &TestEvaler{}).unwrap(), 5.4321);
    ns.create("x".to_string(),1.111).unwrap();
    assert_eq!(ns.eval_bubble(&slab, &TestEvaler{}).unwrap(), 1.111);
    
    assert_eq!(ns.is_reeval(), false);
    ns.start_reeval_mode();
        assert_eq!(ns.is_reeval(), true);

        ns.start_reeval_mode();
            assert_eq!(ns.is_reeval(), true);
            assert_eq!(ns.eval_bubble(&slab, &TestEvaler{}).unwrap(), 1.111);
        ns.end_reeval_mode();

        assert_eq!(ns.is_reeval(), true);
    ns.end_reeval_mode();
    assert_eq!(ns.is_reeval(), false);
}

#[test]
fn corners() {
    let mut p = Parser::new();
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(|_,_| None);
    assert_eq!(
        format!("{:?}", p.parse({slab.clear(); &mut slab.ps}, "(-1) ^ 0.5").unwrap().from(&slab.ps).eval(&slab, &mut ns)),
        "Ok(NaN)");
}

fn my_evalns_cb_function(_:&str, _:Vec<f64>) -> Option<f64> { None }
#[test]
fn evalns_cb_ownership() {
    let _ns = EvalNS::new(my_evalns_cb_function);
    let _ns = EvalNS::new(my_evalns_cb_function);
    // Conclusion: You can pass a function pointer into a function that receives ownership.

    let closure = |_:&str, _:Vec<f64>| None;
    let _ns = EvalNS::new(closure);
    let _ns = EvalNS::new(closure);

    let x = 1.0;
    let closure = |_:&str, _:Vec<f64>| Some(x);
    let _ns = EvalNS::new(closure);
    let _ns = EvalNS::new(closure);

    let mut x = 1.0;
    let closure = |_:&str, _:Vec<f64>| {
        x+=1.0;
        Some(x)
    };
    let _ns = EvalNS::new(closure);
    //let _ns = EvalNS::new(closure);  // Not allowed.

    // Conclusion: Functions and Closures that don't mutate state are effectively Copy.
    //             Closures that mutate state aren't Copy.
    //             Note that the argument type (FnMut vs Fn) doesn't actually matter,
    //             just the implementation matters!
}

#[test]
fn custom_func() {
    let mut p = Parser::new();
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(|name,args| {
        eprintln!("In CB: {}",name);
        match name {
            "x" => Some(1.0),
            "y" => Some(2.0),
            "z" => Some(3.0),
            "foo" => {
                Some(args.get(0).unwrap_or(&std::f64::NAN)*10.0)
            }
            "bar" => {
                Some(args.get(0).unwrap_or(&std::f64::NAN) + args.get(1).unwrap_or(&std::f64::NAN))
            }
            _ => None,
        }
    });
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "x + 1.5").unwrap().from(&slab.ps).eval(&slab, {ns.clear(); &mut ns}),
        Ok(2.5));

    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "x() + 1.5").unwrap().from(&slab.ps).eval(&slab, {ns.clear(); &mut ns}),
        Ok(2.5));

    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "x(1,2,3) + 1.5").unwrap().from(&slab.ps).eval(&slab, {ns.clear(); &mut ns}),
        Ok(2.5));

    eprintln!("I should see TWO x lookups, 1 y, and 1 z:");
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "x(x,y,z) + 1.5").unwrap().from(&slab.ps).eval(&slab, {ns.clear(); &mut ns}),
        Ok(2.5));

    eprintln!("I should see TWO x lookups:");
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "x(x,x,x) + 1.5").unwrap().from(&slab.ps).eval(&slab, {ns.clear(); &mut ns}),
        Ok(2.5));

    eprintln!("I should see TWO x lookups:");
    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "x(1.0) + x(1.1) + x(1.0) + x(1.1)").unwrap().from(&slab.ps).eval(&slab, {ns.clear(); &mut ns}),
        Ok(4.0));

    eprintln!("---------------------------");

    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "foo(1.23)").unwrap().from(&slab.ps).eval(&slab, {ns.clear(); &mut ns}),
        Ok(12.3));

    assert_eq!(
        p.parse({slab.clear(); &mut slab.ps}, "bar(1.23, 3.21)").unwrap().from(&slab.ps).eval(&slab, {ns.clear(); &mut ns}),
        Ok(4.4399999999999995));

    assert_eq!(
        format!("{:?}", p.parse({slab.clear(); &mut slab.ps}, "bar(1.23)").unwrap().from(&slab.ps).eval(&slab, {ns.clear(); &mut ns})),
        "Ok(NaN)");
}

