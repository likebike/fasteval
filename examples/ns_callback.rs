// usage:  cargo run --release --example ns_callback

fn main() -> Result<(), fasteval::Error> {
    let mut num_lookups = 0;
    let mut cb = |name:&str, _args:Vec<f64>| -> Option<f64> {
        num_lookups += 1;
        match name {
            "x" => Some(2.0),
            _ => None,
        }
    };

    let val = fasteval::ez_eval("x * (x + 1)", &mut cb)?;
    assert_eq!(val, 6.0);
    assert_eq!(num_lookups, 2);  // Notice that 'x' was looked-up twice.

    Ok(())
}
