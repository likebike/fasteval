// usage:  cargo run --release --features unsafe-vars --example unsafe-vars

fn main() -> Result<(), fasteval::Error> {
    #[cfg(not(feature="unsafe-vars"))]
    {
        panic!("You must enable the `unsafe-vars` feature to run this example:  cargo run --release --features unsafe-vars --example unsafe-vars");
    }

    // Allow compilation even when the `unsafe-vars` feature is not enabled.
    // This is important so that `cargo test` can succeed.
    #[cfg(feature="unsafe-vars")]
    {
        use fasteval::Evaler;    // use this trait so we can call eval().
        use fasteval::Compiler;  // use this trait so we can call compile().

        let parser = fasteval::Parser::new();
        let mut slab = fasteval::Slab::new();

        // The Unsafe Variable will use a pointer to read this memory location:
        // You must make sure that this variable stays in-scope as long as the
        // expression is in-use.
        let mut deg : f64 = 0.0;

        // Unsafe Variables must be registered before 'parse()'.
        // (Normal Variables only need definitions during the 'eval' phase.)
        unsafe { slab.ps.add_unsafe_var("deg".to_string(), &deg); } // `add_unsafe_var()` only exists if the `unsafe-vars` feature is enabled: `cargo test --features unsafe-vars`

        let expr_str = "sin(deg/360 * 2*pi())";
        let compiled = parser.parse(expr_str, &mut slab.ps)?.from(&slab.ps).compile(&slab.ps, &mut slab.cs);

        let mut ns = fasteval::EmptyNamespace;  // We only define unsafe variables, not normal variables,
                                                // so EmptyNamespace is fine.

        for d in 0..360 {
            deg = d as f64;
            let val = fasteval::eval_compiled!(compiled, &slab, &mut ns);
            eprintln!("sin({}°) = {}", deg, val);
        }

        Ok(())
    }
}
