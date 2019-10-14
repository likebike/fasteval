
#[cfg(test)]
mod tests {

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
    fn cbs() {
        let cbs = CBs::new(|_k| 1.0, |_k| 2.0);
        assert_eq!(cbs.call_f("abc"), 1.0);
        assert_eq!(cbs.call_g("abc"), 2.0);
    }
}

