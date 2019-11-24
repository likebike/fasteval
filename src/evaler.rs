use crate::slab::Slab;
use crate::evalns::EvalNS;
use crate::parser::{Expression,
                    Value::{self, EConstant, EVariable, EUnaryOp, ECallable},
                    Constant,
                    Variable,
                    UnaryOp::{self, EPos, ENeg, ENot, EParens},
                    BinaryOp::{self, EPlus, EMinus, EMul, EDiv, EMod, EExp, ELT, ELTE, EEQ, ENE, EGTE, EGT, EOR, EAND},
                    Callable::{self, EFunc, EPrintFunc, EEvalFunc},
                    Func::{self, EFuncInt, EFuncCeil, EFuncFloor, EFuncAbs, EFuncLog, EFuncRound, EFuncMin, EFuncMax, EFuncE, EFuncPi, EFuncSin, EFuncCos, EFuncTan, EFuncASin, EFuncACos, EFuncATan, EFuncSinH, EFuncCosH, EFuncTanH},
                    PrintFunc,
                    EvalFunc,
                    ExpressionOrString::{EExpr, EStr}};
use crate::compiler::{log, Instruction};

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

    fn var_names(&self, slab:&Slab) -> Result<HashSet<String>,KErr> {
        let mut set = HashSet::new();
        {
            let mut ns = EvalNS::new(|name:&str| {
                set.insert(name.to_string());
                Some(0.0)
            });
            self.eval(slab, &mut ns)?;
        }
        Ok(set)
    }
}

impl Evaler for Expression {
    fn eval(&self, slab:&Slab, ns:&mut EvalNS) -> Result<f64,KErr> {
        // Order of operations: 1) ^  2) */  3) +-
        // Exponentiation should be processed left-to-right.  Think of what 2^3^4 should mean:
        //     2^(3^4)=2417851639229258349412352
        //     (2^3)^4=4096   <--- I choose this one.
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
        ns.eval(slab, &self.first).map(|f| vals.push(f))?;
        for pair in self.pairs.iter() {
            ops.push(pair.0);
            ns.eval(slab, &pair.1).map(|f| vals.push(f))?;
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

        // I am defining rtol and ltor as 'fn' rather than closures to make it extra-clear that they don't capture anything.
        // I need to pass all those items around as args rather than just capturing because Rust doesn't like multiple closures to capture the same stuff when at least one of them mutates.
        let mut eval_op = |ops:&mut Vec<BinaryOp>, i:usize| {
            let result = ops[i].binaryop_eval(vals[i], vals[i+1]);
            vals[i]=result; vals.remove(i+1);
            ops.remove(i);
        };
        fn rtol(eval_op:&mut impl FnMut(&mut Vec<BinaryOp>,usize), ops:&mut Vec<BinaryOp>, search:BinaryOp) {
            // for-loop structure:
            let mut i = ops.len() as i64;
            loop { i-=1; if i<0 { break }
                let i = i as usize;

                if ops[i]==search { eval_op(ops,i); }
            }
        };
        fn ltor(eval_op:&mut impl FnMut(&mut Vec<BinaryOp>,usize), ops:&mut Vec<BinaryOp>, search:BinaryOp) {
            'outer: loop {
                // for-loop structure:
                let mut i : i64 = -1;
                loop { i+=1; if i>=ops.len() as i64 { break 'outer; }
                    let i = i as usize;

                    if ops[i]==search {
                        eval_op(ops,i);
                        continue 'outer;  // Need to restart processing when modifying from the left.
                    }
                }
            }
        };
        fn ltor_multi(eval_op:&mut impl FnMut(&mut Vec<BinaryOp>,usize), ops:&mut Vec<BinaryOp>, search:&[BinaryOp]) {
            'outer: loop {
                // for-loop structure:
                let mut i : i64 = -1;
                loop { i+=1; if i>=ops.len() as i64 { break 'outer; }
                    let i = i as usize;

                    if search.contains(&ops[i]) {
                        eval_op(ops,i);
                        continue 'outer;  // Need to restart processing when modifying from the left.
                    }
                }
            }
        }

        ltor(&mut eval_op, &mut ops, EExp);
        ltor(&mut eval_op, &mut ops, EMod);
        ltor(&mut eval_op, &mut ops, EDiv);
        rtol(&mut eval_op, &mut ops, EMul);
        ltor(&mut eval_op, &mut ops, EMinus);
        rtol(&mut eval_op, &mut ops, EPlus);
        ltor_multi(&mut eval_op, &mut ops, &[ELT, EGT, ELTE, EGTE]);  // TODO: Implement Python-style a<b<c ternary comparison... might as well generalize to N comparisons.
        ltor_multi(&mut eval_op, &mut ops, &[EEQ, ENE]);
        ltor(&mut eval_op, &mut ops, EAND);
        ltor(&mut eval_op, &mut ops, EOR);

        if ops.len()!=0 { return Err(KErr::new("Unhandled Expression ops")); }
        if vals.len()!=1 { return Err(KErr::new("More than one final Expression value")); }
        Ok(vals[0])
    }
}

impl Evaler for Value {
    fn eval(&self, slab:&Slab, ns:&mut EvalNS) -> Result<f64,KErr> {
        match self {
            EConstant(c) => c.eval(slab, ns),
            EVariable(v) => v.eval(slab, ns),
            EUnaryOp(u) => u.eval(slab, ns),
            ECallable(c) => c.eval(slab, ns),
        }
    }
}

impl Evaler for Constant {
    fn eval(&self, _slab:&Slab, _ns:&mut EvalNS) -> Result<f64,KErr> { Ok(self.0) }
}

impl Evaler for Variable {
    fn eval(&self, _slab:&Slab, ns:&mut EvalNS) -> Result<f64,KErr> {
        match ns.get(self.0.as_str()) {
            Some(f) => Ok(f),
            None => Err(KErr::new("variable undefined")),
        }
    }
}

impl Evaler for UnaryOp {
    fn eval(&self, slab:&Slab, ns:&mut EvalNS) -> Result<f64,KErr> {
        match self {
            EPos(val_i) => ns.eval(slab, slab.ps.get_val(*val_i)),
            ENeg(val_i) => Ok(-ns.eval(slab, slab.ps.get_val(*val_i))?),
            ENot(val_i) => Ok(bool_to_f64(ns.eval(slab, slab.ps.get_val(*val_i))?==0.0)),
            EParens(expr_i) => ns.eval(slab, slab.ps.get_expr(*expr_i)),
        }
    }
}

impl BinaryOp {
    // Non-standard eval interface (not generalized yet):
    fn binaryop_eval(&self, left:f64, right:f64) -> f64 {
        match self {
            EPlus => left+right,
            EMinus => left-right,
            EMul => left*right,
            EDiv => left/right,
            EMod => left%right, //left - (left/right).trunc()*right
            EExp => left.powf(right),
            ELT => bool_to_f64(left<right),
            ELTE => bool_to_f64(left<=right),
            EEQ => bool_to_f64(left==right),
            ENE => bool_to_f64(left!=right),
            EGTE => bool_to_f64(left>=right),
            EGT => bool_to_f64(left>right),
            EOR => if left!=0.0 { left }
                   else { right },
            EAND => if left==0.0 { left }
                    else { right },
        }
    }
}

impl Evaler for Callable {
    fn eval(&self, slab:&Slab, ns:&mut EvalNS) -> Result<f64,KErr> {
        match self {
            EFunc(f) => ns.eval(slab, f),
            EEvalFunc(f) => ns.eval(slab, f),
            EPrintFunc(f) => ns.eval(slab, f),
        }
    }
}

impl Evaler for Func {
    fn eval(&self, slab:&Slab, ns:&mut EvalNS) -> Result<f64,KErr> {
        match self {
            EFuncInt(expr_i) => { Ok(ns.eval(slab, slab.ps.get_expr(*expr_i))?.trunc()) }
            EFuncCeil(expr_i) => { Ok(ns.eval(slab, slab.ps.get_expr(*expr_i))?.ceil()) }
            EFuncFloor(expr_i) => { Ok(ns.eval(slab, slab.ps.get_expr(*expr_i))?.floor()) }
            EFuncAbs(expr_i) => { Ok(ns.eval(slab, slab.ps.get_expr(*expr_i))?.abs()) }
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

            EFuncSin(expr_i) => { Ok(ns.eval(slab, slab.ps.get_expr(*expr_i))?.sin()) },
            EFuncCos(expr_i) => { Ok(ns.eval(slab, slab.ps.get_expr(*expr_i))?.cos()) },
            EFuncTan(expr_i) => { Ok(ns.eval(slab, slab.ps.get_expr(*expr_i))?.tan()) },
            EFuncASin(expr_i) => { Ok(ns.eval(slab, slab.ps.get_expr(*expr_i))?.asin()) },
            EFuncACos(expr_i) => { Ok(ns.eval(slab, slab.ps.get_expr(*expr_i))?.acos()) },
            EFuncATan(expr_i) => { Ok(ns.eval(slab, slab.ps.get_expr(*expr_i))?.atan()) },
            EFuncSinH(expr_i) => { Ok(ns.eval(slab, slab.ps.get_expr(*expr_i))?.sinh()) },
            EFuncCosH(expr_i) => { Ok(ns.eval(slab, slab.ps.get_expr(*expr_i))?.cosh()) },
            EFuncTanH(expr_i) => { Ok(ns.eval(slab, slab.ps.get_expr(*expr_i))?.tanh()) },
        }
    }
}

impl Evaler for PrintFunc {
    fn eval(&self, slab:&Slab, ns:&mut EvalNS) -> Result<f64,KErr> {
        let mut val = 0f64;

        fn process_str(s:&str) -> String {
            let s = s.replace("\\n","\n");
            let s = s.replace("\\t","\t");
            s
        }

        if self.0.len()>0 {
            if let EStr(ref fmtstr) = self.0[0] {
                if fmtstr.contains('%') {
                    // printf mode:

                    //let fmtstr = process_str(fmtstr);

                    unimplemented!();  // Make a pure-rust printf libarary.

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

impl Evaler for EvalFunc {
    fn eval(&self, slab:&Slab, ns:&mut EvalNS) -> Result<f64,KErr> {
        // Don't affect the external namespace:
        // If you do, you get some surprising behavior:
        //     var a=0
        //     var b=eval(1,a=9)
        //     var _=print(a,b)   // "0 1"
        //     var _=print(b,a)   // "1 9"
        ns.push_eval(true)?;
        // This is my 'defer ns.pop();' structure:
        let res = (|| -> Result<f64,KErr> {

            for kw in self.kwargs.iter() {
                let val = ns.eval_bubble(slab, slab.ps.get_expr(kw.expr))?;
                ns.create(kw.name.0.clone(), val)?;  // Should we delay the 'create' until after evaluating all the kwargs, so they don't affect each other?
            }

            ns.start_reeval_mode();
            // Another defer structure (a bit overly-complex for this simple case):
            let res = (|| -> Result<f64,KErr> {
                ns.eval(slab, slab.ps.get_expr(self.expr))
            })();
            ns.end_reeval_mode();
            res

        })();
        ns.pop();
        res
    }
}

impl Evaler for Instruction {
    fn eval(&self, slab:&Slab, ns:&mut EvalNS) -> Result<f64,KErr> {
        match self {
            Instruction::IConst(c) => Ok(*c),
            Instruction::IVar(v) => v.eval(slab,ns),

            Instruction::INeg(i) => Ok(-slab.cs.get_instr(*i).eval(slab,ns)?),
            Instruction::INot(i) => Ok(bool_to_f64(slab.cs.get_instr(*i).eval(slab,ns)?==0.0)),
            Instruction::IInv(i) => Ok(1.0/slab.cs.get_instr(*i).eval(slab,ns)?),

            Instruction::IAdd(li,ri) => Ok( slab.cs.get_instr(*li).eval(slab,ns)? +
                                            slab.cs.get_instr(*ri).eval(slab,ns)? ),
            Instruction::IMul(li,ri) => Ok( slab.cs.get_instr(*li).eval(slab,ns)? *
                                            slab.cs.get_instr(*ri).eval(slab,ns)? ),
            Instruction::IMod{dividend, divisor} => Ok( slab.cs.get_instr(*dividend).eval(slab,ns)? %
                                                        slab.cs.get_instr(*divisor).eval(slab,ns)? ),
            Instruction::IExp{base, power} => Ok( slab.cs.get_instr(*base).eval(slab,ns)?.powf( 
                                                  slab.cs.get_instr(*power).eval(slab,ns)? ) ),

            Instruction::ILT(left, right) => Ok( bool_to_f64(slab.cs.get_instr(*left).eval(slab,ns)? <
                                                             slab.cs.get_instr(*right).eval(slab,ns)?) ),
            Instruction::ILTE(left, right) => Ok( bool_to_f64(slab.cs.get_instr(*left).eval(slab,ns)? <=
                                                              slab.cs.get_instr(*right).eval(slab,ns)?) ),
            Instruction::IEQ(left, right) => Ok( bool_to_f64(slab.cs.get_instr(*left).eval(slab,ns)? ==
                                                             slab.cs.get_instr(*right).eval(slab,ns)?) ),
            Instruction::INE(left, right) => Ok( bool_to_f64(slab.cs.get_instr(*left).eval(slab,ns)? !=
                                                             slab.cs.get_instr(*right).eval(slab,ns)?) ),
            Instruction::IGTE(left, right) => Ok( bool_to_f64(slab.cs.get_instr(*left).eval(slab,ns)? >=
                                                              slab.cs.get_instr(*right).eval(slab,ns)?) ),
            Instruction::IGT(left, right) => Ok( bool_to_f64(slab.cs.get_instr(*left).eval(slab,ns)? >
                                                             slab.cs.get_instr(*right).eval(slab,ns)?) ),

            Instruction::IAND(lefti, righti) => {
                let left = slab.cs.get_instr(*lefti).eval(slab,ns)?;
                if left==0.0 { Ok(left) }
                else {
                    Ok(slab.cs.get_instr(*righti).eval(slab,ns)?)
                }
            }
            Instruction::IOR(lefti, righti) => {
                let left = slab.cs.get_instr(*lefti).eval(slab,ns)?;
                if left!=0.0 { Ok(left) }
                else {
                    Ok(slab.cs.get_instr(*righti).eval(slab,ns)?)
                }
            }

            Instruction::IFuncInt(i) => Ok( slab.cs.get_instr(*i).eval(slab,ns)?.trunc() ),
            Instruction::IFuncCeil(i) => Ok( slab.cs.get_instr(*i).eval(slab,ns)?.ceil() ),
            Instruction::IFuncFloor(i) => Ok( slab.cs.get_instr(*i).eval(slab,ns)?.floor() ),
            Instruction::IFuncAbs(i) => Ok( slab.cs.get_instr(*i).eval(slab,ns)?.abs() ),
            Instruction::IFuncLog{base:basei, of:ofi} => {
                let base = slab.cs.get_instr(*basei).eval(slab,ns)?;
                let of = slab.cs.get_instr(*ofi).eval(slab,ns)?;
                Ok(log(base,of))
            }
            Instruction::IFuncRound{modulus:modi, of:ofi} => {
                let modulus = slab.cs.get_instr(*modi).eval(slab,ns)?;
                let of = slab.cs.get_instr(*ofi).eval(slab,ns)?;
                Ok( (of/modulus).round() * modulus )
            }
            Instruction::IFuncMin(li,ri) => {
                let left = slab.cs.get_instr(*li).eval(slab,ns)?;
                let right = slab.cs.get_instr(*ri).eval(slab,ns)?;
                if left<right {
                    Ok(left)
                } else {
                    Ok(right)
                }
            }
            Instruction::IFuncMax(li,ri) => {
                let left = slab.cs.get_instr(*li).eval(slab,ns)?;
                let right = slab.cs.get_instr(*ri).eval(slab,ns)?;
                if left>right {
                    Ok(left)
                } else {
                    Ok(right)
                }
            }

            Instruction::IFuncSin(i) => Ok( slab.cs.get_instr(*i).eval(slab,ns)?.sin() ),
            Instruction::IFuncCos(i) => Ok( slab.cs.get_instr(*i).eval(slab,ns)?.cos() ),
            Instruction::IFuncTan(i) => Ok( slab.cs.get_instr(*i).eval(slab,ns)?.tan() ),
            Instruction::IFuncASin(i) => Ok( slab.cs.get_instr(*i).eval(slab,ns)?.asin() ),
            Instruction::IFuncACos(i) => Ok( slab.cs.get_instr(*i).eval(slab,ns)?.acos() ),
            Instruction::IFuncATan(i) => Ok( slab.cs.get_instr(*i).eval(slab,ns)?.atan() ),
            Instruction::IFuncSinH(i) => Ok( slab.cs.get_instr(*i).eval(slab,ns)?.sinh() ),
            Instruction::IFuncCosH(i) => Ok( slab.cs.get_instr(*i).eval(slab,ns)?.cosh() ),
            Instruction::IFuncTanH(i) => Ok( slab.cs.get_instr(*i).eval(slab,ns)?.tanh() ),

            Instruction::IPrintFunc(pf) => pf.eval(slab,ns),
            Instruction::IEvalFunc(ef) => ef.eval(slab,ns),
        }
    }
}

