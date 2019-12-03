use al::{Slab, Parser};
use kerr::KErr;

#[test]
fn basics() {
    let mut p = Parser::new();
    let mut slab = Slab::new();
    p.parse({slab.clear(); &mut slab.ps}, "12.34 + 43.21 + 11.11").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(12.34)), pairs: [ExprPair(EAdd, EConstant(Constant(43.21))), ExprPair(EAdd, EConstant(Constant(11.11)))] } }, vals:{}, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "12.34 + abs ( -43 - 0.21 ) + 11.11").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(-43.0)), pairs: [ExprPair(ESub, EConstant(Constant(0.21)))] }, 1:Expression { first: EConstant(Constant(12.34)), pairs: [ExprPair(EAdd, ECallable(EStdFunc(EFuncAbs(ExpressionI(0))))), ExprPair(EAdd, EConstant(Constant(11.11)))] } }, vals:{}, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "12.34 + print ( 43.21 ) + 11.11").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(43.21)), pairs: [] }, 1:Expression { first: EConstant(Constant(12.34)), pairs: [ExprPair(EAdd, ECallable(EPrintFunc(PrintFunc([EExpr(ExpressionI(0))])))), ExprPair(EAdd, EConstant(Constant(11.11)))] } }, vals:{}, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "12.34 + eval ( x - y , x = 5 , y=4 ) + 11.11").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: ECallable(EStdFunc(EVar(VarName(`x`)))), pairs: [ExprPair(ESub, ECallable(EStdFunc(EVar(VarName(`y`)))))] }, 1:Expression { first: EConstant(Constant(5.0)), pairs: [] }, 2:Expression { first: EConstant(Constant(4.0)), pairs: [] }, 3:Expression { first: EConstant(Constant(12.34)), pairs: [ExprPair(EAdd, ECallable(EEvalFunc(EvalFunc { expr: ExpressionI(0), kwargs: [KWArg { name: VarName(`x`), expr: ExpressionI(1) }, KWArg { name: VarName(`y`), expr: ExpressionI(2) }] }))), ExprPair(EAdd, EConstant(Constant(11.11)))] } }, vals:{}, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "(-1) ^ 0.5").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(-1.0)), pairs: [] }, 1:Expression { first: EUnaryOp(EParentheses(ExpressionI(0))), pairs: [ExprPair(EExp, EConstant(Constant(0.5)))] } }, vals:{}, instrs:{} }");

}

#[test]
fn consts() {
    let mut p = Parser::new();
    let mut slab = Slab::new();

    p.parse({slab.clear(); &mut slab.ps}, "12.34").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(12.34)), pairs: [] } }, vals:{}, instrs:{} }");
    
    p.parse({slab.clear(); &mut slab.ps}, ".34").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(0.34)), pairs: [] } }, vals:{}, instrs:{} }");
    
    p.parse({slab.clear(); &mut slab.ps}, "12.").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(12.0)), pairs: [] } }, vals:{}, instrs:{} }");

    assert_eq!(p.parse({slab.clear(); &mut slab.ps}, "."), Err(KErr::new("parse<f64> error")));

    assert_eq!(p.parse({slab.clear(); &mut slab.ps}, "12..34"), Err(KErr::new("parse<f64> error")));

    p.parse({slab.clear(); &mut slab.ps}, "12.34k").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(12340.0)), pairs: [] } }, vals:{}, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "12.34K").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(12340.0)), pairs: [] } }, vals:{}, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "12.34M").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(12340000.0)), pairs: [] } }, vals:{}, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "12.34G").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(12340000000.0)), pairs: [] } }, vals:{}, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "12.34T").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(12340000000000.0)), pairs: [] } }, vals:{}, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "12.34m").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(0.01234)), pairs: [] } }, vals:{}, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "12.34u").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(0.00001234)), pairs: [] } }, vals:{}, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "12.34µ").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(0.00001234)), pairs: [] } }, vals:{}, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "12.34n").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(0.00000001234)), pairs: [] } }, vals:{}, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "12.34p").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(0.00000000001234)), pairs: [] } }, vals:{}, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "12.34e56").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(1234000000000000000000000000000000000000000000000000000000.0)), pairs: [] } }, vals:{}, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "12.34e+56").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(1234000000000000000000000000000000000000000000000000000000.0)), pairs: [] } }, vals:{}, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "12.34E56").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(1234000000000000000000000000000000000000000000000000000000.0)), pairs: [] } }, vals:{}, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "12.34E+56").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(1234000000000000000000000000000000000000000000000000000000.0)), pairs: [] } }, vals:{}, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "12.34e-56").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(0.0000000000000000000000000000000000000000000000000000001234)), pairs: [] } }, vals:{}, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "12.34E-56").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(0.0000000000000000000000000000000000000000000000000000001234)), pairs: [] } }, vals:{}, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "+12.34E-56").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(0.0000000000000000000000000000000000000000000000000000001234)), pairs: [] } }, vals:{}, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "-12.34E-56").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(-0.0000000000000000000000000000000000000000000000000000001234)), pairs: [] } }, vals:{}, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "-x").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EUnaryOp(ENeg(ValueI(0))), pairs: [] } }, vals:{ 0:ECallable(EStdFunc(EVar(VarName(`x`)))) }, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "NaN").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(NaN)), pairs: [] } }, vals:{}, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "+NaN").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(NaN)), pairs: [] } }, vals:{}, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "-NaN").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(NaN)), pairs: [] } }, vals:{}, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "inf").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(inf)), pairs: [] } }, vals:{}, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "+inf").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(inf)), pairs: [] } }, vals:{}, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "-inf").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(-inf)), pairs: [] } }, vals:{}, instrs:{} }");



    assert_eq!(p.parse({slab.clear(); &mut slab.ps}, "-infK"), Err(KErr::new("unparsed tokens remaining")));
    assert_eq!(p.parse({slab.clear(); &mut slab.ps}, "NaNK"), Err(KErr::new("unparsed tokens remaining")));
    assert_eq!(p.parse({slab.clear(); &mut slab.ps}, "12.34e56K"), Err(KErr::new("unparsed tokens remaining")));

}

