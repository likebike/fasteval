use crate as al;

use crate::error::Error;
use crate::slab::Slab;
use crate::evalns::EvalNamespace;
use crate::parser::{Expression,
                    Value::{self, EConstant, EUnaryOp, EStdFunc, EPrintFunc},
                    UnaryOp::{self, EPos, ENeg, ENot, EParentheses},
                    BinaryOp::{self, EAdd, ESub, EMul, EDiv, EMod, EExp, ELT, ELTE, EEQ, ENE, EGTE, EGT, EOR, EAND},
                    StdFunc::{self, EVar, EFunc, EFuncInt, EFuncCeil, EFuncFloor, EFuncAbs, EFuncSign, EFuncLog, EFuncRound, EFuncMin, EFuncMax, EFuncE, EFuncPi, EFuncSin, EFuncCos, EFuncTan, EFuncASin, EFuncACos, EFuncATan, EFuncSinH, EFuncCosH, EFuncTanH, EFuncASinH, EFuncACosH, EFuncATanH},
                    PrintFunc,
                    ExpressionOrString::{EExpr, EStr},
                    remove_no_panic};
#[cfg(feature="unsafe-vars")]
use crate::parser::StdFunc::EUnsafeVar;
use crate::compiler::{log, Instruction::{self, IConst, INeg, INot, IInv, IAdd, IMul, IMod, IExp, ILT, ILTE, IEQ, INE, IGTE, IGT, IOR, IAND, IVar, IFunc, IFuncInt, IFuncCeil, IFuncFloor, IFuncAbs, IFuncSign, IFuncLog, IFuncRound, IFuncMin, IFuncMax, IFuncSin, IFuncCos, IFuncTan, IFuncASin, IFuncACos, IFuncATan, IFuncSinH, IFuncCosH, IFuncTanH, IFuncASinH, IFuncACosH, IFuncATanH, IPrintFunc}};
#[cfg(feature="unsafe-vars")]
use crate::compiler::Instruction::IUnsafeVar;

use std::collections::BTreeSet;
use std::f64::consts;
use std::fmt;



#[macro_export]
macro_rules! eval_compiled {
    ($evaler:ident, $slab_ref:expr, $ns_mut:expr) => {
        if let al::IConst(c) = $evaler {
            c
        } else {
            #[cfg(feature="unsafe-vars")]
            {
                if let al::IUnsafeVar{ptr, ..} = $evaler {
                    unsafe { *ptr }
                } else {
                    $evaler.eval($slab_ref, $ns_mut)?
                }
            }

            #[cfg(not(feature="unsafe-vars"))]
            $evaler.eval($slab_ref, $ns_mut)?
        }
    };
    ($evaler:expr, $slab_ref:expr, $ns_mut:expr) => {
        {
            let evaler = $evaler;
            eval_compiled!(evaler, $slab_ref, $ns_mut)
        }
    };
}

#[macro_export]
macro_rules! eval_compiled_ref {
    ($evaler:ident, $slab_ref:expr, $ns_mut:expr) => {
        if let al::IConst(c) = $evaler {
            *c
        } else {
            #[cfg(feature="unsafe-vars")]
            {
                if let al::IUnsafeVar{ptr, ..} = $evaler {
                    unsafe { **ptr }
                } else {
                    $evaler.eval($slab_ref, $ns_mut)?
                }
            }

            #[cfg(not(feature="unsafe-vars"))]
            $evaler.eval($slab_ref, $ns_mut)?
        }
    };
    ($evaler:expr, $slab_ref:expr, $ns_mut:expr) => {
        {
            let evaler = $evaler;
            eval_compiled_ref!(evaler, $slab_ref, $ns_mut)
        }
    };
}



pub trait Evaler : fmt::Debug {
    fn eval(&self, slab:&Slab, ns:&mut impl EvalNamespace) -> Result<f64,Error>;

    // Because of ternary short-circuits, we cannot get a complete list of vars just by doing eval() with a clever callback:
    fn _var_names(&self, slab:&Slab, dst:&mut BTreeSet<String>);
    fn var_names(&self, slab:&Slab) -> BTreeSet<String> {
        let mut set = BTreeSet::new();
        self._var_names(slab,&mut set);
        set
    }
}

impl Evaler for Expression {
    fn _var_names(&self, slab:&Slab, dst:&mut BTreeSet<String>) {
        self.first._var_names(slab,dst);
        for pair in &self.pairs {
            pair.1._var_names(slab,dst);
        }
    }
    fn eval(&self, slab:&Slab, ns:&mut impl EvalNamespace) -> Result<f64,Error> {
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
        vals.push(self.first.eval(slab,ns)?);
        for pair in self.pairs.iter() {
            ops.push(pair.0);
            vals.push(pair.1.eval(slab,ns)?);
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
            for i in (0..ops.len()).rev() {
                let op = match ops.get(i) {
                    Some(op) => *op,
                    None => EOR,  // unreachable
                };
                if op==search {
                    let res = op.binaryop_eval(vals.get(i), vals.get(i+1));
                    match vals.get_mut(i) {
                        Some(val_ref) => *val_ref=res,
                        None => (),  // unreachable
                    };
                    remove_no_panic(vals, i+1);
                    remove_no_panic(ops, i);
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
                            let res = op.binaryop_eval(vals.get(i), vals.get(i+1));
                            match vals.get_mut(i) {
                                Some(val_ref) => *val_ref=res,
                                None => (),  // unreachable
                            };
                            remove_no_panic(vals, i+1);
                            remove_no_panic(ops, i);
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
                            let res = op.binaryop_eval(vals.get(i), vals.get(i+1));
                            match vals.get_mut(i) {
                                Some(val_ref) => *val_ref=res,
                                None => (),  // unreachable
                            };
                            remove_no_panic(vals, i+1);
                            remove_no_panic(ops, i);
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
        ltor_multi(&mut vals, &mut ops, &[ELT, EGT, ELTE, EGTE, EEQ, ENE]);  // TODO: Implement Python-style a<b<c ternary comparison... might as well generalize to N comparisons.
        ltor(&mut vals, &mut ops, EAND);
        ltor(&mut vals, &mut ops, EOR);

        if !ops.is_empty() { return Err(Error::Unreachable); }
        if vals.len()!=1 { return Err(Error::Unreachable); }
        match vals.first() {
            Some(val) => Ok(*val),
            None => Err(Error::Unreachable),
        }
    }
}

impl Evaler for Value {
    fn _var_names(&self, slab:&Slab, dst:&mut BTreeSet<String>) {
        match self {
            EConstant(_) => (),
            EUnaryOp(u) => u._var_names(slab,dst),
            EStdFunc(f) => f._var_names(slab,dst),
            EPrintFunc(f) => f._var_names(slab,dst),
        };
    }
    fn eval(&self, slab:&Slab, ns:&mut impl EvalNamespace) -> Result<f64,Error> {
        match self {
            EConstant(c) => Ok(*c),
            EUnaryOp(u) => u.eval(slab,ns),
            EStdFunc(f) => f.eval(slab,ns),
            EPrintFunc(f) => f.eval(slab,ns),
        }
    }
}

impl Evaler for UnaryOp {
    fn _var_names(&self, slab:&Slab, dst:&mut BTreeSet<String>) {
        match self {
            EPos(val_i) | ENeg(val_i) | ENot(val_i) => get_val!(slab.ps,val_i)._var_names(slab,dst),
            EParentheses(expr_i) => get_expr!(slab.ps,expr_i)._var_names(slab,dst),
        }
    }
    fn eval(&self, slab:&Slab, ns:&mut impl EvalNamespace) -> Result<f64,Error> {
        match self {
            EPos(val_i) => get_val!(slab.ps,val_i).eval(slab,ns),
            ENeg(val_i) => Ok(-get_val!(slab.ps,val_i).eval(slab,ns)?),
            ENot(val_i) => Ok(bool_to_f64!(f64_eq!(get_val!(slab.ps,val_i).eval(slab,ns)?,0.0))),
            EParentheses(expr_i) => get_expr!(slab.ps,expr_i).eval(slab,ns),
        }
    }
}

impl BinaryOp {
    // Non-standard eval interface (not generalized yet):
    fn binaryop_eval(self, left_opt:Option<&f64>, right_opt:Option<&f64>) -> f64 {  // Passing 'self' by value is more efficient than pass-by-reference.
        let left = match left_opt {
            Some(l) => *l,
            None => return std::f64::NAN,
        };
        let right = match right_opt {
            Some(r) => *r,
            None => return std::f64::NAN,
        };
        match self {
            EAdd => left+right,
            ESub => left-right,
            EMul => left*right,
            EDiv => left/right,
            EMod => left%right, //left - (left/right).trunc()*right
            EExp => left.powf(right),
            ELT => bool_to_f64!(left<right),
            ELTE => bool_to_f64!(left<=right),
            EEQ => bool_to_f64!(f64_eq!(left,right)),
            ENE => bool_to_f64!(f64_ne!(left,right)),
            EGTE => bool_to_f64!(left>=right),
            EGT => bool_to_f64!(left>right),
            EOR => if f64_ne!(left,0.0) { left }
                   else { right },
            EAND => if f64_eq!(left,0.0) { left }
                    else { right },
        }
    }
}

macro_rules! eval_var {
    ($ns:ident, $name:ident, $args:expr, $keybuf:expr) => {
        match $ns.get_cached($name,$args,$keybuf) {
            Some(f) => Ok(f),
            None => Err(Error::Undefined($name.to_string())),
        }
    };
}

impl Evaler for StdFunc {
    fn _var_names(&self, slab:&Slab, dst:&mut BTreeSet<String>) {
        match self {
            #[cfg(feature="unsafe-vars")]
            EUnsafeVar{name, ..} => { dst.insert(name.clone()); }

            EVar(s) => { dst.insert(s.clone()); }
            EFunc{name, ..} => { dst.insert(name.clone()); }

            EFuncInt(xi) | EFuncCeil(xi) | EFuncFloor(xi) | EFuncAbs(xi) | EFuncSign(xi) | EFuncSin(xi) | EFuncCos(xi) | EFuncTan(xi) | EFuncASin(xi) | EFuncACos(xi) | EFuncATan(xi) | EFuncSinH(xi) | EFuncCosH(xi) | EFuncTanH(xi) | EFuncASinH(xi) | EFuncACosH(xi) | EFuncATanH(xi) => get_expr!(slab.ps,xi)._var_names(slab,dst),

            EFuncE | EFuncPi => (),

            EFuncLog{base:opt,expr} | EFuncRound{modulus:opt,expr} => {
                match opt {
                    Some(xi) => get_expr!(slab.ps,xi)._var_names(slab,dst),
                    None => (),
                }
                get_expr!(slab.ps,expr)._var_names(slab,dst);
            }
            EFuncMin{first,rest} | EFuncMax{first,rest} => {
                get_expr!(slab.ps,first)._var_names(slab,dst);
                for xi in rest {
                    get_expr!(slab.ps,xi)._var_names(slab,dst);
                }
            }
        };
    }
    fn eval(&self, slab:&Slab, ns:&mut impl EvalNamespace) -> Result<f64,Error> {
        match self {
            #[cfg(feature="unsafe-vars")]
            EUnsafeVar{ptr, ..} => unsafe { Ok(**ptr) },

            EVar(name) => eval_var!(ns, name, Vec::new(), unsafe{ &mut *(&slab.ps.char_buf as *const _ as *mut _) }),
            EFunc{name, args:xis} => {
                let mut args = Vec::with_capacity(xis.len());
                for xi in xis {
                    args.push(get_expr!(slab.ps,xi).eval(slab,ns)?)
                }
                eval_var!(ns, name, args, unsafe{ &mut *(&slab.ps.char_buf as *const _ as *mut _) })
            }

            EFuncInt(expr_i) => Ok(get_expr!(slab.ps,expr_i).eval(slab,ns)?.trunc()),
            EFuncCeil(expr_i) => Ok(get_expr!(slab.ps,expr_i).eval(slab,ns)?.ceil()),
            EFuncFloor(expr_i) => Ok(get_expr!(slab.ps,expr_i).eval(slab,ns)?.floor()),
            EFuncAbs(expr_i) => Ok(get_expr!(slab.ps,expr_i).eval(slab,ns)?.abs()),
            EFuncSign(expr_i) => Ok(get_expr!(slab.ps,expr_i).eval(slab,ns)?.signum()),
            EFuncLog{base:base_opt, expr:expr_i} => {
                let base = match base_opt {
                    Some(b_expr_i) => get_expr!(slab.ps,b_expr_i).eval(slab,ns)?,
                    None => 10.0,
                };
                let n = get_expr!(slab.ps,expr_i).eval(slab,ns)?;
                Ok(log(base,n))
            }
            EFuncRound{modulus:modulus_opt, expr:expr_i} => {
                let modulus = match modulus_opt {
                    Some(m_expr_i) => get_expr!(slab.ps,m_expr_i).eval(slab,ns)?,
                    None => 1.0,
                };
                Ok((get_expr!(slab.ps,expr_i).eval(slab,ns)?/modulus).round() * modulus)
            }
            EFuncMin{first:first_i, rest} => {
                let mut min = get_expr!(slab.ps,first_i).eval(slab,ns)?;
                for x_i in rest.iter() {
                    min = min.min(get_expr!(slab.ps,x_i).eval(slab,ns)?);
                }
                Ok(min)
            }
            EFuncMax{first:first_i, rest} => {
                let mut max = get_expr!(slab.ps,first_i).eval(slab,ns)?;
                for x_i in rest.iter() {
                    max = max.max(get_expr!(slab.ps,x_i).eval(slab,ns)?);
                }
                Ok(max)
            }

            EFuncE => Ok(consts::E),
            EFuncPi => Ok(consts::PI),

            EFuncSin(expr_i) => Ok(get_expr!(slab.ps,expr_i).eval(slab,ns)?.sin()),
            EFuncCos(expr_i) => Ok(get_expr!(slab.ps,expr_i).eval(slab,ns)?.cos()),
            EFuncTan(expr_i) => Ok(get_expr!(slab.ps,expr_i).eval(slab,ns)?.tan()),
            EFuncASin(expr_i) => Ok(get_expr!(slab.ps,expr_i).eval(slab,ns)?.asin()),
            EFuncACos(expr_i) => Ok(get_expr!(slab.ps,expr_i).eval(slab,ns)?.acos()),
            EFuncATan(expr_i) => Ok(get_expr!(slab.ps,expr_i).eval(slab,ns)?.atan()),
            EFuncSinH(expr_i) => Ok(get_expr!(slab.ps,expr_i).eval(slab,ns)?.sinh()),
            EFuncCosH(expr_i) => Ok(get_expr!(slab.ps,expr_i).eval(slab,ns)?.cosh()),
            EFuncTanH(expr_i) => Ok(get_expr!(slab.ps,expr_i).eval(slab,ns)?.tanh()),
            EFuncASinH(expr_i) => Ok(get_expr!(slab.ps,expr_i).eval(slab,ns)?.asinh()),
            EFuncACosH(expr_i) => Ok(get_expr!(slab.ps,expr_i).eval(slab,ns)?.acosh()),
            EFuncATanH(expr_i) => Ok(get_expr!(slab.ps,expr_i).eval(slab,ns)?.atanh()),
        }
    }
}

impl Evaler for PrintFunc {
    fn _var_names(&self, slab:&Slab, dst:&mut BTreeSet<String>) {
        for x_or_s in &self.0 {
            match x_or_s {
                EExpr(xi) => get_expr!(slab.ps,xi)._var_names(slab,dst),
                EStr(_) => (),
            };
        }
    }
    fn eval(&self, slab:&Slab, ns:&mut impl EvalNamespace) -> Result<f64,Error> {
        let mut val = 0f64;

        fn process_str(s:&str) -> String {
            s.replace("\\n","\n").replace("\\t","\t")
        }

        if let Some(EStr(fmtstr)) = self.0.first() {
            if fmtstr.contains('%') {
                // printf mode:

                //let fmtstr = process_str(fmtstr);

                return Err(Error::WrongArgs("printf formatting is not yet implemented".to_string()));  // TODO: Make a pure-rust printf libarary.

                //return Ok(val);
            }
        }

        // Normal Mode:
        let mut out = String::with_capacity(16);
        for (i,a) in self.0.iter().enumerate() {
            if i>0 { out.push(' '); }
            match a {
                EExpr(e_i) => {
                    val = get_expr!(slab.ps,e_i).eval(slab,ns)?;
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
    fn _var_names(&self, slab:&Slab, dst:&mut BTreeSet<String>) {
        match self {
            #[cfg(feature="unsafe-vars")]
            IUnsafeVar{name, ..} => { dst.insert(name.clone()); }

            IVar(s) => { dst.insert(s.clone()); }
            IFunc{name, ..} => { dst.insert(name.clone()); }

            IConst(_) => (),

            INeg(ii) | INot(ii) | IInv(ii) | IFuncInt(ii) | IFuncCeil(ii) | IFuncFloor(ii) | IFuncAbs(ii) | IFuncSign(ii) | IFuncSin(ii) | IFuncCos(ii) | IFuncTan(ii) | IFuncASin(ii) | IFuncACos(ii) | IFuncATan(ii) | IFuncSinH(ii) | IFuncCosH(ii) | IFuncTanH(ii) | IFuncASinH(ii) | IFuncACosH(ii) | IFuncATanH(ii) => get_instr!(slab.cs,ii)._var_names(slab,dst),

            IAdd(li,ri) | IMul(li,ri) | ILT(li,ri) | ILTE(li,ri) | IEQ(li,ri) | INE(li,ri) | IGTE(li,ri) | IGT(li,ri) | IOR(li,ri) | IAND(li,ri) | IFuncMin(li,ri) | IFuncMax(li,ri) | IMod{dividend:li, divisor:ri} | IExp{base:li, power:ri} | IFuncLog{base:li, of:ri} | IFuncRound{modulus:li, of:ri} => {
                get_instr!(slab.cs,li)._var_names(slab,dst);
                get_instr!(slab.cs,ri)._var_names(slab,dst);
            }

            IPrintFunc(pf) => pf._var_names(slab,dst),
        }
    }
    fn eval(&self, slab:&Slab, ns:&mut impl EvalNamespace) -> Result<f64,Error> {
        match self {
            // I have manually ordered these match arms in a way that I feel should deliver good performance:

            IMul(li,ri) => Ok( eval_compiled_ref!(get_instr!(slab.cs,li), slab, ns) *
                                            eval_compiled_ref!(get_instr!(slab.cs,ri), slab, ns) ),
            IAdd(li,ri) => Ok( eval_compiled_ref!(get_instr!(slab.cs,li), slab, ns) +
                                            eval_compiled_ref!(get_instr!(slab.cs,ri), slab, ns) ),
            IExp{base, power} => Ok( eval_compiled_ref!(get_instr!(slab.cs,base), slab, ns).powf(
                                                  eval_compiled_ref!(get_instr!(slab.cs,power), slab, ns) ) ),

            INeg(i) => Ok(-eval_compiled_ref!(get_instr!(slab.cs,i), slab, ns)),
            IInv(i) => Ok(1.0/eval_compiled_ref!(get_instr!(slab.cs,i), slab, ns)),

            IVar(name) => eval_var!(ns, name, Vec::new(), unsafe{ &mut *(&slab.ps.char_buf as *const _ as *mut _) }),
            IFunc{name, args:iis} => {
                let mut args = Vec::with_capacity(iis.len());
                for ii in iis {
                    args.push( eval_compiled_ref!(get_instr!(slab.cs,ii), slab, ns) );
                }
                eval_var!(ns, name, args, unsafe{ &mut *(&slab.ps.char_buf as *const _ as *mut _) })
            },

            IFuncLog{base:basei, of:ofi} => {
                let base = eval_compiled_ref!(get_instr!(slab.cs,basei), slab, ns);
                let of = eval_compiled_ref!(get_instr!(slab.cs,ofi), slab, ns);
                Ok(log(base,of))
            }

            IFuncSin(i) => Ok( eval_compiled_ref!(get_instr!(slab.cs,i), slab, ns).sin() ),
            IFuncCos(i) => Ok( eval_compiled_ref!(get_instr!(slab.cs,i), slab, ns).cos() ),
            IFuncTan(i) => Ok( eval_compiled_ref!(get_instr!(slab.cs,i), slab, ns).tan() ),
            IFuncASin(i) => Ok( eval_compiled_ref!(get_instr!(slab.cs,i), slab, ns).asin() ),
            IFuncACos(i) => Ok( eval_compiled_ref!(get_instr!(slab.cs,i), slab, ns).acos() ),
            IFuncATan(i) => Ok( eval_compiled_ref!(get_instr!(slab.cs,i), slab, ns).atan() ),
            IFuncSinH(i) => Ok( eval_compiled_ref!(get_instr!(slab.cs,i), slab, ns).sinh() ),
            IFuncCosH(i) => Ok( eval_compiled_ref!(get_instr!(slab.cs,i), slab, ns).cosh() ),
            IFuncTanH(i) => Ok( eval_compiled_ref!(get_instr!(slab.cs,i), slab, ns).tanh() ),
            IFuncASinH(i) => Ok( eval_compiled_ref!(get_instr!(slab.cs,i), slab, ns).asinh() ),
            IFuncACosH(i) => Ok( eval_compiled_ref!(get_instr!(slab.cs,i), slab, ns).acosh() ),
            IFuncATanH(i) => Ok( eval_compiled_ref!(get_instr!(slab.cs,i), slab, ns).atanh() ),

            IFuncRound{modulus:modi, of:ofi} => {
                let modulus = eval_compiled_ref!(get_instr!(slab.cs,modi), slab, ns);
                let of = eval_compiled_ref!(get_instr!(slab.cs,ofi), slab, ns);
                Ok( (of/modulus).round() * modulus )
            }
            IMod{dividend, divisor} => Ok( eval_compiled_ref!(get_instr!(slab.cs,dividend), slab, ns) %
                                                        eval_compiled_ref!(get_instr!(slab.cs,divisor), slab, ns) ),

            IFuncAbs(i) => Ok( eval_compiled_ref!(get_instr!(slab.cs,i), slab, ns).abs() ),
            IFuncSign(i) => Ok( eval_compiled_ref!(get_instr!(slab.cs,i), slab, ns).signum() ),
            IFuncInt(i) => Ok( eval_compiled_ref!(get_instr!(slab.cs,i), slab, ns).trunc() ),
            IFuncCeil(i) => Ok( eval_compiled_ref!(get_instr!(slab.cs,i), slab, ns).ceil() ),
            IFuncFloor(i) => Ok( eval_compiled_ref!(get_instr!(slab.cs,i), slab, ns).floor() ),
            IFuncMin(li,ri) => {
                let left = eval_compiled_ref!(get_instr!(slab.cs,li), slab, ns);
                let right = eval_compiled_ref!(get_instr!(slab.cs,ri), slab, ns);
                if left<right {
                    Ok(left)
                } else {
                    Ok(right)
                }
            }
            IFuncMax(li,ri) => {
                let left = eval_compiled_ref!(get_instr!(slab.cs,li), slab, ns);
                let right = eval_compiled_ref!(get_instr!(slab.cs,ri), slab, ns);
                if left>right {
                    Ok(left)
                } else {
                    Ok(right)
                }
            }


            IEQ(left, right) => Ok( bool_to_f64!(f64_eq!(eval_compiled_ref!(get_instr!(slab.cs,left), slab, ns),
                                                                    eval_compiled_ref!(get_instr!(slab.cs,right), slab, ns))) ),
            INE(left, right) => Ok( bool_to_f64!(f64_ne!(eval_compiled_ref!(get_instr!(slab.cs,left), slab, ns),
                                                                    eval_compiled_ref!(get_instr!(slab.cs,right), slab, ns))) ),
            ILT(left, right) => Ok( bool_to_f64!(eval_compiled_ref!(get_instr!(slab.cs,left), slab, ns) <
                                                             eval_compiled_ref!(get_instr!(slab.cs,right), slab, ns)) ),
            ILTE(left, right) => Ok( bool_to_f64!(eval_compiled_ref!(get_instr!(slab.cs,left), slab, ns) <=
                                                              eval_compiled_ref!(get_instr!(slab.cs,right), slab, ns)) ),
            IGTE(left, right) => Ok( bool_to_f64!(eval_compiled_ref!(get_instr!(slab.cs,left), slab, ns) >=
                                                              eval_compiled_ref!(get_instr!(slab.cs,right), slab, ns)) ),
            IGT(left, right) => Ok( bool_to_f64!(eval_compiled_ref!(get_instr!(slab.cs,left), slab, ns) >
                                                             eval_compiled_ref!(get_instr!(slab.cs,right), slab, ns)) ),

            INot(i) => Ok(bool_to_f64!(f64_eq!(eval_compiled_ref!(get_instr!(slab.cs,i), slab, ns),0.0))),
            IAND(lefti, righti) => {
                let left = eval_compiled_ref!(get_instr!(slab.cs,lefti), slab, ns);
                if f64_eq!(left,0.0) { Ok(left) }
                else {
                    Ok(eval_compiled_ref!(get_instr!(slab.cs,righti), slab, ns))
                }
            }
            IOR(lefti, righti) => {
                let left = eval_compiled_ref!(get_instr!(slab.cs,lefti), slab, ns);
                if f64_ne!(left,0.0) { Ok(left) }
                else {
                    Ok(eval_compiled_ref!(get_instr!(slab.cs,righti), slab, ns))
                }
            }


            IPrintFunc(pf) => pf.eval(slab,ns),


            // Put these last because you should be using the eval_compiled*!() macros to eliminate function calls.
            IConst(c) => Ok(*c),
            #[cfg(feature="unsafe-vars")]
            IUnsafeVar{ptr, ..} => unsafe { Ok(**ptr) },
        }
    }
}

