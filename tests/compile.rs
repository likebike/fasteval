use al::{Parser, Compiler, Evaler, Slab, EvalNS, ExpressionI, InstructionI, Variable};
use al::compiler::Instruction::{self, IConst, IVar, INeg, INot, IInv, IAdd, IMul, IMod, IExp, ILT, ILTE, IEQ, INE, IAND, IOR, IFuncInt, IFuncCeil, IFuncFloor, IFuncAbs, IFuncLog, IFuncRound};
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


fn comp(expr_str:&str) -> (Slab, Instruction) {
    let p = Parser::new(None,None);
    let mut slab = Slab::new();
    let instr = p.parse(&mut slab.ps, expr_str).unwrap().from(&slab.ps).compile(&slab.ps, &mut slab.cs);
    (slab, instr)
}

fn comp_chk(expr_str:&str, expect_instr:Instruction, expect_fmt:&str, expect_eval:f64) {
    let p = Parser::new(None,None);
    let mut slab = Slab::new();
    let expr = p.parse(&mut slab.ps, expr_str).unwrap().from(&slab.ps);
    let instr = expr.compile(&slab.ps, &mut slab.cs);

    assert_eq!(instr, expect_instr);
    assert_eq!(format!("{:?}",slab.cs), expect_fmt);
    let mut ns = EvalNS::new(|n| {
        match n {
            "w" => Some(0.0),
            "x" => Some(1.0),
            "y" => Some(2.0),
            "y7" => Some(2.7),
            "z" => Some(3.0),
            _ => None,
        }
    });
    assert_eq!(instr.eval(&slab, &mut ns).unwrap(), expect_eval);

    // Make sure Instruction eval matches normal eval:
    assert_eq!(instr.eval(&slab, &mut ns).unwrap(), expr.eval(&slab, &mut ns).unwrap());
}

#[test]
fn double_neg() {
    assert_eq!(comp("1+1.5").1, IConst(2.5));
    assert_eq!(comp("-1.5").1, IConst(-1.5));
    assert_eq!(comp("--1.5").1, IConst(1.5));
    assert_eq!(comp("1 + -1.5").1, IConst(-0.5));
    assert_eq!(comp("1 + --1.5").1, IConst(2.5));
    assert_eq!(comp("1 + ----1.5").1, IConst(2.5));
    assert_eq!(comp("1 - ----1.5").1, IConst(-0.5));

    assert_eq!(comp("x").1, IVar(Variable("x".to_string())));

    comp_chk("1-1", IConst(0.0), "CompileSlab { instrs: [] }", 0.0);
    comp_chk("1 + x", IAdd(InstructionI(0), InstructionI(1)), "CompileSlab { instrs: [IVar(Variable(`x`)), IConst(1.0)] }", 2.0);
    comp_chk("x + 1", IAdd(InstructionI(0), InstructionI(1)), "CompileSlab { instrs: [IVar(Variable(`x`)), IConst(1.0)] }", 2.0);
    comp_chk("0.5 + x + 0.5", IAdd(InstructionI(0), InstructionI(1)), "CompileSlab { instrs: [IVar(Variable(`x`)), IConst(1.0)] }", 2.0);
    comp_chk("0.5 - x - 0.5", INeg(InstructionI(0)), "CompileSlab { instrs: [IVar(Variable(`x`))] }", -1.0);
    comp_chk("0.5 - -x - 0.5", IVar(Variable("x".to_string())), "CompileSlab { instrs: [] }", 1.0);
    comp_chk("0.5 - --x - 1.5", IAdd(InstructionI(1), InstructionI(2)), "CompileSlab { instrs: [IVar(Variable(`x`)), INeg(InstructionI(0)), IConst(-1.0)] }", -2.0);
    comp_chk("0.5 - ---x - 1.5", IAdd(InstructionI(0), InstructionI(1)), "CompileSlab { instrs: [IVar(Variable(`x`)), IConst(-1.0)] }", 0.0);
    comp_chk("0.5 - (---x) - 1.5", IAdd(InstructionI(0), InstructionI(1)), "CompileSlab { instrs: [IVar(Variable(`x`)), IConst(-1.0)] }", 0.0);
    comp_chk("0.5 - -(--x) - 1.5", IAdd(InstructionI(0), InstructionI(1)), "CompileSlab { instrs: [IVar(Variable(`x`)), IConst(-1.0)] }", 0.0);
    comp_chk("0.5 - --(-x) - 1.5", IAdd(InstructionI(0), InstructionI(1)), "CompileSlab { instrs: [IVar(Variable(`x`)), IConst(-1.0)] }", 0.0);
    comp_chk("0.5 - --(-x - 1.5)", IAdd(InstructionI(4), InstructionI(5)), "CompileSlab { instrs: [IVar(Variable(`x`)), INeg(InstructionI(0)), IConst(-1.5), IAdd(InstructionI(1), InstructionI(2)), INeg(InstructionI(3)), IConst(0.5)] }", 3.0);
    comp_chk("0.5 - --((((-(x)) - 1.5)))", IAdd(InstructionI(4), InstructionI(5)), "CompileSlab { instrs: [IVar(Variable(`x`)), INeg(InstructionI(0)), IConst(-1.5), IAdd(InstructionI(1), InstructionI(2)), INeg(InstructionI(3)), IConst(0.5)] }", 3.0);
    comp_chk("0.5 - -(-(--((((-(x)) - 1.5)))))", IAdd(InstructionI(4), InstructionI(5)), "CompileSlab { instrs: [IVar(Variable(`x`)), INeg(InstructionI(0)), IConst(-1.5), IAdd(InstructionI(1), InstructionI(2)), INeg(InstructionI(3)), IConst(0.5)] }", 3.0);
}

#[test]
fn all_instrs() {
    // IConst:
    comp_chk("1", IConst(1.0), "CompileSlab { instrs: [] }", 1.0);
    comp_chk("-1", IConst(-1.0), "CompileSlab { instrs: [] }", -1.0);

    // IVar:
    comp_chk("x", IVar(Variable("x".to_string())), "CompileSlab { instrs: [] }", 1.0);

    // INeg:
    comp_chk("-x", INeg(InstructionI(0)), "CompileSlab { instrs: [IVar(Variable(`x`))] }", -1.0);

    // INot:
    comp_chk("!x", INot(InstructionI(0)), "CompileSlab { instrs: [IVar(Variable(`x`))] }", 0.0);

    // IInv:
    comp_chk("1/x", IInv(InstructionI(0)), "CompileSlab { instrs: [IVar(Variable(`x`))] }", 1.0);
    
    // IAdd:
    comp_chk("1 + x", IAdd(InstructionI(0), InstructionI(1)), "CompileSlab { instrs: [IVar(Variable(`x`)), IConst(1.0)] }", 2.0);
    comp_chk("1 - x", IAdd(InstructionI(1), InstructionI(2)), "CompileSlab { instrs: [IVar(Variable(`x`)), INeg(InstructionI(0)), IConst(1.0)] }", 0.0);

    // IMul:
    comp_chk("2 * x", IMul(InstructionI(0), InstructionI(1)), "CompileSlab { instrs: [IVar(Variable(`x`)), IConst(2.0)] }", 2.0);
    comp_chk("x * 2", IMul(InstructionI(0), InstructionI(1)), "CompileSlab { instrs: [IVar(Variable(`x`)), IConst(2.0)] }", 2.0);
    comp_chk("x / 2", IMul(InstructionI(0), InstructionI(1)), "CompileSlab { instrs: [IVar(Variable(`x`)), IConst(0.5)] }", 0.5);

    // IMod:
    comp_chk("8 % 3", IConst(2.0), "CompileSlab { instrs: [] }", 2.0);
    comp_chk("8 % z", IMod { dividend: InstructionI(0), divisor: InstructionI(1) }, "CompileSlab { instrs: [IConst(8.0), IVar(Variable(`z`))] }", 2.0);
    comp_chk("-8 % 3", IConst(-2.0), "CompileSlab { instrs: [] }", -2.0);
    comp_chk("8 % -3", IConst(2.0), "CompileSlab { instrs: [] }", 2.0);
    comp_chk("-8 % z", IMod { dividend: InstructionI(0), divisor: InstructionI(1) }, "CompileSlab { instrs: [IConst(-8.0), IVar(Variable(`z`))] }", -2.0);
    comp_chk("8 % -z", IMod { dividend: InstructionI(1), divisor: InstructionI(2) }, "CompileSlab { instrs: [IVar(Variable(`z`)), IConst(8.0), INeg(InstructionI(0))] }", 2.0);
    comp_chk("8 % 3 % 2", IConst(0.0), "CompileSlab { instrs: [] }", 0.0);
    comp_chk("8 % z % 2", IMod { dividend: InstructionI(2), divisor: InstructionI(3) }, "CompileSlab { instrs: [IConst(8.0), IVar(Variable(`z`)), IMod { dividend: InstructionI(0), divisor: InstructionI(1) }, IConst(2.0)] }", 0.0);

    // IExp:
    comp_chk("2 ^ 3", IConst(8.0), "CompileSlab { instrs: [] }", 8.0);
    comp_chk("2 ^ z", IExp { base: InstructionI(0), power: InstructionI(1) }, "CompileSlab { instrs: [IConst(2.0), IVar(Variable(`z`))] }", 8.0);
    comp_chk("4 ^ 0.5", IConst(2.0), "CompileSlab { instrs: [] }", 2.0);
    comp_chk("2 ^ 0.5", IConst(1.4142135623730951), "CompileSlab { instrs: [] }", 1.4142135623730951);
    //comp_chk("-4 ^ 0.5", IConst(std::f64::NAN), "CompileSlab { instrs: [] }", std::f64::NAN);
    comp_chk("y ^ 0.5", IExp { base: InstructionI(0), power: InstructionI(1) }, "CompileSlab { instrs: [IVar(Variable(`y`)), IConst(0.5)] }", 1.4142135623730951);
    comp_chk("2 ^ 3 ^ 2", IConst(64.0), "CompileSlab { instrs: [] }", 64.0);
    comp_chk("2 ^ z ^ 2", IExp { base: InstructionI(2), power: InstructionI(3) }, "CompileSlab { instrs: [IVar(Variable(`z`)), IConst(2.0), IConst(2.0), IMul(InstructionI(0), InstructionI(1))] }", 64.0);
    comp_chk("2 ^ z ^ 1 ^ 2 ^ 1", IExp { base: InstructionI(2), power: InstructionI(3) }, "CompileSlab { instrs: [IVar(Variable(`z`)), IConst(2.0), IConst(2.0), IMul(InstructionI(0), InstructionI(1))] }", 64.0);
    
    // ILT:
    comp_chk("2 < 3", IConst(1.0), "CompileSlab { instrs: [] }", 1.0);
    comp_chk("2 < z", ILT(InstructionI(0), InstructionI(1)), "CompileSlab { instrs: [IConst(2.0), IVar(Variable(`z`))] }", 1.0);
    comp_chk("3 < 3", IConst(0.0), "CompileSlab { instrs: [] }", 0.0);
    comp_chk("3 < z", ILT(InstructionI(0), InstructionI(1)), "CompileSlab { instrs: [IConst(3.0), IVar(Variable(`z`))] }", 0.0);
    comp_chk("1 < 2 < 3", IConst(1.0), "CompileSlab { instrs: [] }", 1.0);

    // ILTE:
    comp_chk("2 <= 3", IConst(1.0), "CompileSlab { instrs: [] }", 1.0);
    comp_chk("2 <= z", ILTE(InstructionI(0), InstructionI(1)), "CompileSlab { instrs: [IConst(2.0), IVar(Variable(`z`))] }", 1.0);
    comp_chk("3 <= 3", IConst(1.0), "CompileSlab { instrs: [] }", 1.0);
    comp_chk("3 <= z", ILTE(InstructionI(0), InstructionI(1)), "CompileSlab { instrs: [IConst(3.0), IVar(Variable(`z`))] }", 1.0);
    comp_chk("4 <= 3", IConst(0.0), "CompileSlab { instrs: [] }", 0.0);
    comp_chk("4 <= z", ILTE(InstructionI(0), InstructionI(1)), "CompileSlab { instrs: [IConst(4.0), IVar(Variable(`z`))] }", 0.0);

    // IEQ:
    comp_chk("2 == 3", IConst(0.0), "CompileSlab { instrs: [] }", 0.0);
    comp_chk("2 == z", IEQ(InstructionI(0), InstructionI(1)), "CompileSlab { instrs: [IConst(2.0), IVar(Variable(`z`))] }", 0.0);
    comp_chk("3 == 3", IConst(1.0), "CompileSlab { instrs: [] }", 1.0);
    comp_chk("3 == z", IEQ(InstructionI(0), InstructionI(1)), "CompileSlab { instrs: [IConst(3.0), IVar(Variable(`z`))] }", 1.0);
    comp_chk("4 == 3", IConst(0.0), "CompileSlab { instrs: [] }", 0.0);
    comp_chk("4 == z", IEQ(InstructionI(0), InstructionI(1)), "CompileSlab { instrs: [IConst(4.0), IVar(Variable(`z`))] }", 0.0);
    comp_chk("4 == z == 1.0", IEQ(InstructionI(2), InstructionI(3)), "CompileSlab { instrs: [IConst(4.0), IVar(Variable(`z`)), IEQ(InstructionI(0), InstructionI(1)), IConst(1.0)] }", 0.0);
    
    // INE:
    comp_chk("2 != 3", IConst(1.0), "CompileSlab { instrs: [] }", 1.0);
    comp_chk("2 != z", INE(InstructionI(0), InstructionI(1)), "CompileSlab { instrs: [IConst(2.0), IVar(Variable(`z`))] }", 1.0);
    comp_chk("3 != 3", IConst(0.0), "CompileSlab { instrs: [] }", 0.0);
    comp_chk("3 != z", INE(InstructionI(0), InstructionI(1)), "CompileSlab { instrs: [IConst(3.0), IVar(Variable(`z`))] }", 0.0);
    comp_chk("4 != 3", IConst(1.0), "CompileSlab { instrs: [] }", 1.0);
    comp_chk("4 != z", INE(InstructionI(0), InstructionI(1)), "CompileSlab { instrs: [IConst(4.0), IVar(Variable(`z`))] }", 1.0);

    // IAND:
    comp_chk("2 and 3", IConst(3.0), "CompileSlab { instrs: [] }", 3.0);
    comp_chk("2 and 3 and 4", IConst(4.0), "CompileSlab { instrs: [] }", 4.0);
    comp_chk("0 and 1 and 2", IConst(0.0), "CompileSlab { instrs: [] }", 0.0);
    comp_chk("1 and 0 and 2", IConst(0.0), "CompileSlab { instrs: [] }", 0.0);
    comp_chk("1 and 2 and 0", IConst(0.0), "CompileSlab { instrs: [] }", 0.0);
    comp_chk("x and 2", IAND(InstructionI(0), InstructionI(1)), "CompileSlab { instrs: [IVar(Variable(`x`)), IConst(2.0)] }", 2.0);
    comp_chk("0 and x", IConst(0.0), "CompileSlab { instrs: [] }", 0.0);
    comp_chk("w and x", IAND(InstructionI(0), InstructionI(1)), "CompileSlab { instrs: [IVar(Variable(`w`)), IVar(Variable(`x`))] }", 0.0);
    
    // IOR:
    comp_chk("2 or 3", IConst(2.0), "CompileSlab { instrs: [] }", 2.0);
    comp_chk("2 or 3 or 4", IConst(2.0), "CompileSlab { instrs: [] }", 2.0);
    comp_chk("0 or 1 or 2", IConst(1.0), "CompileSlab { instrs: [] }", 1.0);
    comp_chk("1 or 0 or 2", IConst(1.0), "CompileSlab { instrs: [] }", 1.0);
    comp_chk("1 or 2 or 0", IConst(1.0), "CompileSlab { instrs: [] }", 1.0);
    comp_chk("x or 2", IOR(InstructionI(0), InstructionI(1)), "CompileSlab { instrs: [IVar(Variable(`x`)), IConst(2.0)] }", 1.0);
    comp_chk("0 or x", IVar(Variable("x".to_string())), "CompileSlab { instrs: [] }", 1.0);
    comp_chk("w or x", IOR(InstructionI(0), InstructionI(1)), "CompileSlab { instrs: [IVar(Variable(`w`)), IVar(Variable(`x`))] }", 1.0);
    comp_chk("x or w", IOR(InstructionI(0), InstructionI(1)), "CompileSlab { instrs: [IVar(Variable(`x`)), IVar(Variable(`w`))] }", 1.0);

    // IFuncInt
    comp_chk("int(2.7)", IConst(2.0), "CompileSlab { instrs: [] }", 2.0);
    comp_chk("int(y7)", IFuncInt(InstructionI(0)), "CompileSlab { instrs: [IVar(Variable(`y7`))] }", 2.0);
    comp_chk("int(-2.7)", IConst(-2.0), "CompileSlab { instrs: [] }", -2.0);
    comp_chk("int(-y7)", IFuncInt(InstructionI(1)), "CompileSlab { instrs: [IVar(Variable(`y7`)), INeg(InstructionI(0))] }", -2.0);

    // IFuncCeil
    comp_chk("ceil(2.7)", IConst(3.0), "CompileSlab { instrs: [] }", 3.0);
    comp_chk("ceil(y7)", IFuncCeil(InstructionI(0)), "CompileSlab { instrs: [IVar(Variable(`y7`))] }", 3.0);
    comp_chk("ceil(-2.7)", IConst(-2.0), "CompileSlab { instrs: [] }", -2.0);
    comp_chk("ceil(-y7)", IFuncCeil(InstructionI(1)), "CompileSlab { instrs: [IVar(Variable(`y7`)), INeg(InstructionI(0))] }", -2.0);
    
    // IFuncFloor
    comp_chk("floor(2.7)", IConst(2.0), "CompileSlab { instrs: [] }", 2.0);
    comp_chk("floor(y7)", IFuncFloor(InstructionI(0)), "CompileSlab { instrs: [IVar(Variable(`y7`))] }", 2.0);
    comp_chk("floor(-2.7)", IConst(-3.0), "CompileSlab { instrs: [] }", -3.0);
    comp_chk("floor(-y7)", IFuncFloor(InstructionI(1)), "CompileSlab { instrs: [IVar(Variable(`y7`)), INeg(InstructionI(0))] }", -3.0);

    // IFuncAbs
    comp_chk("abs(2.7)", IConst(2.7), "CompileSlab { instrs: [] }", 2.7);
    comp_chk("abs(y7)", IFuncAbs(InstructionI(0)), "CompileSlab { instrs: [IVar(Variable(`y7`))] }", 2.7);
    comp_chk("abs(-2.7)", IConst(2.7), "CompileSlab { instrs: [] }", 2.7);
    comp_chk("abs(-y7)", IFuncAbs(InstructionI(1)), "CompileSlab { instrs: [IVar(Variable(`y7`)), INeg(InstructionI(0))] }", 2.7);

    // IFuncLog
    comp_chk("log(1)", IConst(0.0), "CompileSlab { instrs: [] }", 0.0);
    comp_chk("log(10)", IConst(1.0), "CompileSlab { instrs: [] }", 1.0);
    comp_chk("log(2, 10)", IConst(3.321928094887362), "CompileSlab { instrs: [] }", 3.321928094887362);
    comp_chk("log(e(), 10)", IConst(2.302585092994046), "CompileSlab { instrs: [] }", 2.302585092994046);
    comp_chk("log(x)", IFuncLog { base: InstructionI(0), of: InstructionI(1) }, "CompileSlab { instrs: [IConst(10.0), IVar(Variable(`x`))] }", 0.0);
    comp_chk("log(y,x)", IFuncLog { base: InstructionI(0), of: InstructionI(1) }, "CompileSlab { instrs: [IVar(Variable(`y`)), IVar(Variable(`x`))] }", 0.0);

    // IFuncRound
    comp_chk("round(2.7)", IConst(3.0), "CompileSlab { instrs: [] }", 3.0);
    comp_chk("round(-2.7)", IConst(-3.0), "CompileSlab { instrs: [] }", -3.0);
    comp_chk("round(y7)", IFuncRound { modulus: InstructionI(0), of: InstructionI(1) }, "CompileSlab { instrs: [IConst(1.0), IVar(Variable(`y7`))] }", 3.0);
    

}

