use algebra::slab::Slab;
use algebra::parser::Parser;

#[test]
fn parser() {
    let p = Parser{
        is_const_byte:None,
        is_var_byte:None,
    };
    let mut slab = Slab::new();
    p.parse({slab.clear(); &slab}, "12.34 + 43.21 + 11.11").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(12.34)), pairs: SVec8[ ExprPair(EPlus, EConstant(Constant(43.21))), ExprPair(EPlus, EConstant(Constant(11.11))) ] } }, vals:{} }");

    p.parse({slab.clear(); &slab}, "12.34 + abs ( -43 - 0.21 ) + 11.11").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EUnaryOp(ENeg(ValueI(0))), pairs: SVec8[ ExprPair(EMinus, EConstant(Constant(0.21))) ] }, 1:Expression { first: EConstant(Constant(12.34)), pairs: SVec8[ ExprPair(EPlus, ECallable(EFunc(EFuncAbs(ExpressionI(0))))), ExprPair(EPlus, EConstant(Constant(11.11))) ] } }, vals:{ 0:EConstant(Constant(43.0)) } }");

    p.parse({slab.clear(); &slab}, "12.34 + print ( 43.21 ) + 11.11").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(43.21)), pairs: SVec8[] }, 1:Expression { first: EConstant(Constant(12.34)), pairs: SVec8[ ExprPair(EPlus, ECallable(EPrintFunc(PrintFunc(SVec8[ EExpr(ExpressionI(0)) ])))), ExprPair(EPlus, EConstant(Constant(11.11))) ] } }, vals:{} }");

    p.parse({slab.clear(); &slab}, "12.34 + eval ( x - y , x = 5 , y=4 ) + 11.11").unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EVariable(Variable(`x`)), pairs: SVec8[ ExprPair(EMinus, EVariable(Variable(`y`))) ] }, 1:Expression { first: EConstant(Constant(5.0)), pairs: SVec8[] }, 2:Expression { first: EConstant(Constant(4.0)), pairs: SVec8[] }, 3:Expression { first: EConstant(Constant(12.34)), pairs: SVec8[ ExprPair(EPlus, ECallable(EEvalFunc(EvalFunc { expr: ExpressionI(0), kwargs: SVec16[ KWArg { name: Variable(`x`), expr: ExpressionI(1) }, KWArg { name: Variable(`y`), expr: ExpressionI(2) } ] }))), ExprPair(EPlus, EConstant(Constant(11.11))) ] } }, vals:{} }");
}

