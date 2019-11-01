use crate::error::Error;

use std::cell::{Cell, UnsafeCell};

trait StackSlab<T> {
    fn cap() -> usize where Self:Sized;  // https://doc.rust-lang.org/nightly/error-index.html#method-has-no-receiver
    fn len(&self) -> usize;

    // Not possible: (https://doc.rust-lang.org/nomicon/lifetime-mismatch.html)
    // fn push(&mut self, t:T) -> &T
    fn push(&self, t:T) -> Result<(),Error>;
    fn get(&self, i:usize) -> &T;

    fn last(&self) -> &T { self.get(self.len()-1) }
}

macro_rules! def_stackslab {
    ( $n:ident, $s:expr ) => {
        pub struct $n<T> {
            buf: UnsafeCell<[T; $s]>,
            length: Cell<usize>,
        }
        impl<T> $n<T> where T:Default {
            pub fn new() -> Self {
                Self{ buf: UnsafeCell::new(Default::default()),
                      length: Cell::new(0) }
            }
        }
        impl<T> StackSlab<T> for $n<T> {
            fn cap() -> usize { $s }
            fn len(&self) -> usize { self.length.get() }
            fn push(&self, t:T) -> Result<(),Error> {
                let i = self.len();
                if i>=Self::cap() { return Err(Error::new("out-of-bounds")); }
                unsafe { ( &mut *self.buf.get() )[i] = t; }
                self.length.set(i+1);
                Ok(())
            }
            fn get(&self, i:usize) -> &T {
                if i>=self.len() { panic!("out-of-bounds"); }
                //unsafe { return &(& *self.buf.get())[i]; }
                unsafe { &(& *self.buf.get())[i] }
            }
        }
    }
}


def_stackslab!(   StackSlab4,    4);
def_stackslab!(   StackSlab8,    8);
def_stackslab!(  StackSlab16,   16);
def_stackslab!(  StackSlab32,   32);
// def_stackslab!(  StackSlab64,   64);  // 'Default' not implemented for arrays > 32.
// def_stackslab!( StackSlab128,  128);  // See for more init options:
// def_stackslab!( StackSlab256,  256);  // https://www.joshmcguigan.com/blog/array-initialization-rust/
// def_stackslab!( StackSlab512,  512);  // ...Or implement array literal unrolling with a
// def_stackslab!(StackSlab1024, 1024);  // Procedural Macro...


//---- Tests:

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aaa_slab1() {
        let slab = StackSlab4::<i32>::new();
        slab.push(0).unwrap();
        let a1 = slab.last();
        let a2 = slab.last();
        slab.push(1).unwrap();
        let b1 = slab.last();
        let b2 = slab.last();
        eprintln!("{} {} {} {}",a1,a2,b1,b2);
        slab.push(2).unwrap();
        slab.push(3).unwrap();

        assert_eq!(slab.push(4), Err(Error::new("out-of-bounds")));
    }
}

