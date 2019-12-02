use crate::slab::Slab;
use crate::evaler::Evaler;

use kerr::KErr;

use std::collections::BTreeMap;

//---- Types:

pub struct EvalNS<'a> {
    nstack     :NameStack,
    cb         :Box<dyn FnMut(&str, Vec<f64>)->Option<f64> + 'a>,  // I think a reference would be more efficient than a Box, but then I would need to use a funky 'let cb=|n|{}; EvalNS::new(&cb)' syntax.  The Box results in a super convenient pass-the-cb-by-value API interface.
    reeval_mode:i32,
}
struct NameStack(Vec<NameLayer>);
struct NameLayer {
    is_eval:bool,
    m      :BTreeMap<String,f64>,
}

//---- Impls:

impl<'a> EvalNS<'a> {
    #[inline]
    pub fn new<F>(cb:F) -> Self where F:FnMut(&str,Vec<f64>)->Option<f64> + 'a { Self::with_capacity(cb, 8) }
    pub fn with_capacity<F>(cb:F, cap:usize) -> Self where F:FnMut(&str,Vec<f64>)->Option<f64> + 'a {
        let mut ns = EvalNS{
            nstack:NameStack(Vec::with_capacity(cap)),
            cb:Box::new(cb),
            reeval_mode:0,
        };
        ns.push().unwrap();
        ns
    }

    #[inline]
    pub fn push(&mut self) -> Result<usize,KErr> { self.push_eval(self.is_reeval()) }
    pub fn push_eval(&mut self, is_eval:bool) -> Result<usize,KErr> {
        let i = self.nstack.0.len();
        if i>=self.nstack.0.capacity() { return Err(KErr::new("evalns overflow")) }
        self.nstack.0.push(NameLayer{
            is_eval,
            m:BTreeMap::new(),
        });
        Ok(i)
    }

    #[inline]
    pub fn pop(&mut self) {
        self.nstack.0.pop();
    }

    pub fn clear(&mut self) {
        if self.reeval_mode!=0 { panic!("pending reeval"); }
        while !self.nstack.0.is_empty() { self.pop(); }
        self.push().unwrap();
    }

    pub fn eval_bubble(&mut self, slab:&Slab, evaler:& impl Evaler) -> Result<f64,KErr> {
        self.push().map_err(|e| e.pre("eval_bubble ns.push"))?;
        let out = self.eval(slab,evaler);
        self.pop();
        out
    }
    #[inline]
    pub fn eval(&mut self, slab:&Slab, evaler:& impl Evaler) -> Result<f64,KErr> {
        evaler.eval(slab, self).map_err(|e| e.pre(&format!("eval({:?})",evaler)))
    }

    #[inline]
    pub fn start_reeval_mode(&mut self) { self.reeval_mode+=1; }
    #[inline]
    pub fn end_reeval_mode(&mut self) {
        self.reeval_mode-=1;
        if self.reeval_mode<0 { panic!("too many end_reeval_mdoe"); }
    }

    #[inline]
    pub fn is_reeval(&self) -> bool { self.reeval_mode>0 }

    // Later layers take precedence...
    // ...but groups of 'eval' layers should be treated as one layer, and *earlier* layers take precedence!
    pub fn get(&mut self, name:&str, args:Vec<f64>) -> Option<f64> {
        let mut keybuf = String::new();
        let key = if args.is_empty() {
            name
        } else {
            keybuf.reserve(name.len() + 20*args.len());
            keybuf.push_str(name);
            for f in &args {
                keybuf.push_str(" , ");
                keybuf.push_str(&f.to_string());
            };
            keybuf.as_str()
        };

        // We can't use a standard 'for i in (0..ns.len()).rev() {}' loop here because the loop's internal logic needs to modify 'i':
        #[allow(non_snake_case)]
        let mut I = self.nstack.0.len() as i32;  // Use i32 instead of usize because the loop needs this value to go negative.
        loop { I-=1; if I<0 { break }
            let i = I as usize;  // For easier indexing operations.  We know I>=0 at this point.

            if self.nstack.0[i].is_eval {
                // Eval layer: treat neighboring eval layers as a group.
                let mut j = i;
                while j>0 && self.nstack.0[j-1].is_eval { j-=1 }

                for k in j..=i {
                    if let Some(&val) = self.nstack.0[k].m.get(key) { return Some(val); }
                }

                I = j as i32;
            } else {
                // Normal layer
                if let Some(&val) = self.nstack.0[i].m.get(key) { return Some(val); }
            }
        }

        match (self.cb)(name,args) {
            Some(val) => {
                let len = self.nstack.0.len();
                self.nstack.0[len-1].m.insert(key.to_string(),val);
                Some(val)
            }
            None => None,
        }
    }
    pub fn create(&mut self, name:String, val:f64) -> Result<(),KErr> {
        let len = self.nstack.0.len();
        let cur_layer = &mut self.nstack.0[len-1];
        if cur_layer.m.contains_key(&name) { return Err(KErr::new("AlreadyExists")); }
        cur_layer.m.insert(name, val);
        Ok(())
    }
}

