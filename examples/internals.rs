// usage:  cargo run --release --example internals

use fasteval::Compiler;  // use this trait so we can call compile().
fn main() -> Result<(), fasteval::Error> {
    let parser = fasteval::Parser::new();
    let mut slab = fasteval::Slab::new();

    let expr_str = "sin(deg/360 * 2*pi())";
    let expr_ref = parser.parse(expr_str, &mut slab.ps)?.from(&slab.ps);

    // Let's take a look at the parsed AST inside the Slab:
    // If you find this structure confusing, take a look at the compilation
    // AST below because it is simpler.
    assert_eq!(format!("{:?}", slab.ps),
               r#"ParseSlab{ exprs:{ 0:Expression { first: EStdFunc(EVar("deg")), pairs: [ExprPair(EDiv, EConstant(360.0)), ExprPair(EMul, EConstant(2.0)), ExprPair(EMul, EStdFunc(EFuncPi))] }, 1:Expression { first: EStdFunc(EFuncSin(ExpressionI(0))), pairs: [] } }, vals:{} }"#);
               // Pretty-Print:
               // ParseSlab{
               //     exprs:{
               //         0:Expression { first: EStdFunc(EVar("deg")),
               //                        pairs: [ExprPair(EDiv, EConstant(360.0)),
               //                                ExprPair(EMul, EConstant(2.0)),
               //                                ExprPair(EMul, EStdFunc(EFuncPi))]
               //                      },
               //         1:Expression { first: EStdFunc(EFuncSin(ExpressionI(0))),
               //                        pairs: [] }
               //                      },
               //     vals:{}
               // }

    let compiled = expr_ref.compile(&slab.ps, &mut slab.cs);

    // Let's take a look at the compilation results and the AST inside the Slab:
    // Notice that compilation has performed constant-folding: 1/360 * 2*pi = 0.017453292519943295
    // In the results below: IFuncSin(...) represents the sin function.
    //                       InstructionI(1) represents the Instruction stored at index 1.
    //                       IMul(...) represents the multiplication operator.
    //                       'C(0.017...)' represents a constant value of 0.017... .
    //                       IVar("deg") represents a variable named "deg".
    assert_eq!(format!("{:?}", compiled),
               "IFuncSin(InstructionI(1))");
    assert_eq!(format!("{:?}", slab.cs),
               r#"CompileSlab{ instrs:{ 0:IVar("deg"), 1:IMul(InstructionI(0), C(0.017453292519943295)) } }"#);

    Ok(())
}
