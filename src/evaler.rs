use crate::evalns::EvalNS;

use std::collections::HashSet;
use std::cell::RefCell;

//---- Types:

pub trait Evaler {
    fn eval(&self, ns:&mut EvalNS) -> f64;

    fn var_names(&self) -> HashSet<String> {
        let out = RefCell::new(HashSet::new());
        let clos = |name:&str| {
            out.borrow_mut().insert(name.to_string());
            None
        };
        let mut ns = EvalNS::new(&clos);
        self.eval(&mut ns);
        out.into_inner()
    }
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
    fn var_names() {
        
    }
}

