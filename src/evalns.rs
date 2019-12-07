use crate::slab::Slab;
use crate::evaler::Evaler;

use kerr::KErr;

use std::collections::BTreeMap;

//---- Types:

pub struct EvalNS<'a> {
    nstack     :NameStack,
    cb         :Box<dyn FnMut(&str, Vec<f64>)->Option<f64> + 'a>,  // I think a reference would be more efficient than a Box, but then I would need to use a funky 'let cb=|n|{}; EvalNS::new(&cb)' syntax.  The Box results in a super convenient pass-the-cb-by-value API interface.
}
struct NameStack(Vec<NameLayer>);
struct NameLayer(BTreeMap<String,f64>);


//---- Impls:

impl<'a> EvalNS<'a> {
    #[inline]
    pub fn new<F>(cb:F) -> Self where F:FnMut(&str,Vec<f64>)->Option<f64> + 'a { Self::with_capacity(cb, 4) }
    pub fn with_capacity<F>(cb:F, cap:usize) -> Self where F:FnMut(&str,Vec<f64>)->Option<f64> + 'a {
        let mut ns = EvalNS{
            nstack:NameStack(Vec::with_capacity(cap)),
            cb:Box::new(cb),
        };
        ns.push();
        ns
    }

    #[inline]
    pub fn push(&mut self) { 
        self.nstack.0.push(NameLayer(BTreeMap::new()));
    }

    #[inline]
    pub fn pop(&mut self) {
        self.nstack.0.pop();
    }

    pub fn clear(&mut self) {
        while !self.nstack.0.is_empty() { self.pop(); }
        self.push();
    }

    pub fn eval_bubble(&mut self, slab:&Slab, evaler:& impl Evaler) -> Result<f64,KErr> {
        self.push();
        let out = self.eval(slab,evaler);
        self.pop();
        out
    }
    #[inline]
    pub fn eval(&mut self, slab:&Slab, evaler:& impl Evaler) -> Result<f64,KErr> {
        evaler.eval(slab, self).map_err(|e| e.pre(&format!("eval({:?})",evaler)))
    }

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

        for i in (0..self.nstack.0.len()).rev() {
            if let Some(&val) = self.nstack.0[i].0.get(key) { return Some(val); }
        }

        match (self.cb)(name,args) {
            Some(val) => {
                let len = self.nstack.0.len();
                self.nstack.0[len-1].0.insert(key.to_string(),val);
                Some(val)
            }
            None => None,
        }
    }
    pub fn create(&mut self, name:String, val:f64) -> Result<(),KErr> {
        let len = self.nstack.0.len();
        let cur_layer = &mut self.nstack.0[len-1];
        if cur_layer.0.contains_key(&name) { return Err(KErr::new("AlreadyExists")); }
        cur_layer.0.insert(name, val);
        Ok(())
    }
}

