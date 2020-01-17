// usage:  cargo run --release --example slab

use std::collections::BTreeMap;
use fasteval::Evaler;  // use this trait so we can call eval().
fn main() -> Result<(), fasteval::Error> {
    let parser = fasteval::Parser::new();
    let mut slab = fasteval::Slab::new();

    // See the `parse` documentation to understand why we use `from` like this:
    let expr_ref = parser.parse("x + 1", &mut slab.ps)?.from(&slab.ps);

    // Let's evaluate the expression a couple times with different 'x' values:

    let mut map : BTreeMap<String,f64> = BTreeMap::new();
    map.insert("x".to_string(), 1.0);
    let val = expr_ref.eval(&slab, &mut map)?;
    assert_eq!(val, 2.0);

    map.insert("x".to_string(), 2.5);
    let val = expr_ref.eval(&slab, &mut map)?;
    assert_eq!(val, 3.5);

    // Now, let's re-use the Slab for a new expression.
    // (This is much cheaper than allocating a new Slab.)
    // The Slab gets cleared by 'parse()', so you must avoid using
    // the old expr_ref after parsing the new expression.
    // One simple way to avoid this problem is to shadow the old variable:

    let expr_ref = parser.parse("x * 10", &mut slab.ps)?.from(&slab.ps);

    let val = expr_ref.eval(&slab, &mut map)?;
    assert_eq!(val, 25.0);

    Ok(())
}
