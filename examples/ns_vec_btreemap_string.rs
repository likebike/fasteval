// usage:  cargo run --release --example ns_vec_btreemap_string

use std::collections::BTreeMap;
fn main() -> Result<(), fasteval::Error> {
    let mut layer1 = BTreeMap::new();
    layer1.insert("x".to_string(), 2.0);
    layer1.insert("y".to_string(), 3.0);

    let mut layers : Vec<BTreeMap<String,f64>> = vec![layer1];

    let val = fasteval::ez_eval("x * y", &mut layers)?;
    assert_eq!(val, 6.0);

    // Let's add another layer which shadows the previous one:
    let mut layer2 = BTreeMap::new();
    layer2.insert("x".to_string(), 3.0);
    layers.push(layer2);

    let val = fasteval::ez_eval("x * y", &mut layers)?;
    assert_eq!(val, 9.0);

    // Remove the top layer and we'll be back to what we had before:
    layers.pop();

    let val = fasteval::ez_eval("x * y", &mut layers)?;
    assert_eq!(val, 6.0);

    Ok(())
}
