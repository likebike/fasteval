use crate::slab::Slab;
use crate::evaler::Evaler;

use kerr::KErr;

use std::collections::BTreeMap;

//---- Types:

pub trait EvalNamespace {
    fn get_cached(&mut self, name:&str, args:Vec<f64>) -> Option<f64>;
    fn set_cached(&mut self, name:String, val:f64);
    fn create_cached(&mut self, name:String, val:f64) -> Result<(),KErr>;
    fn clear_cached(&mut self);

    #[inline(always)]
    fn eval(&mut self, slab:&Slab, evaler:& impl Evaler) -> Result<f64,KErr> where Self:Sized {
        evaler.eval(slab, self).map_err(|e| e.pre(&format!("eval({:?})",evaler)))
    }
}

pub struct EmptyNamespace;

pub struct FlatNamespace<'a> {
    map:BTreeMap<String,f64>,
    cb :Box<dyn FnMut(&str, Vec<f64>)->Option<f64> + 'a>,  // I think a reference would be more efficient than a Box, but then I would need to use a funky 'let cb=|n|{}; Namespace::new(&cb)' syntax.  The Box results in a super convenient pass-the-cb-by-value API interface.
}

pub struct ScopedNamespace<'a> {
    maps:Vec<BTreeMap<String,f64>>,
    cb  :Box<dyn FnMut(&str, Vec<f64>)->Option<f64> + 'a>,
}

//---- Impls:

#[inline(always)]
fn key_from_nameargs<'a,'b:'a>(keybuf:&'a mut String, name:&'b str, args:&[f64]) -> &'a str {
    if args.is_empty() {
        name
    } else {
        keybuf.reserve(name.len() + 20*args.len());
        keybuf.push_str(name);
        for f in args {
            keybuf.push_str(" , ");
            keybuf.push_str(&f.to_string());
        };
        keybuf.as_str()
    }
}

impl EvalNamespace for BTreeMap<String,f64> {
    fn get_cached(&mut self, name:&str, args:Vec<f64>) -> Option<f64> {
        let mut keybuf = String::new();
        let key = key_from_nameargs(&mut keybuf, name, &args);
        self.get(key).copied()
    }
    // Think of the 'self' BTreeMap as an alternative to a callback.  When you set/create/clear for other Namespace types,
    // it doesn't modify the callback results -- it modifies the Namespace cache.  Therefore, these become no-ops for this type:
    fn set_cached(&mut self, _name:String, _val:f64) { panic!("cannot set cached value in BTreeMap Namespace"); }
    fn create_cached(&mut self, _name:String, _val:f64) -> Result<(),KErr> { panic!("cannot create cached value in BTreeMap Namespace"); }
    fn clear_cached(&mut self) {}
}

impl EvalNamespace for Vec<BTreeMap<String,f64>> {
    fn get_cached(&mut self, name:&str, args:Vec<f64>) -> Option<f64> {
        let mut keybuf = String::new();
        let key = key_from_nameargs(&mut keybuf, name, &args);

        for map in self.iter().rev() {
            if let Some(&val) = map.get(key) { return Some(val); }
        }
        None
    }
    // Think of the 'self' Vec<BTreeMap> as an alternative to a callback.  When you set/create/clear for other Namespace types,
    // it doesn't modify the callback results -- it modifies the Namespace cache.  Therefore, these become no-ops for this type:
    fn set_cached(&mut self, _name:String, _val:f64) { panic!("cannot set cached value in Vec<BTreeMap> Namespace"); }
    fn create_cached(&mut self, _name:String, _val:f64) -> Result<(),KErr> { panic!("cannot create cached value in Vec<BTreeMap> Namespace"); }
    fn clear_cached(&mut self) {}
}

impl EvalNamespace for EmptyNamespace {
    fn get_cached(&mut self, _name:&str, _args:Vec<f64>) -> Option<f64> { None }
    fn set_cached(&mut self, _name:String, _val:f64) { panic!("cannot set cached value in EmptyNamespace"); }
    fn create_cached(&mut self, _name:String, _val:f64) -> Result<(),KErr> { panic!("cannot create cached value in EmptyNamespace"); }
    fn clear_cached(&mut self) {}
}


impl EvalNamespace for FlatNamespace<'_> {
    fn get_cached(&mut self, name:&str, args:Vec<f64>) -> Option<f64> {
        let mut keybuf = String::new();
        let key = key_from_nameargs(&mut keybuf, name, &args);

        if let Some(&val) = self.map.get(key) { return Some(val); }

        match (self.cb)(name,args) {
            Some(val) => {
                self.map.insert(key.to_string(),val);
                Some(val)
            }
            None => None,
        }
    }
    fn set_cached(&mut self, name:String, val:f64) {
        self.map.insert(name, val);
    }
    fn create_cached(&mut self, name:String, val:f64) -> Result<(),KErr> {
        if self.map.contains_key(&name) { return Err(KErr::new("AlreadyExists")); }
        self.map.insert(name, val);
        Ok(())
    }
    fn clear_cached(&mut self) {
        self.map = BTreeMap::new();
    }
}

impl<'a> FlatNamespace<'a> {
    #[inline]
    pub fn new<F>(cb:F) -> Self where F:FnMut(&str,Vec<f64>)->Option<f64> + 'a {
        FlatNamespace{
            map:BTreeMap::new(),
            cb :Box::new(cb),
        }
    }
}

impl EvalNamespace for ScopedNamespace<'_> {
    fn get_cached(&mut self, name:&str, args:Vec<f64>) -> Option<f64> {
        let mut keybuf = String::new();
        let key = key_from_nameargs(&mut keybuf, name, &args);

        for map in self.maps.iter().rev() {
            if let Some(&val) = map.get(key) { return Some(val); }
        }

        match (self.cb)(name,args) {
            Some(val) => {
                self.maps.last_mut().unwrap().insert(key.to_string(),val);
                Some(val)
            }
            None => None,
        }
    }
    fn set_cached(&mut self, name:String, val:f64) {
        self.maps.last_mut().unwrap().insert(name, val);
    }
    fn create_cached(&mut self, name:String, val:f64) -> Result<(),KErr> {
        let cur_layer = self.maps.last_mut().unwrap();
        if cur_layer.contains_key(&name) { return Err(KErr::new("AlreadyExists")); }
        cur_layer.insert(name, val);
        Ok(())
    }
    fn clear_cached(&mut self) {
        self.maps = Vec::with_capacity(self.maps.len());  // Assume the future usage will be similar to historical usage.
        self.push();
    }
}

impl<'a> ScopedNamespace<'a> {
    #[inline]
    pub fn new<F>(cb:F) -> Self where F:FnMut(&str,Vec<f64>)->Option<f64> + 'a {
        let mut ns = ScopedNamespace{
            maps:Vec::with_capacity(2),
            cb  :Box::new(cb),
        };
        ns.push();
        ns
    }

    #[inline]
    pub fn push(&mut self) {
        self.maps.push(BTreeMap::new());
    }
    #[inline]
    pub fn pop(&mut self) {
        self.maps.pop();
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
}

