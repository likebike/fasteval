// ---- Results (2019-11-20 on a 2012 i7 laptop) ----
// al:
//     test eval_only           ... bench:    253 ns/iter (+/- 89)
//     test eval_only_100x      ... bench: 25,378 ns/iter (+/- 5,003)
//     test ez                  ... bench:    976 ns/iter (+/- 106)
//     test native_100x         ... bench:     48 ns/iter (+/- 5)
//     test parse_and_eval      ... bench:    800 ns/iter (+/- 105)
//     test parse_and_eval_100x ... bench: 79,619 ns/iter (+/- 12,509)
//
// caldyn:







#![feature(test)]
extern crate test;  // 'extern crate' seems to be required for this scenario: https://github.com/rust-lang/rust/issues/57288
use test::{Bencher, black_box};

use al::{Slab, Parser, EvalNS, Evaler, ez_eval};

#[bench]
fn ez(b:&mut Bencher) {
    b.iter(|| {
        black_box(ez_eval("(3 * (3 + 3) / 3)").unwrap());
    });
}

#[bench]
fn parse_and_eval(b:&mut Bencher) {
    let p = Parser::new(None,None);
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(|_| None);

    b.iter(|| {
        black_box(p.parse({slab.clear(); &mut slab}, "(3 * (3 + 3) / 3)").unwrap().from(&slab).eval(&slab, &mut ns).unwrap());
    });
}

// Let's see how much the benchmark system is affected by its self:
#[bench]
fn parse_and_eval_100x(b:&mut Bencher) {
    let p = Parser::new(None,None);
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(|_| None);

    b.iter(|| {
        for _ in 0..100 {
            black_box(p.parse({slab.clear(); &mut slab}, "(3 * (3 + 3) / 3)").unwrap().from(&slab).eval(&slab, &mut ns).unwrap());
        }
    });
}

#[bench]
fn eval_only(b:&mut Bencher) {
    let p = Parser::new(None,None);
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(|_| None);
    let expr_ref = p.parse(&mut slab, "(3 * (3 + 3) / 3)").unwrap().from(&slab);

    b.iter(|| {
        black_box(expr_ref.eval(&slab, &mut ns).unwrap());
    });
}

#[bench]
fn eval_only_100x(b:&mut Bencher) {
    let p = Parser::new(None,None);
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(|_| None);
    let expr_ref = p.parse(&mut slab, "(3 * (3 + 3) / 3)").unwrap().from(&slab);

    b.iter(|| {
        for _ in 0..100 {
            black_box(expr_ref.eval(&slab, &mut ns).unwrap());
        }
    });
}

#[bench]
fn native_100x(b:&mut Bencher) {
    b.iter(|| {
        for _ in 0..100 {
            black_box(3.0 * (3.0 + 3.0) / 3.0);
        }
    });
}

