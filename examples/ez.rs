// usage:  cargo run --release --example ez

// In case you didn't know, Rust allows `main()` to return a `Result`.
// This lets us use the `?` operator inside of `main()`.  Very convenient!
fn main() -> Result<(), fasteval::Error> {
    // This example doesn't use any variables, so just use an EmptyNamespace:
    let mut ns = fasteval::EmptyNamespace;

    let val = fasteval::ez_eval(
        "1+2*3/4^5%6 + log(100K) + log(e(),100) + [3*(3-3)/3] + (2<3) && 1.23",    &mut ns)?;
    //    |            |      |    |   |          |               |   |
    //    |            |      |    |   |          |               |   boolean logic with short-circuit support
    //    |            |      |    |   |          |               comparisons
    //    |            |      |    |   |          square-brackets act like parenthesis
    //    |            |      |    |   built-in constants: e(), pi()
    //    |            |      |    'log' can take an optional first 'base' argument, defaults to 10
    //    |            |      numeric literal with suffix: n, µ, m, K, M, G, T
    //    |            many built-in functions: print, int, ceil, floor, abs, sign, log, round, min, max, sin, asin, ...
    //    standard binary operators

    assert_eq!(val, 1.23);

    Ok(())
}
