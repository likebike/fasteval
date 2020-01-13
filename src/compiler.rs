//! This module compiles parsed `Expression`s into an optimized AST node called an `Instruction`.
//! The compiled form is much faster, especially for constants.
//!
//! # Compile-time Optimizations
//!
//! ## Constant Folding
//! Operations with constants can be calculated at compile time so
//! they don't need to be calculated during `eval()`.
//!
//! For example, `1 + x + 1` will be compiled into `x + 2`, saving some time during `eval()`.
//!
//! If the entire expression only consists of constants (no variables),
//! then the expression can be reduced to a final result at compile time,
//! resulting in near-native speed during `eval()`.
//!
//! ## Algebraic Simplification
//! * Subtraction is converted to Addition.
//! * Division is converted to Multiplication.
//! * Built-in functions with constant arguments are evaluated.
//! * Constant terms are combined.
//! * Logical operator short-circuits are applied and no-op branches are discarded.
//!
//! ## Optimized Memory Layout and Execution
//! * Variable-length `Expression`/`Value` AST nodes are converted into constant-sized `Instruction` nodes.
//! * The `IC` enumeration helps to eliminate expensive function calls.



use crate::slab::{ParseSlab, CompileSlab};
use crate::parser::{Expression, ExprPair, Value, UnaryOp::{self, EPos, ENeg, ENot, EParentheses}, BinaryOp::{self, EOR, EAND, ENE, EEQ, EGTE, ELTE, EGT, ELT, EAdd, ESub, EMul, EDiv, EMod, EExp}, StdFunc::{self, EVar, EFunc, EFuncInt, EFuncCeil, EFuncFloor, EFuncAbs, EFuncSign, EFuncLog, EFuncRound, EFuncMin, EFuncMax, EFuncE, EFuncPi, EFuncSin, EFuncCos, EFuncTan, EFuncASin, EFuncACos, EFuncATan, EFuncSinH, EFuncCosH, EFuncTanH, EFuncASinH, EFuncACosH, EFuncATanH}, PrintFunc};
#[cfg(feature="unsafe-vars")]
use crate::parser::StdFunc::EUnsafeVar;


/// `true` --> `1.0`,  `false` --> `0.0`
#[macro_export]
macro_rules! bool_to_f64 {
    ($b:expr) => {
        if $b { 1.0 }
        else { 0.0 }
    };
}


/// An `InstructionI` represents an index into `Slab.cs.instrs`.
///
/// It behaves much like a pointer or reference, but it is 'safe' (unlike a raw
/// pointer) and is not managed by the Rust borrow checker (unlike a reference).
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct InstructionI(pub usize);

/// This enumeration boosts performance because it eliminates expensive function calls for constant values.
#[derive(Debug, PartialEq)]
pub enum IC {
    I(InstructionI),
    C(f64),
}

macro_rules! instr_to_ic {
    ($cslab:ident, $instr:ident) => {
        match $instr {
            IConst(c) => IC::C(c),
            _ => IC::I($cslab.push_instr($instr)),
        }
    }
}
macro_rules! ic_to_instr {
    ($cslab:expr, $dst:ident, $ic:ident) => {
        match $ic {
            IC::C(c) => {
                $dst = IConst(*c);
                &$dst
            }
            IC::I(i) => get_instr!($cslab,i),
        }
    }
}

/// An `Instruction` is an optimized AST node resulting from compilation.
#[derive(Debug, PartialEq)]
pub enum Instruction {
    //---- Primitive Value Types:
    IConst(f64),

    //---- Unary Ops:
    // Parentheses is a noop
    // Pos is a noop
    INeg(InstructionI),
    INot(InstructionI),
    IInv(InstructionI),

    //---- Binary Math Ops:
    IAdd(InstructionI, IC),
    // A Sub(x) is converted to an Add(Neg(x)).
    IMul(InstructionI, IC),
    // A Div(n,d) is converted to a Mul(n,Inv(d)).
    IMod{dividend:IC, divisor:IC},
    IExp{base:IC, power:IC},

    //---- Binary Comparison Ops:
    ILT(IC, IC),
    ILTE(IC, IC),
    IEQ(IC, IC),
    INE(IC, IC),
    IGTE(IC, IC),
    IGT(IC, IC),

    //---- Binary Logic Ops:
    IOR(InstructionI, IC),
    IAND(InstructionI, IC),

    //---- Callables:
    IVar(String),
    #[cfg(feature="unsafe-vars")]
    IUnsafeVar{name:String, ptr:*const f64},
    IFunc{name:String, args:Vec<IC>},

    IFuncInt(InstructionI),
    IFuncCeil(InstructionI),
    IFuncFloor(InstructionI),
    IFuncAbs(InstructionI),
    IFuncSign(InstructionI),
    IFuncLog{base:IC, of:IC},
    IFuncRound{modulus:IC, of:IC},
    IFuncMin(InstructionI, IC),
    IFuncMax(InstructionI, IC),

    IFuncSin(InstructionI),
    IFuncCos(InstructionI),
    IFuncTan(InstructionI),
    IFuncASin(InstructionI),
    IFuncACos(InstructionI),
    IFuncATan(InstructionI),
    IFuncSinH(InstructionI),
    IFuncCosH(InstructionI),
    IFuncTanH(InstructionI),
    IFuncASinH(InstructionI),
    IFuncACosH(InstructionI),
    IFuncATanH(InstructionI),

    IPrintFunc(PrintFunc),  // Not optimized (it would be pointless because of i/o bottleneck).
}
use Instruction::{IConst, INeg, INot, IInv, IAdd, IMul, IMod, IExp, ILT, ILTE, IEQ, INE, IGTE, IGT, IOR, IAND, IVar, IFunc, IFuncInt, IFuncCeil, IFuncFloor, IFuncAbs, IFuncSign, IFuncLog, IFuncRound, IFuncMin, IFuncMax, IFuncSin, IFuncCos, IFuncTan, IFuncASin, IFuncACos, IFuncATan, IFuncSinH, IFuncCosH, IFuncTanH, IFuncASinH, IFuncACosH, IFuncATanH, IPrintFunc};
#[cfg(feature="unsafe-vars")]
use Instruction::IUnsafeVar;

impl Default for Instruction {
    fn default() -> Self { IConst(std::f64::NAN) }
}


/// You must `use` the `Compiler` trait before you can call `.compile()` on parsed `Expression`s.
pub trait Compiler {
    /// Turns a parsed `Expression` into a compiled `Instruction`.
    ///
    /// Cannot fail, unless you run out of memory.
    fn compile(&self, pslab:&ParseSlab, cslab:&mut CompileSlab) -> Instruction;
}


#[derive(Debug)]
struct ExprSlice<'s> {
    first: &'s Value,
    pairs: Vec<&'s ExprPair>,
}
impl<'s> ExprSlice<'s> {
    fn new(first:&Value) -> ExprSlice<'_> {
        ExprSlice{
            first,
            pairs:Vec::with_capacity(8),
        }
    }
    fn from_expr(expr:&Expression) -> ExprSlice<'_> {
        let mut sl = ExprSlice::new(&expr.first);
        for exprpairref in expr.pairs.iter() { sl.pairs.push(exprpairref) }
        sl
    }
    fn split(&self, bop:BinaryOp, dst:&mut Vec<ExprSlice<'s>>) {
        dst.push(ExprSlice::new(&self.first));
        for exprpair in self.pairs.iter() {
            if exprpair.0==bop {
                dst.push(ExprSlice::new(&exprpair.1));
            } else {
                match dst.last_mut() {
                    Some(cur) => cur.pairs.push(exprpair),
                    None => (),  // unreachable
                }
            }
        }
    }
    fn split_multi(&self, search:&[BinaryOp], xsdst:&mut Vec<ExprSlice<'s>>, opdst:&mut Vec<&'s BinaryOp>) {
        xsdst.push(ExprSlice::new(&self.first));
        for exprpair in self.pairs.iter() {
            if search.contains(&exprpair.0) {
                xsdst.push(ExprSlice::new(&exprpair.1));
                opdst.push(&exprpair.0);
            } else {
                match xsdst.last_mut() {
                    Some(cur) => cur.pairs.push(exprpair),
                    None => (),  // unreachable
                }
            }
        }
    }
}

/// Uses `EPSILON` to determine equality of two `f64`s.
#[macro_export]
macro_rules! f64_eq {
    ($l:ident, $r:literal) => {
        ($l-$r).abs() <= 8.0*std::f64::EPSILON
    };
    ($l:ident, $r:ident) => {
        ($l-$r).abs() <= 8.0*std::f64::EPSILON
    };
    ($l:expr, $r:literal) => {
        ($l-$r).abs() <= 8.0*std::f64::EPSILON
    };
    ($l:expr, $r:expr) => {
        (($l)-($r)).abs() <= 8.0*std::f64::EPSILON
    };
}

/// Uses `EPSILON` to determine inequality of two `f64`s.
///
/// This is exactly the same as saying `!f64_eq(x,y)` but it is slightly more efficient.
#[macro_export]
macro_rules! f64_ne {
    ($l:ident, $r:literal) => {
        ($l-$r).abs() > 8.0*std::f64::EPSILON
    };
    ($l:ident, $r:ident) => {
        ($l-$r).abs() > 8.0*std::f64::EPSILON
    };
    ($l:expr, $r:literal) => {
        ($l-$r).abs() > 8.0*std::f64::EPSILON
    };
    ($l:expr, $r:expr) => {
        (($l)-($r)).abs() > 8.0*std::f64::EPSILON
    };
}
fn neg_wrap(instr:Instruction, cslab:&mut CompileSlab) -> Instruction {
    if let IConst(c) = instr {
        IConst(-c)
    } else if let INeg(i) = instr {
        cslab.take_instr(i)
    } else {
        INeg(cslab.push_instr(instr))
    }
}
fn not_wrap(instr:Instruction, cslab:&mut CompileSlab) -> Instruction {
    if let IConst(c) = instr {
        IConst(bool_to_f64!(f64_eq!(c,0.0)))
    } else if let INot(i) = instr {
        cslab.take_instr(i)
    } else {
        INot(cslab.push_instr(instr))
    }
}
fn inv_wrap(instr:Instruction, cslab:&mut CompileSlab) -> Instruction {
    if let IConst(c) = instr {
        IConst(1.0/c)
    } else if let IInv(i) = instr {
        cslab.take_instr(i)
    } else {
        IInv(cslab.push_instr(instr))
    }
}
fn compile_mul(instrs:Vec<Instruction>, cslab:&mut CompileSlab) -> Instruction {
    let mut out = IConst(1.0); let mut out_set = false;
    let mut const_prod = 1.0;
    for instr in instrs {
        if let IConst(c) = instr {
            const_prod *= c;  // Floats don't overflow.
        } else {
            if out_set {
                out = IMul(cslab.push_instr(out), IC::I(cslab.push_instr(instr)));
            } else {
                out = instr;
                out_set = true;
            }
        }
    }
    if f64_ne!(const_prod,1.0) {
        if out_set {
            out = IMul(cslab.push_instr(out), IC::C(const_prod));
        } else {
            out = IConst(const_prod);
        }
    }
    out
}
fn compile_add(instrs:Vec<Instruction>, cslab:&mut CompileSlab) -> Instruction {
    let mut out = IConst(0.0); let mut out_set = false;
    let mut const_sum = 0.0;
    for instr in instrs {
        if let IConst(c) = instr {
            const_sum += c;  // Floats don't overflow.
        } else {
            if out_set {
                out = IAdd(cslab.push_instr(out), IC::I(cslab.push_instr(instr)));
            } else {
                out = instr;
                out_set = true;
            }
        }
    }
    if f64_ne!(const_sum,0.0) {
        if out_set {
            out = IAdd(cslab.push_instr(out), IC::C(const_sum));
        } else {
            out = IConst(const_sum);
        }
    }
    out
}
pub(crate) fn log(base:f64, n:f64) -> f64 {
    // Can't use floating point in 'match' patterns.  :(
    if f64_eq!(base,2.0) { return n.log2(); }
    if f64_eq!(base,10.0) { return n.log10(); }
    n.log(base)
}

// Can't inline recursive functions:
fn push_mul_leaves(instrs:&mut Vec<Instruction>, cslab:&mut CompileSlab, li:InstructionI, ric:IC) {
    // Take 'r' before 'l' for a chance for more efficient memory usage:
    match ric {
        IC::I(ri) => {
            let instr = cslab.take_instr(ri);
            if let IMul(rli,rric) = instr {
                push_mul_leaves(instrs,cslab,rli,rric);
            } else {
                instrs.push(instr);
            }
        }
        IC::C(c) => instrs.push(IConst(c)),
    };

    let instr = cslab.take_instr(li);
    if let IMul(lli,lric) = instr {
        push_mul_leaves(instrs,cslab,lli,lric);
    } else {
        instrs.push(instr);
    }
}
fn push_add_leaves(instrs:&mut Vec<Instruction>, cslab:&mut CompileSlab, li:InstructionI, ric:IC) {
    // Take 'r' before 'l' for a chance for more efficient memory usage:
    match ric {
        IC::I(ri) => {
            let instr = cslab.take_instr(ri);
            if let IAdd(rli,rric) = instr {
                push_add_leaves(instrs,cslab,rli,rric);
            } else {
                instrs.push(instr);
            }
        }
        IC::C(c) => instrs.push(IConst(c)),
    };

    let instr = cslab.take_instr(li);
    if let IAdd(lli,lric) = instr {
        push_add_leaves(instrs,cslab,lli,lric);
    } else {
        instrs.push(instr);
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
        // Exp

        // Find the lowest-priority BinaryOp:
        let mut lowest_op = match self.pairs.first() {
            Some(p0) => p0.0,
            None => return self.first.compile(pslab,cslab),
        };
        for exprpair in self.pairs.iter() {
            if exprpair.0<lowest_op { lowest_op=exprpair.0 }
        }

        // All comparisons have equal precedence:
        if lowest_op==EEQ || lowest_op==ENE || lowest_op==ELT || lowest_op==EGT || lowest_op==ELTE || lowest_op==EGTE {
            let mut ops = Vec::<&BinaryOp>::with_capacity(4);
            let mut xss = Vec::<ExprSlice>::with_capacity(ops.len()+1);
            self.split_multi(&[EEQ, ENE, ELT, EGT, ELTE, EGTE], &mut xss, &mut ops);
            let mut out = match xss.first() {
                Some(xs) => xs.compile(pslab,cslab),
                None => IConst(std::f64::NAN),  // unreachable
            };
            for (i,op) in ops.into_iter().enumerate() {
                let instr = match xss.get(i+1) {
                    Some(xs) => xs.compile(pslab,cslab),
                    None => IConst(std::f64::NAN),  // unreachable
                };
                if let IConst(l) = out {
                    if let IConst(r) = instr {
                        out = match op {
                            EEQ => IConst(bool_to_f64!(f64_eq!(l,r))),
                            ENE => IConst(bool_to_f64!(f64_ne!(l,r))),
                            ELT => IConst(bool_to_f64!(l<r)),
                            EGT => IConst(bool_to_f64!(l>r)),
                            ELTE => IConst(bool_to_f64!(l<=r)),
                            EGTE => IConst(bool_to_f64!(l>=r)),
                            _ => IConst(std::f64::NAN),  // unreachable
                        };
                        continue;
                    }
                }
                out = match op {
                    EEQ => IEQ(instr_to_ic!(cslab,out), instr_to_ic!(cslab,instr)),
                    ENE => INE(instr_to_ic!(cslab,out), instr_to_ic!(cslab,instr)),
                    ELT => ILT(instr_to_ic!(cslab,out), instr_to_ic!(cslab,instr)),
                    EGT => IGT(instr_to_ic!(cslab,out), instr_to_ic!(cslab,instr)),
                    ELTE => ILTE(instr_to_ic!(cslab,out), instr_to_ic!(cslab,instr)),
                    EGTE => IGTE(instr_to_ic!(cslab,out), instr_to_ic!(cslab,instr)),
                    _ => IConst(std::f64::NAN),  // unreachable
                };
            }
            return out;
        }

        match lowest_op {
            EOR => {
                let mut xss = Vec::<ExprSlice>::with_capacity(4);
                self.split(EOR, &mut xss);
                let mut out = IConst(0.0); let mut out_set = false;
                for xs in xss.iter() {
                    let instr = xs.compile(pslab,cslab);
                    if out_set {
                        out = IOR(cslab.push_instr(out), instr_to_ic!(cslab,instr));
                    } else {
                        if let IConst(c) = instr {
                            if f64_ne!(c,0.0) { return instr; }
                            // out = instr;     // Skip this 0 value (mostly so I don't complicate my logic in 'if out_set' since I can assume that any set value is non-const).
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
                    let instr = xs.compile(pslab,cslab);
                    if let IConst(c) = instr {
                        if f64_eq!(c,0.0) { return instr; }
                    }
                    if out_set {
                        if let IConst(_) = out {
                            // If we get here, we know that the const is non-zero.
                            out = instr;
                        } else {
                            out = IAND(cslab.push_instr(out), instr_to_ic!(cslab,instr));
                        }
                    } else {
                        out = instr;
                        out_set = true;
                    }
                }
                out
            }
            EAdd => {
                let mut xss = Vec::<ExprSlice>::with_capacity(4);
                self.split(EAdd, &mut xss);
                let mut instrs = Vec::<Instruction>::with_capacity(xss.len());
                for xs in xss {
                    let instr = xs.compile(pslab,cslab);
                    if let IAdd(li,ric) = instr {
                        push_add_leaves(&mut instrs,cslab,li,ric);  // Flatten nested structures like "x - 1 + 2 - 3".
                    } else {
                        instrs.push(instr);
                    }
                }
                compile_add(instrs,cslab)
            }
            ESub => {
                // Note: We don't need to push_add_leaves from here because Sub has a higher precedence than Add.

                let mut xss = Vec::<ExprSlice>::with_capacity(4);
                self.split(ESub, &mut xss);
                let mut instrs = Vec::<Instruction>::with_capacity(xss.len());
                for (i,xs) in xss.into_iter().enumerate() {
                    let instr = xs.compile(pslab,cslab);
                    if i==0 {
                        instrs.push(instr);
                    } else {
                        instrs.push(neg_wrap(instr,cslab));
                    }
                }
                compile_add(instrs,cslab)
            }
            EMul => {
                let mut xss = Vec::<ExprSlice>::with_capacity(4);
                self.split(EMul, &mut xss);
                let mut instrs = Vec::<Instruction>::with_capacity(xss.len());
                for xs in xss {
                    let instr = xs.compile(pslab,cslab);
                    if let IMul(li,ric) = instr {
                        push_mul_leaves(&mut instrs,cslab,li,ric);  // Flatten nested structures like "deg/360 * 2*pi()".
                    } else {
                        instrs.push(instr);
                    }
                }
                compile_mul(instrs,cslab)
            }
            EDiv => {
                // Note: We don't need to push_mul_leaves from here because Div has a higher precedence than Mul.

                let mut xss = Vec::<ExprSlice>::with_capacity(4);
                self.split(EDiv, &mut xss);
                let mut instrs = Vec::<Instruction>::with_capacity(xss.len());
                for (i,xs) in xss.into_iter().enumerate() {
                    let instr = xs.compile(pslab,cslab);
                    if i==0 {
                        instrs.push(instr);
                    } else {
                        instrs.push(inv_wrap(instr,cslab));
                    }
                }
                compile_mul(instrs,cslab)
            }
//          EDiv => {
//              let mut xss = Vec::<ExprSlice>::with_capacity(4);
//              self.split(EDiv, &mut xss);
//              let mut out = IConst(1.0); let mut out_set = false;
//              let mut const_prod = 1.0;
//              let mut is_first = true;
//              for xs in xss.iter() {
//                  let instr = xs.compile(pslab,cslab);
//                  if let IConst(c) = instr {
//                      if is_first {
//                          const_prod *= c;  // Floats don't overflow.
//                      } else {
//                          const_prod /= c;
//                      }
//                  } else {
//                      if is_first {
//                          if out_set {
//                              out = IMul(cslab.push_instr(out), cslab.push_instr(instr));
//                          } else {
//                              out = instr;
//                              out_set = true;
//                          }
//                      } else {
//                          let instr = inv_wrap(instr,cslab);
//                          if out_set {
//                              out = IMul(cslab.push_instr(out), cslab.push_instr(instr));
//                          } else {
//                              out = instr;
//                              out_set = true;
//                          }
//                      }
//                  }
//                  is_first = false;
//              }
//              if f64_ne!(const_prod,1.0) {
//                  if out_set {
//                      out = IMul(cslab.push_instr(out), cslab.push_instr(IConst(const_prod)));
//                  } else {
//                      out = IConst(const_prod);
//                  }
//              }
//              out
//          }
            EMod => {
                let mut xss = Vec::<ExprSlice>::with_capacity(2);
                self.split(EMod, &mut xss);
                let mut out = IConst(0.0); let mut out_set = false;
                for xs in xss.iter() {
                    let instr = xs.compile(pslab,cslab);
                    if out_set {
                        if let IConst(dividend) = out {
                            if let IConst(divisor) = instr {
                                out = IConst(dividend%divisor);
                                continue;
                            }
                        }
                        out = IMod{dividend:instr_to_ic!(cslab,out), divisor:instr_to_ic!(cslab,instr)};
                    } else {
                        out = instr;
                        out_set = true;
                    }
                }
                out
            }
            EExp => {  // Right-to-Left Associativity
                let mut xss = Vec::<ExprSlice>::with_capacity(2);
                self.split(EExp, &mut xss);
                let mut out = IConst(0.0); let mut out_set = false;
                for xs in xss.into_iter().rev() {
                    let instr = xs.compile(pslab,cslab);
                    if out_set {
                        if let IConst(power) = out {
                            if let IConst(base) = instr {
                                out = IConst(base.powf(power));
                                continue;
                            }
                        }
                        out = IExp{base:instr_to_ic!(cslab,instr), power:instr_to_ic!(cslab,out)};
                    } else {
                        out = instr;
                        out_set = true;
                    }
                }
                out
            }
//          EExp => {  // Left-to-Right Associativity
//              let mut xss = Vec::<ExprSlice>::with_capacity(2);
//              self.split(EExp, &mut xss);
//              let mut pow_instrs = Vec::<Instruction>::with_capacity(xss.len()-1);
//              let mut base = IConst(0.0);
//              for (i,xs) in xss.into_iter().enumerate() {
//                  let instr = xs.compile(pslab,cslab);
//                  if i==0 {
//                      base = instr;
//                  } else {
//                      pow_instrs.push(instr);
//                  }
//              }
//              let power = compile_mul(pow_instrs,cslab);
//              if let IConst(b) = base {
//                  if let IConst(p) = power {
//                      return IConst(b.powf(p));
//                  }
//              }
//              IExp{base:cslab.push_instr(base), power:cslab.push_instr(power)}
//          }
            ENE | EEQ | EGTE | ELTE | EGT | ELT => IConst(std::f64::NAN),  // unreachable
        }
    }
}

impl Compiler for Expression {
    fn compile(&self, pslab:&ParseSlab, cslab:&mut CompileSlab) -> Instruction {
        let top = ExprSlice::from_expr(&self);
        top.compile(pslab,cslab)
    }
}

impl Compiler for Value {
    fn compile(&self, pslab:&ParseSlab, cslab:&mut CompileSlab) -> Instruction {
        match self {
            Value::EConstant(c) => IConst(*c),
            Value::EUnaryOp(u) => u.compile(pslab,cslab),
            Value::EStdFunc(f) => f.compile(pslab,cslab),
            Value::EPrintFunc(pf) => IPrintFunc(pf.clone()),
        }
    }
}

impl Compiler for UnaryOp {
    fn compile(&self, pslab:&ParseSlab, cslab:&mut CompileSlab) -> Instruction {
        match self {
            EPos(i) => get_val!(pslab,i).compile(pslab,cslab),
            ENeg(i) => {
                let instr = get_val!(pslab,i).compile(pslab,cslab);
                if let IConst(c) = instr {
                    IConst(-c)
                } else {
                    neg_wrap(instr,cslab)
                }
            }
            ENot(i) => {
                let instr = get_val!(pslab,i).compile(pslab,cslab);
                if let IConst(c) = instr {
                    IConst(bool_to_f64!(f64_eq!(c,0.0)))
                } else {
                    not_wrap(instr,cslab)
                }
            }
            EParentheses(i) => get_expr!(pslab,i).compile(pslab,cslab),
        }
    }
}

impl Compiler for StdFunc {
    fn compile(&self, pslab:&ParseSlab, cslab:&mut CompileSlab) -> Instruction {
        match self {
            EVar(name) => IVar(name.clone()),
            #[cfg(feature="unsafe-vars")]
            EUnsafeVar{name,ptr} => IUnsafeVar{name:name.clone(), ptr:*ptr},
            EFunc{name, args:xis} => {
                let mut args = Vec::<IC>::with_capacity(xis.len());
                for xi in xis {
                    let instr = get_expr!(pslab,xi).compile(pslab,cslab);
                    args.push(instr_to_ic!(cslab,instr));
                }
                IFunc{name:name.clone(), args}
            }

            EFuncInt(i) => {
                let instr = get_expr!(pslab,i).compile(pslab,cslab);
                if let IConst(c) = instr {
                    IConst(c.trunc())
                } else {
                    IFuncInt(cslab.push_instr(instr))
                }
            }
            EFuncCeil(i) => {
                let instr = get_expr!(pslab,i).compile(pslab,cslab);
                if let IConst(c) = instr {
                    IConst(c.ceil())
                } else {
                    IFuncCeil(cslab.push_instr(instr))
                }
            }
            EFuncFloor(i) => {
                let instr = get_expr!(pslab,i).compile(pslab,cslab);
                if let IConst(c) = instr {
                    IConst(c.floor())
                } else {
                    IFuncFloor(cslab.push_instr(instr))
                }
            }
            EFuncAbs(i) => {
                let instr = get_expr!(pslab,i).compile(pslab,cslab);
                if let IConst(c) = instr {
                    IConst(c.abs())
                } else {
                    IFuncAbs(cslab.push_instr(instr))
                }
            }
            EFuncSign(i) => {
                let instr = get_expr!(pslab,i).compile(pslab,cslab);
                if let IConst(c) = instr {
                    IConst(c.signum())
                } else {
                    IFuncSign(cslab.push_instr(instr))
                }
            }
            EFuncLog{base:baseopt, expr:i} => {
                let base = match baseopt {
                    Some(bi) => get_expr!(pslab,bi).compile(pslab,cslab),
                    None => IConst(10.0),
                };
                let instr = get_expr!(pslab,i).compile(pslab,cslab);
                if let IConst(b) = base {
                    if let IConst(n) = instr {
                        return IConst(log(b,n));
                    }
                }
                IFuncLog{base:instr_to_ic!(cslab,base), of:instr_to_ic!(cslab,instr)}
            }
            EFuncRound{modulus:modopt, expr:i} => {
                let modulus = match modopt {
                    Some(mi) => get_expr!(pslab,mi).compile(pslab,cslab),
                    None => IConst(1.0),
                };
                let instr = get_expr!(pslab,i).compile(pslab,cslab);
                if let IConst(m) = modulus {
                    if let IConst(n) = instr {
                        return IConst( (n/m).round() * m );  // Floats don't overflow.
                    }
                }
                IFuncRound{modulus:instr_to_ic!(cslab,modulus), of:instr_to_ic!(cslab,instr)}
            }
            EFuncMin{first:fi, rest:is} => {
                let first = get_expr!(pslab,fi).compile(pslab,cslab);
                let mut rest = Vec::<Instruction>::with_capacity(is.len());
                for i in is { rest.push(get_expr!(pslab,i).compile(pslab,cslab)); }
                let mut out = IConst(0.0); let mut out_set = false;
                let mut const_min = 0.0; let mut const_min_set = false;
                if let IConst(f) = first {
                    const_min = f;
                    const_min_set = true;
                } else {
                    out = first;
                    out_set = true;
                }
                for instr in rest {
                    if let IConst(f) = instr {
                        if const_min_set {
                            if f<const_min { const_min=f; }
                        } else {
                            const_min = f;
                            const_min_set = true;
                        }
                    } else {
                        if out_set {
                            out = IFuncMin(cslab.push_instr(out), IC::I(cslab.push_instr(instr)));
                        } else {
                            out = instr;
                            out_set = true;
                        }
                    }
                }
                if const_min_set {
                    if out_set {
                        out = IFuncMin(cslab.push_instr(out), IC::C(const_min));
                    } else {
                        out = IConst(const_min);
                        // out_set = true;  // Comment out so the compiler doesn't complain about unused assignments.
                    }
                }
                //assert!(out_set);
                out
            }
            EFuncMax{first:fi, rest:is} => {
                let first = get_expr!(pslab,fi).compile(pslab,cslab);
                let mut rest = Vec::<Instruction>::with_capacity(is.len());
                for i in is { rest.push(get_expr!(pslab,i).compile(pslab,cslab)); }
                let mut out = IConst(0.0); let mut out_set = false;
                let mut const_max = 0.0; let mut const_max_set = false;
                if let IConst(f) = first {
                    const_max = f;
                    const_max_set = true;
                } else {
                    out = first;
                    out_set = true;
                }
                for instr in rest {
                    if let IConst(f) = instr {
                        if const_max_set {
                            if f>const_max { const_max=f; }
                        } else {
                            const_max = f;
                            const_max_set = true;
                        }
                    } else {
                        if out_set {
                            out = IFuncMax(cslab.push_instr(out), IC::I(cslab.push_instr(instr)));
                        } else {
                            out = instr;
                            out_set = true;
                        }
                    }
                }
                if const_max_set {
                    if out_set {
                        out = IFuncMax(cslab.push_instr(out), IC::C(const_max));
                    } else {
                        out = IConst(const_max);
                        // out_set = true;  // Comment out so the compiler doesn't complain about unused assignments.
                    }
                }
                //assert!(out_set);
                out
            }

            EFuncE => IConst(std::f64::consts::E),
            EFuncPi => IConst(std::f64::consts::PI),

            EFuncSin(i) => {
                let instr = get_expr!(pslab,i).compile(pslab,cslab);
                if let IConst(c) = instr {
                    IConst(c.sin())
                } else {
                    IFuncSin(cslab.push_instr(instr))
                }
            }
            EFuncCos(i) => {
                let instr = get_expr!(pslab,i).compile(pslab,cslab);
                if let IConst(c) = instr {
                    IConst(c.cos())
                } else {
                    IFuncCos(cslab.push_instr(instr))
                }
            }
            EFuncTan(i) => {
                let instr = get_expr!(pslab,i).compile(pslab,cslab);
                if let IConst(c) = instr {
                    IConst(c.tan())
                } else {
                    IFuncTan(cslab.push_instr(instr))
                }
            }
            EFuncASin(i) => {
                let instr = get_expr!(pslab,i).compile(pslab,cslab);
                if let IConst(c) = instr {
                    IConst(c.asin())
                } else {
                    IFuncASin(cslab.push_instr(instr))
                }
            }
            EFuncACos(i) => {
                let instr = get_expr!(pslab,i).compile(pslab,cslab);
                if let IConst(c) = instr {
                    IConst(c.acos())
                } else {
                    IFuncACos(cslab.push_instr(instr))
                }
            }
            EFuncATan(i) => {
                let instr = get_expr!(pslab,i).compile(pslab,cslab);
                if let IConst(c) = instr {
                    IConst(c.atan())
                } else {
                    IFuncATan(cslab.push_instr(instr))
                }
            }
            EFuncSinH(i) => {
                let instr = get_expr!(pslab,i).compile(pslab,cslab);
                if let IConst(c) = instr {
                    IConst(c.sinh())
                } else {
                    IFuncSinH(cslab.push_instr(instr))
                }
            }
            EFuncCosH(i) => {
                let instr = get_expr!(pslab,i).compile(pslab,cslab);
                if let IConst(c) = instr {
                    IConst(c.cosh())
                } else {
                    IFuncCosH(cslab.push_instr(instr))
                }
            }
            EFuncTanH(i) => {
                let instr = get_expr!(pslab,i).compile(pslab,cslab);
                if let IConst(c) = instr {
                    IConst(c.tanh())
                } else {
                    IFuncTanH(cslab.push_instr(instr))
                }
            }
            EFuncASinH(i) => {
                let instr = get_expr!(pslab,i).compile(pslab,cslab);
                if let IConst(c) = instr {
                    IConst(c.asinh())
                } else {
                    IFuncASinH(cslab.push_instr(instr))
                }
            }
            EFuncACosH(i) => {
                let instr = get_expr!(pslab,i).compile(pslab,cslab);
                if let IConst(c) = instr {
                    IConst(c.acosh())
                } else {
                    IFuncACosH(cslab.push_instr(instr))
                }
            }
            EFuncATanH(i) => {
                let instr = get_expr!(pslab,i).compile(pslab,cslab);
                if let IConst(c) = instr {
                    IConst(c.atanh())
                } else {
                    IFuncATanH(cslab.push_instr(instr))
                }
            }
        }
    }
}

