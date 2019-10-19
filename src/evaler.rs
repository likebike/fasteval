use crate::evalns::EvalNS;
use crate::error::Error;
use crate::grammar::{Expression, BinaryOp};

use std::collections::HashSet;

//---- Types:

pub trait Evaler {
    fn eval(&self, ns:&mut EvalNS) -> Result<f64, Error>;

    fn var_names(&self) -> Result<HashSet<String>, Error> {
        // let out = RefCell::new(HashSet::new());
        // let clos = |name:&str| {
        //     out.borrow_mut().insert(name.to_string());
        //     None
        // };
        // let mut ns = EvalNS::new(&clos);
        // self.eval(&mut ns)?;
        // out.into_inner()



        let mut set = HashSet::new();
        {
            let mut ns = EvalNS::new(|name:&str| {
                set.insert(name.to_string());
                None
            });
            self.eval(&mut ns)?;
        }
        Ok(set)
    }
}

impl Evaler for Expression {
    fn eval(&self, ns:&mut EvalNS) -> Result<f64, Error> {
        if self.0.len()%2!=1 { return Err(Error::new("Expression len should always be odd")) }

        // Order of operations: 1) ^  2) */  3) +-
        // Exponentiation should be processed right-to-left.  Think of what 2^3^4 should mean:
        //     2^(3^4)=2417851639229258349412352   <--- I choose this one.
        //     (2^3)^4=4096
        // Direction of processing doesn't matter for Addition and Multiplication:
        //     (((3+4)+5)+6)==(3+(4+(5+6))), (((3*4)*5)*6)==(3*(4*(5*6)))
        // ...But Subtraction and Division must be processed left-to-right:
        //     (((6-5)-4)-3)!=(6-(5-(4-3))), (((6/5)/4)/3)!=(6/(5/(4/3)))

        let mut vals : Vec<f64>      = Vec::with_capacity(self.0.len()/2+1);
        let mut ops  : Vec<BinaryOp> = Vec::with_capacity(self.0.len()/2  );


// vals,ops:=make([]float64, len(e)/2+1),make([]BinaryOp, len(e)/2)
// for i:=0; i<len(e); i+=2 {
//     vals[i/2]=ns.EvalBubble(e[i].(evaler))
//     if i<len(e)-1 { ops[i/2]=e[i+1].(BinaryOp) }
// }
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
// rtol("^"); ltor("%"); ltor("/"); rtol("*"); ltor("-"); rtol("+"); ltor("<"); ltor(">"); ltor("<="); ltor(">="); ltor("=="); ltor("!="); ltor("and"); ltor("or")
// if len(ops)!=0 { panic(errors.New("Unhandled Expression ops")) }
// if len(vals)!=1 { panic(errors.New("More than one final Expression value")) }
// return vals[0]


        unimplemented!();
    }
}

//---- Tests:

// #[cfg(test)]
// mod tests {
//     use super::*;
// 
//     struct TestEvaler;
//     impl Evaler for TestEvaler {
//         fn eval(&self, ns:&mut EvalNS) -> f64 {
//             match ns.get("x") {
//                 Some(v) => v,
//                 None => 1.23,
//             }
//         }
//     }
// 
//     #[test]
//     fn var_names() {
//         
//     }
// }

