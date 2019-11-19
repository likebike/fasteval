use algebra::EZ;

use kerr::KErr;

#[test]
fn ez() {
    let mut ez = EZ::new();
    assert_eq!(ez.eval("3+3-3/3"), Ok(5.0));
    assert_eq!(ez.eval("3abc+3-3/3"), Err(KErr::new("unparsed tokens remaining")));
}

