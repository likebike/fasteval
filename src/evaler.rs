use crate as al;

use crate::slab::Slab;
use crate::evalns::EvalNS;
use crate::parser::{Expression,
                    Value::{self, EConstant, EUnaryOp, EStdFunc, EPrintFunc},
                    Constant,
                    UnaryOp::{self, EPos, ENeg, ENot, EParentheses},
                    BinaryOp::{self, EAdd, ESub, EMul, EDiv, EMod, EExp, ELT, ELTE, EEQ, ENE, EGTE, EGT, EOR, EAND},
                    VarName,
                    StdFunc::{self, EVar, EFunc, EFuncInt, EFuncCeil, EFuncFloor, EFuncAbs, EFuncSign, EFuncLog, EFuncRound, EFuncMin, EFuncMax, EFuncE, EFuncPi, EFuncSin, EFuncCos, EFuncTan, EFuncASin, EFuncACos, EFuncATan, EFuncSinH, EFuncCosH, EFuncTanH, EFuncASinH, EFuncACosH, EFuncATanH},
                    PrintFunc,
                    ExpressionOrString::{EExpr, EStr}};
#[cfg(feature="unsafe-vars")]
use crate::parser::StdFunc::EUnsafeVar;
use crate::compiler::{log, f64_eq, f64_ne, Instruction};

use kerr::KErr;

use std::collections::HashSet;
use std::f64::consts;
use std::fmt;


#[inline]
pub fn bool_to_f64(b:bool) -> f64 {
    if b { 1.0 }
    else { 0.0 }
}


pub trait Evaler : fmt::Debug {
    fn eval(&self, slab:&Slab, ns:&mut EvalNS) -> Result<f64,KErr>;

    fn var_names(&self, slab:&Slab) -> Result<HashSet<String>,KErr> where Self:Sized {
        let mut set = HashSet::new();
        {
            let mut ns = EvalNS::new(|name:&str, _args:Vec<f64>| {
                set.insert(name.to_string());
                Some(0.0)
            });
            ns.eval(slab, self)?;
        }
        Ok(set)
    }
}

impl Evaler for Expression {
    fn eval(&self, slab:&Slab, ns:&mut EvalNS) -> Result<f64,KErr> {
        // Order of operations: 1) ^  2) */  3) +-
        // Exponentiation should be processed right-to-left.  Think of what 2^3^4 should mean:
        //     2^(3^4)=2417851639229258349412352   <--- I choose this one.  https://codeplea.com/exponentiation-associativity-options
        //     (2^3)^4=4096
        // Direction of processing doesn't matter for Addition and Multiplication:
        //     (((3+4)+5)+6)==(3+(4+(5+6))), (((3*4)*5)*6)==(3*(4*(5*6)))
        // ...But Subtraction and Division must be processed left-to-right:
        //     (((6-5)-4)-3)!=(6-(5-(4-3))), (((6/5)/4)/3)!=(6/(5/(4/3)))


        // // ---- Go code, for comparison ----
        // // vals,ops:=make([]float64, len(e)/2+1),make([]BinaryOp, len(e)/2)
        // // for i:=0; i<len(e); i+=2 {
        // //     vals[i/2]=ns.EvalBubble(e[i].(evaler))
        // //     if i<len(e)-1 { ops[i/2]=e[i+1].(BinaryOp) }
        // // }

        // if self.0.len()%2!=1 { return Err(KErr::new("Expression len should always be odd")) }
        // let mut vals : Vec<f64>      = Vec::with_capacity(self.0.len()/2+1);
        // let mut ops  : Vec<BinaryOp> = Vec::with_capacity(self.0.len()/2  );
        // for (i,tok) in self.0.iter().enumerate() {
        //     match tok {
        //         EValue(val) => {
        //             if i%2==1 { return Err(KErr::new("Found value at odd index")) }
        //             match ns.eval_bubble(val) {
        //                 Ok(f) => vals.push(f),
        //                 Err(e) => return Err(e.pre(&format!("eval_bubble({:?})",val))),
        //             }
        //         }
        //         EBinaryOp(bop) => {
        //             if i%2==0 { return Err(KErr::new("Found binaryop at even index")) }
        //             ops.push(*bop);
        //         }
        //     }
        // }

        // Code for new Expression data structure:
        let mut vals = Vec::<f64>::with_capacity(self.pairs.len()+1);
        let mut ops  = Vec::<BinaryOp>::with_capacity(self.pairs.len());
        vals.push(ns.eval(slab, &self.first)?);
        for pair in self.pairs.iter() {
            ops.push(pair.0);
            vals.push(ns.eval(slab, &pair.1)?);
        }


        // ---- Go code, for comparison ----
        // evalOp:=func(i int) {
        //     result:=ops[i]._Eval(vals[i], vals[i+1])
        //     vals=append(append(vals[:i], result), vals[i+2:]...)
        //     ops=append(ops[:i], ops[i+1:]...)
        // }
        // rtol:=func(s BinaryOp) { for i:=len(ops)-1; i>=0; i-- { if ops[i]==s { evalOp(i) } } }
        // ltor:=func(s BinaryOp) {
        //     loop:
        //     for i:=0; i<len(ops); i++ { if ops[i]==s { evalOp(i); goto loop } }  // Need to restart processing when modifying from the left.
        // }

        #[inline(always)]
        fn rtol(vals:&mut Vec<f64>, ops:&mut Vec<BinaryOp>, search:BinaryOp) {
            let mut i : i64 = ops.len() as i64;
            loop {
                i-=1; if i<0 { break; }
                let i = i as usize;

                let op = ops[i];
                if op==search {
                    vals[i] = op.binaryop_eval(vals[i], vals[i+1]);
                    vals.remove(i+1);
                    ops.remove(i);
                }
            }
        }
        #[inline(always)]
        fn ltor(vals:&mut Vec<f64>, ops:&mut Vec<BinaryOp>, search:BinaryOp) {
            let mut i = 0;
            loop {
                match ops.get(i) {
                    None => break,
                    Some(op) => {
                        if *op==search {
                            vals[i] = op.binaryop_eval(vals[i], vals[i+1]);
                            vals.remove(i+1);
                            ops.remove(i);
                        } else {
                            i+=1;
                        }
                    }
                }
            }
        }
        #[inline(always)]
        fn ltor_multi(vals:&mut Vec<f64>, ops:&mut Vec<BinaryOp>, search:&[BinaryOp]) {
            let mut i = 0;
            loop {
                match ops.get(i) {
                    None => break,
                    Some(op) => {
                        if search.contains(op) {
                            vals[i] = op.binaryop_eval(vals[i], vals[i+1]);
                            vals.remove(i+1);
                            ops.remove(i);
                        } else {
                            i+=1;
                        }
                    }
                }
            }
        }

        // Keep the order of these statements in-sync with parser.rs BinaryOp priority values:
        rtol(&mut vals, &mut ops, EExp);  // https://codeplea.com/exponentiation-associativity-options
        ltor(&mut vals, &mut ops, EMod);
        ltor(&mut vals, &mut ops, EDiv);
        rtol(&mut vals, &mut ops, EMul);
        ltor(&mut vals, &mut ops, ESub);
        rtol(&mut vals, &mut ops, EAdd);
        ltor_multi(&mut vals, &mut ops, &[ELT, EGT, ELTE, EGTE]);  // TODO: Implement Python-style a<b<c ternary comparison... might as well generalize to N comparisons.
        ltor_multi(&mut vals, &mut ops, &[EEQ, ENE]);
        ltor(&mut vals, &mut ops, EAND);
        ltor(&mut vals, &mut ops, EOR);

        if !ops.is_empty() { return Err(KErr::new("Unhandled Expression ops")); }
        if vals.len()!=1 { return Err(KErr::new("More than one final Expression value")); }
        Ok(vals[0])
    }
}

impl Evaler for Value {
    fn eval(&self, slab:&Slab, ns:&mut EvalNS) -> Result<f64,KErr> {
        match self {
            EConstant(c) => ns.eval(slab, c),
            EUnaryOp(u) => ns.eval(slab, u),
            EStdFunc(f) => ns.eval(slab, f),
            EPrintFunc(f) => ns.eval(slab, f),
        }
    }
}

impl Evaler for Constant {
    fn eval(&self, _slab:&Slab, _ns:&mut EvalNS) -> Result<f64,KErr> { Ok(self.0) }
}

impl Evaler for UnaryOp {
    fn eval(&self, slab:&Slab, ns:&mut EvalNS) -> Result<f64,KErr> {
        match self {
            EPos(val_i) => ns.eval(slab, slab.ps.get_val(*val_i)),
            ENeg(val_i) => Ok(-ns.eval(slab, slab.ps.get_val(*val_i))?),
            ENot(val_i) => Ok(bool_to_f64(f64_eq(ns.eval(slab, slab.ps.get_val(*val_i))?,0.0))),
            EParentheses(expr_i) => ns.eval(slab, slab.ps.get_expr(*expr_i)),
        }
    }
}

impl BinaryOp {
    // Non-standard eval interface (not generalized yet):
    fn binaryop_eval(self, left:f64, right:f64) -> f64 {  // Passing 'self' by value is more efficient than pass-by-reference.
        match self {
            EAdd => left+right,
            ESub => left-right,
            EMul => left*right,
            EDiv => left/right,
            EMod => left%right, //left - (left/right).trunc()*right
            EExp => left.powf(right),
            ELT => bool_to_f64(left<right),
            ELTE => bool_to_f64(left<=right),
            EEQ => bool_to_f64(f64_eq(left,right)),
            ENE => bool_to_f64(f64_ne(left,right)),
            EGTE => bool_to_f64(left>=right),
            EGT => bool_to_f64(left>right),
            EOR => if f64_ne(left,0.0) { left }
                   else { right },
            EAND => if f64_eq(left,0.0) { left }
                    else { right },
        }
    }
}

fn eval_var(ns:&mut EvalNS, name:&VarName, args:Vec<f64>) -> Result<f64,KErr> {
    match ns.get(&name.0,args) {
        Some(f) => Ok(f),
        None => Err(KErr::new(&format!("variable undefined: {}",name))),
    }
}

impl Evaler for StdFunc {
    fn eval(&self, slab:&Slab, ns:&mut EvalNS) -> Result<f64,KErr> {
        match self {
            #[cfg(feature="unsafe-vars")]
            EUnsafeVar{name:_,ptr} => unsafe { Ok(**ptr) },

            EVar(name) => eval_var(ns, name, Vec::new()),
            EFunc{name, args:xis} => {
                let mut args = Vec::with_capacity(xis.len());
                for xi in xis {
                    args.push(ns.eval(slab, slab.ps.get_expr(*xi))?)
                }
                eval_var(ns, name, args)
            }

            EFuncInt(expr_i) => Ok(ns.eval(slab, slab.ps.get_expr(*expr_i))?.trunc()),
            EFuncCeil(expr_i) => Ok(ns.eval(slab, slab.ps.get_expr(*expr_i))?.ceil()),
            EFuncFloor(expr_i) => Ok(ns.eval(slab, slab.ps.get_expr(*expr_i))?.floor()),
            EFuncAbs(expr_i) => Ok(ns.eval(slab, slab.ps.get_expr(*expr_i))?.abs()),
            EFuncSign(expr_i) => Ok(ns.eval(slab, slab.ps.get_expr(*expr_i))?.signum()),
            EFuncLog{base:base_opt, expr:expr_i} => {
                let base = match base_opt {
                    Some(b_expr_i) => ns.eval(slab, slab.ps.get_expr(*b_expr_i))?,
                    None => 10.0,
                };
                let n = ns.eval(slab, slab.ps.get_expr(*expr_i))?;
                Ok(log(base,n))
            }
            EFuncRound{modulus:modulus_opt, expr:expr_i} => {
                let modulus = match modulus_opt {
                    Some(m_expr_i) => ns.eval(slab, slab.ps.get_expr(*m_expr_i))?,
                    None => 1.0,
                };
                Ok((ns.eval(slab, slab.ps.get_expr(*expr_i))?/modulus).round() * modulus)
            }
            EFuncMin{first:first_i, rest} => {
                let mut min = ns.eval(slab, slab.ps.get_expr(*first_i))?;
                for x_i in rest.iter() {
                    min = min.min(ns.eval(slab, slab.ps.get_expr(*x_i))?);
                }
                Ok(min)
            }
            EFuncMax{first:first_i, rest} => {
                let mut max = ns.eval(slab, slab.ps.get_expr(*first_i))?;
                for x_i in rest.iter() {
                    max = max.max(ns.eval(slab, slab.ps.get_expr(*x_i))?);
                }
                Ok(max)
            }

            EFuncE => Ok(consts::E),
            EFuncPi => Ok(consts::PI),

            EFuncSin(expr_i) => Ok(ns.eval(slab, slab.ps.get_expr(*expr_i))?.sin()),
            EFuncCos(expr_i) => Ok(ns.eval(slab, slab.ps.get_expr(*expr_i))?.cos()),
            EFuncTan(expr_i) => Ok(ns.eval(slab, slab.ps.get_expr(*expr_i))?.tan()),
            EFuncASin(expr_i) => Ok(ns.eval(slab, slab.ps.get_expr(*expr_i))?.asin()),
            EFuncACos(expr_i) => Ok(ns.eval(slab, slab.ps.get_expr(*expr_i))?.acos()),
            EFuncATan(expr_i) => Ok(ns.eval(slab, slab.ps.get_expr(*expr_i))?.atan()),
            EFuncSinH(expr_i) => Ok(ns.eval(slab, slab.ps.get_expr(*expr_i))?.sinh()),
            EFuncCosH(expr_i) => Ok(ns.eval(slab, slab.ps.get_expr(*expr_i))?.cosh()),
            EFuncTanH(expr_i) => Ok(ns.eval(slab, slab.ps.get_expr(*expr_i))?.tanh()),
            EFuncASinH(expr_i) => Ok(ns.eval(slab, slab.ps.get_expr(*expr_i))?.asinh()),
            EFuncACosH(expr_i) => Ok(ns.eval(slab, slab.ps.get_expr(*expr_i))?.acosh()),
            EFuncATanH(expr_i) => Ok(ns.eval(slab, slab.ps.get_expr(*expr_i))?.atanh()),
        }
    }
}

impl Evaler for PrintFunc {
    fn eval(&self, slab:&Slab, ns:&mut EvalNS) -> Result<f64,KErr> {
        let mut val = 0f64;

        #[inline]
        fn process_str(s:&str) -> String {
            s.replace("\\n","\n").replace("\\t","\t")
        }

        if !self.0.is_empty() {
            if let EStr(ref fmtstr) = self.0[0] {
                if fmtstr.contains('%') {
                    // printf mode:

                    //let fmtstr = process_str(fmtstr);

                    unimplemented!();  // TODO: Make a pure-rust printf libarary.

                    //return Ok(val);
                }
            }
        }

        // Normal Mode:
        let mut out = String::with_capacity(16);
        for (i,a) in self.0.iter().enumerate() {
            if i>0 { out.push(' '); }
            match a {
                EExpr(e_i) => {
                    val = ns.eval(slab, slab.ps.get_expr(*e_i))?;
                    out.push_str(&val.to_string());
                }
                EStr(s) => out.push_str(&process_str(s))
            }
        }
        eprintln!("{}", out);

        Ok(val)
    }
}

impl Evaler for Instruction {
    #[inline]
    fn eval(&self, slab:&Slab, ns:&mut EvalNS) -> Result<f64,KErr> {
        match self {
            Instruction::IConst(c) => Ok(*c),

            #[cfg(feature="unsafe-vars")]
            Instruction::IUnsafeVar{name:_,ptr} => unsafe { Ok(**ptr) },

            Instruction::INeg(i) => Ok(-eval_instr_ref!(slab.cs.get_instr(*i), slab, ns)),
            Instruction::INot(i) => Ok(bool_to_f64(f64_eq(eval_instr_ref!(slab.cs.get_instr(*i), slab, ns),0.0))),
            Instruction::IInv(i) => Ok(1.0/eval_instr_ref!(slab.cs.get_instr(*i), slab, ns)),

            Instruction::IAdd(li,ri) => Ok( eval_instr_ref!(slab.cs.get_instr(*li), slab, ns) +
                                            eval_instr_ref!(slab.cs.get_instr(*ri), slab, ns) ),
            Instruction::IMul(li,ri) => Ok( eval_instr_ref!(slab.cs.get_instr(*li), slab, ns) *
                                            eval_instr_ref!(slab.cs.get_instr(*ri), slab, ns) ),
            Instruction::IMod{dividend, divisor} => Ok( eval_instr_ref!(slab.cs.get_instr(*dividend), slab, ns) %
                                                        eval_instr_ref!(slab.cs.get_instr(*divisor), slab, ns) ),
            Instruction::IExp{base, power} => Ok( eval_instr_ref!(slab.cs.get_instr(*base), slab, ns).powf( 
                                                  eval_instr_ref!(slab.cs.get_instr(*power), slab, ns) ) ),

            Instruction::ILT(left, right) => Ok( bool_to_f64(eval_instr_ref!(slab.cs.get_instr(*left), slab, ns) <
                                                             eval_instr_ref!(slab.cs.get_instr(*right), slab, ns)) ),
            Instruction::ILTE(left, right) => Ok( bool_to_f64(eval_instr_ref!(slab.cs.get_instr(*left), slab, ns) <=
                                                              eval_instr_ref!(slab.cs.get_instr(*right), slab, ns)) ),
            Instruction::IEQ(left, right) => Ok( bool_to_f64(f64_eq(eval_instr_ref!(slab.cs.get_instr(*left), slab, ns),
                                                                    eval_instr_ref!(slab.cs.get_instr(*right), slab, ns))) ),
            Instruction::INE(left, right) => Ok( bool_to_f64(f64_ne(eval_instr_ref!(slab.cs.get_instr(*left), slab, ns),
                                                                    eval_instr_ref!(slab.cs.get_instr(*right), slab, ns))) ),
            Instruction::IGTE(left, right) => Ok( bool_to_f64(eval_instr_ref!(slab.cs.get_instr(*left), slab, ns) >=
                                                              eval_instr_ref!(slab.cs.get_instr(*right), slab, ns)) ),
            Instruction::IGT(left, right) => Ok( bool_to_f64(eval_instr_ref!(slab.cs.get_instr(*left), slab, ns) >
                                                             eval_instr_ref!(slab.cs.get_instr(*right), slab, ns)) ),

            Instruction::IAND(lefti, righti) => {
                let left = eval_instr_ref!(slab.cs.get_instr(*lefti), slab, ns);
                if f64_eq(left,0.0) { Ok(left) }
                else {
                    Ok(eval_instr_ref!(slab.cs.get_instr(*righti), slab, ns))
                }
            }
            Instruction::IOR(lefti, righti) => {
                let left = eval_instr_ref!(slab.cs.get_instr(*lefti), slab, ns);
                if f64_ne(left,0.0) { Ok(left) }
                else {
                    Ok(eval_instr_ref!(slab.cs.get_instr(*righti), slab, ns))
                }
            }

            Instruction::IVar(name) => eval_var(ns, name, Vec::new()),
            Instruction::IFunc{name, args:iis} => {
                let mut args = Vec::with_capacity(iis.len());
                for ii in iis {
                    args.push( eval_instr_ref!(slab.cs.get_instr(*ii), slab, ns) );
                }
                eval_var(ns, name, args)
            },

            Instruction::IFuncInt(i) => Ok( eval_instr_ref!(slab.cs.get_instr(*i), slab, ns).trunc() ),
            Instruction::IFuncCeil(i) => Ok( eval_instr_ref!(slab.cs.get_instr(*i), slab, ns).ceil() ),
            Instruction::IFuncFloor(i) => Ok( eval_instr_ref!(slab.cs.get_instr(*i), slab, ns).floor() ),
            Instruction::IFuncAbs(i) => Ok( eval_instr_ref!(slab.cs.get_instr(*i), slab, ns).abs() ),
            Instruction::IFuncSign(i) => Ok( eval_instr_ref!(slab.cs.get_instr(*i), slab, ns).signum() ),
            Instruction::IFuncLog{base:basei, of:ofi} => {
                let base = eval_instr_ref!(slab.cs.get_instr(*basei), slab, ns);
                let of = eval_instr_ref!(slab.cs.get_instr(*ofi), slab, ns);
                Ok(log(base,of))
            }
            Instruction::IFuncRound{modulus:modi, of:ofi} => {
                let modulus = eval_instr_ref!(slab.cs.get_instr(*modi), slab, ns);
                let of = eval_instr_ref!(slab.cs.get_instr(*ofi), slab, ns);
                Ok( (of/modulus).round() * modulus )
            }
            Instruction::IFuncMin(li,ri) => {
                let left = eval_instr_ref!(slab.cs.get_instr(*li), slab, ns);
                let right = eval_instr_ref!(slab.cs.get_instr(*ri), slab, ns);
                if left<right {
                    Ok(left)
                } else {
                    Ok(right)
                }
            }
            Instruction::IFuncMax(li,ri) => {
                let left = eval_instr_ref!(slab.cs.get_instr(*li), slab, ns);
                let right = eval_instr_ref!(slab.cs.get_instr(*ri), slab, ns);
                if left>right {
                    Ok(left)
                } else {
                    Ok(right)
                }
            }

            Instruction::IFuncSin(i) => Ok( eval_instr_ref!(slab.cs.get_instr(*i), slab, ns).sin() ),
            Instruction::IFuncCos(i) => Ok( eval_instr_ref!(slab.cs.get_instr(*i), slab, ns).cos() ),
            Instruction::IFuncTan(i) => Ok( eval_instr_ref!(slab.cs.get_instr(*i), slab, ns).tan() ),
            Instruction::IFuncASin(i) => Ok( eval_instr_ref!(slab.cs.get_instr(*i), slab, ns).asin() ),
            Instruction::IFuncACos(i) => Ok( eval_instr_ref!(slab.cs.get_instr(*i), slab, ns).acos() ),
            Instruction::IFuncATan(i) => Ok( eval_instr_ref!(slab.cs.get_instr(*i), slab, ns).atan() ),
            Instruction::IFuncSinH(i) => Ok( eval_instr_ref!(slab.cs.get_instr(*i), slab, ns).sinh() ),
            Instruction::IFuncCosH(i) => Ok( eval_instr_ref!(slab.cs.get_instr(*i), slab, ns).cosh() ),
            Instruction::IFuncTanH(i) => Ok( eval_instr_ref!(slab.cs.get_instr(*i), slab, ns).tanh() ),
            Instruction::IFuncASinH(i) => Ok( eval_instr_ref!(slab.cs.get_instr(*i), slab, ns).asinh() ),
            Instruction::IFuncACosH(i) => Ok( eval_instr_ref!(slab.cs.get_instr(*i), slab, ns).acosh() ),
            Instruction::IFuncATanH(i) => Ok( eval_instr_ref!(slab.cs.get_instr(*i), slab, ns).atanh() ),

            Instruction::IPrintFunc(pf) => ns.eval(slab,pf),
        }
    }
}

