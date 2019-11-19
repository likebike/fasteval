use al::{Slab, Parser, EvalNS, Evaler};
use al::evaler::bool_to_f64;

use kerr::KErr;

use std::mem;
use std::collections::HashSet;

#[test]
fn eval() {
    let p = Parser::new(None,None);
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(|n| match n {
        "x" => Some(1.0),
        "y" => Some(2.0),
        "z" => Some(3.0),
        _ => None,
    });

    // Sanity check:
    assert_eq!(p.parse(&mut slab,"3+3-3/3").unwrap().from(&slab).eval(&slab, &mut ns).unwrap(), 5.0);

    assert_eq!(p.parse(&mut slab,"x+y+z").unwrap().from(&slab).eval(&slab, &mut ns).unwrap(), 6.0);

    assert_eq!(p.parse(&mut slab,"x+y+z+a").unwrap().from(&slab).eval(&slab, &mut ns), Err(KErr::new("variable undefined")));

    assert_eq!(p.parse(&mut slab,"x+eval(x)+x").unwrap().from(&slab).eval(&slab, &mut ns).unwrap(), 3.0);
    
    assert_eq!(p.parse(&mut slab,"x+eval(x, x=10)+x").unwrap().from(&slab).eval(&slab, &mut ns).unwrap(), 12.0);

    assert_eq!(p.parse(&mut slab,"x+eval(y, x=10, y=x+1)+x").unwrap().from(&slab).eval(&slab, &mut ns).unwrap(), 13.0);

    assert_eq!(p.parse(&mut slab,"x+eval(y, y=x+1, x=10)+x").unwrap().from(&slab).eval(&slab, &mut ns).unwrap(), 4.0);

    assert_eq!(p.parse(&mut slab,"x+eval(x, x=10)+eval(x, x=20)+x").unwrap().from(&slab).eval(&slab, &mut ns).unwrap(), 32.0);

    assert_eq!(p.parse(&mut slab,"x+eval( eval(x, x=10)+eval(x, x=20), x=30 )+x").unwrap().from(&slab).eval(&slab, &mut ns).unwrap(), 62.0);
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
    let p = Parser::new(None,None);
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(|_| None);
    assert_eq!(p.parse(&mut slab, "123.456").unwrap().from(&slab).eval(&slab, &mut ns).unwrap(), 123.456f64);
}

#[test]
fn aaa_basics() {
    let p = Parser::new(None,None);
    let mut slab = Slab::new();

    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "12.34 + 43.21 + 11.11").unwrap().from(&slab).var_names(&slab).unwrap(),
        HashSet::new());

    let mut ns = EvalNS::new(|_| None);
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "12.34 + 43.21 + 11.11").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(66.66));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "12.34 + 43.21 - 11.11").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(44.44));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "11.11 * 3").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(33.33));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "33.33 / 3").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(11.11));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "33.33 % 3").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(0.3299999999999983));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "1 and 2").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(2.0));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "2 or 0").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(2.0));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "1 > 0").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(1.0));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "1 < 0").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(0.0));

    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "+5.5").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(5.5));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "-5.5").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(-5.5));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "!5.5").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(0.0));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "!0").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(1.0));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "(3 * 3 + 3 / 3)").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(10.0));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "(3 * (3 + 3) / 3)").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(6.0));

    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "4.4 + -5.5").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(-1.0999999999999996));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "4.4 + +5.5").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(9.9));

    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "x + 1").unwrap().from(&slab).eval(&slab, &mut ns),
        Err(KErr::new("variable undefined")));

    let mut ns = EvalNS::new(|_| Some(3.0));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "x + 1").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(4.0));

    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "1.2 + int(3.4)").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(4.2));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "1.2 + ceil(3.4)").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(5.2));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "1.2 + floor(3.4)").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(4.2));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "1.2 + abs(-3.4)").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(4.6));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "1.2 + log(1)").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(1.2));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "1.2 + log(10)").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(2.2));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "1.2 + log(0)").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(std::f64::NEG_INFINITY));
    assert!(p.parse({slab.clear(); &mut slab}, "1.2 + log(-1)").unwrap().from(&slab).eval(&slab, &mut ns).unwrap().is_nan());
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "1.2 + round(3.4)").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(4.2));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "1.2 + round(0.5, 3.4)").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(4.7));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "1.2 + round(-3.4)").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(-1.8));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "1.2 + round(0.5, -3.4)").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(-2.3));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "1.2 + min(1,2,0,3.3,-1)").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(0.19999999999999996));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "1.2 + min(1)").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(2.2));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "1.2 + max(1,2,0,3.3,-1)").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(4.5));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "1.2 + max(1)").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(2.2));

    assert_eq!(
        p.parse({slab.clear(); &mut slab}, r#"12.34 + print ( 43.21, "yay" ) + 11.11"#).unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(66.66));

    assert_eq!(
        p.parse({slab.clear(); &mut slab}, r#"12.34 + eval ( x + 43.21 - y, x=2.5, y = 2.5 ) + 11.11"#).unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(66.66));

    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "e()").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(2.718281828459045));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "pi()").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(3.141592653589793));

    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "sin(pi()/2)").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(1.0));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "cos(pi()/2)").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(0.00000000000000006123233995736766));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "tan(pi()/4)").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(0.9999999999999999));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "asin(1)").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(1.5707963267948966));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "acos(0)").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(1.5707963267948966));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "atan(1)").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(0.7853981633974483));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "sinh(pi()/2)").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(2.3012989023072947));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "cosh(pi()/2)").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(2.5091784786580567));
    assert_eq!(
        p.parse({slab.clear(); &mut slab}, "tanh(pi()/4)").unwrap().from(&slab).eval(&slab, &mut ns),
        Ok(0.6557942026326724));
}


#[derive(Debug)]
struct TestEvaler;
impl Evaler for TestEvaler {
    fn eval(&self, _slab:&Slab, ns:&mut EvalNS) -> Result<f64,KErr> {
        match ns.get("x") {
            Some(v) => Ok(v),
            None => Ok(1.23),
        }
    }
}

#[test]
fn aaa_evalns_basics() {
    let slab = Slab::new();
    let mut ns = EvalNS::new(|_| Some(5.4321));
    assert_eq!(ns.eval_bubble(&slab, &TestEvaler{}).unwrap(), 5.4321);
    ns.create("x".to_string(),1.111).unwrap();
    assert_eq!(ns.eval_bubble(&slab, &TestEvaler{}).unwrap(), 1.111);
    
    assert_eq!(ns.is_normal(), true);
    ns.start_reeval_mode();
        assert_eq!(ns.is_normal(), false);

        ns.start_reeval_mode();
            assert_eq!(ns.is_normal(), false);
            assert_eq!(ns.eval_bubble(&slab, &TestEvaler{}).unwrap(), 1.111);
        ns.end_reeval_mode();

        assert_eq!(ns.is_normal(), false);
    ns.end_reeval_mode();
    assert_eq!(ns.is_normal(), true);
}

