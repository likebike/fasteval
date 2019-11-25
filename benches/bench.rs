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

use al::{Parser, Compiler, Evaler, Slab, EvalNS, ez_eval};

#[bench]
fn ez(b:&mut Bencher) {
    b.iter(|| {
        black_box(ez_eval("(3 * (3 + 3) / 3)").unwrap());
    });
}

#[bench]
fn parse_eval(b:&mut Bencher) {
    let p = Parser::new(None,None);
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(|_| None);

    b.iter(|| {
        black_box(p.parse({slab.clear(); &mut slab.ps}, "(3 * (3 + 3) / 3)").unwrap().from(&slab.ps).eval(&slab, &mut ns).unwrap());
    });
}

// Let's see how much the benchmark system is affected by its self:
#[bench]
fn parse_eval_1000x(b:&mut Bencher) {
    let p = Parser::new(None,None);
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(|_| None);

    b.iter(|| {
        for _ in 0..1000 {
            black_box(p.parse({slab.clear(); &mut slab.ps}, "(3 * (3 + 3) / 3)").unwrap().from(&slab.ps).eval(&slab, &mut ns).unwrap());
        }
    });
}

#[bench]
fn preparse_eval(b:&mut Bencher) {
    let p = Parser::new(None,None);
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(|_| None);
    let expr_ref = p.parse(&mut slab.ps, "(3 * (3 + 3) / 3)").unwrap().from(&slab.ps);

    b.iter(|| {
        black_box(expr_ref.eval(&slab, &mut ns).unwrap());
    });
}

#[bench]
fn preparse_eval_1000x(b:&mut Bencher) {
    let p = Parser::new(None,None);
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(|_| None);
    let expr_ref = p.parse(&mut slab.ps, "(3 * (3 + 3) / 3)").unwrap().from(&slab.ps);

    b.iter(|| {
        for _ in 0..1000 {
            black_box(expr_ref.eval(&slab, &mut ns).unwrap());
        }
    });
}

#[bench]
fn parse_compile_eval(b:&mut Bencher) {
    let p = Parser::new(None,None);
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(|_| None);

    b.iter(|| {
        black_box(p.parse({slab.clear(); &mut slab.ps}, "(3 * (3 + 3) / 3)").unwrap().from(&slab.ps).compile(&slab.ps, &mut slab.cs).eval(&slab, &mut ns).unwrap());
    });
}

#[bench]
fn preparse_precompile_eval(b:&mut Bencher) {
    let p = Parser::new(None,None);
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(|_| None);
    let expr_ref = p.parse(&mut slab.ps, "(3 * (3 + 3) / 3)").unwrap().from(&slab.ps);
    let instr = expr_ref.compile(&slab.ps, &mut slab.cs);

    b.iter(|| {
        black_box(if let al::IConst(c) = instr {
                      c
                  } else {
                      instr.eval(&slab, &mut ns).unwrap()
                  });
    });
}

#[bench]
fn preparse_precompile_eval_1000x(b:&mut Bencher) {
    let p = Parser::new(None,None);
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(|_| None);
    let expr_ref = p.parse(&mut slab.ps, "(3 * (3 + 3) / 3)").unwrap().from(&slab.ps);
    let instr = expr_ref.compile(&slab.ps, &mut slab.cs);

    b.iter(|| {
        for _ in 0..1000 {
            black_box(if let al::IConst(c) = instr {
                          c
                      } else {
                          instr.eval(&slab, &mut ns).unwrap()
                      });
        }
    });
}

#[bench]
#[allow(non_snake_case)]
fn preparse_precompile_eval_100B(_:&mut Bencher) {
    let p = Parser::new(None,None);
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(|_| None);
    let expr_ref = p.parse(&mut slab.ps, "(3 * (3 + 3) / 3)").unwrap().from(&slab.ps);
    let instr = expr_ref.compile(&slab.ps, &mut slab.cs);

    let start = std::time::Instant::now();
    for _ in 0..100 {
        for _ in 0..1_000_000_000 {
            black_box(if let al::IConst(c) = instr {
                          c
                      } else {
                          instr.eval(&slab, &mut ns).unwrap()
                      });
        }
    }
    eprintln!("bench time: {}", start.elapsed().as_secs_f64());
}

#[bench]
fn native_1000x(b:&mut Bencher) {
    b.iter(|| {
        for _ in 0..1000 {
            black_box(3.0 * (3.0 + 3.0) / 3.0);
        }
    });
}

