
#[cfg(test)]
mod tests {
    use std::cell::{Cell, UnsafeCell};

    ////////////////////////////////////////////////////////////////////////////
    // I'm new to Rust, so here is a tiny experiment where I learn how to store closures in structs:
    struct CBs<F,G> where F:Fn(&str)->f64,
                          G:Fn(&str)->f64 {  // It looks like we will need to use big, redundant 'where' clauses until Trait Aliases are implemented: https://github.com/rust-lang/rust/issues/41517
        f:F,
        g:G,
    }

    impl<F,G> CBs<F,G> where F:Fn(&str)->f64,
                             G:Fn(&str)->f64 {
        fn new(f:F, g:G) -> Self {
            CBs{f,g}
        }
        fn call_f(&self, k:&str) -> f64 { (self.f)(k) }
        fn call_g(&self, k:&str) -> f64 { (self.g)(k) }
    }

    #[test]
    fn aaa_cbs() {
        let cbs = CBs::new(|_k| 1.0, |_k| 2.0);
        assert_eq!(cbs.call_f("abc"), 1.0);
        assert_eq!(cbs.call_g("abc"), 2.0);
    }



    ////////////////////////////////////////////////////////////////////////////
    // Lifetime experiments:
    // #[derive(Debug)]
    // struct A {
    //     v:  i32,
    //     bs: [B ; 3],
    // }

    // #[derive(Debug)]
    // struct B {
    //     v: i32,
    //     a: A,
    // }

    // fn livevil_f1() -> A {
    //     let a0 = A{v:0, bs:&[]};
    //     let b0 = B{v:0, a:a0};

    //     let a1 = A{v:1, bs:&[]};
    //     let b1 = B{v:1, a:a1};

    //     let a2 = A{v:2, bs:&[]};
    //     let b2 = B{v:2, a:a2};

    //     let bs = [b0, b1, b2];
    //     A{v:1, bs:bs}
    // }

    // #[test]
    // fn aaa_livevil() {
    //     let a = livevil_f1();
    //     println!("{:?}",a);
    // }



    ////////////////////////////////////////////////////////////////////////////
    // Stack Slab-allocation experiments:
    const STACKSLAB_SIZE : usize = 8;
    struct StackSlab8<T> {
        buf: UnsafeCell<[T; STACKSLAB_SIZE]>,
        len: Cell<usize>,
    }
    impl<T> StackSlab8<T> where T:Default {
        fn new() -> Self {
            Self{
                buf: UnsafeCell::new(Default::default()),
                len: Cell::new(0),
            }
        }
    }
    impl<T> StackSlab8<T> {
        //// Not possible: (https://doc.rust-lang.org/nomicon/lifetime-mismatch.html)
        //fn push(&mut self, t:T) -> &T {
        //    if self.len>=STACKSLAB_SIZE { panic!("out-of-bounds"); }
        //    let i = self.len;
        //    self.buf[i]=t;
        //    self.len+=1;
        //    self.get(i)
        //}
        fn push(&self, t:T) {
            let i = self.len.get();
            if i>=STACKSLAB_SIZE { panic!("out-of-bounds"); }
            unsafe { ( &mut *self.buf.get() )[i] = t; }
            self.len.set(i+1);
        }
        fn get(&self, i:usize) -> &T {
            if i>=self.len.get() { panic!("out-of-bounds"); }
            unsafe { return &(& *self.buf.get())[i]; }
        }
        fn last(&self) -> &T { self.get(self.len.get()-1) }
    }

    #[test]
    fn aaa_slab1() {
        let slab = StackSlab8::<i32>::new();
        slab.push(0);
        let a1 = slab.last();
        let a2 = slab.last();
        slab.push(1);
        let b1 = slab.last();
        let b2 = slab.last();
        eprintln!("{} {} {} {}",a1,a2,b1,b2);
        slab.push(2);
        slab.push(3);
        slab.push(4);
        slab.push(5);
        slab.push(6);
        slab.push(7);

        //slab.push(8);
    }


    struct StackSlab8b<T> {
        buf: [T; STACKSLAB_SIZE],
        len: usize,
    }
    impl<T> StackSlab8b<T> where T:Default {
        fn new() -> Self {
            Self{
                buf: Default::default(),
                len: 0,
            }
        }
    }
    impl<T> StackSlab8b<T> {
        fn push(&mut self, t:T) {
            if self.len>=STACKSLAB_SIZE { panic!("out-of-bounds"); }
            self.buf[self.len] = t;
            self.len += 1;
        }
        fn get(&self, i:usize) -> &T {
            if i>=self.len { panic!("out-of-bounds"); }
            &self.buf[i]
        }
        fn last(&self) -> &T { self.get(self.len-1) }
    }

    #[test]
    fn aaa_slab2() {
        let mut slab = StackSlab8b::<i32>::new();
        slab.push(0);
        let a1 = slab.last();
        let a2 = slab.last();
        //slab.push(1);       // Not allowed.  The compiler can't track disjoint-borrows/borrow-splitting of arrays.
        let b1 = slab.last();
        let b2 = slab.last();
        eprintln!("{} {} {} {}",a1,a2,b1,b2);
    }








    #[test]
    fn aaa_nested_method_calls() {
        let mut v : Vec<usize> = Vec::new();
        v.push(v.len());  // didn't work before Non-Lexical-Lifetimes.
    }
}

