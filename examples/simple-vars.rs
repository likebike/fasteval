// usage:  cargo run --release --example simple-vars

use std::collections::BTreeMap;
fn main() -> Result<(), fasteval::Error> {
    let mut map : BTreeMap<String,f64> = BTreeMap::new();
    map.insert("x".to_string(), 1.0);
    map.insert("y".to_string(), 2.0);
    map.insert("z".to_string(), 3.0);

    let val = fasteval::ez_eval(r#"x + print("y:",y) + z"#,    &mut map)?;
    //                                 |
    //                                 prints "y: 2" to stderr and then evaluates to 2.0

    assert_eq!(val, 6.0);

    Ok(())
}
