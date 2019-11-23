use crate::slab::{ParseSlab, CompileSlab};
use crate::parser::{Expression, ExprPair, Value, Variable, UnaryOp, BinaryOp::{self, EOR, EAND, ENE, EEQ, EGTE, ELTE, EGT, ELT, EPlus, EMinus, EMul, EDiv, EMod, EExp}};
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
    IAdd(InstructionI, InstructionI),
    // A Sub(x) is converted to an Add(Neg(x)).
    IMul(InstructionI, InstructionI),
    // A Div(n,d) is converted to a Mul(n,Inv(d)).
    IMod{dividend:InstructionI, divisor:InstructionI},
    IExp{base:InstructionI, power:InstructionI},

    //---- Binary Comparison Ops:
    ILT(InstructionI, InstructionI),
    ILTE(InstructionI, InstructionI),
    IEQ(InstructionI, InstructionI),
    INE(InstructionI, InstructionI),
    IGTE(InstructionI, InstructionI),
    IGT(InstructionI, InstructionI),

    //---- Binary Logic Ops:
    IOR(InstructionI, InstructionI),
    IAND(InstructionI, InstructionI),

    //---- Callables:
    IFuncInt(InstructionI),
    IFuncCeil(InstructionI),
    IFuncFloor(InstructionI),
    IFuncAbs(InstructionI),
    IFuncLog{base:InstructionI, of:InstructionI},
    IFuncRound{modulus:InstructionI, of:InstructionI},
    IFuncMin(InstructionI, InstructionI),
    IFuncMax(InstructionI, InstructionI),
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
    fn split_multi(&self, search:&[BinaryOp], xsdst:&mut Vec<ExprSlice<'s>>, opdst:&mut Vec<&'s BinaryOp>) {
        xsdst.push(ExprSlice::new(&self.first));
        let mut cur = xsdst.last_mut().unwrap();
        for exprpair in self.pairs.iter() {
            if search.contains(&exprpair.0) {
                xsdst.push(ExprSlice::new(&exprpair.1));
                opdst.push(&exprpair.0);
                cur = xsdst.last_mut().unwrap();
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
fn inv_wrap(instr:Instruction, cslab:&mut CompileSlab) -> Instruction {
    if let IInv(i) = instr {
        cslab.take_instr(i)
    } else {
        IInv(cslab.push_instr(instr))
    }
}
impl Compiler for ExprSlice<'_> {
    fn compile(&self, pslab:&ParseSlab, cslab:&mut CompileSlab) -> Instruction {
        // Associative:  (2+3)+4 = 2+(3+4)
        // Commutative:  1+2 = 2+1
        //
        //          Only         Only
        // Neither  Associative  Commutative  Both
        // -------  -----------  -----------  ----
        // GTE      (none)       (none)       OR
        // LTE                                AND
        // GT                                 NE
        // LT                                 EQ
        // Minus (opt with neg & add)         Plus
        // Div (opt with inv & mul)           Mul
        // Mod
        // Exp (opt exponent with mul)

        if self.pairs.len()==0 {
            return self.first.compile(pslab, cslab);
        }

        // Find the lowest-priority BinaryOp:
        let mut lowest_op = self.pairs[0].0;
        for exprpair in self.pairs.iter() {
            if exprpair.0<lowest_op { lowest_op=exprpair.0 }
        }

        // All comparisons have equal precedence:
        if lowest_op==ELT || lowest_op==EGT || lowest_op==ELTE || lowest_op==EGTE {
            let mut ops = Vec::<&BinaryOp>::with_capacity(4);
            let mut xss = Vec::<ExprSlice>::with_capacity(ops.len()+1);
            self.split_multi(&[ELT, EGT, ELTE, EGTE], &mut xss, &mut ops);
            let mut out = xss[0].compile(pslab, cslab);
            for i in 0..ops.len() {
                let op = ops[i];
                let instr = xss[i+1].compile(pslab, cslab);
                if let IConst(l) = out {
                    if let IConst(r) = instr {
                        out = match op {
                            ELT => IConst(bool_to_f64(l<r)),
                            EGT => IConst(bool_to_f64(l>r)),
                            ELTE => IConst(bool_to_f64(l<=r)),
                            EGTE => IConst(bool_to_f64(l>=r)),
                            _ => unreachable!(),
                        };
                        continue;
                    }
                }
                out = match op {
                    ELT => ILT(cslab.push_instr(out), cslab.push_instr(instr)),
                    EGT => IGT(cslab.push_instr(out), cslab.push_instr(instr)),
                    ELTE => ILTE(cslab.push_instr(out), cslab.push_instr(instr)),
                    EGTE => IGTE(cslab.push_instr(out), cslab.push_instr(instr)),
                    _ => unreachable!(),
                }
            }
            return out;
        }

        // EQ and NE have equal precedence:
        if lowest_op==EEQ || lowest_op==ENE {
            let mut ops = Vec::<&BinaryOp>::with_capacity(4);
            let mut xss = Vec::<ExprSlice>::with_capacity(ops.len()+1);
            self.split_multi(&[EEQ, ENE], &mut xss, &mut ops);
            let mut out = xss[0].compile(pslab, cslab);
            for i in 0..ops.len() {
                let op = ops[i];
                let instr = xss[i+1].compile(pslab, cslab);
                if let IConst(l) = out {
                    if let IConst(r) = instr {
                        out = match op {
                            EEQ => IConst(bool_to_f64(l==r)),
                            ENE => IConst(bool_to_f64(l!=r)),
                            _ => unreachable!(),
                        };
                        continue;
                    }
                }
                out = match op {
                    EEQ => IEQ(cslab.push_instr(out), cslab.push_instr(instr)),
                    ENE => INE(cslab.push_instr(out), cslab.push_instr(instr)),
                    _ => unreachable!(),
                }
            }
            return out;
        }

        match lowest_op {
            EOR => {
                let mut xss = Vec::<ExprSlice>::with_capacity(4);
                self.split(EOR, &mut xss);
                let mut out = IConst(0.0); let mut out_set = false;
                for xs in xss.iter() {
                    let instr = xs.compile(pslab, cslab);
                    if out_set {
                        out = IOR(cslab.push_instr(out), cslab.push_instr(instr));
                    } else {
                        if let IConst(c) = instr {
                            if c!=0.0 { return instr; }
                            // out = instr;     // Skip this 0 value (mostly so I don't complicate my logic in 'if out_set').
                            // out_set = true;
                        } else {
                            out = instr;
                            out_set = true;
                        }
                    }
                }
                out
            }
            EAND => {
                let mut xss = Vec::<ExprSlice>::with_capacity(4);
                self.split(EAND, &mut xss);
                let mut out = IConst(1.0); let mut out_set = false;
                for xs in xss.iter() {
                    let instr = xs.compile(pslab, cslab);
                    if instr == IConst(0.0) { return instr; }
                    if out_set {
                        if let IConst(_) = out {
                            out = instr;
                        } else {
                            out = IAND(cslab.push_instr(out), cslab.push_instr(instr));
                        }
                    } else {
                        out = instr;
                        out_set = true;
                    }
                }
                out
            }
            ENE => unreachable!(),
            EEQ => unreachable!(),
            EGTE => unreachable!(),
            ELTE => unreachable!(),
            EGT => unreachable!(),
            ELT => unreachable!(),
            EPlus => {
                let mut xss = Vec::<ExprSlice>::with_capacity(4);
                self.split(EPlus, &mut xss);
                let mut out = IConst(0.0); let mut out_set = false;
                let mut const_sum = 0.0;
                for xs in xss.iter() {
                    let instr = xs.compile(pslab, cslab);
                    if let IConst(c) = instr {
                        const_sum += c;
                    } else {
                        if out_set {
                            out = IAdd(cslab.push_instr(out), cslab.push_instr(instr));
                        } else {
                            out = instr;
                            out_set = true;
                        }
                    }
                }
                if const_sum!=0.0 {
                    if out_set {
                        out = IAdd(cslab.push_instr(out), cslab.push_instr(IConst(const_sum)));
                    } else {
                        out = IConst(const_sum);
                    }
                }
                out
            }
            EMinus => {
                let mut xss = Vec::<ExprSlice>::with_capacity(4);
                self.split(EMinus, &mut xss);
                let mut out = IConst(0.0); let mut out_set = false;
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
                            if out_set {
                                out = IAdd(cslab.push_instr(out), cslab.push_instr(instr));
                            } else {
                                out = instr;
                                out_set = true;
                            }
                        } else {
                            let instr = neg_wrap(instr,cslab);
                            if out_set {
                                out = IAdd(cslab.push_instr(out), cslab.push_instr(instr));
                            } else {
                                out = instr;
                                out_set = true;
                            }
                        }
                    }
                    is_first = false;
                }
                if const_sum!=0.0 {
                    if out_set {
                        out = IAdd(cslab.push_instr(out), cslab.push_instr(IConst(const_sum)));
                    } else {
                        out = IConst(const_sum);
                    }
                }
                out
            }
            EMul => {
                let mut xss = Vec::<ExprSlice>::with_capacity(4);
                self.split(EMul, &mut xss);
                let mut out = IConst(1.0); let mut out_set = false;
                let mut const_prod = 1.0;
                for xs in xss.iter() {
                    let instr = xs.compile(pslab, cslab);
                    if let IConst(c) = instr {
                        const_prod *= c;
                    } else {
                        if out_set {
                            out = IMul(cslab.push_instr(out), cslab.push_instr(instr));
                        } else {
                            out = instr;
                            out_set = true;
                        }
                    }
                }
                if const_prod!=1.0 {
                    if out_set {
                        out = IMul(cslab.push_instr(out), cslab.push_instr(IConst(const_prod)));
                    } else {
                        out = IConst(const_prod);
                    }
                }
                out
            }
            EDiv => {
                let mut xss = Vec::<ExprSlice>::with_capacity(4);
                self.split(EDiv, &mut xss);
                let mut out = IConst(1.0); let mut out_set = false;
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
                            if out_set {
                                out = IMul(cslab.push_instr(out), cslab.push_instr(instr));
                            } else {
                                out = instr;
                                out_set = true;
                            }
                        } else {
                            let instr = inv_wrap(instr,cslab);
                            if out_set {
                                out = IMul(cslab.push_instr(out), cslab.push_instr(instr));
                            } else {
                                out = instr;
                                out_set = true;
                            }
                        }
                    }
                    is_first = false;
                }
                if const_prod!=1.0 {
                    if out_set {
                        out = IMul(cslab.push_instr(out), cslab.push_instr(IConst(const_prod)));
                    } else {
                        out = IConst(const_prod);
                    }
                }
                out
            }
            EMod => {
                let mut xss = Vec::<ExprSlice>::with_capacity(2);
                self.split(EMod, &mut xss);
                if xss.len()!=2 { unreachable!(); }
                let divisor = xss.pop().unwrap().compile(pslab, cslab);
                let dividend = xss.pop().unwrap().compile(pslab, cslab);
                if let IConst(dr) = divisor {       // let_chains aren't working yet.
                    if let IConst(dd) = dividend {  //
                        return IConst(dd%dr);
                    }
                }
                IMod{dividend:cslab.push_instr(dividend), divisor:cslab.push_instr(divisor)}
            }
            EExp => {
                let mut xss = Vec::<ExprSlice>::with_capacity(2);
                self.split(EExp, &mut xss);
                if xss.len()!=2 { unreachable!(); }
                let power = xss.pop().unwrap().compile(pslab, cslab);
                let base = xss.pop().unwrap().compile(pslab, cslab);
                if let IConst(p) = power {     // let_chains aren't working yet.
                    if let IConst(b) = base {  //
                        return IConst(b.powf(p));
                    }
                }
                IExp{base:cslab.push_instr(base), power:cslab.push_instr(power)}
            }
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

