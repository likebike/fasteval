use algebra::slab::Slab;

#[test]
fn parser() {
    let p = Parser{
        is_const_byte:None,
        is_var_byte:None,
    };
    assert!(p.call_is_var_byte(Some(b'a'),0));
    assert!(!p.call_is_const_byte(Some(b'a'),0));

    let p = Parser{
        is_const_byte:Some(&|_:u8, _:usize| true),
        is_var_byte:None,
    };
    assert!(p.call_is_const_byte(Some(b'a'),0));

    let p = Parser{
        is_const_byte:None,
        is_var_byte:None,
    };
    
    {
        let bsarr = b"12.34";
        let bs = &mut &bsarr[..];
        assert_eq!(p.read_value(bs), Ok(EConstant(Constant(12.34))));
    }

    let mut slab : Slab;
    assert_eq!(p.parse({slab=Slab::new(); &slab}, "12.34 + 43.21 + 11.11"),
               Ok(Expression{
                    first:EConstant(Constant(12.34)),
                    pairs:Box::new([
                        ExprPair(EPlus, EConstant(Constant(43.21))),
                        ExprPair(EPlus, EConstant(Constant(11.11)))])}));

    assert_eq!(p.parse({slab=Slab::new(); &slab}, "12.34 + abs ( -43 - 0.21 ) + 11.11"),
               Ok(Expression {
                    first:EConstant(Constant(12.34)),
                    pairs:Box::new([
                        ExprPair(EPlus, ECallable(EFunc(EFuncAbs(Box::new(Expression {
                            first:EUnaryOp(ENeg(Box::new(EConstant(Constant(43.0))))),
                            pairs:Box::new([ExprPair(EMinus, EConstant(Constant(0.21)))]) }))))),
                        ExprPair(EPlus, EConstant(Constant(11.11)))]) }));

    assert_eq!(p.parse({slab=Slab::new(); &slab}, "12.34 + print ( 43.21 ) + 11.11"),
               Ok(Expression {
                    first:EConstant(Constant(12.34)),
                    pairs:Box::new([
                        ExprPair(EPlus, ECallable(EPrintFunc(PrintFunc(Box::new([
                            EExpr(Box::new(Expression {
                                first:EConstant(Constant(43.21)),
                                pairs:Box::new([]) }))]))))),
                        ExprPair(EPlus, EConstant(Constant(11.11)))]) }));

    assert_eq!(p.parse({slab=Slab::new(); &slab}, "12.34 + eval ( x - y , x = 5 , y=4 ) + 11.11"),
               Ok(Expression {
                    first:EConstant(Constant(12.34)),
                    pairs:Box::new([
                        ExprPair(EPlus, ECallable(EEvalFunc(EvalFunc {
                            expr:Box::new(Expression {
                                first:EVariable(Variable("x".to_string())),
                                pairs:Box::new([ExprPair(EMinus, EVariable(Variable("y".to_string())))]) }),
                            kwargs:Box::new([
                                KWArg { name: Variable("x".to_string()), expr:Box::new(Expression { first: EConstant(Constant(5.0)), pairs:Box::new([]) }) },
                                KWArg { name: Variable("y".to_string()), expr:Box::new(Expression { first: EConstant(Constant(4.0)), pairs:Box::new([]) }) }]) }))),
                        ExprPair(EPlus, EConstant(Constant(11.11)))]) }));
}

