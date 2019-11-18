use algebra::{Slab, Parser, EvalNS, Evaler};

use kerr::KErr;

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
    assert_eq!(p.parse(&mut slab,"3+3-3/3").unwrap().get(&slab).eval(&slab, &mut ns).unwrap(), 5.0);

    assert_eq!(p.parse(&mut slab,"x+y+z").unwrap().get(&slab).eval(&slab, &mut ns).unwrap(), 6.0);

    assert_eq!(p.parse(&mut slab,"x+y+z+a").unwrap().get(&slab).eval(&slab, &mut ns), Err(KErr::new("variable undefined")));

    assert_eq!(p.parse(&mut slab,"x+eval(x)+x").unwrap().get(&slab).eval(&slab, &mut ns).unwrap(), 3.0);
    
    assert_eq!(p.parse(&mut slab,"x+eval(x, x=10)+x").unwrap().get(&slab).eval(&slab, &mut ns).unwrap(), 12.0);

    assert_eq!(p.parse(&mut slab,"x+eval(y, x=10, y=x+1)+x").unwrap().get(&slab).eval(&slab, &mut ns).unwrap(), 13.0);

    assert_eq!(p.parse(&mut slab,"x+eval(y, y=x+1, x=10)+x").unwrap().get(&slab).eval(&slab, &mut ns).unwrap(), 4.0);

    assert_eq!(p.parse(&mut slab,"x+eval(x, x=10)+eval(x, x=20)+x").unwrap().get(&slab).eval(&slab, &mut ns).unwrap(), 32.0);

    assert_eq!(p.parse(&mut slab,"x+eval( eval(x, x=10)+eval(x, x=20), x=30 )+x").unwrap().get(&slab).eval(&slab, &mut ns).unwrap(), 62.0);
}

