use fasteval::{Error, Slab, Parser};

#[test]
fn basics() {
    let mut slab = Slab::new();
    Parser::new().parse("12.34 + 43.21 + 11.11", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(12.34), pairs: [ExprPair(EAdd, EConstant(43.21)), ExprPair(EAdd, EConstant(11.11))] } }, vals:{}, instrs:{} }");

    Parser::new().parse("12.34 + abs ( -43 - 0.21 ) + 11.11", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(-43.0), pairs: [ExprPair(ESub, EConstant(0.21))] }, 1:Expression { first: EConstant(12.34), pairs: [ExprPair(EAdd, EStdFunc(EFuncAbs(ExpressionI(0)))), ExprPair(EAdd, EConstant(11.11))] } }, vals:{}, instrs:{} }");

    Parser::new().parse("12.34 + abs [ -43 - 0.21 ] + 11.11", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(-43.0), pairs: [ExprPair(ESub, EConstant(0.21))] }, 1:Expression { first: EConstant(12.34), pairs: [ExprPair(EAdd, EStdFunc(EFuncAbs(ExpressionI(0)))), ExprPair(EAdd, EConstant(11.11))] } }, vals:{}, instrs:{} }");

    Parser::new().parse("12.34 + print ( 43.21 ) + 11.11", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(43.21), pairs: [] }, 1:Expression { first: EConstant(12.34), pairs: [ExprPair(EAdd, EPrintFunc(PrintFunc([EExpr(ExpressionI(0))]))), ExprPair(EAdd, EConstant(11.11))] } }, vals:{}, instrs:{} }");

    Parser::new().parse("12.34 + print [ 43.21 ] + 11.11", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(43.21), pairs: [] }, 1:Expression { first: EConstant(12.34), pairs: [ExprPair(EAdd, EPrintFunc(PrintFunc([EExpr(ExpressionI(0))]))), ExprPair(EAdd, EConstant(11.11))] } }, vals:{}, instrs:{} }");

    Parser::new().parse("(-1) ^ 0.5", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(-1.0), pairs: [] }, 1:Expression { first: EUnaryOp(EParentheses(ExpressionI(0))), pairs: [ExprPair(EExp, EConstant(0.5))] } }, vals:{}, instrs:{} }");

    Parser::new().parse("(1 + [2 - (3 * 4) / 5] ^ 6) % 7", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(3.0), pairs: [ExprPair(EMul, EConstant(4.0))] }, 1:Expression { first: EConstant(2.0), pairs: [ExprPair(ESub, EUnaryOp(EParentheses(ExpressionI(0)))), ExprPair(EDiv, EConstant(5.0))] }, 2:Expression { first: EConstant(1.0), pairs: [ExprPair(EAdd, EUnaryOp(EParentheses(ExpressionI(1)))), ExprPair(EExp, EConstant(6.0))] }, 3:Expression { first: EUnaryOp(EParentheses(ExpressionI(2))), pairs: [ExprPair(EMod, EConstant(7.0))] } }, vals:{}, instrs:{} }");

}

#[test]
fn consts() {
    let mut slab = Slab::new();

    Parser::new().parse("12.34", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(12.34), pairs: [] } }, vals:{}, instrs:{} }");

    Parser::new().parse(".34", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(0.34), pairs: [] } }, vals:{}, instrs:{} }");

    Parser::new().parse("12.", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(12.0), pairs: [] } }, vals:{}, instrs:{} }");

    assert_eq!(Parser::new().parse(".", &mut slab.ps), Err(Error::ParseF64(".".to_string())));

    assert_eq!(Parser::new().parse("12..34", &mut slab.ps), Err(Error::ParseF64("12..34".to_string())));

    Parser::new().parse("12.34k", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(12340.0), pairs: [] } }, vals:{}, instrs:{} }");

    Parser::new().parse("12.34K", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(12340.0), pairs: [] } }, vals:{}, instrs:{} }");

    Parser::new().parse("12.34M", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(12340000.0), pairs: [] } }, vals:{}, instrs:{} }");

    Parser::new().parse("12.34G", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(12340000000.0), pairs: [] } }, vals:{}, instrs:{} }");

    Parser::new().parse("12.34T", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(12340000000000.0), pairs: [] } }, vals:{}, instrs:{} }");

    Parser::new().parse("12.34m", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(0.01234), pairs: [] } }, vals:{}, instrs:{} }");

    Parser::new().parse("12.34u", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(0.00001234), pairs: [] } }, vals:{}, instrs:{} }");

    Parser::new().parse("12.34µ", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(0.00001234), pairs: [] } }, vals:{}, instrs:{} }");

    Parser::new().parse("12.34n", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(0.00000001234), pairs: [] } }, vals:{}, instrs:{} }");

    Parser::new().parse("12.34p", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(0.00000000001234), pairs: [] } }, vals:{}, instrs:{} }");

    Parser::new().parse("12.34e56", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(1234000000000000000000000000000000000000000000000000000000.0), pairs: [] } }, vals:{}, instrs:{} }");

    Parser::new().parse("12.34e+56", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(1234000000000000000000000000000000000000000000000000000000.0), pairs: [] } }, vals:{}, instrs:{} }");

    Parser::new().parse("12.34E56", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(1234000000000000000000000000000000000000000000000000000000.0), pairs: [] } }, vals:{}, instrs:{} }");

    Parser::new().parse("12.34E+56", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(1234000000000000000000000000000000000000000000000000000000.0), pairs: [] } }, vals:{}, instrs:{} }");

    Parser::new().parse("12.34e-56", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(0.0000000000000000000000000000000000000000000000000000001234), pairs: [] } }, vals:{}, instrs:{} }");

    Parser::new().parse("12.34E-56", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(0.0000000000000000000000000000000000000000000000000000001234), pairs: [] } }, vals:{}, instrs:{} }");

    Parser::new().parse("+12.34E-56", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(0.0000000000000000000000000000000000000000000000000000001234), pairs: [] } }, vals:{}, instrs:{} }");

    Parser::new().parse("-12.34E-56", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(-0.0000000000000000000000000000000000000000000000000000001234), pairs: [] } }, vals:{}, instrs:{} }");

    Parser::new().parse("-x", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EUnaryOp(ENeg(ValueI(0))), pairs: [] } }, vals:{ 0:EStdFunc(EVar(\"x\")) }, instrs:{} }");

    Parser::new().parse("NaN", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(NaN), pairs: [] } }, vals:{}, instrs:{} }");

    Parser::new().parse("+NaN", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(NaN), pairs: [] } }, vals:{}, instrs:{} }");

    Parser::new().parse("-NaN", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(NaN), pairs: [] } }, vals:{}, instrs:{} }");

    Parser::new().parse("inf", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(inf), pairs: [] } }, vals:{}, instrs:{} }");

    Parser::new().parse("+inf", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(inf), pairs: [] } }, vals:{}, instrs:{} }");

    Parser::new().parse("-inf", &mut slab.ps).unwrap();
    assert_eq!(format!("{:?}",&slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(-inf), pairs: [] } }, vals:{}, instrs:{} }");



    assert_eq!(Parser::new().parse("-infK", &mut slab.ps), Err(Error::UnparsedTokensRemaining("K".to_string())));
    assert_eq!(Parser::new().parse("NaNK", &mut slab.ps), Err(Error::UnparsedTokensRemaining("K".to_string())));
    assert_eq!(Parser::new().parse("12.34e56K", &mut slab.ps), Err(Error::UnparsedTokensRemaining("K".to_string())));

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

    Parser::new().parse("ua + ub + 5", &mut slab.ps).unwrap();
    assert_eq!(replace_addrs(format!("{:?}",&slab)),
"Slab{ exprs:{ 0:Expression { first: EStdFunc(EUnsafeVar { name: \"ua\", ptr: 0x???????????? }), pairs: [ExprPair(EAdd, EStdFunc(EUnsafeVar { name: \"ub\", ptr: 0x???????????? })), ExprPair(EAdd, EConstant(5.0))] } }, vals:{}, instrs:{} }");
}

