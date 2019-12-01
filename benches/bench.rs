// ---- Results (2019-11-26 on a 2012 i7 laptop) ----
// al:
//     "(3 * (3 + 3) / 3)"
//     test ez                             ... bench:         870 ns/iter (+/- 192)
//     test native_1000x                   ... bench:         345 ns/iter (+/- 28)
//     test parse_compile_eval             ... bench:         917 ns/iter (+/- 178)
//     test parse_eval                     ... bench:         682 ns/iter (+/- 29)
//     test parse_eval_1000x               ... bench:     691,054 ns/iter (+/- 82,412)
//     test preparse_eval                  ... bench:         262 ns/iter (+/- 39)
//     test preparse_eval_1000x            ... bench:     282,084 ns/iter (+/- 23,198)
//     test preparse_precompile_eval       ... bench:           0 ns/iter (+/- 0)
//     test preparse_precompile_eval_1000x ... bench:         767 ns/iter (+/- 151)
//
//     "3 * 3 - 3 / 3"
//     test ez                             ... bench:         591 ns/iter (+/- 120)
//     test native_1000x                   ... bench:         345 ns/iter (+/- 54)
//     test parse_compile_eval             ... bench:         685 ns/iter (+/- 215)
//     test parse_eval                     ... bench:         417 ns/iter (+/- 73)
//     test parse_eval_1000x               ... bench:     411,828 ns/iter (+/- 25,187)
//     test preparse_eval                  ... bench:         130 ns/iter (+/- 29)
//     test preparse_eval_1000x            ... bench:     130,887 ns/iter (+/- 28,369)
//     test preparse_precompile_eval       ... bench:           0 ns/iter (+/- 0)
//     test preparse_precompile_eval_1000x ... bench:         786 ns/iter (+/- 224)
//
//     "2 ^ 3 ^ 4"  = 2417851639229258300000000
//     test ez                             ... bench:         611 ns/iter (+/- 33)
//     test native_1000x                   ... bench:         343 ns/iter (+/- 16)
//     test parse_compile_eval             ... bench:         585 ns/iter (+/- 101)
//     test parse_eval                     ... bench:         428 ns/iter (+/- 91)
//     test parse_eval_1000x               ... bench:     433,551 ns/iter (+/- 128,885)
//     test preparse_eval                  ... bench:         231 ns/iter (+/- 59)
//     test preparse_eval_1000x            ... bench:     222,698 ns/iter (+/- 54,141)
//     test preparse_precompile_eval       ... bench:           0 ns/iter (+/- 0)
//     test preparse_precompile_eval_1000x ... bench:         758 ns/iter (+/- 67)
//
//     "x * 2"
//     test ez                             ... bench:         760 ns/iter (+/- 76)
//     test native_1000x                   ... bench:         723 ns/iter (+/- 125)
//     test parse_compile_eval             ... bench:         479 ns/iter (+/- 34)
//     test parse_eval                     ... bench:         325 ns/iter (+/- 146)
//     test parse_eval_1000x               ... bench:     329,780 ns/iter (+/- 67,712)
//     test preparse_eval                  ... bench:         112 ns/iter (+/- 28)
//     test preparse_eval_1000x            ... bench:     108,477 ns/iter (+/- 11,279)
//     test preparse_precompile_eval       ... bench:          38 ns/iter (+/- 5)
//     test preparse_precompile_eval_1000x ... bench:      39,255 ns/iter (+/- 8,353)
//
//     "sin(x)"
//     test ez                             ... bench:         873 ns/iter (+/- 93)
//     test native_1000x                   ... bench:      17,866 ns/iter (+/- 2,230)
//     test parse_compile_eval             ... bench:         475 ns/iter (+/- 39)
//     test parse_eval                     ... bench:         512 ns/iter (+/- 69)
//     test parse_eval_1000x               ... bench:     500,407 ns/iter (+/- 169,872)
//     test preparse_eval                  ... bench:         147 ns/iter (+/- 31)
//     test preparse_eval_1000x            ... bench:     153,511 ns/iter (+/- 33,782)
//     test preparse_precompile_eval       ... bench:          61 ns/iter (+/- 8)
//     test preparse_precompile_eval_1000x ... bench:      62,003 ns/iter (+/- 7,727)
//
//     "(-z + (z^2 - 4*x*y)^0.5) / (2*x)"
//     test ez                             ... bench:       2,859 ns/iter (+/- 333)
//     test native_1000x                   ... bench:       5,248 ns/iter (+/- 241)
//     test parse_compile_eval             ... bench:       3,023 ns/iter (+/- 967)
//     test parse_eval                     ... bench:       2,070 ns/iter (+/- 493)
//     test parse_eval_1000x               ... bench:   2,009,740 ns/iter (+/- 482,957)
//     test preparse_eval                  ... bench:         807 ns/iter (+/- 102)
//     test preparse_eval_1000x            ... bench:     824,232 ns/iter (+/- 98,840)
//     test preparse_precompile_eval       ... bench:         278 ns/iter (+/- 20)
//     test preparse_precompile_eval_1000x ... bench:     276,372 ns/iter (+/- 17,863)
//
//     "((((87))) - 73) + (97 + (((15 / 55 * ((31)) + 35))) + (15 - (9)) - (39 / 26) / 20 / 91 + 27 / (33 * 26 + 28 - (7) / 10 + 66 * 6) + 60 / 35 - ((29) - (69) / 44 / (92)) / (89) + 2 + 87 / 47 * ((2)) * 83 / 98 * 42 / (((67)) * ((97))) / (34 / 89 + 77) - 29 + 70 * (20)) + ((((((92))) + 23 * (98) / (95) + (((99) * (41))) + (5 + 41) + 10) - (36) / (6 + 80 * 52 + (90))))"
//     test ez                             ... bench:      13,493 ns/iter (+/- 3,746)
//     test native_1000x                   ... bench:         333 ns/iter (+/- 84)
//     test parse_compile_eval             ... bench:      16,411 ns/iter (+/- 1,590)
//     test parse_eval                     ... bench:      12,718 ns/iter (+/- 1,127)
//     test parse_eval_1000x               ... bench:  12,933,763 ns/iter (+/- 1,609,815)
//     test preparse_eval                  ... bench:       4,561 ns/iter (+/- 202)
//     test preparse_eval_1000x            ... bench:   4,634,764 ns/iter (+/- 622,744)
//     test preparse_precompile_eval       ... bench:           0 ns/iter (+/- 0)
//     test preparse_precompile_eval_1000x ... bench:         733 ns/iter (+/- 50)
//
//
// python3:
//     "(3 * (3 + 3) / 3)"
//     user@asus:~$ ( echo 'x=[0]'; echo 'for i in range(100000000):'; echo '  x[0]=(3 * (3 + 3) / 3)'; echo 'print(x)')  | time python3
//     7.36user 0.01system 0:07.38elapsed  -->  73.8 ns/op
//
//     "3 * 3 - 3 / 3"
//     user@asus:~$ ( echo 'x=[0]'; echo 'for i in range(100000000):'; echo '  x[0]=3 * 3 - 3 / 3'; echo 'print(x)')  | time python3
//     7.20user 0.00system 0:07.21elapsed  -->  72.1 ns/op
//
//     "2 ^ 3 ^ 4"  = 2417851639229258349412352
//     user@asus:~$ ( echo 'x=[0]'; echo 'for i in range(100000000):'; echo '  x[0]=2**3**4'; echo 'print(x)')  | time python3
//     39.55user 0.00system 0:39.55elapsed  -->  395.5 ns/op
//
//     "x * 2"
//     user@asus:~$ ( echo '_,x,y,z=[0],1,2,3'; echo 'for i in range(100000000):'; echo '  _[0]=x*2'; echo 'print(_)')  | time python3
//     10.14user 0.00system 0:10.14elapsed  -->  101.4 ns/op
//
//     "sin(x)"
//     user@asus:~$ ( echo 'import math'; echo '_,x,y,z=[0],1,2,3'; echo 'for i in range(100000000):'; echo '  _[0]=math.sin(x)'; echo 'print(_)')  | time python3
//     19.67user 0.00system 0:19.70elapsed  -->  197 ns/op
//
//     "(-z + (z^2 - 4*x*y)^0.5) / (2*x)"
//     user@asus:~$ ( echo '_,x,y,z=[0],1,2,3'; echo 'for i in range(100000000):'; echo '  _[0]=(-z + (z**2 - 4*x*y)**0.5) / (2*x)'; echo 'print(_)')  | time python3
//     56.92user 0.00system 0:56.92elapsed  -->  569 ns/op
//
//     "((((87))) - 73) + (97 + (((15 / 55 * ((31)) + 35))) + (15 - (9)) - (39 / 26) / 20 / 91 + 27 / (33 * 26 + 28 - (7) / 10 + 66 * 6) + 60 / 35 - ((29) - (69) / 44 / (92)) / (89) + 2 + 87 / 47 * ((2)) * 83 / 98 * 42 / (((67)) * ((97))) / (34 / 89 + 77) - 29 + 70 * (20)) + ((((((92))) + 23 * (98) / (95) + (((99) * (41))) + (5 + 41) + 10) - (36) / (6 + 80 * 52 + (90))))"
//     user@asus:~$ ( echo '_,x,y,z=[0],1,2,3'; echo 'for i in range(100000000):'; echo '  _[0]=((((87))) - 73) + (97 + (((15 / 55 * ((31)) + 35))) + (15 - (9)) - (39 / 26) / 20 / 91 + 27 / (33 * 26 + 28 - (7) / 10 + 66 * 6) + 60 / 35 - ((29) - (69) / 44 / (92)) / (89) + 2 + 87 / 47 * ((2)) * 83 / 98 * 42 / (((67)) * ((97))) / (34 / 89 + 77) - 29 + 70 * (20)) + ((((((92))) + 23 * (98) / (95) + (((99) * (41))) + (5 + 41) + 10) - (36) / (6 + 80 * 52 + (90))))'; echo 'print(_)')  | time python3
//     7.24user 0.01system 0:07.26elapsed  -->  72.6 ns/op
//
//
// bc:
//     user@asus:~$ echo 'for (i=0; i<1000000; i++) { (3 * (3 + 3) / 3) }' | time bc >/dev/null
//     1.71user 0.32system 0:02.04elapsed  -->  2040 ns/op
//
//     user@asus:~$ echo 'for (i=0; i<1000000; i++) { 3*3-3/3 }' | time bc >/dev/null
//     1.43user 0.22system 0:01.66elapsed  -->  1660 ns/op
//
//     user@asus:~$ echo 'for (i=0; i<1000000; i++) { 2 ^ 3 ^ 4 }' | time bc >/dev/null = 2417851639229258349412352
//     2.33user 0.21system 0:02.55elapsed  -->  2550 ns/op
//
//     user@asus:~$ echo 'x=1; for (i=0; i<1000000; i++) { x * 2 }' | time bc >/dev/null
//     0.74user 0.27system 0:01.01elapsed  -->  1010 ns/op
//
//     user@asus:~$ echo 'x=1; for (i=0; i<1000000; i++) { s(x) }' | time bc -l >/dev/null
//     40.82user 0.40system 0:41.24elapsed  -->  41240 ns/op
//
//     user@asus:~$ echo 'x=1; y=2; z=3; for (i=0; i<1000000; i++) { (-z + sqrt(z^2 - 4*x*y)) / (2*x) }' | time bc >/dev/null
//     1.93user 0.27system 0:02.20elapsed  -->  2200 ns/op
//
//     user@asus:~$ echo 'for (i=0; i<1000000; i++) { ((((87))) - 73) + (97 + (((15 / 55 * ((31)) + 35))) + (15 - (9)) - (39 / 26) / 20 / 91 + 27 / (33 * 26 + 28 - (7) / 10 + 66 * 6) + 60 / 35 - ((29) - (69) / 44 / (92)) / (89) + 2 + 87 / 47 * ((2)) * 83 / 98 * 42 / (((67)) * ((97))) / (34 / 89 + 77) - 29 + 70 * (20)) + ((((((92))) + 23 * (98) / (95) + (((99) * (41))) + (5 + 41) + 10) - (36) / (6 + 80 * 52 + (90)))) }' | time bc >/dev/null
//     10.95user 0.30system 0:11.26elapsed  -->  11260 ns/op
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
//
//     "(-z + (z^2 - 4*x*y)^0.5) / (2*x)" --> -z => 0 - z
//     test ez                             ... bench:       4,440 ns/iter (+/- 618)
//     test preparse_precompile_eval_1000x ... bench:     525,066 ns/iter (+/- 64,388)
//
//     "((((87))) - 73) + (97 + (((15 / 55 * ((31)) + 35))) + (15 - (9)) - (39 / 26) / 20 / 91 + 27 / (33 * 26 + 28 - (7) / 10 + 66 * 6) + 60 / 35 - ((29) - (69) / 44 / (92)) / (89) + 2 + 87 / 47 * ((2)) * 83 / 98 * 42 / (((67)) * ((97))) / (34 / 89 + 77) - 29 + 70 * (20)) + ((((((92))) + 23 * (98) / (95) + (((99) * (41))) + (5 + 41) + 10) - (36) / (6 + 80 * 52 + (90))))"
//     test ez                             ... bench:      24,598 ns/iter (+/- 4,140)
//     test preparse_precompile_eval_1000x ... bench:       4,418 ns/iter (+/- 429)
//
//
// tinyexpr:
//     "(3 * (3 + 3) / 3)"
//     test bench_interp ... bench:       1,171 ns/iter (+/- 120)
//
//     "3 * 3 - 3 / 3"
//     test bench_interp ... bench:         895 ns/iter (+/- 50)
//
//     "2 ^ (3 ^ 4)" = 2417851639229258300000000
//     test bench_interp ... bench:         816 ns/iter (+/- 83)
//
//     "((((87))) - 73) + (97 + (((15 / 55 * ((31)) + 35))) + (15 - (9)) - (39 / 26) / 20 / 91 + 27 / (33 * 26 + 28 - (7) / 10 + 66 * 6) + 60 / 35 - ((29) - (69) / 44 / (92)) / (89) + 2 + 87 / 47 * ((2)) * 83 / 98 * 42 / (((67)) * ((97))) / (34 / 89 + 77) - 29 + 70 * (20)) + ((((((92))) + 23 * (98) / (95) + (((99) * (41))) + (5 + 41) + 10) - (36) / (6 + 80 * 52 + (90))))"
//     test bench_interp ... bench:      38,422 ns/iter (+/- 6,510)
//
//
// calc:
//     "(3 * (3 + 3) / 3)"
//     test eval_1000x ... bench:   1,675,179 ns/iter (+/- 295,930)
//
//     "3 * 3 - 3 / 3"
//     test eval_1000x ... bench:   1,445,273 ns/iter (+/- 210,599)
//
//     "2 ** 3 ** 4" = 2417851639229258349412352
//     test eval_1000x ... bench:   2,275,338 ns/iter (+/- 351,933)
//
//     "x * 2"
//     test eval_1000x ... bench:     792,132 ns/iter (+/- 145,850)
//
//     "sin(x)"
//     N/A
//
//     "(-z + (z^2 - 4*x*y)^0.5) / (2*x)"
//     test eval_1000x ... bench:  26,565,727 ns/iter (+/- 3,870,655)
//
//     "((((87))) - 73) + (97 + (((15 / 55 * ((31)) + 35))) + (15 - (9)) - (39 / 26) / 20 / 91 + 27 / (33 * 26 + 28 - (7) / 10 + 66 * 6) + 60 / 35 - ((29) - (69) / 44 / (92)) / (89) + 2 + 87 / 47 * ((2)) * 83 / 98 * 42 / (((67)) * ((97))) / (34 / 89 + 77) - 29 + 70 * (20)) + ((((((92))) + 23 * (98) / (95) + (((99) * (41))) + (5 + 41) + 10) - (36) / (6 + 80 * 52 + (90))))"
//     test eval_1000x ... bench:  44,810,253 ns/iter (+/- 5,380,532)
//
//
// meval:
//     "(3 * (3 + 3) / 3)"
//     test parse_eval    ... bench:       3,341 ns/iter (+/- 254)
//     test preparse_eval ... bench:       1,482 ns/iter (+/- 121)
//
//     "3 * 3 - 3 / 3"
//     test parse_eval    ... bench:       2,630 ns/iter (+/- 332)
//     test preparse_eval ... bench:       1,564 ns/iter (+/- 187)
//
//     "2 ^ 3 ^ 4"  = 2417851639229258300000000
//     test parse_eval    ... bench:       2,622 ns/iter (+/- 352)
//     test preparse_eval ... bench:       1,683 ns/iter (+/- 319)
//
//     "x * 2"
//     test parse_eval    ... bench:       2,289 ns/iter (+/- 344)
//     test preparse_eval ... bench:       1,484 ns/iter (+/- 80)
//
//     "sin(x)"
//     test parse_eval    ... bench:       2,476 ns/iter (+/- 323)
//     test preparse_eval ... bench:       1,521 ns/iter (+/- 166)
//
//     "(-z + (z^2 - 4*x*y)^0.5) / (2*x)"
//     test parse_eval    ... bench:       5,830 ns/iter (+/- 641)
//     test preparse_eval ... bench:       1,803 ns/iter (+/- 471)
//
//     "((((87))) - 73) + (97 + (((15 / 55 * ((31)) + 35))) + (15 - (9)) - (39 / 26) / 20 / 91 + 27 / (33 * 26 + 28 - (7) / 10 + 66 * 6) + 60 / 35 - ((29) - (69) / 44 / (92)) / (89) + 2 + 87 / 47 * ((2)) * 83 / 98 * 42 / (((67)) * ((97))) / (34 / 89 + 77) - 29 + 70 * (20)) + ((((((92))) + 23 * (98) / (95) + (((99) * (41))) + (5 + 41) + 10) - (36) / (6 + 80 * 52 + (90))))"
//     test parse_eval    ... bench:      25,371 ns/iter (+/- 8,285)
//     test preparse_eval ... bench:       2,642 ns/iter (+/- 163)
//
//
// rsc:
//     "(3 * (3 + 3) / 3)"
//     test ez            ... bench:       1,438 ns/iter (+/- 130)
//     test parse_eval    ... bench:       1,434 ns/iter (+/- 98)
//     test preparse_eval ... bench:          92 ns/iter (+/- 16)
//
//     "3 * 3 - 3 / 3"
//     test ez            ... bench:       1,291 ns/iter (+/- 150)
//     test parse_eval    ... bench:       1,330 ns/iter (+/- 464)
//     test preparse_eval ... bench:         114 ns/iter (+/- 11)
//
//     "2 ^ (3 ^ 4)"  = 2417851639229258300000000
//     test ez            ... bench:       1,283 ns/iter (+/- 141)
//     test parse_eval    ... bench:       1,306 ns/iter (+/- 113)
//     test preparse_eval ... bench:         244 ns/iter (+/- 165)
//
//     "x * 2"
//     test ez            ... N/A
//     test parse_eval    ... bench:       1,962 ns/iter (+/- 150)
//     test preparse_eval ... bench:         117 ns/iter (+/- 26)
//
//     "sin(x)"
//     test ez            ... N/A
//     test parse_eval    ... bench:       2,262 ns/iter (+/- 385)
//     test preparse_eval ... bench:         158 ns/iter (+/- 22)
//
//     "(-z + (z^2 - 4*x*y)^0.5) / (2*x)"
//     test ez            ... N/A
//     test parse_eval    ... bench:       5,808 ns/iter (+/- 499)
//     test preparse_eval ... bench:         370 ns/iter (+/- 103)
//
//     "((((87))) - 73) + (97 + (((15 / 55 * ((31)) + 35))) + (15 - (9)) - (39 / 26) / 20 / 91 + 27 / (33 * 26 + 28 - (7) / 10 + 66 * 6) + 60 / 35 - ((29) - (69) / 44 / (92)) / (89) + 2 + 87 / 47 * ((2)) * 83 / 98 * 42 / (((67)) * ((97))) / (34 / 89 + 77) - 29 + 70 * (20)) + ((((((92))) + 23 * (98) / (95) + (((99) * (41))) + (5 + 41) + 10) - (36) / (6 + 80 * 52 + (90))))"
//     test ez            ... bench:      20,343 ns/iter (+/- 2,515)
//     test parse_eval    ... bench:      24,555 ns/iter (+/- 6,041)
//     test preparse_eval ... bench:       1,491 ns/iter (+/- 146)




#![feature(test)]
extern crate test;  // 'extern crate' seems to be required for this scenario: https://github.com/rust-lang/rust/issues/57288
use test::{Bencher, black_box};

use al::{Parser, Compiler, Evaler, Slab, EvalNS, ez_eval};

use std::collections::HashMap;

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
static EXPR : &'static str = "3 * 3 - 3 / 3";
//static EXPR : &'static str = "2 ^ 3 ^ 4";
//static EXPR : &'static str = "x * 2";
//static EXPR : &'static str = "sin(x)";
//static EXPR : &'static str = "(-z + (z^2 - 4*x*y)^0.5) / (2*x)";
//static EXPR : &'static str = "((((87))) - 73) + (97 + (((15 / 55 * ((31)) + 35))) + (15 - (9)) - (39 / 26) / 20 / 91 + 27 / (33 * 26 + 28 - (7) / 10 + 66 * 6) + 60 / 35 - ((29) - (69) / 44 / (92)) / (89) + 2 + 87 / 47 * ((2)) * 83 / 98 * 42 / (((67)) * ((97))) / (34 / 89 + 77) - 29 + 70 * (20)) + ((((((92))) + 23 * (98) / (95) + (((99) * (41))) + (5 + 41) + 10) - (36) / (6 + 80 * 52 + (90))))";

#[bench]
fn native_1000x(bencher:&mut Bencher) {
    #[allow(dead_code)]
    fn x() -> f64 { black_box(1.0) }
    #[allow(unused_variables)]
    let (a,b,c) = (1.0f64, 3.0f64, 2.0f64);
    bencher.iter(|| {
        for _ in 0..1000 {
            //black_box(3.0 * (3.0 + 3.0) / 3.0);
            //black_box(3.0 * 3.0 - 3.0 / 3.0);
            //black_box(2.0f64.powf(3.0).powf(4.0));
            //black_box(x() * 2.0);
            //black_box(x().sin());
            //black_box( (-b + (b.powf(2.0) - 4.0*a*c).powf(0.5)) / (2.0*a) );
            black_box( ((((87.))) - 73.) + (97. + (((15. / 55. * ((31.)) + 35.))) + (15. - (9.)) - (39. / 26.) / 20. / 91. + 27. / (33. * 26. + 28. - (7.) / 10. + 66. * 6.) + 60. / 35. - ((29.) - (69.) / 44. / (92.)) / (89.) + 2. + 87. / 47. * ((2.)) * 83. / 98. * 42. / (((67.)) * ((97.))) / (34. / 89. + 77.) - 29. + 70. * (20.)) + ((((((92.))) + 23. * (98.) / (95.) + (((99.) * (41.))) + (5. + 41.) + 10.) - (36.) / (6. + 80. * 52. + (90.)))) );
        }
    });
}

#[bench]
fn ez(b:&mut Bencher) {
    let mut vars=HashMap::new();
    vars.insert("x".to_string(),1.0);
    vars.insert("y".to_string(),2.0);
    vars.insert("z".to_string(),3.0);

    b.iter(|| {
        black_box(ez_eval(EXPR, &vars).unwrap());
    });
}

#[bench]
fn parse_eval(b:&mut Bencher) {
    let mut p = Parser::new();
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(evalcb);

    b.iter(|| {
        black_box(p.parse({slab.clear(); &mut slab.ps}, EXPR).unwrap().from(&slab.ps).eval(&slab, &mut ns).unwrap());
    });
}

// Let's see how much the benchmark system is affected by its self:
#[bench]
fn parse_eval_1000x(b:&mut Bencher) {
    let mut p = Parser::new();
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
    let mut p = Parser::new();
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(evalcb);
    let expr_ref = p.parse(&mut slab.ps, EXPR).unwrap().from(&slab.ps);

    b.iter(|| {
        black_box(expr_ref.eval(&slab, &mut ns).unwrap());
    });
}

#[bench]
fn preparse_eval_1000x(b:&mut Bencher) {
    let mut p = Parser::new();
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
    let mut p = Parser::new();
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(evalcb);

    b.iter(|| {
        black_box(p.parse({slab.clear(); &mut slab.ps}, EXPR).unwrap().from(&slab.ps).compile(&slab.ps, &mut slab.cs).eval(&slab, &mut ns).unwrap());
    });
}

#[bench]
fn preparse_precompile_eval(b:&mut Bencher) {
    let mut p = Parser::new();
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
    let mut p = Parser::new();
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
//     let mut p = Parser::new();
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

