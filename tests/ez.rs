use al::ez_eval;

use kerr::KErr;

#[test]
fn ez() {
    assert_eq!(ez_eval("3+3-3/3"), Ok(5.0));
    assert_eq!(ez_eval("3abc+3-3/3"), Err(KErr::new("unparsed tokens remaining")));
}

