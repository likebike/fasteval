use al::{Parser, Compiler, Evaler, Slab, EvalNS, ExpressionI, InstructionI};
use al::compiler::Instruction::{IConst};
use kerr::KErr;

#[test]
fn slab_overflow() {
    let p = Parser::new(None,None);
    let mut slab = Slab::with_capacity(2);
    assert_eq!(p.parse({slab.clear(); &mut slab.ps}, "1 + 2 + -3 + ( +4 )"), Ok(ExpressionI(1)));
    assert_eq!(format!("{:?}", slab),
"Slab{ exprs:{ 0:Expression { first: EUnaryOp(EPos(ValueI(1))), pairs: [] }, 1:Expression { first: EConstant(Constant(1.0)), pairs: [ExprPair(EPlus, EConstant(Constant(2.0))), ExprPair(EPlus, EUnaryOp(ENeg(ValueI(0)))), ExprPair(EPlus, EUnaryOp(EParens(ExpressionI(0))))] } }, vals:{ 0:EConstant(Constant(3.0)), 1:EConstant(Constant(4.0)) }, instrs:{} }");

    assert_eq!(p.parse({slab.clear(); &mut slab.ps}, "1 + 2 + -3 + ( ++4 )"), Err(KErr::new("slab val overflow")));
}

#[test]
fn basics() {
    let p = Parser::new(None,None);
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(|_| None);

    let expr_i = p.parse({slab.clear(); &mut slab.ps}, "3*3-3/3+1").unwrap();
    let expr_ref = slab.ps.get_expr(expr_i);
    let instr = expr_ref.compile(&slab.ps, &mut slab.cs);
    assert_eq!(instr, IConst(9.0));
    assert_eq!(format!("{:?}", slab), 
"Slab{ exprs:{ 0:Expression { first: EConstant(Constant(3.0)), pairs: [ExprPair(EMul, EConstant(Constant(3.0))), ExprPair(EMinus, EConstant(Constant(3.0))), ExprPair(EDiv, EConstant(Constant(3.0))), ExprPair(EPlus, EConstant(Constant(1.0)))] } }, vals:{}, instrs:{} }");
    assert_eq!(instr.eval(&slab, &mut ns), Ok(9.0));
    assert_eq!(instr.eval(&slab, &mut ns), Ok(9.0));
}

