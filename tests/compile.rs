use al::{parse, Compiler, Evaler, Error, Slab, EmptyNamespace, FlatNamespace, ExpressionI, InstructionI, eval_compiled, eval_compiled_ref};
use al::parser::{PrintFunc, ExpressionOrString::{EExpr, EStr}};
#[cfg(feature="eval-builtin")]
use al::parser::{EvalFunc, KWArg};
use al::compiler::Instruction::{self, IConst, INeg, INot, IInv, IAdd, IMul, IMod, IExp, ILT, ILTE, IEQ, INE, IGTE, IGT, IAND, IOR, IVar, IFunc, IFuncInt, IFuncCeil, IFuncFloor, IFuncAbs, IFuncSign, IFuncLog, IFuncRound, IFuncMin, IFuncMax, IFuncSin, IFuncCos, IFuncTan, IFuncASin, IFuncACos, IFuncATan, IFuncSinH, IFuncCosH, IFuncTanH, IFuncASinH, IFuncACosH, IFuncATanH, IPrintFunc};
#[cfg(feature="eval-builtin")]
use al::compiler::Instruction::IEvalFunc;

#[test]
fn slab_overflow() {
    let mut slab = Slab::with_capacity(2);
    assert_eq!(parse("1 + 2 + -3 + ( +4 )", {slab.clear(); &mut slab.ps}), Ok(ExpressionI(1)));
    assert_eq!(format!("{:?}", slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(4.0), pairs: [] }, 1:Expression { first: EConstant(1.0), pairs: [ExprPair(EAdd, EConstant(2.0)), ExprPair(EAdd, EConstant(-3.0)), ExprPair(EAdd, EUnaryOp(EParentheses(ExpressionI(0))))] } }, vals:{}, instrs:{} }");

    assert_eq!(parse("1 + 2 + -3 + ( ++4 )", {slab.clear(); &mut slab.ps}), Ok(ExpressionI(1)));
    assert_eq!(format!("{:?}", slab),
"Slab{ exprs:{ 0:Expression { first: EUnaryOp(EPos(ValueI(0))), pairs: [] }, 1:Expression { first: EConstant(1.0), pairs: [ExprPair(EAdd, EConstant(2.0)), ExprPair(EAdd, EConstant(-3.0)), ExprPair(EAdd, EUnaryOp(EParentheses(ExpressionI(0))))] } }, vals:{ 0:EConstant(4.0) }, instrs:{} }");

    assert_eq!(parse("1 + 2 + -3 + ( +++4 )", {slab.clear(); &mut slab.ps}), Ok(ExpressionI(1)));
    assert_eq!(format!("{:?}", slab),
"Slab{ exprs:{ 0:Expression { first: EUnaryOp(EPos(ValueI(1))), pairs: [] }, 1:Expression { first: EConstant(1.0), pairs: [ExprPair(EAdd, EConstant(2.0)), ExprPair(EAdd, EConstant(-3.0)), ExprPair(EAdd, EUnaryOp(EParentheses(ExpressionI(0))))] } }, vals:{ 0:EConstant(4.0), 1:EUnaryOp(EPos(ValueI(0))) }, instrs:{} }");

    assert_eq!(parse("1 + 2 + -3 + ( ++++4 )", {slab.clear(); &mut slab.ps}), Err(Error::SlabOverflow));
}

#[test]
fn basics() {
    let mut slab = Slab::new();
    let mut ns = EmptyNamespace;

    let expr_i = parse("3*3-3/3+1", {slab.clear(); &mut slab.ps}).unwrap();
    let expr_ref = slab.ps.get_expr(expr_i);
    let instr = expr_ref.compile(&slab.ps, &mut slab.cs);
    assert_eq!(instr, IConst(9.0));
    assert_eq!(format!("{:?}", slab),
"Slab{ exprs:{ 0:Expression { first: EConstant(3.0), pairs: [ExprPair(EMul, EConstant(3.0)), ExprPair(ESub, EConstant(3.0)), ExprPair(EDiv, EConstant(3.0)), ExprPair(EAdd, EConstant(1.0))] } }, vals:{}, instrs:{} }");

    (|| -> Result<(),Error> {
        assert_eq!(eval_compiled_ref!(&instr, &slab, &mut ns), 9.0);
        assert_eq!(eval_compiled_ref!(&instr, &slab, &mut ns), 9.0);
        Ok(())
    })().unwrap();
}


fn comp(expr_str:&str) -> (Slab, Instruction) {
    let mut slab = Slab::new();
    let instr = parse(expr_str, &mut slab.ps).unwrap().from(&slab.ps).compile(&slab.ps, &mut slab.cs);
    (slab, instr)
}

fn comp_chk(expr_str:&str, expect_instr:Instruction, expect_fmt:&str, expect_eval:f64) {
    let mut slab = Slab::new();
    let expr = parse(expr_str, &mut slab.ps).unwrap().from(&slab.ps);
    let instr = expr.compile(&slab.ps, &mut slab.cs);

    assert_eq!(instr, expect_instr);
    assert_eq!(format!("{:?}",slab.cs), expect_fmt);

    let mut ns = FlatNamespace::new(|name,args| {
        match name {
            "w" => Some(0.0),
            "x" => Some(1.0),
            "y" => Some(2.0),
            "y7" => Some(2.7),
            "z" => Some(3.0),
            "foo" => Some(args[0]*10.0),
            "bar" => Some(args[0]+args[1]),
            _ => None,
        }
    });

    (|| -> Result<(),Error> {
        assert_eq!(eval_compiled_ref!(&instr, &slab, &mut ns), expect_eval);

        // Make sure Instruction eval matches normal eval:
        assert_eq!(eval_compiled_ref!(&instr, &slab, &mut ns), expr.eval(&slab, &mut ns).unwrap());

        Ok(())
    })().unwrap();
}
#[cfg(feature="unsafe-vars")]
fn unsafe_comp_chk(expr_str:&str, expect_fmt:&str, expect_eval:f64) {
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
    let w = 0.0;
    let x = 1.0;
    let y = 2.0;
    let y7 = 2.7;
    let z = 3.0;
    unsafe {
        slab.ps.add_unsafe_var("w".to_string(), &w);
        slab.ps.add_unsafe_var("x".to_string(), &x);
        slab.ps.add_unsafe_var("y".to_string(), &y);
        slab.ps.add_unsafe_var("y7".to_string(), &y7);
        slab.ps.add_unsafe_var("z".to_string(), &z);
    }

    let expr = parse(expr_str, &mut slab.ps).unwrap().from(&slab.ps);
    let instr = expr.compile(&slab.ps, &mut slab.cs);

    assert_eq!(replace_addrs(format!("{:?}",slab.cs)), expect_fmt);

    (|| -> Result<(),Error> {
        let mut ns = EmptyNamespace;
        assert_eq!(eval_compiled_ref!(&instr, &slab, &mut ns), expect_eval);

        // Make sure Instruction eval matches normal eval:
        assert_eq!(eval_compiled_ref!(&instr, &slab, &mut ns), expr.eval(&slab, &mut ns).unwrap());

        Ok(())
    })().unwrap();
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

    assert_eq!(comp("x").1, IVar("x".to_string()));

    comp_chk("1-1", IConst(0.0), "CompileSlab{ instrs:{} }", 0.0);
    comp_chk("1 + x", IAdd(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"x\"), 1:IConst(1.0) } }", 2.0);
    comp_chk("x + 1", IAdd(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"x\"), 1:IConst(1.0) } }", 2.0);
    comp_chk("0.5 + x + 0.5", IAdd(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"x\"), 1:IConst(1.0) } }", 2.0);
    comp_chk("0.5 - x - 0.5", INeg(InstructionI(0)), "CompileSlab{ instrs:{ 0:IVar(\"x\") } }", -1.0);
    comp_chk("0.5 - -x - 0.5", IVar("x".to_string()), "CompileSlab{ instrs:{} }", 1.0);
    comp_chk("0.5 - --x - 1.5", IAdd(InstructionI(1), InstructionI(2)), "CompileSlab{ instrs:{ 0:IVar(\"x\"), 1:INeg(InstructionI(0)), 2:IConst(-1.0) } }", -2.0);
    comp_chk("0.5 - ---x - 1.5", IAdd(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"x\"), 1:IConst(-1.0) } }", 0.0);
    comp_chk("0.5 - (---x) - 1.5", IAdd(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"x\"), 1:IConst(-1.0) } }", 0.0);
    comp_chk("0.5 - -(--x) - 1.5", IAdd(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"x\"), 1:IConst(-1.0) } }", 0.0);
    comp_chk("0.5 - --(-x) - 1.5", IAdd(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"x\"), 1:IConst(-1.0) } }", 0.0);
    comp_chk("0.5 - --(-x - 1.5)", IAdd(InstructionI(4), InstructionI(5)), "CompileSlab{ instrs:{ 0:IVar(\"x\"), 1:INeg(InstructionI(0)), 2:IConst(-1.5), 3:IAdd(InstructionI(1), InstructionI(2)), 4:INeg(InstructionI(3)), 5:IConst(0.5) } }", 3.0);
    comp_chk("0.5 - --((((-(x)) - 1.5)))", IAdd(InstructionI(4), InstructionI(5)), "CompileSlab{ instrs:{ 0:IVar(\"x\"), 1:INeg(InstructionI(0)), 2:IConst(-1.5), 3:IAdd(InstructionI(1), InstructionI(2)), 4:INeg(InstructionI(3)), 5:IConst(0.5) } }", 3.0);
    comp_chk("0.5 - -(-(--((((-(x)) - 1.5)))))", IAdd(InstructionI(4), InstructionI(5)), "CompileSlab{ instrs:{ 0:IVar(\"x\"), 1:INeg(InstructionI(0)), 2:IConst(-1.5), 3:IAdd(InstructionI(1), InstructionI(2)), 4:INeg(InstructionI(3)), 5:IConst(0.5) } }", 3.0);
}

#[test]
fn all_instrs() {
    // IConst:
    comp_chk("1", IConst(1.0), "CompileSlab{ instrs:{} }", 1.0);
    comp_chk("-1", IConst(-1.0), "CompileSlab{ instrs:{} }", -1.0);

    // IVar:
    comp_chk("x", IVar("x".to_string()), "CompileSlab{ instrs:{} }", 1.0);
    comp_chk("x()", IFunc { name:"x".to_string(), args:vec![] }, "CompileSlab{ instrs:{} }", 1.0);
    comp_chk("x[]", IFunc { name:"x".to_string(), args:vec![] }, "CompileSlab{ instrs:{} }", 1.0);

    // INeg:
    comp_chk("-x", INeg(InstructionI(0)), "CompileSlab{ instrs:{ 0:IVar(\"x\") } }", -1.0);

    // INot:
    comp_chk("!x", INot(InstructionI(0)), "CompileSlab{ instrs:{ 0:IVar(\"x\") } }", 0.0);

    // IInv:
    comp_chk("1/x", IInv(InstructionI(0)), "CompileSlab{ instrs:{ 0:IVar(\"x\") } }", 1.0);

    // IAdd:
    comp_chk("1 + x", IAdd(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"x\"), 1:IConst(1.0) } }", 2.0);
    comp_chk("1 - x", IAdd(InstructionI(1), InstructionI(2)), "CompileSlab{ instrs:{ 0:IVar(\"x\"), 1:INeg(InstructionI(0)), 2:IConst(1.0) } }", 0.0);
    comp_chk("x + 2+pi()-360", IAdd(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"x\"), 1:IConst(-354.8584073464102) } }", -353.8584073464102);
    comp_chk("x-360 + 2+pi()", IAdd(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"x\"), 1:IConst(-354.8584073464102) } }", -353.8584073464102);
    comp_chk("1 - -(x-360 + 2+pi())", IAdd(InstructionI(2), InstructionI(3)), "CompileSlab{ instrs:{ 0:IVar(\"x\"), 1:IConst(-354.8584073464102), 2:IAdd(InstructionI(0), InstructionI(1)), 3:IConst(1.0) } }", -352.8584073464102);
    comp_chk("3 + 3 - 3 + 3 - 3 + 3", IConst(6.0), "CompileSlab{ instrs:{} }", 6.0);
    comp_chk("3 + x - 3 + 3 + y - 3", IAdd(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"x\"), 1:IVar(\"y\") } }", 3.0);

    // IMul:
    comp_chk("2 * x", IMul(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"x\"), 1:IConst(2.0) } }", 2.0);
    comp_chk("x * 2", IMul(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"x\"), 1:IConst(2.0) } }", 2.0);
    comp_chk("x / 2", IMul(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"x\"), 1:IConst(0.5) } }", 0.5);
    comp_chk("x * 2*pi()/360", IMul(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"x\"), 1:IConst(0.017453292519943295) } }", 0.017453292519943295);
    comp_chk("x/360 * 2*pi()", IMul(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"x\"), 1:IConst(0.017453292519943295) } }", 0.017453292519943295);
    comp_chk("1 / -(x/360 * 2*pi())", IInv(InstructionI(3)), "CompileSlab{ instrs:{ 0:IVar(\"x\"), 1:IConst(0.017453292519943295), 2:IMul(InstructionI(0), InstructionI(1)), 3:INeg(InstructionI(2)) } }", -57.29577951308232);
    comp_chk("3 * 3 / 3 * 3 / 3 * 3", IConst(9.0), "CompileSlab{ instrs:{} }", 9.0);
    comp_chk("3 * x / 3 * 3 * y / 3", IMul(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"x\"), 1:IVar(\"y\") } }", 2.0);

    // IMod:
    comp_chk("8 % 3", IConst(2.0), "CompileSlab{ instrs:{} }", 2.0);
    comp_chk("8 % z", IMod { dividend: InstructionI(0), divisor: InstructionI(1) }, "CompileSlab{ instrs:{ 0:IConst(8.0), 1:IVar(\"z\") } }", 2.0);
    comp_chk("-8 % 3", IConst(-2.0), "CompileSlab{ instrs:{} }", -2.0);
    comp_chk("8 % -3", IConst(2.0), "CompileSlab{ instrs:{} }", 2.0);
    comp_chk("-8 % z", IMod { dividend: InstructionI(0), divisor: InstructionI(1) }, "CompileSlab{ instrs:{ 0:IConst(-8.0), 1:IVar(\"z\") } }", -2.0);
    comp_chk("8 % -z", IMod { dividend: InstructionI(1), divisor: InstructionI(2) }, "CompileSlab{ instrs:{ 0:IVar(\"z\"), 1:IConst(8.0), 2:INeg(InstructionI(0)) } }", 2.0);
    comp_chk("8 % 3 % 2", IConst(0.0), "CompileSlab{ instrs:{} }", 0.0);
    comp_chk("8 % z % 2", IMod { dividend: InstructionI(2), divisor: InstructionI(3) }, "CompileSlab{ instrs:{ 0:IConst(8.0), 1:IVar(\"z\"), 2:IMod { dividend: InstructionI(0), divisor: InstructionI(1) }, 3:IConst(2.0) } }", 0.0);

    // IExp:
    comp_chk("2 ^ 3", IConst(8.0), "CompileSlab{ instrs:{} }", 8.0);
    comp_chk("2 ^ z", IExp { base: InstructionI(0), power: InstructionI(1) }, "CompileSlab{ instrs:{ 0:IConst(2.0), 1:IVar(\"z\") } }", 8.0);
    comp_chk("4 ^ 0.5", IConst(2.0), "CompileSlab{ instrs:{} }", 2.0);
    comp_chk("2 ^ 0.5", IConst(1.4142135623730951), "CompileSlab{ instrs:{} }", 1.4142135623730951);
    //comp_chk("-4 ^ 0.5", IConst(std::f64::NAN), "CompileSlab{ instrs:{} }", std::f64::NAN);
    comp_chk("y ^ 0.5", IExp { base: InstructionI(0), power: InstructionI(1) }, "CompileSlab{ instrs:{ 0:IVar(\"y\"), 1:IConst(0.5) } }", 1.4142135623730951);
    comp_chk("2 ^ 3 ^ 2", IConst(512.0), "CompileSlab{ instrs:{} }", 512.0);
    comp_chk("2 ^ z ^ 2", IExp { base: InstructionI(2), power: InstructionI(3) }, "CompileSlab{ instrs:{ 0:IVar(\"z\"), 1:IConst(2.0), 2:IConst(2.0), 3:IExp { base: InstructionI(0), power: InstructionI(1) } } }", 512.0);
    comp_chk("2 ^ z ^ 1 ^ 2 ^ 1", IExp { base: InstructionI(2), power: InstructionI(3) }, "CompileSlab{ instrs:{ 0:IVar(\"z\"), 1:IConst(1.0), 2:IConst(2.0), 3:IExp { base: InstructionI(0), power: InstructionI(1) } } }", 8.0);

    // ILT:
    comp_chk("2 < 3", IConst(1.0), "CompileSlab{ instrs:{} }", 1.0);
    comp_chk("2 < z", ILT(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(2.0), 1:IVar(\"z\") } }", 1.0);
    comp_chk("3 < 3", IConst(0.0), "CompileSlab{ instrs:{} }", 0.0);
    comp_chk("3 < z", ILT(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.0), 1:IVar(\"z\") } }", 0.0);
    comp_chk("1 < 2 < 3", IConst(1.0), "CompileSlab{ instrs:{} }", 1.0);

    // ILTE:
    comp_chk("2 <= 3", IConst(1.0), "CompileSlab{ instrs:{} }", 1.0);
    comp_chk("2 <= z", ILTE(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(2.0), 1:IVar(\"z\") } }", 1.0);
    comp_chk("3 <= 3", IConst(1.0), "CompileSlab{ instrs:{} }", 1.0);
    comp_chk("3 <= z", ILTE(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.0), 1:IVar(\"z\") } }", 1.0);
    comp_chk("4 <= 3", IConst(0.0), "CompileSlab{ instrs:{} }", 0.0);
    comp_chk("4 <= z", ILTE(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(4.0), 1:IVar(\"z\") } }", 0.0);

    // IEQ:
    comp_chk("2 == 3", IConst(0.0), "CompileSlab{ instrs:{} }", 0.0);
    comp_chk("2 == z", IEQ(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(2.0), 1:IVar(\"z\") } }", 0.0);
    comp_chk("3 == 3", IConst(1.0), "CompileSlab{ instrs:{} }", 1.0);
    comp_chk("3 == z", IEQ(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.0), 1:IVar(\"z\") } }", 1.0);
    comp_chk("4 == 3", IConst(0.0), "CompileSlab{ instrs:{} }", 0.0);
    comp_chk("4 == z", IEQ(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(4.0), 1:IVar(\"z\") } }", 0.0);
    comp_chk("4 == z == 1.0", IEQ(InstructionI(2), InstructionI(3)), "CompileSlab{ instrs:{ 0:IConst(4.0), 1:IVar(\"z\"), 2:IEQ(InstructionI(0), InstructionI(1)), 3:IConst(1.0) } }", 0.0);
    comp_chk("3.1 == z", IEQ(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.1), 1:IVar(\"z\") } }", 0.0);
    comp_chk("3.01 == z", IEQ(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.01), 1:IVar(\"z\") } }", 0.0);
    comp_chk("3.001 == z", IEQ(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.001), 1:IVar(\"z\") } }", 0.0);
    comp_chk("3.0001 == z", IEQ(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.0001), 1:IVar(\"z\") } }", 0.0);
    comp_chk("3.00001 == z", IEQ(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.00001), 1:IVar(\"z\") } }", 0.0);
    comp_chk("3.000001 == z", IEQ(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.000001), 1:IVar(\"z\") } }", 0.0);
    comp_chk("3.0000001 == z", IEQ(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.0000001), 1:IVar(\"z\") } }", 0.0);
    comp_chk("3.00000001 == z", IEQ(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.00000001), 1:IVar(\"z\") } }", 0.0);
    comp_chk("3.000000001 == z", IEQ(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.000000001), 1:IVar(\"z\") } }", 0.0);
    comp_chk("3.0000000001 == z", IEQ(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.0000000001), 1:IVar(\"z\") } }", 0.0);
    comp_chk("3.00000000001 == z", IEQ(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.00000000001), 1:IVar(\"z\") } }", 0.0);
    comp_chk("3.000000000001 == z", IEQ(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.000000000001), 1:IVar(\"z\") } }", 0.0);
    comp_chk("3.0000000000001 == z", IEQ(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.0000000000001), 1:IVar(\"z\") } }", 0.0);
    comp_chk("3.00000000000001 == z", IEQ(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.00000000000001), 1:IVar(\"z\") } }", 0.0);
    comp_chk("3.000000000000001 == z", IEQ(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.000000000000001), 1:IVar(\"z\") } }", 1.0);
    comp_chk("3.0000000000000001 == z", IEQ(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.0), 1:IVar(\"z\") } }", 1.0);

    // INE:
    comp_chk("2 != 3", IConst(1.0), "CompileSlab{ instrs:{} }", 1.0);
    comp_chk("2 != z", INE(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(2.0), 1:IVar(\"z\") } }", 1.0);
    comp_chk("3 != 3", IConst(0.0), "CompileSlab{ instrs:{} }", 0.0);
    comp_chk("3 != z", INE(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.0), 1:IVar(\"z\") } }", 0.0);
    comp_chk("4 != 3", IConst(1.0), "CompileSlab{ instrs:{} }", 1.0);
    comp_chk("4 != z", INE(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(4.0), 1:IVar(\"z\") } }", 1.0);
    comp_chk("3.1 != z", INE(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.1), 1:IVar(\"z\") } }", 1.0);
    comp_chk("3.01 != z", INE(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.01), 1:IVar(\"z\") } }", 1.0);
    comp_chk("3.001 != z", INE(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.001), 1:IVar(\"z\") } }", 1.0);
    comp_chk("3.0001 != z", INE(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.0001), 1:IVar(\"z\") } }", 1.0);
    comp_chk("3.00001 != z", INE(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.00001), 1:IVar(\"z\") } }", 1.0);
    comp_chk("3.000001 != z", INE(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.000001), 1:IVar(\"z\") } }", 1.0);
    comp_chk("3.0000001 != z", INE(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.0000001), 1:IVar(\"z\") } }", 1.0);
    comp_chk("3.00000001 != z", INE(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.00000001), 1:IVar(\"z\") } }", 1.0);
    comp_chk("3.000000001 != z", INE(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.000000001), 1:IVar(\"z\") } }", 1.0);
    comp_chk("3.0000000001 != z", INE(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.0000000001), 1:IVar(\"z\") } }", 1.0);
    comp_chk("3.00000000001 != z", INE(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.00000000001), 1:IVar(\"z\") } }", 1.0);
    comp_chk("3.000000000001 != z", INE(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.000000000001), 1:IVar(\"z\") } }", 1.0);
    comp_chk("3.0000000000001 != z", INE(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.0000000000001), 1:IVar(\"z\") } }", 1.0);
    comp_chk("3.00000000000001 != z", INE(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.00000000000001), 1:IVar(\"z\") } }", 1.0);
    comp_chk("3.000000000000001 != z", INE(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.000000000000001), 1:IVar(\"z\") } }", 0.0);
    comp_chk("3.0000000000000001 != z", INE(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.0), 1:IVar(\"z\") } }", 0.0);

    // IGTE:
    comp_chk("2 >= 3", IConst(0.0), "CompileSlab{ instrs:{} }", 0.0);
    comp_chk("2 >= z", IGTE(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(2.0), 1:IVar(\"z\") } }", 0.0);
    comp_chk("3 >= 3", IConst(1.0), "CompileSlab{ instrs:{} }", 1.0);
    comp_chk("3 >= z", IGTE(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(3.0), 1:IVar(\"z\") } }", 1.0);
    comp_chk("4 >= 3", IConst(1.0), "CompileSlab{ instrs:{} }", 1.0);
    comp_chk("4 >= z", IGTE(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IConst(4.0), 1:IVar(\"z\") } }", 1.0);

    // IGT:
    comp_chk("3 > 2", IConst(1.0), "CompileSlab{ instrs:{} }", 1.0);
    comp_chk("z > 2", IGT(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"z\"), 1:IConst(2.0) } }", 1.0);
    comp_chk("3 > 3", IConst(0.0), "CompileSlab{ instrs:{} }", 0.0);
    comp_chk("z > 3", IGT(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"z\"), 1:IConst(3.0) } }", 0.0);
    comp_chk("3 > 2 > 1", IConst(0.0), "CompileSlab{ instrs:{} }", 0.0);

    // IAND:
    comp_chk("2 and 3", IConst(3.0), "CompileSlab{ instrs:{} }", 3.0); comp_chk("2 && 3", IConst(3.0), "CompileSlab{ instrs:{} }", 3.0);
    comp_chk("2 and 3 and 4", IConst(4.0), "CompileSlab{ instrs:{} }", 4.0); comp_chk("2 && 3 && 4", IConst(4.0), "CompileSlab{ instrs:{} }", 4.0);
    comp_chk("0 and 1 and 2", IConst(0.0), "CompileSlab{ instrs:{} }", 0.0); comp_chk("0 && 1 && 2", IConst(0.0), "CompileSlab{ instrs:{} }", 0.0);
    comp_chk("1 and 0 and 2", IConst(0.0), "CompileSlab{ instrs:{} }", 0.0); comp_chk("1 && 0 && 2", IConst(0.0), "CompileSlab{ instrs:{} }", 0.0);
    comp_chk("1 and 2 and 0", IConst(0.0), "CompileSlab{ instrs:{} }", 0.0); comp_chk("1 && 2 && 0", IConst(0.0), "CompileSlab{ instrs:{} }", 0.0);
    comp_chk("x and 2", IAND(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"x\"), 1:IConst(2.0) } }", 2.0);
    comp_chk("0 and x", IConst(0.0), "CompileSlab{ instrs:{} }", 0.0);
    comp_chk("w and x", IAND(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"w\"), 1:IVar(\"x\") } }", 0.0);

    // IOR:
    comp_chk("2 or 3", IConst(2.0), "CompileSlab{ instrs:{} }", 2.0); comp_chk("2 || 3", IConst(2.0), "CompileSlab{ instrs:{} }", 2.0);
    comp_chk("2 or 3 or 4", IConst(2.0), "CompileSlab{ instrs:{} }", 2.0); comp_chk("2 || 3 || 4", IConst(2.0), "CompileSlab{ instrs:{} }", 2.0);
    comp_chk("0 or 1 or 2", IConst(1.0), "CompileSlab{ instrs:{} }", 1.0); comp_chk("0 || 1 || 2", IConst(1.0), "CompileSlab{ instrs:{} }", 1.0);
    comp_chk("1 or 0 or 2", IConst(1.0), "CompileSlab{ instrs:{} }", 1.0); comp_chk("1 || 0 || 2", IConst(1.0), "CompileSlab{ instrs:{} }", 1.0);
    comp_chk("1 or 2 or 0", IConst(1.0), "CompileSlab{ instrs:{} }", 1.0); comp_chk("1 || 2 || 0", IConst(1.0), "CompileSlab{ instrs:{} }", 1.0);
    comp_chk("x or 2", IOR(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"x\"), 1:IConst(2.0) } }", 1.0);
    comp_chk("0 or x", IVar("x".to_string()), "CompileSlab{ instrs:{} }", 1.0);
    comp_chk("w or x", IOR(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"w\"), 1:IVar(\"x\") } }", 1.0);
    comp_chk("x or w", IOR(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"x\"), 1:IVar(\"w\") } }", 1.0);

    // IVar
    comp_chk("x", IVar("x".to_string()), "CompileSlab{ instrs:{} }", 1.0);
    {
        let (_s,i) = comp("int");
        assert_eq!(i, IVar("int".to_string()));

        let (_s,i) = comp("print");
        assert_eq!(i, IVar("print".to_string()));

        let (_s,i) = comp("eval");
        assert_eq!(i, IVar("eval".to_string()));
    }

    // IUnsafeVar
    #[cfg(feature="unsafe-vars")]
    {
        unsafe_comp_chk("x", "CompileSlab{ instrs:{} }", 1.0);
        unsafe_comp_chk("x + y", "CompileSlab{ instrs:{ 0:IUnsafeVar { name: \"x\", ptr: 0x???????????? }, 1:IUnsafeVar { name: \"y\", ptr: 0x???????????? } } }", 3.0);
        unsafe_comp_chk("x() + y", "CompileSlab{ instrs:{ 0:IUnsafeVar { name: \"x\", ptr: 0x???????????? }, 1:IUnsafeVar { name: \"y\", ptr: 0x???????????? } } }", 3.0);
        unsafe_comp_chk("x(x,y,z) + y", "CompileSlab{ instrs:{ 0:IUnsafeVar { name: \"x\", ptr: 0x???????????? }, 1:IUnsafeVar { name: \"y\", ptr: 0x???????????? } } }", 3.0);
    }

    // IFunc
    comp_chk("foo(2.7)", IFunc { name: "foo".to_string(), args:vec![InstructionI(0)] }, "CompileSlab{ instrs:{ 0:IConst(2.7) } }", 27.0);
    comp_chk("foo(2.7, 3.4)", IFunc { name: "foo".to_string(), args:vec![InstructionI(0), InstructionI(1)] }, "CompileSlab{ instrs:{ 0:IConst(2.7), 1:IConst(3.4) } }", 27.0);

    // IFuncInt
    comp_chk("int(2.7)", IConst(2.0), "CompileSlab{ instrs:{} }", 2.0);
    comp_chk("int(y7)", IFuncInt(InstructionI(0)), "CompileSlab{ instrs:{ 0:IVar(\"y7\") } }", 2.0);
    comp_chk("int(-2.7)", IConst(-2.0), "CompileSlab{ instrs:{} }", -2.0);
    comp_chk("int(-y7)", IFuncInt(InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"y7\"), 1:INeg(InstructionI(0)) } }", -2.0);

    // IFuncCeil
    comp_chk("ceil(2.7)", IConst(3.0), "CompileSlab{ instrs:{} }", 3.0);
    comp_chk("ceil(y7)", IFuncCeil(InstructionI(0)), "CompileSlab{ instrs:{ 0:IVar(\"y7\") } }", 3.0);
    comp_chk("ceil(-2.7)", IConst(-2.0), "CompileSlab{ instrs:{} }", -2.0);
    comp_chk("ceil(-y7)", IFuncCeil(InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"y7\"), 1:INeg(InstructionI(0)) } }", -2.0);

    // IFuncFloor
    comp_chk("floor(2.7)", IConst(2.0), "CompileSlab{ instrs:{} }", 2.0);
    comp_chk("floor(y7)", IFuncFloor(InstructionI(0)), "CompileSlab{ instrs:{ 0:IVar(\"y7\") } }", 2.0);
    comp_chk("floor(-2.7)", IConst(-3.0), "CompileSlab{ instrs:{} }", -3.0);
    comp_chk("floor(-y7)", IFuncFloor(InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"y7\"), 1:INeg(InstructionI(0)) } }", -3.0);

    // IFuncAbs
    comp_chk("abs(2.7)", IConst(2.7), "CompileSlab{ instrs:{} }", 2.7);
    comp_chk("abs(y7)", IFuncAbs(InstructionI(0)), "CompileSlab{ instrs:{ 0:IVar(\"y7\") } }", 2.7);
    comp_chk("abs(-2.7)", IConst(2.7), "CompileSlab{ instrs:{} }", 2.7);
    comp_chk("abs(-y7)", IFuncAbs(InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"y7\"), 1:INeg(InstructionI(0)) } }", 2.7);

    // IFuncSign
    comp_chk("sign(2.7)", IConst(1.0), "CompileSlab{ instrs:{} }", 1.0);
    comp_chk("sign(y7)", IFuncSign(InstructionI(0)), "CompileSlab{ instrs:{ 0:IVar(\"y7\") } }", 1.0);
    comp_chk("sign(-2.7)", IConst(-1.0), "CompileSlab{ instrs:{} }", -1.0);
    comp_chk("sign(-y7)", IFuncSign(InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"y7\"), 1:INeg(InstructionI(0)) } }", -1.0);

    // IFuncLog
    comp_chk("log(1)", IConst(0.0), "CompileSlab{ instrs:{} }", 0.0);
    comp_chk("log(10)", IConst(1.0), "CompileSlab{ instrs:{} }", 1.0);
    comp_chk("log(2, 10)", IConst(3.321928094887362), "CompileSlab{ instrs:{} }", 3.321928094887362);
    comp_chk("log(e(), 10)", IConst(2.302585092994046), "CompileSlab{ instrs:{} }", 2.302585092994046);
    comp_chk("log(x)", IFuncLog { base: InstructionI(0), of: InstructionI(1) }, "CompileSlab{ instrs:{ 0:IConst(10.0), 1:IVar(\"x\") } }", 0.0);
    comp_chk("log(y,x)", IFuncLog { base: InstructionI(0), of: InstructionI(1) }, "CompileSlab{ instrs:{ 0:IVar(\"y\"), 1:IVar(\"x\") } }", 0.0);

    // IFuncRound
    comp_chk("round(2.7)", IConst(3.0), "CompileSlab{ instrs:{} }", 3.0);
    comp_chk("round(-2.7)", IConst(-3.0), "CompileSlab{ instrs:{} }", -3.0);
    comp_chk("round(y7)", IFuncRound { modulus: InstructionI(0), of: InstructionI(1) }, "CompileSlab{ instrs:{ 0:IConst(1.0), 1:IVar(\"y7\") } }", 3.0);

    // IFuncMin
    comp_chk("min(2.7)", IConst(2.7), "CompileSlab{ instrs:{} }", 2.7);
    comp_chk("min(2.7, 3.7)", IConst(2.7), "CompileSlab{ instrs:{} }", 2.7);
    comp_chk("min(4.7, 3.7, 2.7)", IConst(2.7), "CompileSlab{ instrs:{} }", 2.7);
    comp_chk("min(y7)", IVar("y7".to_string()), "CompileSlab{ instrs:{} }", 2.7);
    comp_chk("min(4.7, y7, 3.7)", IFuncMin(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"y7\"), 1:IConst(3.7) } }", 2.7);
    comp_chk("min(3.7, y7, 4.7)", IFuncMin(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"y7\"), 1:IConst(3.7) } }", 2.7);

    // IFuncMax
    comp_chk("max(2.7)", IConst(2.7), "CompileSlab{ instrs:{} }", 2.7);
    comp_chk("max(2.7, 1.7)", IConst(2.7), "CompileSlab{ instrs:{} }", 2.7);
    comp_chk("max(0.7, 1.7, 2.7)", IConst(2.7), "CompileSlab{ instrs:{} }", 2.7);
    comp_chk("max(y7)", IVar("y7".to_string()), "CompileSlab{ instrs:{} }", 2.7);
    comp_chk("max(0.7, y7, 1.7)", IFuncMax(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"y7\"), 1:IConst(1.7) } }", 2.7);
    comp_chk("max(1.7, y7, 0.7)", IFuncMax(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"y7\"), 1:IConst(1.7) } }", 2.7);

    // IFuncSin
    comp_chk("sin(0)", IConst(0.0), "CompileSlab{ instrs:{} }", 0.0);
    comp_chk("round(0.000001, sin(pi()))", IConst(0.0), "CompileSlab{ instrs:{} }", 0.0);
    comp_chk("sin(pi()/2)", IConst(1.0), "CompileSlab{ instrs:{} }", 1.0);
    comp_chk("sin(w)", IFuncSin(InstructionI(0)), "CompileSlab{ instrs:{ 0:IVar(\"w\") } }", 0.0);
    comp_chk("sin(pi()/y)", IFuncSin(InstructionI(3)), "CompileSlab{ instrs:{ 0:IVar(\"y\"), 1:IInv(InstructionI(0)), 2:IConst(3.141592653589793), 3:IMul(InstructionI(1), InstructionI(2)) } }", 1.0);

    // IFuncCos
    comp_chk("cos(0)", IConst(1.0), "CompileSlab{ instrs:{} }", 1.0);
    comp_chk("cos(pi())", IConst(-1.0), "CompileSlab{ instrs:{} }", -1.0);
    comp_chk("round(0.000001, cos(pi()/2))", IConst(0.0), "CompileSlab{ instrs:{} }", 0.0);
    comp_chk("cos(w)", IFuncCos(InstructionI(0)), "CompileSlab{ instrs:{ 0:IVar(\"w\") } }", 1.0);
    comp_chk("round(0.000001, cos(pi()/y))", IFuncRound { modulus: InstructionI(4), of: InstructionI(5) }, "CompileSlab{ instrs:{ 0:IVar(\"y\"), 1:IInv(InstructionI(0)), 2:IConst(3.141592653589793), 3:IMul(InstructionI(1), InstructionI(2)), 4:IConst(0.000001), 5:IFuncCos(InstructionI(3)) } }", 0.0);

    // IFuncTan
    comp_chk("tan(0)", IConst(0.0), "CompileSlab{ instrs:{} }", 0.0);
    comp_chk("tan(w)", IFuncTan(InstructionI(0)), "CompileSlab{ instrs:{ 0:IVar(\"w\") } }", 0.0);

    // IFuncASin
    comp_chk("asin(0)", IConst(0.0), "CompileSlab{ instrs:{} }", 0.0);
    comp_chk("asin(w)", IFuncASin(InstructionI(0)), "CompileSlab{ instrs:{ 0:IVar(\"w\") } }", 0.0);

    // IFuncACos
    comp_chk("acos(0)", IConst(1.5707963267948966), "CompileSlab{ instrs:{} }", 1.5707963267948966);
    comp_chk("acos(w)", IFuncACos(InstructionI(0)), "CompileSlab{ instrs:{ 0:IVar(\"w\") } }", 1.5707963267948966);

    // IFuncATan
    comp_chk("atan(0)", IConst(0.0), "CompileSlab{ instrs:{} }", 0.0);
    comp_chk("atan(w)", IFuncATan(InstructionI(0)), "CompileSlab{ instrs:{ 0:IVar(\"w\") } }", 0.0);

    // IFuncSinH
    comp_chk("sinh(0)", IConst(0.0), "CompileSlab{ instrs:{} }", 0.0);
    comp_chk("sinh(w)", IFuncSinH(InstructionI(0)), "CompileSlab{ instrs:{ 0:IVar(\"w\") } }", 0.0);

    // IFuncCosH
    comp_chk("cosh(0)", IConst(1.0), "CompileSlab{ instrs:{} }", 1.0);
    comp_chk("cosh(w)", IFuncCosH(InstructionI(0)), "CompileSlab{ instrs:{ 0:IVar(\"w\") } }", 1.0);

    // IFuncTanH
    comp_chk("tanh(0)", IConst(0.0), "CompileSlab{ instrs:{} }", 0.0);
    comp_chk("tanh(w)", IFuncTanH(InstructionI(0)), "CompileSlab{ instrs:{ 0:IVar(\"w\") } }", 0.0);

    // IFuncASinH
    comp_chk("asinh(0)", IConst(0.0), "CompileSlab{ instrs:{} }", 0.0);
    comp_chk("asinh(w)", IFuncASinH(InstructionI(0)), "CompileSlab{ instrs:{ 0:IVar(\"w\") } }", 0.0);

    // IFuncACosH
    comp_chk("acosh(1)", IConst(0.0), "CompileSlab{ instrs:{} }", 0.0);
    comp_chk("acosh(x)", IFuncACosH(InstructionI(0)), "CompileSlab{ instrs:{ 0:IVar(\"x\") } }", 0.0);

    // IFuncATanH
    comp_chk("atanh(0)", IConst(0.0), "CompileSlab{ instrs:{} }", 0.0);
    comp_chk("atanh(w)", IFuncATanH(InstructionI(0)), "CompileSlab{ instrs:{ 0:IVar(\"w\") } }", 0.0);

    // IPrintFunc
    comp_chk(r#"print("test",1.23)"#, IPrintFunc(PrintFunc(vec![EStr("test".to_string()), EExpr(ExpressionI(0))])), "CompileSlab{ instrs:{} }", 1.23);
}

#[test]
fn custom_func() {
    comp_chk("x + 1", IAdd(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IVar(\"x\"), 1:IConst(1.0) } }", 2.0);

    comp_chk("x() + 1", IAdd(InstructionI(0), InstructionI(1)), "CompileSlab{ instrs:{ 0:IFunc { name: \"x\", args: [] }, 1:IConst(1.0) } }", 2.0);

    comp_chk("x(1,2,3) + 1", IAdd(InstructionI(3), InstructionI(4)), "CompileSlab{ instrs:{ 0:IConst(1.0), 1:IConst(2.0), 2:IConst(3.0), 3:IFunc { name: \"x\", args: [InstructionI(0), InstructionI(1), InstructionI(2)] }, 4:IConst(1.0) } }", 2.0);

    comp_chk("x(1, 1+1, 1+1+1) + 1", IAdd(InstructionI(3), InstructionI(4)), "CompileSlab{ instrs:{ 0:IConst(1.0), 1:IConst(2.0), 2:IConst(3.0), 3:IFunc { name: \"x\", args: [InstructionI(0), InstructionI(1), InstructionI(2)] }, 4:IConst(1.0) } }", 2.0);
}

#[test]
fn eval_macro() {
    fn wrapped() -> Result<(),Error> {
        let mut ns = EmptyNamespace;
        let mut slab = Slab::new();

        let expr = parse("5", &mut slab.ps).unwrap().from(&slab.ps);
        let instr = expr.compile(&slab.ps, &mut slab.cs);
        assert_eq!(eval_compiled_ref!(&instr, &slab, &mut ns), 5.0);
        (|| -> Result<(),Error> {
            assert_eq!(eval_compiled_ref!(&instr, &slab, &mut ns), 5.0);
            Ok(())
        })().unwrap();
        assert_eq!(eval_compiled!(instr, &slab, &mut ns), 5.0);

        #[cfg(feature="unsafe-vars")]
        {
            let x = 1.0;
            unsafe { slab.ps.add_unsafe_var("x".to_string(), &x) }
            let expr = parse("x", {slab.clear(); &mut slab.ps}).unwrap().from(&slab.ps);
            let instr = expr.compile(&slab.ps, &mut slab.cs);
            assert_eq!(eval_compiled_ref!(&instr, &slab, &mut ns), 1.0);
            (|| -> Result<(),Error> {
                assert_eq!(eval_compiled_ref!(&instr, &slab, &mut ns), 1.0);
                Ok(())
            })().unwrap();
            assert_eq!(eval_compiled!(instr, &slab, &mut ns), 1.0);
        }

        Ok(())
    }

    wrapped().unwrap();
}

