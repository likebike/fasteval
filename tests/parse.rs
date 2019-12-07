use al::{Slab, parse};
use kerr::KErr;

#[test]
fn basics() {
    let mut slab = Slab::new();
    parse({slab.clear(); &mut slab.ps}, "12.34 + 43.21 + 11.11").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(12.34)), pairs: [ExprPair(EAdd, EConstant(Constant(43.21))), ExprPair(EAdd, EConstant(Constant(11.11)))] } }, vals:{}, instrs:{} }");

    parse({slab.clear(); &mut slab.ps}, "12.34 + abs ( -43 - 0.21 ) + 11.11").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(-43.0)), pairs: [ExprPair(ESub, EConstant(Constant(0.21)))] }, 1:Expression { first: EConstant(Constant(12.34)), pairs: [ExprPair(EAdd, EStdFunc(EFuncAbs(ExpressionI(0)))), ExprPair(EAdd, EConstant(Constant(11.11)))] } }, vals:{}, instrs:{} }");

    parse({slab.clear(); &mut slab.ps}, "12.34 + print ( 43.21 ) + 11.11").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(43.21)), pairs: [] }, 1:Expression { first: EConstant(Constant(12.34)), pairs: [ExprPair(EAdd, EPrintFunc(PrintFunc([EExpr(ExpressionI(0))]))), ExprPair(EAdd, EConstant(Constant(11.11)))] } }, vals:{}, instrs:{} }");

    parse({slab.clear(); &mut slab.ps}, "(-1) ^ 0.5").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(-1.0)), pairs: [] }, 1:Expression { first: EUnaryOp(EParentheses(ExpressionI(0))), pairs: [ExprPair(EExp, EConstant(Constant(0.5)))] } }, vals:{}, instrs:{} }");

}

#[test]
fn consts() {
    let mut slab = Slab::new();

    parse({slab.clear(); &mut slab.ps}, "12.34").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(12.34)), pairs: [] } }, vals:{}, instrs:{} }");
    
    parse({slab.clear(); &mut slab.ps}, ".34").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(0.34)), pairs: [] } }, vals:{}, instrs:{} }");
    
    parse({slab.clear(); &mut slab.ps}, "12.").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(12.0)), pairs: [] } }, vals:{}, instrs:{} }");

    assert_eq!(parse({slab.clear(); &mut slab.ps}, "."), Err(KErr::new("parse<f64> error")));

    assert_eq!(parse({slab.clear(); &mut slab.ps}, "12..34"), Err(KErr::new("parse<f64> error")));

    parse({slab.clear(); &mut slab.ps}, "12.34k").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(12340.0)), pairs: [] } }, vals:{}, instrs:{} }");

    parse({slab.clear(); &mut slab.ps}, "12.34K").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(12340.0)), pairs: [] } }, vals:{}, instrs:{} }");

    parse({slab.clear(); &mut slab.ps}, "12.34M").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(12340000.0)), pairs: [] } }, vals:{}, instrs:{} }");

    parse({slab.clear(); &mut slab.ps}, "12.34G").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(12340000000.0)), pairs: [] } }, vals:{}, instrs:{} }");

    parse({slab.clear(); &mut slab.ps}, "12.34T").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(12340000000000.0)), pairs: [] } }, vals:{}, instrs:{} }");

    parse({slab.clear(); &mut slab.ps}, "12.34m").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(0.01234)), pairs: [] } }, vals:{}, instrs:{} }");

    parse({slab.clear(); &mut slab.ps}, "12.34u").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(0.00001234)), pairs: [] } }, vals:{}, instrs:{} }");

    parse({slab.clear(); &mut slab.ps}, "12.34µ").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(0.00001234)), pairs: [] } }, vals:{}, instrs:{} }");

    parse({slab.clear(); &mut slab.ps}, "12.34n").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(0.00000001234)), pairs: [] } }, vals:{}, instrs:{} }");

    parse({slab.clear(); &mut slab.ps}, "12.34p").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(0.00000000001234)), pairs: [] } }, vals:{}, instrs:{} }");

    parse({slab.clear(); &mut slab.ps}, "12.34e56").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(1234000000000000000000000000000000000000000000000000000000.0)), pairs: [] } }, vals:{}, instrs:{} }");

    parse({slab.clear(); &mut slab.ps}, "12.34e+56").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(1234000000000000000000000000000000000000000000000000000000.0)), pairs: [] } }, vals:{}, instrs:{} }");

    parse({slab.clear(); &mut slab.ps}, "12.34E56").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(1234000000000000000000000000000000000000000000000000000000.0)), pairs: [] } }, vals:{}, instrs:{} }");

    parse({slab.clear(); &mut slab.ps}, "12.34E+56").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(1234000000000000000000000000000000000000000000000000000000.0)), pairs: [] } }, vals:{}, instrs:{} }");

    parse({slab.clear(); &mut slab.ps}, "12.34e-56").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(0.0000000000000000000000000000000000000000000000000000001234)), pairs: [] } }, vals:{}, instrs:{} }");

    parse({slab.clear(); &mut slab.ps}, "12.34E-56").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(0.0000000000000000000000000000000000000000000000000000001234)), pairs: [] } }, vals:{}, instrs:{} }");

    parse({slab.clear(); &mut slab.ps}, "+12.34E-56").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(0.0000000000000000000000000000000000000000000000000000001234)), pairs: [] } }, vals:{}, instrs:{} }");

    parse({slab.clear(); &mut slab.ps}, "-12.34E-56").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(-0.0000000000000000000000000000000000000000000000000000001234)), pairs: [] } }, vals:{}, instrs:{} }");

    parse({slab.clear(); &mut slab.ps}, "-x").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EUnaryOp(ENeg(ValueI(0))), pairs: [] } }, vals:{ 0:EStdFunc(EVar(VarName(`x`))) }, instrs:{} }");

    parse({slab.clear(); &mut slab.ps}, "NaN").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(NaN)), pairs: [] } }, vals:{}, instrs:{} }");

    parse({slab.clear(); &mut slab.ps}, "+NaN").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(NaN)), pairs: [] } }, vals:{}, instrs:{} }");

    parse({slab.clear(); &mut slab.ps}, "-NaN").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(NaN)), pairs: [] } }, vals:{}, instrs:{} }");

    parse({slab.clear(); &mut slab.ps}, "inf").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(inf)), pairs: [] } }, vals:{}, instrs:{} }");

    parse({slab.clear(); &mut slab.ps}, "+inf").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(inf)), pairs: [] } }, vals:{}, instrs:{} }");

    parse({slab.clear(); &mut slab.ps}, "-inf").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(-inf)), pairs: [] } }, vals:{}, instrs:{} }");



    assert_eq!(parse({slab.clear(); &mut slab.ps}, "-infK"), Err(KErr::new("unparsed tokens remaining")));
    assert_eq!(parse({slab.clear(); &mut slab.ps}, "NaNK"), Err(KErr::new("unparsed tokens remaining")));
    assert_eq!(parse({slab.clear(); &mut slab.ps}, "12.34e56K"), Err(KErr::new("unparsed tokens remaining")));

}

#[test]
#[cfg(feature="unsafe-vars")]
fn unsafe_var() {
    fn replace_addrs(mut s:String) -> String {
        let mut start=0;
        loop {
            match s[start..].find(" 0x") {
                None => break,
                Some(i) => {
                    let v = unsafe { s.as_mut_vec() };

                    start = start+i+3;
                    loop {
                        match v.get(start) {
                            None => break,
                            Some(&b) => {
                                if (b'0'<=b && b<=b'9') || (b'a'<=b && b<=b'f') {
                                    v[start]=b'?';
                                    start+=1;
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }
            };
        }
        s
    }

    let mut slab = Slab::new();

    let ua = 1.23;
    let ub = 4.56;
    unsafe {
        slab.ps.add_unsafe_var("ua".to_string(), &ua);
        slab.ps.add_unsafe_var("ub".to_string(), &ub);
    }

    parse({slab.clear(); &mut slab.ps}, "ua + ub + 5").unwrap();
    assert_eq!(replace_addrs(format!("{:?}",&slab)),
"Slab{ exprs:{ 0:Expression { first: EStdFunc(EUnsafeVar { name: VarName(`ua`), ptr: 0x???????????? }), pairs: [ExprPair(EAdd, EStdFunc(EUnsafeVar { name: VarName(`ub`), ptr: 0x???????????? })), ExprPair(EAdd, EConstant(Constant(5.0)))] } }, vals:{}, instrs:{} }");
}

