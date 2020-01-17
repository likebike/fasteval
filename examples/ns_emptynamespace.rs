// usage:  cargo run --release --example ns_emptynamespace

fn main() -> Result<(), fasteval::Error> {
    let mut ns = fasteval::EmptyNamespace;

    let val = fasteval::ez_eval("sin(pi()/2)", &mut ns)?;
    assert_eq!(val, 1.0);

    Ok(())
}
