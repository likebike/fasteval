use crate::error::Error;

use std::cell::{Cell, UnsafeCell};
use std::fmt;

pub trait StackVec<T> {
    fn cap() -> usize where Self:Sized;  // https://doc.rust-lang.org/nightly/error-index.html#method-has-no-receiver
    fn len(&self) -> usize;

    // Not possible: (https://doc.rust-lang.org/nomicon/lifetime-mismatch.html)
    // fn push(&mut self, t:T) -> &T
    fn push(&self, t:T) -> Result<usize,Error>;
    fn get(&self, i:usize) -> &T;
}
pub trait StackString {
    fn cap() -> usize where Self:Sized;  // https://doc.rust-lang.org/nightly/error-index.html#method-has-no-receiver
    fn len(&self) -> usize;

    fn push(&self, t:T) -> Result<usize,Error>;
}

macro_rules! def_stackvec {
    ( $name:ident, $strname:ident, $size:expr, $init:expr ) => {
        pub struct $name<T> where T:PartialEq {
            buf: UnsafeCell<[Option<T>; $size]>,  // I'm using Option mainly for efficient drops.  Also enables me to hardcode the initial values.
            length: Cell<usize>,
        }
        impl<T> $name<T> where T:PartialEq {
            pub fn new() -> Self {
                Self{ buf: UnsafeCell::new($init),
                      length: Cell::new(0) }
            }
        }
        impl<T> StackVec<T> for $name<T> where T:PartialEq {
            fn cap() -> usize { $size }
            fn len(&self) -> usize { self.length.get() }
            fn push(&self, t:T) -> Result<usize,Error> {
                let i = self.len();
                if i>=Self::cap() { return Err(Error::new("out-of-bounds")); }
                unsafe { ( &mut *self.buf.get() )[i] = Some(t); }
                self.length.set(i+1);
                Ok(i)
            }
            fn get(&self, i:usize) -> &T {
                if i>=self.len() { panic!("out-of-bounds"); }
                unsafe { (& *self.buf.get())[i].as_ref().unwrap() }
            }
        }
        impl<T> fmt::Debug for $name<T> where T:PartialEq {
            fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
                Ok(())
            }
        }
        impl<T> PartialEq for $name<T> where T:PartialEq {
            fn eq(&self, other:&Self) -> bool {
                if self.len()!=other.len() { return false }
                // ...TODO...
                true
            }
        }

        pub struct $strname($name<u8>);
        impl $strname {
            fn new() -> Self { Self($name::new()) }
        }
        impl StackString for $strname {
            fn cap() -> usize { $name::cap() }
            fn len(&self) -> usize { self.0.len() }
            fn push(&self, b:u8) -> Result<usize,Error> { self.0.push(b) }
        }
    }
}


// I'm using this very-verbose array-of-Nones because Rust can't do loops in declarative macros, and also because 'Default' is not implemented for arrays with len>32.
def_stackvec!(   StackVec2,    StackString2,    2, [None,None,]);
def_stackvec!(   StackVec4,    StackString4,    4, [None,None,None,None,]);
def_stackvec!(   StackVec8,    StackString8,    8, [None,None,None,None,None,None,None,None,]);
def_stackvec!(  StackVec16,   StackString16,   16, [None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,]);
def_stackvec!(  StackVec32,   StackString32,   32, [None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,]);
def_stackvec!(  StackVec64,   StackString64,   64, [None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,]);
def_stackvec!( StackVec128,  StackString128,  128, [None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,]);
def_stackvec!( StackVec256,  StackString256,  256, [None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,]);
def_stackvec!( StackVec512,  StackString512,  512, [None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,]);
def_stackvec!(StackVec1024, StackString1024, 1024, [None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,]);



//---- Tests:

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn aaa_vec1() {
        let vec = StackVec4::<i32>::new();
        let ai = vec.push(0).unwrap();
        let bi = vec.push(1).unwrap();
        eprintln!("{} {}",vec.get(ai),vec.get(bi));
        vec.push(2).unwrap();
        vec.push(3).unwrap();

        assert_eq!(vec.push(4), Err(Error::new("out-of-bounds")));
    }


    #[derive(PartialEq)]
    struct Dropper(i32);
    impl Drop for Dropper {
        fn drop(&mut self) {
            eprintln!("in Dropper.drop: {}", self.0);
        }
    }
    impl Default for Dropper {
        fn default() -> Self { Self(0) }
    }

    #[test]
    fn aaa_vec2() {
        let vec = StackVec4::<Dropper>::new();
        assert_eq!(vec.len(),0);
        vec.push(Dropper(1)).unwrap();
        vec.push(Dropper(2)).unwrap();
        vec.push(Dropper(3)).unwrap();
    }


    // Just an experiment, to see how 'drop' works when overwriting values,
    // and also to verify that we really are mutating the memory we expect:
    impl<T> StackVec4<T> where T:PartialEq {
        fn set(&self, i:usize, t:T) {
            unsafe { ( &mut *self.buf.get() )[i] = Some(t); }
        }
    }
    impl<T> Drop for StackVec4<T> where T:PartialEq {
        fn drop(&mut self) {
            eprintln!("in stackvec drop");
        }
    }

    #[test]
    fn aaa_vec3() {
        let vec = StackVec4::<Dropper>::new();
        assert_eq!(vec.len(),0);

        let i0 = vec.push(Dropper(1)).unwrap();
        assert_eq!(i0,0);
        let ref0 = vec.get(i0);
        assert_eq!(ref0.0,1);

        vec.push(Dropper(2)).unwrap();

        vec.set(0, Dropper(-1));
        assert_eq!(ref0.0,-1);

        vec.set(0, Dropper(-11));
        assert_eq!(ref0.0,-11);

        vec.set(3, Dropper(-3));

        vec.push(Dropper(3)).unwrap();
        vec.push(Dropper(4)).unwrap();
    }

    #[test]
    fn aaa_optionlayout() {
        eprintln!("i32 size: {},  Option<i32> size: {}", size_of::<i32>(), size_of::<Option<i32>>());
    }

}

