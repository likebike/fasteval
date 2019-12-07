// Here is how I run benchmarks:
//     cargo bench &>/dev/null; while true; do echo "time: $(date +%s)"; cargo bench; done >bench.out
//     cargo bench --features unsafe-vars &>/dev/null; while true; do echo "time: $(date +%s)"; cargo bench --features unsafe-vars; done >bench.out
//     cat bench.out | awk -v "now=$(date +%s)" '$1=="time:"{when=$2}  $3=="..." && $4=="bench:" {gsub(/,/, "", $5); v=$5+0; if (t[$2]=="" || v<t[$2]){t[$2]=v; w[$2]=when;}} END{for (k in t) { printf "%-40s %9d ns/iter    %5ds ago\n",k,t[k],now-w[k] }}' | sort



// ---- Results (2019-12-04 on a 2012 laptop with Intel(R) Core(TM) i7-3610QM CPU @ 2.30GHz) ----
// al:
//     "(3 * (3 + 3) / 3)"
//     Normal:
//     ez                                             787 ns/iter
//     native_1000x                                   336 ns/iter
//     parse_compile_eval                             883 ns/iter
//     parse_eval_1000x                            633231 ns/iter
//     parse_eval_1x                                  636 ns/iter
//     parse_nsbubble_eval                            648 ns/iter
//     preparse_eval_1000x                         209757 ns/iter
//     preparse_eval                                  210 ns/iter
//     preparse_precompile_eval                         0 ns/iter
//     preparse_precompile_eval_1000x                 746 ns/iter
//     preparse_precompile_nsbubble_eval                0 ns/iter
//     --features unsafe-vars:
//     ez                                             811 ns/iter
//     native_1000x                                   339 ns/iter
//     parse_compile_eval                             883 ns/iter
//     parse_eval_1000x                            648133 ns/iter
//     parse_eval_1x                                  636 ns/iter
//     parse_nsbubble_eval                            656 ns/iter
//     preparse_eval_1000x                         203559 ns/iter
//     preparse_eval                                  206 ns/iter
//     preparse_precompile_eval_1000x                 982 ns/iter
//     preparse_precompile_eval                         1 ns/iter
//     preparse_precompile_eval_unsafe_1000x          987 ns/iter
//     preparse_precompile_nsbubble_eval                1 ns/iter
//
//     "3 * 3 - 3 / 3"
//     Normal:

//     ez                                        586 ns/iter
//     native_1000x                              333 ns/iter
//     parse_compile_eval                        651 ns/iter
//     parse_eval_1000x                       403900 ns/iter
//     parse_eval                                402 ns/iter
//     parse_nsbubble_eval                       416 ns/iter
//     preparse_eval_1000x                    107222 ns/iter
//     preparse_eval                             107 ns/iter
//     preparse_precompile_eval                    0 ns/iter
//     preparse_precompile_eval_1000x            753 ns/iter
//     preparse_precompile_nsbubble_eval           0 ns/iter
//     --features unsafe-vars:
//     ez                                        584 ns/iter
//     native_1000x                              335 ns/iter
//     parse_compile_eval                        692 ns/iter
//     parse_eval_1000x                       405304 ns/iter
//     parse_eval                                403 ns/iter
//     parse_nsbubble_eval                       410 ns/iter
//     preparse_eval_1000x                    109489 ns/iter
//     preparse_eval                             109 ns/iter
//     preparse_precompile_eval_1000x            967 ns/iter
//     preparse_precompile_eval                    1 ns/iter
//     preparse_precompile_eval_unsafe_1000x     968 ns/iter
//     preparse_precompile_nsbubble_eval           1 ns/iter
//
//     "2 ^ 3 ^ 4"  = 2417851639229258300000000
//     Normal:
//     ez                                             603 ns/iter
//     native_1000x                                   336 ns/iter
//     parse_compile_eval                             573 ns/iter
//     parse_eval_1000x                            417272 ns/iter
//     parse_eval                                     421 ns/iter
//     parse_nsbubble_eval                            433 ns/iter
//     preparse_eval_1000x                         207101 ns/iter
//     preparse_eval                                  203 ns/iter
//     preparse_precompile_eval                         0 ns/iter
//     preparse_precompile_eval_1000x                 758 ns/iter
//     preparse_precompile_nsbubble_eval                0 ns/iter
//     --features unsafe-vars:
//     ez                                             603 ns/iter
//     native_1000x                                   335 ns/iter
//     parse_compile_eval                             555 ns/iter
//     parse_eval_1000x                            420423 ns/iter
//     parse_eval                                     415 ns/iter
//     parse_nsbubble_eval                            430 ns/iter
//     preparse_eval_1000x                         200798 ns/iter
//     preparse_eval                                  201 ns/iter
//     preparse_precompile_eval_1000x                 968 ns/iter
//     preparse_precompile_eval                         1 ns/iter
//     preparse_precompile_eval_unsafe_1000x          961 ns/iter
//     preparse_precompile_nsbubble_eval                1 ns/iter
//
//     "x * 2"
//     Normal:
//     ez                                             527 ns/iter
//     native_1000x                                   719 ns/iter
//     parse_compile_eval                             386 ns/iter
//     parse_eval_1000x                            263581 ns/iter
//     parse_eval                                     258 ns/iter
//     parse_nsbubble_eval                            376 ns/iter
//     preparse_eval_1000x                          92112 ns/iter
//     preparse_eval                                   91 ns/iter
//     preparse_precompile_eval_1000x               31222 ns/iter
//     preparse_precompile_eval                        31 ns/iter
//     preparse_precompile_nsbubble_eval               30 ns/iter
//     --features unsafe-vars:
//     ez                                             605 ns/iter
//     native_1000x                                   718 ns/iter
//     parse_compile_eval                             393 ns/iter
//     parse_eval_1000x                            267318 ns/iter
//     parse_eval                                     267 ns/iter
//     parse_nsbubble_eval                            406 ns/iter
//     preparse_eval_1000x                          95605 ns/iter
//     preparse_eval                                   97 ns/iter
//     preparse_precompile_eval_1000x               31508 ns/iter
//     preparse_precompile_eval                        31 ns/iter
//     preparse_precompile_eval_unsafe_1000x         8022 ns/iter
//     preparse_precompile_nsbubble_eval               31 ns/iter
//
//     "sin(x)"
//     Normal:
//     test ez                                ... bench:         677 ns/iter (+/- 86)
//     test native_1000x                      ... bench:      17,453 ns/iter (+/- 2,589)
//     test parse_compile_eval                ... bench:         385 ns/iter (+/- 78)
//     test parse_eval_1000x                  ... bench:     385,391 ns/iter (+/- 32,235)
//     test parse_eval                        ... bench:         392 ns/iter (+/- 46)
//     test parse_nsbubble_eval               ... bench:         527 ns/iter (+/- 71)
//     test preparse_eval_1000x               ... bench:     137,070 ns/iter (+/- 10,268)
//     test preparse_eval                     ... bench:         138 ns/iter (+/- 11)
//     test preparse_precompile_eval_1000x    ... bench:      55,116 ns/iter (+/- 4,872)
//     test preparse_precompile_eval          ... bench:          56 ns/iter (+/- 10)
//     test preparse_precompile_nsbubble_eval ... bench:         162 ns/iter (+/- 23)
//     --features unsafe-vars:
//     test ez                                    ... bench:         711 ns/iter (+/- 52)
//     test native_1000x                          ... bench:      17,773 ns/iter (+/- 1,983)
//     test parse_compile_eval                    ... bench:         380 ns/iter (+/- 54)
//     test parse_eval_1000x                      ... bench:     386,350 ns/iter (+/- 25,731)
//     test parse_eval                            ... bench:         384 ns/iter (+/- 60)
//     test parse_nsbubble_eval                   ... bench:         522 ns/iter (+/- 206)
//     test preparse_eval_1000x                   ... bench:     139,814 ns/iter (+/- 10,343)
//     test preparse_eval                         ... bench:         141 ns/iter (+/- 8)
//     test preparse_precompile_eval_1000x        ... bench:      55,861 ns/iter (+/- 8,464)
//     test preparse_precompile_eval              ... bench:          55 ns/iter (+/- 6)
//     test preparse_precompile_eval_unsafe_1000x ... bench:      23,117 ns/iter (+/- 1,192)
//     test preparse_precompile_nsbubble_eval     ... bench:          56 ns/iter (+/- 3)
//
//     "(-z + (z^2 - 4*x*y)^0.5) / (2*x)"
//     Normal:
//     test ez                                ... bench:       2,381 ns/iter (+/- 627)
//     test native_1000x                      ... bench:       5,108 ns/iter (+/- 323)
//     test parse_compile_eval                ... bench:       2,809 ns/iter (+/- 1,032)
//     test parse_eval_1000x                  ... bench:   1,548,632 ns/iter (+/- 111,813)
//     test parse_eval                        ... bench:       1,629 ns/iter (+/- 470)
//     test parse_nsbubble_eval               ... bench:       1,963 ns/iter (+/- 169)
//     test preparse_eval_1000x               ... bench:     603,712 ns/iter (+/- 95,657)
//     test preparse_eval                     ... bench:         593 ns/iter (+/- 39)
//     test preparse_precompile_eval_1000x    ... bench:     252,384 ns/iter (+/- 36,933)
//     test preparse_precompile_eval          ... bench:         257 ns/iter (+/- 104)
//     test preparse_precompile_nsbubble_eval ... bench:         508 ns/iter (+/- 65)
//     --features unsafe-vars:
//     test ez                                    ... bench:       2,723 ns/iter (+/- 312)
//     test native_1000x                          ... bench:       5,316 ns/iter (+/- 158)
//     test parse_compile_eval                    ... bench:       2,973 ns/iter (+/- 533)
//     test parse_eval_1000x                      ... bench:   1,719,108 ns/iter (+/- 148,490)
//     test parse_eval                            ... bench:       1,762 ns/iter (+/- 285)
//     test parse_nsbubble_eval                   ... bench:       2,147 ns/iter (+/- 323)
//     test preparse_eval_1000x                   ... bench:     651,436 ns/iter (+/- 148,266)
//     test preparse_eval                         ... bench:         651 ns/iter (+/- 223)
//     test preparse_precompile_eval_1000x        ... bench:     267,852 ns/iter (+/- 75,311)
//     test preparse_precompile_eval              ... bench:         266 ns/iter (+/- 52)
//     test preparse_precompile_eval_unsafe_1000x ... bench:     118,533 ns/iter (+/- 20,489)
//     test preparse_precompile_nsbubble_eval     ... bench:         265 ns/iter (+/- 49)
//
//     "((((87))) - 73) + (97 + (((15 / 55 * ((31)) + 35))) + (15 - (9)) - (39 / 26) / 20 / 91 + 27 / (33 * 26 + 28 - (7) / 10 + 66 * 6) + 60 / 35 - ((29) - (69) / 44 / (92)) / (89) + 2 + 87 / 47 * ((2)) * 83 / 98 * 42 / (((67)) * ((97))) / (34 / 89 + 77) - 29 + 70 * (20)) + ((((((92))) + 23 * (98) / (95) + (((99) * (41))) + (5 + 41) + 10) - (36) / (6 + 80 * 52 + (90))))"
//     Normal:
//     test ez                                ... bench:      12,028 ns/iter (+/- 1,079)
//     test native_1000x                      ... bench:         330 ns/iter (+/- 20)
//     test parse_compile_eval                ... bench:      15,835 ns/iter (+/- 2,976)
//     test parse_eval_1000x                  ... bench:  12,197,838 ns/iter (+/- 1,633,395)
//     test parse_eval                        ... bench:      11,935 ns/iter (+/- 1,365)
//     test parse_nsbubble_eval               ... bench:      12,235 ns/iter (+/- 861)
//     test preparse_eval_1000x               ... bench:   3,674,860 ns/iter (+/- 568,795)
//     test preparse_eval                     ... bench:       3,665 ns/iter (+/- 536)
//     test preparse_precompile_eval_1000x    ... bench:         747 ns/iter (+/- 53)
//     test preparse_precompile_eval          ... bench:           0 ns/iter (+/- 0)
//     test preparse_precompile_nsbubble_eval ... bench:           0 ns/iter (+/- 0)
//     --features unsafe-vars:
//     test ez                                    ... bench:      13,132 ns/iter (+/- 2,016)
//     test native_1000x                          ... bench:         357 ns/iter (+/- 76)
//     test parse_compile_eval                    ... bench:      17,259 ns/iter (+/- 3,186)
//     test parse_eval_1000x                      ... bench:  13,110,320 ns/iter (+/- 2,096,098)
//     test parse_eval                            ... bench:      12,481 ns/iter (+/- 3,424)
//     test parse_nsbubble_eval                   ... bench:      13,131 ns/iter (+/- 2,855)
//     test preparse_eval_1000x                   ... bench:   4,018,177 ns/iter (+/- 1,119,021)
//     test preparse_eval                         ... bench:       3,997 ns/iter (+/- 735)
//     test preparse_precompile_eval_1000x        ... bench:         997 ns/iter (+/- 413)
//     test preparse_precompile_eval              ... bench:           1 ns/iter (+/- 0)
//     test preparse_precompile_eval_unsafe_1000x ... bench:         990 ns/iter (+/- 230)
//     test preparse_precompile_nsbubble_eval     ... bench:           1 ns/iter (+/- 0)
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
// tinyexpr-rs:
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
// tinyexpr-c:
//     "(3 * (3 + 3) / 3)"
//     te_interp  :  748 ns/iter
//     parse_compile_eval  :  762 ns/iter
//     preparse_precompile_eval  :  2.8 ns/iter
//
//     "3 * 3 - 3 / 3"
//     te_interp  :  615 ns/iter
//     parse_compile_eval  :  630 ns/iter
//     preparse_precompile_eval  :  2.8 ns/iter
//
//     "2 ^ (3 ^ 4)"  = 2417851639229258349412352.000000
//     te_interp  :  585 ns/iter
//     parse_compile_eval  :  580 ns/iter
//     preparse_precompile_eval  :  2.8 ns/iter
//
//     "x * 2"
//     parse_compile_eval  :  221 ns/iter
//     preparse_precompile_eval  :  9.4 ns/iter
//
//     "sin(x)"
//     parse_compile_eval  :  249 ns/iter
//     preparse_precompile_eval  :  21.4 ns/iter
//
//     "(-z + sqrt(z^2 - 4*x*y)) / (2*x)"
//     parse_compile_eval  :  1507 ns/iter
//     preparse_precompile_eval  :  117 ns/iter
//
//     "((((87))) - 73) + (97 + (((15 / 55 * ((31)) + 35))) + (15 - (9)) - (39 / 26) / 20 / 91 + 27 / (33 * 26 + 28 - (7) / 10 + 66 * 6) + 60 / 35 - ((29) - (69) / 44 / (92)) / (89) + 2 + 87 / 47 * ((2)) * 83 / 98 * 42 / (((67)) * ((97))) / (34 / 89 + 77) - 29 + 70 * (20)) + ((((((92))) + 23 * (98) / (95) + (((99) * (41))) + (5 + 41) + 10) - (36) / (6 + 80 * 52 + (90))))"
//     te_interp  :  12,423 ns/iter
//     parse_compile_eval  :  12,222 ns/iter
//     preparse_precompile_eval  :  2.8 ns/iter
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

use al::{parse, Compiler, Evaler, Slab, EvalNS, ez_eval, eval_instr_ref_or_panic};

use std::collections::BTreeMap;
use std::f64::NAN;


//fn evalcb(_:&str) -> Option<f64> { None }
fn evalcb(name:&str, args:Vec<f64>) -> Option<f64> {
    match name {
        "x" => Some(1.0),
        "y" => Some(2.0),
        "z" => Some(3.0),
        "foo" => Some(args.get(0).unwrap_or(&NAN)*10.0),
        "bar" => Some(args.get(0).unwrap_or(&NAN) + args.get(1).unwrap_or(&NAN)),
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
            black_box(3.0 * 3.0 - 3.0 / 3.0);
            //black_box(2.0f64.powf(3.0).powf(4.0));
            //black_box(x() * 2.0);
            //black_box(x().sin());
            //black_box( (-b + (b.powf(2.0) - 4.0*a*c).powf(0.5)) / (2.0*a) );
            //black_box( ((((87.))) - 73.) + (97. + (((15. / 55. * ((31.)) + 35.))) + (15. - (9.)) - (39. / 26.) / 20. / 91. + 27. / (33. * 26. + 28. - (7.) / 10. + 66. * 6.) + 60. / 35. - ((29.) - (69.) / 44. / (92.)) / (89.) + 2. + 87. / 47. * ((2.)) * 83. / 98. * 42. / (((67.)) * ((97.))) / (34. / 89. + 77.) - 29. + 70. * (20.)) + ((((((92.))) + 23. * (98.) / (95.) + (((99.) * (41.))) + (5. + 41.) + 10.) - (36.) / (6. + 80. * 52. + (90.)))) );
        }
    });
}

#[bench]
fn ez(b:&mut Bencher) {
    let mut vars=BTreeMap::new();
    vars.insert("x".to_string(),1.0);
    vars.insert("y".to_string(),2.0);
    vars.insert("z".to_string(),3.0);

    b.iter(|| {
        black_box(ez_eval(EXPR, &vars).unwrap());
    });
}

#[bench]
fn parse_eval_1x(b:&mut Bencher) {
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(evalcb);

    b.iter(|| {
        black_box(parse({slab.clear(); &mut slab.ps}, EXPR).unwrap().from(&slab.ps).eval(&slab, &mut ns).unwrap());
    });
}

#[bench]
fn parse_nsbubble_eval(b:&mut Bencher) {
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(evalcb);

    b.iter(|| {
        let expr_ref = parse({slab.clear(); &mut slab.ps}, EXPR).unwrap().from(&slab.ps);
        black_box(
            ns.eval_bubble(&slab, expr_ref).unwrap()
        );
    });
}

// Let's see how much the benchmark system is affected by its self:
#[bench]
fn parse_eval_1000x(b:&mut Bencher) {
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(evalcb);

    b.iter(|| {
        for _ in 0..1000 {
            black_box(parse({slab.clear(); &mut slab.ps}, EXPR).unwrap().from(&slab.ps).eval(&slab, &mut ns).unwrap());
        }
    });
}

// #[bench]
// #[allow(non_snake_case)]
// fn parse_eval_100M(b:&mut Bencher) {
//     let mut slab = Slab::new();
//     let mut ns = EvalNS::new(evalcb);
// 
//     b.iter(|| {
//         for _ in 0..100_000_000 {
//             black_box(parse({slab.clear(); &mut slab.ps}, EXPR).unwrap().from(&slab.ps).eval(&slab, &mut ns).unwrap());
//         }
//     });
// }

#[bench]
fn preparse_eval(b:&mut Bencher) {
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(evalcb);
    let expr_ref = parse(&mut slab.ps, EXPR).unwrap().from(&slab.ps);

    b.iter(|| {
        black_box(expr_ref.eval(&slab, &mut ns).unwrap());
    });
}

#[bench]
fn preparse_eval_1000x(b:&mut Bencher) {
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(evalcb);
    let expr_ref = parse(&mut slab.ps, EXPR).unwrap().from(&slab.ps);

    b.iter(|| {
        for _ in 0..1000 {
            black_box(expr_ref.eval(&slab, &mut ns).unwrap());
        }
    });
}

#[bench]
fn parse_compile_eval(b:&mut Bencher) {
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(evalcb);

    b.iter(|| {
        black_box(parse({slab.clear(); &mut slab.ps}, EXPR).unwrap().from(&slab.ps).compile(&slab.ps, &mut slab.cs).eval(&slab, &mut ns).unwrap());
    });
}

#[bench]
fn preparse_precompile_eval(b:&mut Bencher) {
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(evalcb);
    let instr = parse(&mut slab.ps, EXPR).unwrap().from(&slab.ps).compile(&slab.ps, &mut slab.cs);

    b.iter(|| {
        black_box(eval_instr_ref_or_panic!(&instr, &slab, &mut ns));
    });
}

#[bench]
fn preparse_precompile_nsbubble_eval(b:&mut Bencher) {
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(evalcb);
    let instr = parse(&mut slab.ps, EXPR).unwrap().from(&slab.ps).compile(&slab.ps, &mut slab.cs);

    b.iter(|| {
        black_box(eval_instr_ref_or_panic!(&instr, &slab, &mut ns));
    });
}

#[bench]
fn preparse_precompile_eval_1000x(b:&mut Bencher) {
    let mut slab = Slab::new();
    let mut ns = EvalNS::new(evalcb);
    let instr = parse(&mut slab.ps, EXPR).unwrap().from(&slab.ps).compile(&slab.ps, &mut slab.cs);

    b.iter(|| {
        for _ in 0..1000 {
            black_box(eval_instr_ref_or_panic!(&instr, &slab, &mut ns));
        }
    });
}

// #[bench]
// #[allow(non_snake_case)]
// fn preparse_precompile_eval_100B(_:&mut Bencher) {
//     let mut slab = Slab::new();
//     let mut ns = EvalNS::new(evalcb);
//     let instr = parse(&mut slab.ps, EXPR).unwrap().from(&slab.ps).compile(&slab.ps, &mut slab.cs);
//
//     let start = std::time::Instant::now();
//     for _ in 0..100 {
//         for _ in 0..1_000_000_000 {
//             black_box(eval_instr_ref_or_panic!(&instr, &slab, &mut ns));
//         }
//     }
//     eprintln!("bench time: {}", start.elapsed().as_secs_f64());
// }

#[bench]
#[cfg(feature="unsafe-vars")]
fn preparse_precompile_eval_unsafe_1000x(b:&mut Bencher) {
    let mut slab = Slab::new();
    let x = 1.0;
    let y = 2.0;
    let z = 3.0;
    let foo = 0.0;
    let bar = 0.0;
    unsafe {
        slab.ps.add_unsafe_var("x".to_string(), &x);
        slab.ps.add_unsafe_var("y".to_string(), &y);
        slab.ps.add_unsafe_var("z".to_string(), &z);
        slab.ps.add_unsafe_var("foo".to_string(), &foo);
        slab.ps.add_unsafe_var("bar".to_string(), &bar);
    }

    let mut ns = EvalNS::new(|_,_| None);
    let instr = parse(&mut slab.ps, EXPR).unwrap().from(&slab.ps).compile(&slab.ps, &mut slab.cs);

    b.iter(|| {
        for _ in 0..1000 {
            black_box(eval_instr_ref_or_panic!(&instr, &slab, &mut ns));
        }
    });
}

// #[bench]
// #[cfg(feature="unsafe-vars")]
// #[allow(non_snake_case)]
// fn preparse_precompile_eval_unsafe_1B(_:&mut Bencher) {
//     let mut slab = Slab::new();
//     let x = 1.0;
//     let y = 2.0;
//     let z = 3.0;
//     let foo = 0.0;
//     let bar = 0.0;
//     unsafe {
//         slab.ps.add_unsafe_var("x".to_string(), &x);
//         slab.ps.add_unsafe_var("y".to_string(), &y);
//         slab.ps.add_unsafe_var("z".to_string(), &z);
//         slab.ps.add_unsafe_var("foo".to_string(), &foo);
//         slab.ps.add_unsafe_var("bar".to_string(), &bar);
//     }
//
//     let mut ns = EvalNS::new(|_,_| None);
//     let instr = parse(&mut slab.ps, EXPR).unwrap().from(&slab.ps).compile(&slab.ps, &mut slab.cs);
//
//     let start = std::time::Instant::now();
//     //for _ in 0..100 {
//         for _ in 0..1_000_000_000 {
//             black_box(eval_instr_ref_or_panic!(&instr, &slab, &mut ns));
//         }
//     //}
//     eprintln!("bench time: {}", start.elapsed().as_secs_f64());
// }

