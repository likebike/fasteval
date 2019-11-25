// ---- Results (2019-11-26 on a 2012 i7 laptop) ----
// al:
//     "(3 * (3 + 3) / 3)"
//     test ez                             ... bench:     994 ns/iter (+/- 91)
//     test native_1000x                   ... bench:     338 ns/iter (+/- 17)
//     test parse_compile_eval             ... bench:   1,079 ns/iter (+/- 80)
//     test parse_eval                     ... bench:     883 ns/iter (+/- 99)
//     test parse_eval_1000x               ... bench: 821,838 ns/iter (+/- 36,518)
//     test preparse_eval                  ... bench:     250 ns/iter (+/- 11)
//     test preparse_eval_1000x            ... bench: 250,766 ns/iter (+/- 13,942)
//     test preparse_precompile_eval       ... bench:       0 ns/iter (+/- 0)
//     test preparse_precompile_eval_1000x ... bench:     764 ns/iter (+/- 63)
//
//     "3 * 3 - 3 / 3"
//     test ez                             ... bench:     712 ns/iter (+/- 131)
//     test native_1000x                   ... bench:     337 ns/iter (+/- 25)
//     test parse_compile_eval             ... bench:     802 ns/iter (+/- 46)
//     test parse_eval                     ... bench:     544 ns/iter (+/- 50)
//     test parse_eval_1000x               ... bench: 521,086 ns/iter (+/- 69,003)
//     test preparse_eval                  ... bench:     121 ns/iter (+/- 23)
//     test preparse_eval_1000x            ... bench: 122,997 ns/iter (+/- 11,658)
//     test preparse_precompile_eval       ... bench:       0 ns/iter (+/- 0)
//     test preparse_precompile_eval_1000x ... bench:     745 ns/iter (+/- 46)
//
//     "2 ^ 3 ^ 4"  = 4096
//     test ez                             ... bench:     746 ns/iter (+/- 92)
//     test native_1000x                   ... bench:     343 ns/iter (+/- 21)
//     test parse_compile_eval             ... bench:     596 ns/iter (+/- 60)
//     test parse_eval                     ... bench:     558 ns/iter (+/- 58)
//     test parse_eval_1000x               ... bench: 533,614 ns/iter (+/- 45,338)
//     test preparse_eval                  ... bench:     234 ns/iter (+/- 18)
//     test preparse_eval_1000x            ... bench: 242,505 ns/iter (+/- 29,837)
//     test preparse_precompile_eval       ... bench:       0 ns/iter (+/- 0)
//     test preparse_precompile_eval_1000x ... bench:     748 ns/iter (+/- 40)
//
//     "x * 2"
//     test ez                             ... N/A
//     test native_1000x                   ... bench:     726 ns/iter (+/- 47)
//     test parse_compile_eval             ... bench:     610 ns/iter (+/- 98)
//     test parse_eval                     ... bench:     467 ns/iter (+/- 33)
//     test parse_eval_1000x               ... bench: 465,864 ns/iter (+/- 66,033)
//     test preparse_eval                  ... bench:     113 ns/iter (+/- 9)
//     test preparse_eval_1000x            ... bench: 113,535 ns/iter (+/- 8,249)
//     test preparse_precompile_eval       ... bench:      39 ns/iter (+/- 3)
//     test preparse_precompile_eval_1000x ... bench:  39,024 ns/iter (+/- 4,490)
//
//     "sin(x)"
//     test ez                             ... N/A
//     test native_1000x                   ... bench:  17,917 ns/iter (+/- 1,873)
//     test parse_compile_eval             ... bench:     659 ns/iter (+/- 68)
//     test parse_eval                     ... bench:     657 ns/iter (+/- 27)
//     test parse_eval_1000x               ... bench: 672,694 ns/iter (+/- 55,975)
//     test preparse_eval                  ... bench:     152 ns/iter (+/- 13)
//     test preparse_eval_1000x            ... bench: 155,374 ns/iter (+/- 12,574)
//     test preparse_precompile_eval       ... bench:      61 ns/iter (+/- 6)
//     test preparse_precompile_eval_1000x ... bench:  61,859 ns/iter (+/- 15,816)
//
//
// caldyn:
//     "(3 * (3 + 3) / 3)", No Context
//     test ez                             ... bench:       1,191 ns/iter (+/- 315)
//     test preparse_precompile_eval_1000x ... bench:       4,193 ns/iter (+/- 217)
//
//     "(3 * (3 + 3) / 3)", Normal Context
//     test ez                             ... bench:       1,298 ns/iter (+/- 70)
//     test preparse_precompile_eval_1000x ... bench:       4,273 ns/iter (+/- 233)
//
//     "(3 * (3 + 3) / 3)", Callback Context
//     test ez                             ... bench:       1,286 ns/iter (+/- 158)
//     test preparse_precompile_eval_1000x ... bench:       4,223 ns/iter (+/- 236)
//
//     "3 * 3 - 3 / 3", Callback Context
//     test ez                             ... bench:       1,070 ns/iter (+/- 80)
//     test preparse_precompile_eval_1000x ... bench:       4,245 ns/iter (+/- 190)
//
//     "2 ^ 3 ^ 4", = 2417851639229258300000000.0, Callback Context
//     test ez                             ... bench:         867 ns/iter (+/- 75)
//     test preparse_precompile_eval_1000x ... bench:       4,182 ns/iter (+/- 238)
//
//     "x * 2", Callback Context
//     test ez                             ... bench:         607 ns/iter (+/- 61)
//     test preparse_precompile_eval_1000x ... bench:      77,540 ns/iter (+/- 12,490)
//
//     "sin(x)", Callback Context
//     test ez                             ... bench:         573 ns/iter (+/- 54)
//     test preparse_precompile_eval_1000x ... bench:      97,861 ns/iter (+/- 6,063)








#![feature(test)]
extern crate test;  // 'extern crate' seems to be required for this scenario: https://github.com/rust-lang/rust/issues/57288
use test::{Bencher, black_box};

use al::{Parser, Compiler, Evaler, Slab, EvalNS, ez_eval};

//fn evalcb(_:&str) -> Option<f64> { None }
fn evalcb(name:&str) -> Option<f64> {
    match name {
        "x" => Some(1.0),
        "y" => Some(2.0),
        "z" => Some(3.0),
        _ => None,
    }
}

//static EXPR : &'static str = "(3 * (3 + 3) / 3)";
//static EXPR : &'static str = "3 * 3 - 3 / 3";
//static EXPR : &'static str = "2 ^ 3 ^ 4";
//static EXPR : &'static str = "x * 2";
static EXPR : &'static str = "sin(x)";

#[bench]
fn ez(b:&mut Bencher) {
    b.iter(|| {
        black_box(ez_eval(EXPR).unwrap());
    });
}

#[bench]
fn parse_eval(b:&mut Bencher) {
    let p = Parser::new(None,None);
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(evalcb);

    b.iter(|| {
        black_box(p.parse({slab.clear(); &mut slab.ps}, EXPR).unwrap().from(&slab.ps).eval(&slab, &mut ns).unwrap());
    });
}

// Let's see how much the benchmark system is affected by its self:
#[bench]
fn parse_eval_1000x(b:&mut Bencher) {
    let p = Parser::new(None,None);
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(evalcb);

    b.iter(|| {
        for _ in 0..1000 {
            black_box(p.parse({slab.clear(); &mut slab.ps}, EXPR).unwrap().from(&slab.ps).eval(&slab, &mut ns).unwrap());
        }
    });
}

#[bench]
fn preparse_eval(b:&mut Bencher) {
    let p = Parser::new(None,None);
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(evalcb);
    let expr_ref = p.parse(&mut slab.ps, EXPR).unwrap().from(&slab.ps);

    b.iter(|| {
        black_box(expr_ref.eval(&slab, &mut ns).unwrap());
    });
}

#[bench]
fn preparse_eval_1000x(b:&mut Bencher) {
    let p = Parser::new(None,None);
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(evalcb);
    let expr_ref = p.parse(&mut slab.ps, EXPR).unwrap().from(&slab.ps);

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
    let mut ns = EvalNS::new(evalcb);

    b.iter(|| {
        black_box(p.parse({slab.clear(); &mut slab.ps}, EXPR).unwrap().from(&slab.ps).compile(&slab.ps, &mut slab.cs).eval(&slab, &mut ns).unwrap());
    });
}

#[bench]
fn preparse_precompile_eval(b:&mut Bencher) {
    let p = Parser::new(None,None);
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(evalcb);
    let expr_ref = p.parse(&mut slab.ps, EXPR).unwrap().from(&slab.ps);
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
    let mut ns = EvalNS::new(evalcb);
    let expr_ref = p.parse(&mut slab.ps, EXPR).unwrap().from(&slab.ps);
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

// #[bench]
// #[allow(non_snake_case)]
// fn preparse_precompile_eval_100B(_:&mut Bencher) {
//     let p = Parser::new(None,None);
//     let mut slab = Slab::new();
//     let mut ns = EvalNS::new(evalcb);
//     let expr_ref = p.parse(&mut slab.ps, EXPR).unwrap().from(&slab.ps);
//     let instr = expr_ref.compile(&slab.ps, &mut slab.cs);
// 
//     let start = std::time::Instant::now();
//     for _ in 0..100 {
//         for _ in 0..1_000_000_000 {
//             black_box(if let al::IConst(c) = instr {
//                           c
//                       } else {
//                           instr.eval(&slab, &mut ns).unwrap()
//                       });
//         }
//     }
//     eprintln!("bench time: {}", start.elapsed().as_secs_f64());
// }

#[bench]
fn native_1000x(b:&mut Bencher) {
    fn x() -> f64 { black_box(1.0) }
    b.iter(|| {
        for _ in 0..1000 {
            //black_box(3.0 * (3.0 + 3.0) / 3.0);
            //black_box(3.0 * 3.0 - 3.0 / 3.0);
            //black_box(2.0f64.powf(3.0).powf(4.0));
            //black_box(x() * 2.0);
            black_box(x().sin());
        }
    });
}

