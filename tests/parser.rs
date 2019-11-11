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
               "
               Ok(&Expression{
                    first:EConstant(Constant(12.34)),
                    pairs:vec![ExprPair(EPlus, EConstant(Constant(43.21))),
                               ExprPair(EPlus, EConstant(Constant(11.11)))].into()})");

    p.parse({slab.clear(); &slab}, "12.34 + abs ( -43 - 0.21 ) + 11.11").unwrap();
    assert_eq!(format!("{:?}",&slab),
               "Ok(Expression {
                    first:EConstant(Constant(12.34)),
                    pairs:Box::new([
                        ExprPair(EPlus, ECallable(EFunc(EFuncAbs(Box::new(Expression {
                            first:EUnaryOp(ENeg(Box::new(EConstant(Constant(43.0))))),
                            pairs:vec![ExprPair(EMinus, EConstant(Constant(0.21)))].into() }))))),
                        ExprPair(EPlus, EConstant(Constant(11.11)))]) })");

    p.parse({slab.clear(); &slab}, "12.34 + print ( 43.21 ) + 11.11").unwrap();
    assert_eq!(format!("{:?}",&slab),
               "Ok(Expression {
                    first:EConstant(Constant(12.34)),
                    pairs:Box::new([
                        ExprPair(EPlus, ECallable(EPrintFunc(PrintFunc(Box::new([
                            EExpr(Box::new(Expression {
                                first:EConstant(Constant(43.21)),
                                pairs:Box::new([]) }))]))))),
                        ExprPair(EPlus, EConstant(Constant(11.11)))]) })");

    p.parse({slab.clear(); &slab}, "12.34 + eval ( x - y , x = 5 , y=4 ) + 11.11").unwrap();
    assert_eq!(format!("{:?}",&slab),
               r#"Ok(Expression {
                    first:EConstant(Constant(12.34)),
                    pairs:Box::new([
                        ExprPair(EPlus, ECallable(EEvalFunc(EvalFunc {
                            expr:Box::new(Expression {
                                first:EVariable(Variable("x".to_string())),
                                pairs:Box::new([ExprPair(EMinus, EVariable(Variable("y".to_string())))]) }),
                            kwargs:Box::new([
                                KWArg { name: Variable("x".to_string()), expr:Box::new(Expression { first: EConstant(Constant(5.0)), pairs:Box::new([]) }) },
                                KWArg { name: Variable("y".to_string()), expr:Box::new(Expression { first: EConstant(Constant(4.0)), pairs:Box::new([]) }) }]) }))),
                        ExprPair(EPlus, EConstant(Constant(11.11)))]) })"#);
}

