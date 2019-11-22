use crate::slab::{ParseSlab, CompileSlab};
use crate::parser::{Expression, ExprPair, Value, Variable, UnaryOp, BinaryOp};
use crate::evaler::bool_to_f64;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct InstructionI(pub usize);

#[derive(Debug, PartialEq)]
pub enum Instruction {
    //---- Primitive Value Types:
    IConst(f64),
    IVar(Variable),

    //---- Unary Ops:
    // Parens is a noop
    // Pos is a noop
    INeg(InstructionI),
    INot(InstructionI),
    IInv(InstructionI),

    //---- Binary Math Ops:
    IAdd(Vec<InstructionI>),
    // A Sub(x) is converted to an Add(Neg(x)).
    IMul(Vec<InstructionI>),
    // A Div(n,d) is converted to a Mul(n,Inv(d)).
    IMod{dividend:InstructionI, divisor:InstructionI},
    IExp{base:InstructionI, pow:InstructionI},

    //---- Binary Comparison Ops:
    ILT(InstructionI, InstructionI),
    ILTE(InstructionI, InstructionI),
    IEQ(InstructionI, InstructionI),
    INE(InstructionI, InstructionI),
    IGTE(InstructionI, InstructionI),
    IGT(InstructionI, InstructionI),

    //---- Binary Logic Ops:
    IOR(Vec<InstructionI>),
    IAND(Vec<InstructionI>),

    //---- Callables:
    IFuncInt(InstructionI),
    IFuncCeil(InstructionI),
    IFuncFloor(InstructionI),
    IFuncAbs(InstructionI),
    IFuncLog{base:InstructionI, of:InstructionI},
    IFuncRound{modulus:InstructionI, of:InstructionI},
    IFuncMin(Vec<InstructionI>),
    IFuncMax(Vec<InstructionI>),
    IFuncSin(InstructionI),
    IFuncCos(InstructionI),
    IFuncTan(InstructionI),
    IFuncASin(InstructionI),
    IFuncACos(InstructionI),
    IFuncATan(InstructionI),
    IFuncSinH(InstructionI),
    IFuncCosH(InstructionI),
    IFuncTanH(InstructionI),

    IPrintFunc(Vec<InstructionOrString>),
    IEvalFunc{expr:InstructionI, kwargs:Vec<KWArg>},
}
use Instruction::{IConst, IVar, INeg, INot, IInv, IAdd, IMul, IMod, IExp, ILT, ILTE, IEQ, INE, IGTE, IGT, IOR, IAND, IFuncInt, IFuncCeil, IFuncFloor, IFuncAbs, IFuncLog, IFuncRound, IFuncMin, IFuncMax, IFuncSin, IFuncCos, IFuncTan, IFuncASin, IFuncACos, IFuncATan, IFuncSinH, IFuncCosH, IFuncTanH, IPrintFunc, IEvalFunc};

#[derive(Debug, PartialEq)]
pub enum InstructionOrString {
    EInstr(InstructionI),
    EStr(String),
}

#[derive(Debug, PartialEq)]
pub struct KWArg {
    pub name: Variable,
    pub instr: InstructionI,
}



pub trait Compiler {
    fn compile(&self, pslab:&ParseSlab, cslab:&mut CompileSlab) -> Instruction;
}


#[derive(Debug)]
struct ExprSlice<'s> {
    first: &'s Value,
    pairs: Vec<&'s ExprPair>,
}
impl<'s> ExprSlice<'s> {
    fn new<'a>(first:&'a Value) -> ExprSlice<'a> {
        ExprSlice{
            first:first,
            pairs:Vec::with_capacity(8),
        }
    }
    fn from_expr<'x>(expr:&'x Expression) -> ExprSlice<'x> {
        let mut sl = ExprSlice::new(&/*'x*/ expr.first);  // possible???
        for exprpairref in expr.pairs.iter() { sl.pairs.push(exprpairref) }
        sl
    }
    fn split(&self, bop:BinaryOp, dst:&mut Vec<ExprSlice<'s>>) {
        dst.push(ExprSlice::new(&self.first));
        let mut cur = dst.last_mut().unwrap();
        for exprpair in self.pairs.iter() {
            if exprpair.0==bop {
                dst.push(ExprSlice::new(&exprpair.1));
                cur = dst.last_mut().unwrap();
            } else {
                cur.pairs.push(exprpair);
            }
        }
    }
}

fn neg_wrap(instr:Instruction, cslab:&mut CompileSlab) -> Instruction {
    if let INeg(i) = instr {
        cslab.take_instr(i)
    } else {
        INeg(cslab.push_instr(instr))
    }
}
fn not_wrap(instr:Instruction, cslab:&mut CompileSlab) -> Instruction {
    if let INot(i) = instr {
        cslab.take_instr(i)
    } else {
        INot(cslab.push_instr(instr))
    }
}
impl Compiler for ExprSlice<'_> {
    fn compile(&self, pslab:&ParseSlab, cslab:&mut CompileSlab) -> Instruction {
        if self.pairs.len()==0 {
            return self.first.compile(pslab, cslab);
        }

        // Find the lowest-priority BinaryOp:
        let mut lowest_op = self.pairs[0].0;
        for exprpair in self.pairs.iter() {
            if exprpair.0<lowest_op { lowest_op=exprpair.0 }
        }

        match lowest_op {
            BinaryOp::EOR => todo!(),
            BinaryOp::EAND => todo!(),
            BinaryOp::ENE => todo!(),
            BinaryOp::EEQ => todo!(),
            BinaryOp::EGTE => todo!(),
            BinaryOp::ELTE => todo!(),
            BinaryOp::EGT => todo!(),
            BinaryOp::ELT => todo!(),
            BinaryOp::EPlus => {
                let mut xss = Vec::<ExprSlice>::with_capacity(8);
                self.split(BinaryOp::EPlus, &mut xss);
                let mut instrs = Vec::<Instruction>::with_capacity(xss.len());
                let mut const_sum = 0.0;
                for xs in xss.iter() {
                    let instr = xs.compile(pslab, cslab);
                    if let IConst(c) = instr {
                        const_sum += c;
                    } else {
                        instrs.push(instr);
                    }
                }
                if const_sum!=0.0 { instrs.push(IConst(const_sum)); }
                
                if instrs.len()==0 { return IConst(0.0); }
                if instrs.len()==1 { return instrs.pop().unwrap(); }
                let mut instris = Vec::<InstructionI>::with_capacity(instrs.len());
                for instr in instrs.into_iter() {
                    instris.push(cslab.push_instr(instr));
                }
                IAdd(instris)
            }
            BinaryOp::EMinus => {
                let mut xss = Vec::<ExprSlice>::with_capacity(8);
                self.split(BinaryOp::EMinus, &mut xss);
                let mut instrs = Vec::<Instruction>::with_capacity(xss.len());
                let mut const_sum = 0.0;
                let mut is_first = true;
                for xs in xss.iter() {
                    let instr = xs.compile(pslab, cslab);
                    if let IConst(c) = instr {
                        if is_first {
                            const_sum += c;
                        } else {
                            const_sum -= c;
                        }
                    } else {
                        if is_first {
                            instrs.push(instr);
                        } else {
                            instrs.push(neg_wrap(instr,cslab));
                        }
                    }
                    is_first = false;
                }
                if const_sum!=0.0 { instrs.push(IConst(const_sum)); }

                if instrs.len()==0 { return IConst(0.0); }
                if instrs.len()==1 { return instrs.pop().unwrap(); }
                let mut instris = Vec::<InstructionI>::with_capacity(instrs.len());
                for instr in instrs.into_iter() {
                    instris.push(cslab.push_instr(instr));
                }
                IAdd(instris)
            }
            BinaryOp::EMul => {
                let mut xss = Vec::<ExprSlice>::with_capacity(8);
                self.split(BinaryOp::EMul, &mut xss);
                let mut instrs = Vec::<Instruction>::with_capacity(xss.len());
                let mut const_prod = 1.0;
                for xs in xss.iter() {
                    let instr = xs.compile(pslab, cslab);
                    if let IConst(c) = instr {
                        const_prod *= c;
                    } else {
                        instrs.push(instr);
                    }
                }
                if const_prod!=1.0 { instrs.push(IConst(const_prod)); }

                if instrs.len()==0 { return IConst(1.0); }
                if instrs.len()==1 { return instrs.pop().unwrap(); }
                let mut instris = Vec::<InstructionI>::with_capacity(instrs.len());
                for instr in instrs.into_iter() {
                    instris.push(cslab.push_instr(instr));
                }
                IMul(instris)
            }
            BinaryOp::EDiv => {
                let mut xss = Vec::<ExprSlice>::with_capacity(8);
                self.split(BinaryOp::EDiv, &mut xss);
                let mut instrs = Vec::<Instruction>::with_capacity(xss.len());
                let mut const_prod = 1.0;
                let mut is_first = true;
                for xs in xss.iter() {
                    let instr = xs.compile(pslab, cslab);
                    if let IConst(c) = instr {
                        if is_first {
                            const_prod *= c;
                        } else {
                            const_prod /= c;
                        }
                    } else {
                        if is_first {
                            instrs.push(instr);
                        } else {
                            instrs.push(IInv(cslab.push_instr(instr)));
                        }
                    }
                    is_first = false;
                }
                if const_prod!=1.0 { instrs.push(IConst(const_prod)); }

                if instrs.len()==0 { return IConst(1.0); }
                if instrs.len()==1 { return instrs.pop().unwrap(); }
                let mut instris = Vec::<InstructionI>::with_capacity(instrs.len());
                for instr in instrs.into_iter() {
                    instris.push(cslab.push_instr(instr));
                }
                IMul(instris)
            }
            BinaryOp::EMod => {
                let mut xss = Vec::<ExprSlice>::with_capacity(2);
                self.split(BinaryOp::EMod, &mut xss);
                if xss.len()!=2 { unreachable!(); }
                let divisor = xss.pop().unwrap().compile(pslab, cslab);
                let dividend = xss.pop().unwrap().compile(pslab, cslab);
                if let IConst(dr) = divisor {       // let_chains aren't working yet.
                    if let IConst(dd) = dividend {  //
                        return IConst(dd%dr);
                    }
                }
                return IMod{dividend:cslab.push_instr(dividend), divisor:cslab.push_instr(divisor)};
            }
            BinaryOp::EExp => todo!(),
        }
    }
}

impl Compiler for Expression {
    fn compile(&self, pslab:&ParseSlab, cslab:&mut CompileSlab) -> Instruction {
        let top = ExprSlice::from_expr(&self);
        top.compile(pslab, cslab)
    }
}

impl Compiler for Value {
    fn compile(&self, pslab:&ParseSlab, cslab:&mut CompileSlab) -> Instruction {
        match self {
            Value::EConstant(c) => IConst(c.0),
            Value::EVariable(v) => IVar(Variable(v.0.clone())),
            Value::EUnaryOp(u) => {
                match u {
                    UnaryOp::EPos(i) => pslab.get_val(*i).compile(pslab,cslab),
                    UnaryOp::ENeg(i) => {
                        let instr = pslab.get_val(*i).compile(pslab,cslab);
                        if let IConst(c) = instr {
                            IConst(-c)
                        } else {
                            neg_wrap(instr,cslab)
                        }
                    }
                    UnaryOp::ENot(i) => {
                        let instr = pslab.get_val(*i).compile(pslab,cslab);
                        if let IConst(c) = instr {
                            IConst(bool_to_f64(c==0.0))
                        } else {
                            not_wrap(instr,cslab)
                        }
                    }
                    UnaryOp::EParens(i) => pslab.get_expr(*i).compile(pslab,cslab),
                }
            }
            Value::ECallable(f) => todo!(),
        }
    }
}

