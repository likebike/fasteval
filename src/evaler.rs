use crate::evalns::EvalNS;
use crate::error::Error;
use crate::grammar::{Expression,
                     Value::{self, EConstant, EVariable, EUnaryOp, ECallable},
                     Constant,
                     Variable,
                     UnaryOp::{self, EPos, ENeg, ENot, EParens},
                     BinaryOp::{self, EPlus, EMinus, EMul, EDiv, EMod, EExp, ELT, ELTE, EEQ, ENE, EGTE, EGT, EOR, EAND},
                     Callable::{self, EFunc, EPrintFunc, EEvalFunc},
                     Func::{self, EFuncInt, EFuncAbs, EFuncLog, EFuncRound, EFuncMin, EFuncMax},
                     PrintFunc,
                     EvalFunc,
                     ExpressionOrString::{EExpr, EStr}};

use std::collections::HashSet;

//---- Types:

pub trait Evaler {
    fn eval(&self, ns:&mut EvalNS) -> Result<f64, Error>;

    fn var_names(&self) -> Result<HashSet<String>, Error> {
        let mut set = HashSet::new();
        {
            let mut ns = EvalNS::new(|name:&str| {
                set.insert(name.to_string());
                Some(0.0)
            });
            self.eval(&mut ns)?;
        }
        Ok(set)
    }
}

impl Evaler for Expression {
    fn eval(&self, ns:&mut EvalNS) -> Result<f64, Error> {
        // Order of operations: 1) ^  2) */  3) +-
        // Exponentiation should be processed right-to-left.  Think of what 2^3^4 should mean:
        //     2^(3^4)=2417851639229258349412352   <--- I choose this one.
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

        // if self.0.len()%2!=1 { return Err(Error::new("Expression len should always be odd")) }
        // let mut vals : Vec<f64>      = Vec::with_capacity(self.0.len()/2+1);
        // let mut ops  : Vec<BinaryOp> = Vec::with_capacity(self.0.len()/2  );
        // for (i,tok) in self.0.iter().enumerate() {
        //     match tok {
        //         EValue(val) => {
        //             if i%2==1 { return Err(Error::new("Found value at odd index")) }
        //             match ns.eval_bubble(val) {
        //                 Ok(f) => vals.push(f),
        //                 Err(e) => return Err(e.pre(&format!("eval_bubble({:?})",val))),
        //             }
        //         }
        //         EBinaryOp(bop) => {
        //             if i%2==0 { return Err(Error::new("Found binaryop at even index")) }
        //             ops.push(*bop);
        //         }
        //     }
        // }

        // Code for new Expression data structure:
        let mut vals : Vec<f64>      = Vec::with_capacity(self.pairs.len()/2+1);
        let mut ops  : Vec<BinaryOp> = Vec::with_capacity(self.pairs.len()/2  );
        match ns.eval_bubble(&self.first) {
            Ok(f) => vals.push(f),
            Err(e) => return Err(e.pre(&format!("eval_bubble({:?})",self.first))),
        }
        for pair in self.pairs.iter() {
            ops.push(pair.0);
            match ns.eval_bubble(&pair.1) {
                Ok(f) => vals.push(f),
                Err(e) => return Err(e.pre(&format!("eval_bubble({:?})",pair.1))),
            }
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
        fn rtol(eval_op:&mut dyn FnMut(&mut Vec<BinaryOp>,usize), ops:&mut Vec<BinaryOp>, op:BinaryOp) {
            // for-loop structure:
            let mut i = ops.len() as i64;
            loop { i-=1; if i<0 { break }
                let i = i as usize;

                if ops[i]==op { eval_op(ops,i); }
            }
        };
        fn ltor(eval_op:&mut dyn FnMut(&mut Vec<BinaryOp>,usize), ops:&mut Vec<BinaryOp>, op:BinaryOp) {
            'outer: loop {
                // for-loop structure:
                let mut i : i64 = -1;
                loop { i+=1; if i>=ops.len() as i64 { break 'outer; }
                    let i = i as usize;

                    if ops[i]==op {
                        eval_op(ops,i);
                        continue 'outer;  // Need to restart processing when modifying from the left.
                    }
                }
            }
        };

        rtol(&mut eval_op, &mut ops, EExp);
        ltor(&mut eval_op, &mut ops, EMod);
        ltor(&mut eval_op, &mut ops, EDiv);
        rtol(&mut eval_op, &mut ops, EMul);
        ltor(&mut eval_op, &mut ops, EMinus);
        rtol(&mut eval_op, &mut ops, EPlus);
        ltor(&mut eval_op, &mut ops, ELT);
        ltor(&mut eval_op, &mut ops, EGT);
        ltor(&mut eval_op, &mut ops, ELTE);
        ltor(&mut eval_op, &mut ops, EGTE);
        ltor(&mut eval_op, &mut ops, EEQ);
        ltor(&mut eval_op, &mut ops, ENE);
        ltor(&mut eval_op, &mut ops, EAND);
        ltor(&mut eval_op, &mut ops, EOR);

        if ops.len()!=0 { return Err(Error::new("Unhandled Expression ops")); }
        if vals.len()!=1 { return Err(Error::new("More than one final Expression value")); }
        Ok(vals[0])
    }
}

impl Evaler for Value {
    fn eval(&self, ns:&mut EvalNS) -> Result<f64, Error> {
        match self {
            EConstant(c) => c.eval(ns),
            EVariable(v) => v.eval(ns),
            EUnaryOp(u) => u.eval(ns),
            ECallable(c) => c.eval(ns),
        }
    }
}

impl Evaler for Constant {
    fn eval(&self, _ns:&mut EvalNS) -> Result<f64, Error> { Ok(self.0) }
}

impl Evaler for Variable {
    fn eval(&self, ns:&mut EvalNS) -> Result<f64, Error> {
        match ns.get(&self.0) {
            Some(f) => Ok(f),
            None => Err(Error::new("variable undefined")),
        }
    }
}

impl Evaler for UnaryOp {
    fn eval(&self, ns:&mut EvalNS) -> Result<f64, Error> {
        match self {
            EPos(box_val) => ns.eval_bubble(box_val.as_ref()),
            ENeg(box_val) => Ok(-ns.eval_bubble(box_val.as_ref())?),
            ENot(box_val) => Ok(bool_to_f64(ns.eval_bubble(box_val.as_ref())?==0.0)),
            EParens(expr) => ns.eval_bubble(expr.as_ref()),
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
    fn eval(&self, ns:&mut EvalNS) -> Result<f64, Error> {
        match self {
            EFunc(f) => ns.eval_bubble(f),
            EEvalFunc(f) => ns.eval_bubble(f),
            EPrintFunc(f) => ns.eval_bubble(f),
        }
    }
}

impl Evaler for Func {
    fn eval(&self, ns:&mut EvalNS) -> Result<f64, Error> {
        match self {
            EFuncInt(expr) => { Ok(ns.eval_bubble(expr.as_ref())?.trunc()) }
            EFuncAbs(expr) => { Ok(ns.eval_bubble(expr.as_ref())?.abs()) }
            EFuncLog{base,val} => {
                let base = match base {
                    Some(b_expr) => ns.eval_bubble(b_expr.as_ref())?,
                    None => 10.0,
                };
                Ok(ns.eval_bubble(val.as_ref())?.log(base))
            }
            EFuncRound{modulus,val} => {
                let modulus = match modulus {
                    Some(m_expr) => ns.eval_bubble(m_expr.as_ref())?,
                    None => 1.0,
                };
                Ok((ns.eval_bubble(val.as_ref())?/modulus).round() * modulus)
            }
            EFuncMin{first,rest} => {
                let mut min = ns.eval_bubble(first.as_ref())?;
                for x in rest.iter() {
                    min = min.min(ns.eval_bubble(x)?);
                }
                Ok(min)
            }
            EFuncMax{first,rest} => {
                let mut max = ns.eval_bubble(first.as_ref())?;
                for x in rest.iter() {
                    max = max.max(ns.eval_bubble(x)?);
                }
                Ok(max)
            }
        }
    }
}

impl Evaler for PrintFunc {
    fn eval(&self, ns:&mut EvalNS) -> Result<f64, Error> {
        let mut val = 0f64;

        fn process_str(s:&str) -> String {
            let s = s.replace("\\n","\n");
            let s = s.replace("\\t","\t");
            s
        }

        if self.0.len()>0 {
            if let EStr(ref fmtstr) = (*self.0)[0] {
                if fmtstr.contains("%") {
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
                EExpr(e) => {
                    val = ns.eval_bubble(e.as_ref())?;
                    out.push_str(&format!("{}",val));
                }
                EStr(s) => out.push_str(&process_str(s)),
            }
        }
        eprintln!("{}", out);

        Ok(val)
    }
}

impl Evaler for EvalFunc {
    fn eval(&self, ns:&mut EvalNS) -> Result<f64, Error> {
        // Don't affect the external namespace:
        // If you do, you get some surprising behavior:
        //     var a=0
        //     var b=eval(1,a=9)
        //     var _=print(a,b)   // "0 1"
        //     var _=print(b,a)   // "1 9"
        ns.push_eval(true);
        // This is my 'defer ns.pop();' structure:
        let res = (|| -> Result<f64, Error> {

            for kw in self.kwargs.iter() {
                let val = ns.eval_bubble(kw.expr.as_ref())?;
                ns.create(kw.name.0.as_ref(), val)?;
            }

            ns.start_reeval_mode();
            // Another defer structure (a bit overly-complex for this simple case):
            let res = (|| -> Result<f64, Error> {
                ns.eval_bubble(self.expr.as_ref())
            })();
            ns.end_reeval_mode();
            res

        })();
        ns.pop();
        res
    }
}



pub fn bool_to_f64(b:bool) -> f64 {
    if b { 1.0 }
    else { 0.0 }
}




//---- Tests:

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;
    use std::time::Instant;

    #[test]
    fn aaa_util() {
        assert_eq!(bool_to_f64(true), 1.0);
        assert_eq!(bool_to_f64(false), 0.0);
    }

    #[test]
    fn aaa_basics() {
        let p = Parser{
            is_const_byte:None,
            is_var_byte:None,
        };
        assert_eq!(
            p.parse("12.34 + 43.21 + 11.11").unwrap().var_names().unwrap(),
            HashSet::new());

        let mut ns = EvalNS::new(|_| None);
        assert_eq!(
            p.parse("12.34 + 43.21 + 11.11").unwrap().eval(&mut ns),
            Ok(66.66));
        assert_eq!(
            p.parse("12.34 + 43.21 - 11.11").unwrap().eval(&mut ns),
            Ok(44.44));
        assert_eq!(
            p.parse("11.11 * 3").unwrap().eval(&mut ns),
            Ok(33.33));
        assert_eq!(
            p.parse("33.33 / 3").unwrap().eval(&mut ns),
            Ok(11.11));
        assert_eq!(
            p.parse("33.33 % 3").unwrap().eval(&mut ns),
            Ok(0.3299999999999983));
        assert_eq!(
            p.parse("1 and 2").unwrap().eval(&mut ns),
            Ok(2.0));
        assert_eq!(
            p.parse("2 or 0").unwrap().eval(&mut ns),
            Ok(2.0));
        assert_eq!(
            p.parse("1 > 0").unwrap().eval(&mut ns),
            Ok(1.0));
        assert_eq!(
            p.parse("1 < 0").unwrap().eval(&mut ns),
            Ok(0.0));

        assert_eq!(
            p.parse("+5.5").unwrap().eval(&mut ns),
            Ok(5.5));
        assert_eq!(
            p.parse("-5.5").unwrap().eval(&mut ns),
            Ok(-5.5));
        assert_eq!(
            p.parse("!5.5").unwrap().eval(&mut ns),
            Ok(0.0));
        assert_eq!(
            p.parse("!0").unwrap().eval(&mut ns),
            Ok(1.0));
        assert_eq!(
            p.parse("(3 * 3 + 3 / 3)").unwrap().eval(&mut ns),
            Ok(10.0));
        assert_eq!(
            p.parse("(3 * (3 + 3) / 3)").unwrap().eval(&mut ns),
            Ok(6.0));

        assert_eq!(
            p.parse("4.4 + -5.5").unwrap().eval(&mut ns),
            Ok(-1.0999999999999996));
        assert_eq!(
            p.parse("4.4 + +5.5").unwrap().eval(&mut ns),
            Ok(9.9));

        assert_eq!(
            p.parse("x + 1").unwrap().eval(&mut ns),
            Err(Error::new("variable undefined")));

        let mut ns = EvalNS::new(|_| Some(3.0));
        assert_eq!(
            p.parse("x + 1").unwrap().eval(&mut ns),
            Ok(4.0));

        assert_eq!(
            p.parse("1.2 + int(3.4)").unwrap().eval(&mut ns),
            Ok(4.2));
        assert_eq!(
            p.parse("1.2 + abs(-3.4)").unwrap().eval(&mut ns),
            Ok(4.6));
        assert_eq!(
            p.parse("1.2 + log(1)").unwrap().eval(&mut ns),
            Ok(1.2));
        assert_eq!(
            p.parse("1.2 + log(10)").unwrap().eval(&mut ns),
            Ok(2.2));
        assert_eq!(
            p.parse("1.2 + log(0)").unwrap().eval(&mut ns),
            Ok(std::f64::NEG_INFINITY));
        assert!(p.parse("1.2 + log(-1)").unwrap().eval(&mut ns).unwrap().is_nan());
        assert_eq!(
            p.parse("1.2 + round(3.4)").unwrap().eval(&mut ns),
            Ok(4.2));
        assert_eq!(
            p.parse("1.2 + round(0.5, 3.4)").unwrap().eval(&mut ns),
            Ok(4.7));
        assert_eq!(
            p.parse("1.2 + round(-3.4)").unwrap().eval(&mut ns),
            Ok(-1.8));
        assert_eq!(
            p.parse("1.2 + round(0.5, -3.4)").unwrap().eval(&mut ns),
            Ok(-2.3));
        assert_eq!(
            p.parse("1.2 + min(1,2,0,3.3,-1)").unwrap().eval(&mut ns),
            Ok(0.19999999999999996));
        assert_eq!(
            p.parse("1.2 + min(1)").unwrap().eval(&mut ns),
            Ok(2.2));
        assert_eq!(
            p.parse("1.2 + max(1,2,0,3.3,-1)").unwrap().eval(&mut ns),
            Ok(4.5));
        assert_eq!(
            p.parse("1.2 + max(1)").unwrap().eval(&mut ns),
            Ok(2.2));

        assert_eq!(
            p.parse(r#"12.34 + print ( 43.21, "yay" ) + 11.11"#).unwrap().eval(&mut ns),
            Ok(66.66));

        assert_eq!(
            p.parse(r#"12.34 + eval ( x + 43.21 - y, x=2.5, y = 2.5 ) + 11.11"#).unwrap().eval(&mut ns),
            Ok(66.66));

    }

    #[test]
    fn bench() {
        eprintln!();

        let p = Parser{
            is_const_byte:None,
            is_var_byte:None,
        };
        let mut ns = EvalNS::new(|_| None);

        let count = 1000000;

        {
            let mut sum = 0f64;
            let start = Instant::now();
            for _ in 0..count {
                let expr = p.parse("(3 * (3 + 3) / 3)").unwrap();
                match expr.eval(&mut ns) {
                    Ok(f) => { sum+=f; }
                    Err(e) => panic!(format!("error during benchmark: {}",e)),
                }
            }
            eprintln!("eval bench: {}  {}",sum,Instant::now().duration_since(start).as_secs_f64());
        }

        {
            let mut sum = 0f64;
            let start = Instant::now();
            for _ in 0..count {
                let x = 3.0 * (3.0 + 3.0) / 3.0;
                sum+=x;
            }
            eprintln!("raw  bench: {}  {}",sum,Instant::now().duration_since(start).as_secs_f64());
        }
    }
}

