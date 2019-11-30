use al::{Slab, Parser};

#[test]
fn basics() {
    let mut p = Parser::new();
    let mut slab = Slab::new();
    p.parse({slab.clear(); &mut slab.ps}, "12.34 + 43.21 + 11.11").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(12.34)), pairs: [ExprPair(EAdd, EConstant(Constant(43.21))), ExprPair(EAdd, EConstant(Constant(11.11)))] } }, vals:{}, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "12.34 + abs ( -43 - 0.21 ) + 11.11").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EUnaryOp(ENeg(ValueI(0))), pairs: [ExprPair(ESub, EConstant(Constant(0.21)))] }, 1:Expression { first: EConstant(Constant(12.34)), pairs: [ExprPair(EAdd, ECallable(EFunc(EFuncAbs(ExpressionI(0))))), ExprPair(EAdd, EConstant(Constant(11.11)))] } }, vals:{ 0:EConstant(Constant(43.0)) }, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "12.34 + print ( 43.21 ) + 11.11").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(43.21)), pairs: [] }, 1:Expression { first: EConstant(Constant(12.34)), pairs: [ExprPair(EAdd, ECallable(EPrintFunc(PrintFunc([EExpr(ExpressionI(0))])))), ExprPair(EAdd, EConstant(Constant(11.11)))] } }, vals:{}, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "12.34 + eval ( x - y , x = 5 , y=4 ) + 11.11").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EVariable(Variable(`x`)), pairs: [ExprPair(ESub, EVariable(Variable(`y`)))] }, 1:Expression { first: EConstant(Constant(5.0)), pairs: [] }, 2:Expression { first: EConstant(Constant(4.0)), pairs: [] }, 3:Expression { first: EConstant(Constant(12.34)), pairs: [ExprPair(EAdd, ECallable(EEvalFunc(EvalFunc { expr: ExpressionI(0), kwargs: [KWArg { name: Variable(`x`), expr: ExpressionI(1) }, KWArg { name: Variable(`y`), expr: ExpressionI(2) }] }))), ExprPair(EAdd, EConstant(Constant(11.11)))] } }, vals:{}, instrs:{} }");

    p.parse({slab.clear(); &mut slab.ps}, "(-1) ^ 0.5").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EUnaryOp(ENeg(ValueI(0))), pairs: [] }, 1:Expression { first: EUnaryOp(EParentheses(ExpressionI(0))), pairs: [ExprPair(EExp, EConstant(Constant(0.5)))] } }, vals:{ 0:EConstant(Constant(1.0)) }, instrs:{} }");

}

