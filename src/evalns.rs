use crate::error::Error::{self, *};
use crate::evaler::Evaler;

use std::collections::HashMap;

//---- Types:

pub struct EvalNS<'a> {
    ns         :NameStack,
    cb         :&'a Fn(&str)->Option<f64>,
    reeval_mode:i32,
}
struct NameStack(Vec<NameLayer>);
struct NameLayer {
    is_eval:bool,
    m      :HashMap<String,f64>,
}

//---- Impls:

impl<'a> EvalNS<'a> {
    pub fn new(cb:&'a Fn(&str)->Option<f64>) -> Self {
        let mut ns = EvalNS{
            ns:NameStack::new(),
            cb:cb,
            reeval_mode:0,
        };
        ns.push();
        ns
    }

    fn push(&mut self) { self.push_eval(self.is_reeval()) }
    fn push_eval(&mut self, is_eval:bool) {
        self.ns.0.push(NameLayer{
            is_eval:is_eval,
            m:HashMap::new(),
        });
    }
    fn pop(&mut self) {
        match self.ns.0.pop() {
            Some(_) => {},
            None => panic!("too many pops"),
        }
    }
    pub fn eval_bubble(&mut self, evaler:&dyn Evaler) -> f64 {
        self.push();
        let out = evaler.eval(self);
        self.pop();
        out
    }

    fn start_reeval_mode(&mut self) { self.reeval_mode+=1; }
    fn end_reeval_mode(&mut self) {
        self.reeval_mode-=1;
        if self.reeval_mode<0 { panic!("too many end_reeval_mdoe"); }
    }

    fn is_normal(&self) -> bool { self.reeval_mode==0 }
    fn is_reeval(&self) -> bool { self.reeval_mode>0 }

    // Later layers take precedence...
    // ...but groups of 'eval' layers should be treated as one layer, and *earlier* layers take precedence!
    pub fn get(&mut self, name:&str) -> Option<f64> {

        // This is the closest thing I can think of to a c-style 'for' loop:
        //     for i:=len(me.NS)-1;i>=0;i-- {...}
        #[allow(non_snake_case)]
        let mut I = self.ns.0.len() as i32;  // Use i32 instead of usize because the loop needs this value to go negative.
        loop { I-=1; if I<0 { break }
            let i = I as usize;  // For easier indexing operations.  We know I>0 at this point.

            if self.ns.0[i].is_eval {
                eprintln!("EvalNS get eval group is un-tested.  (Waiting for implementation of eval.)");
                // Eval layer: treat neighboring eval layers as a group.
                let mut j = i;
                while j>0 && self.ns.0[j-1].is_eval { j-=1 }

                let mut k = j - 1;  // -1 for loop initial increment.
                loop { k+=1; if k>i { break }
                    match self.ns.0[k].m.get(name) {
                        Some(&val) => return Some(val),
                        None => {}
                    }
                }

                I = j as i32;
            } else {
                // Normal layer
                match self.ns.0[i].m.get(name) {
                    Some(&val) => return Some(val),
                    None => {}
                }
            }
        }

        match (self.cb)(name) {
            Some(val) => {
                self.ns.0.last_mut().unwrap().m.insert(name.to_string(),val);
                Some(val)
            }
            None => None,
        }
    }
    pub fn create(&mut self, name:&str, val:f64) -> Result<(),Error> {
        let cur_layer = self.ns.0.last_mut().unwrap();
        if cur_layer.m.contains_key(name) { return Err(AlreadyExists); }
        cur_layer.m.insert(name.to_string(), val);
        Ok(())
    }
}

impl NameStack {
    fn new() -> Self { NameStack(Vec::new()) }
}

//---- Tests:

#[cfg(test)]
mod tests {
    use super::*;
    
    struct TestEvaler;
    impl Evaler for TestEvaler {
        fn eval(&self, ns:&mut EvalNS) -> f64 {
            match ns.get("x") {
                Some(v) => v,
                None => 1.23,
            }
        }
    }

    #[test]
    fn basics() {
        let mut ns = EvalNS::new(&|_n| Some(5.4321));
        assert_eq!(ns.eval_bubble(&TestEvaler{}), 5.4321);
        ns.create("x",1.111).unwrap();
        assert_eq!(ns.eval_bubble(&TestEvaler{}), 1.111);
        
        assert_eq!(ns.is_normal(), true);
        ns.start_reeval_mode();
            assert_eq!(ns.is_normal(), false);

            ns.start_reeval_mode();
                assert_eq!(ns.is_normal(), false);
                assert_eq!(ns.eval_bubble(&TestEvaler{}), 1.111);
            ns.end_reeval_mode();

            assert_eq!(ns.is_normal(), false);
        ns.end_reeval_mode();
        assert_eq!(ns.is_normal(), true);
    }
}

