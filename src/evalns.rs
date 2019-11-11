use crate::slab::Slab;
use crate::evaler::Evaler;

use kerr::KErr;
use stacked::{SVec, SVec64};

use std::collections::HashMap;

//---- Types:

pub struct EvalNS<'a> {
    ns         :NameStack,
    cb         :Box<dyn FnMut(&str)->Option<f64> + 'a>,  // I think a reference would be more efficient than a Box, but then I would need to use a funky 'let cb=|n|{}; EvalNS::new(&cb)' syntax.  The Box results in a super convenient pass-the-cb-by-value API interface.
    reeval_mode:i32,
}
struct NameStack(SVec64<NameLayer>);
struct NameLayer {
    is_eval:bool,
    m      :HashMap<String,f64>,
}

//---- Impls:

impl<'a> EvalNS<'a> {
    pub fn new<F>(cb:F) -> Self where F:FnMut(&str)->Option<f64> + 'a {
        let mut ns = EvalNS{
            ns:NameStack::new(),
            cb:Box::new(cb),
            reeval_mode:0,
        };
        ns.push().unwrap();
        ns
    }

    pub fn push(&mut self) -> Result<usize,KErr> { self.push_eval(self.is_reeval()) }
    pub fn push_eval(&mut self, is_eval:bool) -> Result<usize,KErr> {
        self.ns.0.push(NameLayer{
            is_eval:is_eval,
            m:HashMap::new(),
        })
    }
    pub fn pop(&mut self) {
        self.ns.0.pop();
    }
    pub fn eval_bubble(&mut self, slab:&Slab, evaler:&impl Evaler) -> Result<f64,KErr> {
        self.push().map_err(|e| e.pre("eval_bubble ns.push"))?;
        let out = evaler.eval(slab, self).map_err(|e| e.pre(&format!("eval_bubble({:?})",evaler)));
        self.pop();
        out
    }

    pub fn start_reeval_mode(&mut self) { self.reeval_mode+=1; }
    pub fn end_reeval_mode(&mut self) {
        self.reeval_mode-=1;
        if self.reeval_mode<0 { panic!("too many end_reeval_mdoe"); }
    }

    #[allow(dead_code)]
    fn is_normal(&self) -> bool { self.reeval_mode==0 }
    fn is_reeval(&self) -> bool { self.reeval_mode>0 }

    // Later layers take precedence...
    // ...but groups of 'eval' layers should be treated as one layer, and *earlier* layers take precedence!
    pub fn get(&mut self, name:&str) -> Option<f64> {

        // We can't use a standard 'for i in (0..ns.len()).rev() {}' loop here because the loop's internal logic needs to modify 'i':
        #[allow(non_snake_case)]
        let mut I = self.ns.0.len() as i32;  // Use i32 instead of usize because the loop needs this value to go negative.
        loop { I-=1; if I<0 { break }
            let i = I as usize;  // For easier indexing operations.  We know I>=0 at this point.

            if self.ns.0[i].is_eval {
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
                let len = self.ns.0.len();
                self.ns.0[len-1].m.insert(name.to_string(),val);
                Some(val)
            }
            None => None,
        }
    }
    pub fn create(&mut self, name:&str, val:f64) -> Result<(),KErr> {
        let len = self.ns.0.len();
        let cur_layer = &mut self.ns.0[len-1];
        if cur_layer.m.contains_key(name) { return Err(KErr::new("AlreadyExists")); }
        cur_layer.m.insert(name.to_string(), val);
        Ok(())
    }
}

impl NameStack {
    fn new() -> Self { NameStack(SVec64::new()) }
}

//---- Tests:

#[cfg(test)]
mod tests {
    use super::*;
    
    #[derive(Debug)]
    struct TestEvaler;
    impl Evaler for TestEvaler {
        fn eval(&self, _slab:&Slab, ns:&mut EvalNS) -> Result<f64,KErr> {
            match ns.get("x") {
                Some(v) => Ok(v),
                None => Ok(1.23),
            }
        }
    }

    #[test]
    fn aaa_basics() {
        let slab = Slab::new();
        let mut ns = EvalNS::new(|_| Some(5.4321));
        assert_eq!(ns.eval_bubble(&slab, &TestEvaler{}).unwrap(), 5.4321);
        ns.create("x",1.111).unwrap();
        assert_eq!(ns.eval_bubble(&slab, &TestEvaler{}).unwrap(), 1.111);
        
        assert_eq!(ns.is_normal(), true);
        ns.start_reeval_mode();
            assert_eq!(ns.is_normal(), false);

            ns.start_reeval_mode();
                assert_eq!(ns.is_normal(), false);
                assert_eq!(ns.eval_bubble(&slab, &TestEvaler{}).unwrap(), 1.111);
            ns.end_reeval_mode();

            assert_eq!(ns.is_normal(), false);
        ns.end_reeval_mode();
        assert_eq!(ns.is_normal(), true);
    }
}

