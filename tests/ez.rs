use algebra::EZ;

#[test]
fn ez() {
    let mut ez = EZ::new();
    assert_eq!(ez.eval("3+3-3/3"), Ok(5.0));
}

