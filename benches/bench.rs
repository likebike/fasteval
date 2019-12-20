
// Here is how I run benchmarks:
//     while true; do echo "time: $(date +%s)"; RUSTFLAGS="--emit=asm" cargo bench; done >bench.out
//     while true; do echo "time: $(date +%s)"; RUSTFLAGS="--emit=asm" cargo bench --features unsafe-vars; done >bench.out
//     cat bench.out | awk -v "now=$(date +%s)" '$1=="time:"{when=$2}  $3=="..." && $4=="bench:" {gsub(/,/, "", $5); v=$5+0; if (t[$2]=="" || v<t[$2]){t[$2]=v; w[$2]=when;}} END{for (k in t) { printf "%-40s %9d ns/iter    %5ds ago\n",k,t[k],now-w[k] }}' | sort



// ---- Results (2019-12-04 on a 2012 laptop with Intel(R) Core(TM) i7-3610QM CPU @ 2.30GHz) ----
// al:
//     "(3 * (3 + 3) / 3)"
//     BTreeMap, --emit=asm:
//     ez                                             610 ns/iter     1934s ago
//     native_1000x                                   320 ns/iter     2087s ago
//     parse_compile_eval_1000x                    724547 ns/iter      940s ago
//     parse_eval_1000x                            477833 ns/iter     1841s ago
//     parse_nsbubble_eval_1000x                   487980 ns/iter     1861s ago
//     parser::internal_tests::spaces_1M            11631 ns/iter     1994s ago
//     preparse_eval_1000x                         170040 ns/iter     1919s ago
//     preparse_precompile_eval_1000x                 618 ns/iter     2070s ago
//     preparse_precompile_nsbubble_eval_1000x       9797 ns/iter     1934s ago
//     BTreeMap, --emit=asm, --features unsafe-vars:
//     ez                                             624 ns/iter     1115s ago
//     native_1000x                                   325 ns/iter     1518s ago
//     parse_compile_eval_1000x                    775976 ns/iter     1572s ago
//     parse_eval_1000x                            499386 ns/iter     1572s ago
//     parse_eval_unsafe_1000x                     495709 ns/iter     1659s ago
//     parse_nsbubble_eval_1000x                   509171 ns/iter     1489s ago
//     parser::internal_tests::spaces_1M            11853 ns/iter     1489s ago
//     preparse_eval_1000x                         169035 ns/iter     1451s ago
//     preparse_precompile_eval_1000x                 945 ns/iter     2369s ago
//     preparse_precompile_eval_unsafe_1000x          939 ns/iter      950s ago
//     preparse_precompile_nsbubble_eval_1000x      10004 ns/iter     1543s ago
//
//     "3 * 3 - 3 / 3"
//     BTreeMap, --emit=asm:
//     ez                                             390 ns/iter     1152s ago
//     native_1000x                                   321 ns/iter     3181s ago
//     parse_compile_eval_1000x                    572960 ns/iter     1119s ago
//     parse_eval_1000x                            293849 ns/iter     1170s ago
//     parse_nsbubble_eval_1000x                   303399 ns/iter     1152s ago
//     parser::internal_tests::spaces_1M            11680 ns/iter     2434s ago
//     preparse_eval_1000x                          87054 ns/iter     1119s ago
//     preparse_precompile_eval_1000x                 620 ns/iter     1170s ago
//     preparse_precompile_nsbubble_eval_1000x       9813 ns/iter     1119s ago
//     BTreeMap, --emit=asm, --features unsafe-vars:
//     ez                                             407 ns/iter     1530s ago
//     native_1000x                                   319 ns/iter     1439s ago
//     parse_compile_eval_1000x                    601357 ns/iter      965s ago
//     parse_eval_1000x                            292443 ns/iter     1279s ago
//     parse_eval_unsafe_1000x                     290481 ns/iter     1258s ago
//     parse_nsbubble_eval_1000x                   307644 ns/iter     1126s ago
//     parser::internal_tests::spaces_1M            11659 ns/iter     2111s ago
//     preparse_eval_1000x                          83932 ns/iter     1682s ago
//     preparse_precompile_eval_1000x                 925 ns/iter     1302s ago
//     preparse_precompile_eval_unsafe_1000x          923 ns/iter     1279s ago
//     preparse_precompile_nsbubble_eval_1000x       9812 ns/iter     2055s ago
//
//     "2 ^ 3 ^ 4"  = 2417851639229258300000000
//     BTreeMap, --emit=asm:
//     ez                                             458 ns/iter      546s ago
//     native_1000x                                   322 ns/iter     1017s ago
//     parse_compile_eval_1000x                    447158 ns/iter     1441s ago
//     parse_eval_1000x                            327303 ns/iter      572s ago
//     parse_nsbubble_eval_1000x                   341402 ns/iter     1112s ago
//     parser::internal_tests::spaces_1M            11770 ns/iter     1098s ago
//     preparse_eval_1000x                         180946 ns/iter     3283s ago
//     preparse_precompile_eval_1000x                 622 ns/iter     1274s ago
//     preparse_precompile_nsbubble_eval_1000x       9928 ns/iter      992s ago
//     BTreeMap, --emit=asm, --features unsafe-vars:
//     ez                                             432 ns/iter     7823s ago
//     native_1000x                                   324 ns/iter    10067s ago
//     parse_compile_eval_1000x                    459533 ns/iter     3670s ago
//     parse_eval_1000x                            328942 ns/iter     7215s ago
//     parse_eval_unsafe_1000x                     327694 ns/iter     7402s ago
//     parse_nsbubble_eval_1000x                   343165 ns/iter     8167s ago
//     parser::internal_tests::spaces_1M            11755 ns/iter    12199s ago
//     preparse_eval_1000x                         181459 ns/iter     2964s ago
//     preparse_precompile_eval_1000x                 933 ns/iter    12999s ago
//     preparse_precompile_eval_unsafe_1000x          935 ns/iter    10733s ago
//     preparse_precompile_nsbubble_eval_1000x       9849 ns/iter     7720s ago
//
//     "x * 2"
//     BTreeMap, --emit=asm:
//     ez                                             316 ns/iter      538s ago
//     native_1000x                                   687 ns/iter     1615s ago
//     parse_compile_eval_1000x                    297842 ns/iter     1615s ago
//     parse_eval_1000x                            182330 ns/iter     3751s ago
//     parse_nsbubble_eval_1000x                   262956 ns/iter     3857s ago
//     parser::internal_tests::spaces_1M            11812 ns/iter     2248s ago
//     preparse_eval_1000x                          70666 ns/iter     3324s ago
//     preparse_precompile_eval_1000x               21366 ns/iter     3459s ago
//     preparse_precompile_nsbubble_eval_1000x      91714 ns/iter     5514s ago
//     BTreeMap, --emit=asm, --features unsafe-vars:
//     ez                                             291 ns/iter     3200s ago
//     native_1000x                                   689 ns/iter     1082s ago
//     parse_compile_eval_1000x                    312200 ns/iter      500s ago
//     parse_eval_1000x                            189106 ns/iter     1599s ago
//     parse_eval_unsafe_1000x                     182333 ns/iter     1102s ago
//     parse_nsbubble_eval_1000x                   280231 ns/iter      715s ago
//     parser::internal_tests::spaces_1M            11649 ns/iter      938s ago
//     preparse_eval_1000x                          69640 ns/iter      654s ago
//     preparse_precompile_eval_1000x               21497 ns/iter     1833s ago
//     preparse_precompile_eval_unsafe_1000x         7977 ns/iter      740s ago
//     preparse_precompile_nsbubble_eval_1000x      94730 ns/iter     2498s ago
//
//     "sin(x)"
//     BTreeMap, --emit=asm:
//     ez                                             374 ns/iter      511s ago
//     native_1000x                                 16791 ns/iter     1214s ago
//     parse_compile_eval_1000x                    294037 ns/iter      821s ago
//     parse_eval_1000x                            242237 ns/iter      895s ago
//     parse_nsbubble_eval_1000x                   342756 ns/iter      771s ago
//     parser::internal_tests::spaces_1M            11670 ns/iter      806s ago
//     preparse_eval_1000x                          79784 ns/iter      771s ago
//     preparse_precompile_eval_1000x               38181 ns/iter      755s ago
//     preparse_precompile_nsbubble_eval_1000x     110285 ns/iter     1029s ago
//     BTreeMap, --emit=asm, --features unsafe-vars:
//     ez                                             379 ns/iter     5094s ago
//     native_1000x                                 16804 ns/iter     5386s ago
//     parse_compile_eval_1000x                    302278 ns/iter     1342s ago
//     parse_eval_1000x                            273672 ns/iter     3506s ago
//     parse_eval_unsafe_1000x                     248795 ns/iter     4559s ago
//     parse_nsbubble_eval_1000x                   334456 ns/iter     3222s ago
//     parser::internal_tests::spaces_1M            11655 ns/iter     1868s ago
//     preparse_eval_1000x                          76958 ns/iter     4811s ago
//     preparse_precompile_eval_1000x               37705 ns/iter     2437s ago
//     preparse_precompile_eval_unsafe_1000x        23471 ns/iter     5686s ago
//     preparse_precompile_nsbubble_eval_1000x     110606 ns/iter     5062s ago
//
//     "(-z + (z^2 - 4*x*y)^0.5) / (2*x)"
//     BTreeMap, --emit=asm:
//     ez                                            1408 ns/iter    10889s ago
//     native_1000x                                   322 ns/iter    14196s ago
//     parse_compile_eval_1000x                   2357775 ns/iter     6273s ago
//     parse_eval_1000x                           1184392 ns/iter     1455s ago
//     parse_nsbubble_eval_1000x                  1419954 ns/iter    11039s ago
//     parser::internal_tests::spaces_1M            11644 ns/iter     9840s ago
//     preparse_eval_1000x                         472655 ns/iter     8178s ago
//     preparse_precompile_eval_1000x              201106 ns/iter     8788s ago
//     preparse_precompile_nsbubble_eval_1000x     367644 ns/iter    10012s ago
//     BTreeMap, --emit=asm, --features unsafe-vars:
//     ez                                            1476 ns/iter     7876s ago with unsafe to avoid allocation
//     native_1000x                                   318 ns/iter     8786s ago
//     parse_compile_eval_1000x                   2436218 ns/iter     4501s ago
//     parse_eval_1000x                           1222108 ns/iter     4652s ago
//     parse_eval_unsafe_1000x                    1187956 ns/iter     8751s ago
//     parse_nsbubble_eval_1000x                  1416827 ns/iter     8946s ago
//     parser::internal_tests::spaces_1M            11557 ns/iter     4236s ago
//     preparse_eval_1000x                         467931 ns/iter     2673s ago
//     preparse_precompile_eval_1000x              199312 ns/iter     1985s ago
//     preparse_precompile_eval_unsafe_1000x       121572 ns/iter     2687s ago
//     preparse_precompile_nsbubble_eval_1000x     353696 ns/iter     8603s ago
//     ez                                            1487 ns/iter     4958s ago with String allocation
//     native_1000x                                   321 ns/iter     4794s ago
//     parse_compile_eval_1000x                   2528829 ns/iter     3995s ago
//     parse_eval_1000x                           1200250 ns/iter      871s ago
//     parse_eval_unsafe_1000x                    1166532 ns/iter     2900s ago
//     parse_nsbubble_eval_1000x                  1411843 ns/iter      500s ago
//     parser::internal_tests::spaces_1M            11671 ns/iter     4536s ago
//     preparse_eval_1000x                         479185 ns/iter     3359s ago
//     preparse_precompile_eval_1000x              200055 ns/iter     1515s ago
//     preparse_precompile_eval_unsafe_1000x       122676 ns/iter     4763s ago
//     preparse_precompile_nsbubble_eval_1000x     368102 ns/iter     3134s ago
//
//     "((((87))) - 73) + (97 + (((15 / 55 * ((31)) + 35))) + (15 - (9)) - (39 / 26) / 20 / 91 + 27 / (33 * 26 + 28 - (7) / 10 + 66 * 6) + 60 / 35 - ((29) - (69) / 44 / (92)) / (89) + 2 + 87 / 47 * ((2)) * 83 / 98 * 42 / (((67)) * ((97))) / (34 / 89 + 77) - 29 + 70 * (20)) + ((((((92))) + 23 * (98) / (95) + (((99) * (41))) + (5 + 41) + 10) - (36) / (6 + 80 * 52 + (90))))"
//     BTreeMap, --emit=asm:
//     ez                                            9376 ns/iter     8285s ago
//     native_1000x                                   320 ns/iter    33906s ago
//     parse_compile_eval_1000x                  14659614 ns/iter    19760s ago
//     parse_eval_1000x                           9432013 ns/iter    24311s ago
//     parse_nsbubble_eval_1000x                  9496883 ns/iter    23521s ago
//     parser::internal_tests::spaces_1M            11642 ns/iter    33047s ago
//     preparse_eval_1000x                        2916288 ns/iter     3344s ago
//     preparse_precompile_eval_1000x                 618 ns/iter    12722s ago
//     preparse_precompile_nsbubble_eval_1000x       9798 ns/iter     3098s ago
//     BTreeMap, --emit=asm, --features unsafe-vars:
//     ez                                            9542 ns/iter     1751s ago
//     native_1000x                                   320 ns/iter      585s ago
//     parse_compile_eval_1000x                  15370356 ns/iter     2463s ago
//     parse_eval_1000x                           9782538 ns/iter     1489s ago
//     parse_eval_unsafe_1000x                    9687478 ns/iter     1164s ago
//     parse_nsbubble_eval_1000x                  9739020 ns/iter      502s ago
//     parser::internal_tests::spaces_1M            11657 ns/iter      627s ago
//     preparse_eval_1000x                        2914600 ns/iter     2975s ago
//     preparse_precompile_eval_1000x                 927 ns/iter     2742s ago
//     preparse_precompile_eval_unsafe_1000x          927 ns/iter     2586s ago
//     preparse_precompile_nsbubble_eval_1000x       9812 ns/iter     2769s ago
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

use al::{parse, Compiler, Evaler, Layered, Slab, EmptyNamespace, FlatNamespace, ScopedNamespace, Bubble, ez_eval, eval_compiled, eval_compiled_ref};

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

macro_rules! Namespace {
    () => {
        {
            let mut map = BTreeMap::new();
            map.insert("x".to_string(), 1.0);
            map.insert("y".to_string(), 2.0);
            map.insert("z".to_string(), 3.0);
            map
        }

        //EmptyNamespace

        //FlatNamespace::new(evalcb)

        //ScopedNamespace::new(evalcb)
    }
}

//static EXPR : &'static str = "(3 * (3 + 3) / 3)";
//static EXPR : &'static str = "3 * 3 - 3 / 3";
//static EXPR : &'static str = "2 ^ 3 ^ 4";
//static EXPR : &'static str = "x * 2";
//static EXPR : &'static str = "sin(x)";
static EXPR : &'static str = "(-z + (z^2 - 4*x*y)^0.5) / (2*x)";
//static EXPR : &'static str = "((((87))) - 73) + (97 + (((15 / 55 * ((31)) + 35))) + (15 - (9)) - (39 / 26) / 20 / 91 + 27 / (33 * 26 + 28 - (7) / 10 + 66 * 6) + 60 / 35 - ((29) - (69) / 44 / (92)) / (89) + 2 + 87 / 47 * ((2)) * 83 / 98 * 42 / (((67)) * ((97))) / (34 / 89 + 77) - 29 + 70 * (20)) + ((((((92))) + 23 * (98) / (95) + (((99) * (41))) + (5 + 41) + 10) - (36) / (6 + 80 * 52 + (90))))";

#[bench]
fn native_1000x(bencher:&mut Bencher) {
    // Silence compiler warnings about unused imports:
    let _ = EmptyNamespace;  let _ = FlatNamespace::new(|_,_| None);


    #[allow(dead_code)]
    fn x() -> f64 { black_box(1.0) }
    #[allow(unused_variables)]
    let (a,b,c) = (1.0f64, 3.0f64, 2.0f64);
    bencher.iter(|| {
        //let (a,b,c) = (a,b,c);  // Localize
        for _ in 0..1000 {
            //black_box(3.0 * (3.0 + 3.0) / 3.0);
            //black_box(3.0 * 3.0 - 3.0 / 3.0);
            //black_box(2.0f64.powf(3.0).powf(4.0));
            //black_box(x() * 2.0);
            //black_box(x().sin());
            black_box( (-b + (b.powf(2.0) - 4.0*a*c).powf(0.5)) / (2.0*a) );
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
        black_box(match ez_eval(EXPR, &mut vars) {
            Ok(f) => f,
            Err(_) => 0.0,
        });
    });
}

#[bench]
fn parse_eval_1000x(b:&mut Bencher) {
    let mut slab = Slab::new();
    let mut ns = Namespace!();

    b.iter(|| {
        let _ = (|| -> Result<(),al::Error> {
            for _ in 0..1000 {
                black_box(parse(EXPR, {slab.clear(); &mut slab.ps})?.from(&slab.ps).eval(&slab, &mut ns)?);
            }
            Ok(())
        })();
    });
}

#[bench]
fn parse_nsbubble_eval_1000x(b:&mut Bencher) {
    let mut slab = Slab::new();
    let mut ns = ScopedNamespace::new(evalcb);

    b.iter(|| {
        let _ = (|| -> Result<(),al::Error> {
            for _ in 0..1000 {
                let expr_ref = parse(EXPR, {slab.clear(); &mut slab.ps})?.from(&slab.ps);
                let mut bub = Bubble::new(&mut ns);  bub.push();
                black_box( expr_ref.eval(&slab, &mut bub)? );
            }
            Ok(())
        })();
    });
}

#[bench]
#[cfg(feature="unsafe-vars")]
fn parse_eval_unsafe_1000x(b:&mut Bencher) {
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

    let mut ns = EmptyNamespace;

    b.iter(|| {
        let _ = (|| -> Result<(),al::Error> {
            for _ in 0..1000 {
                black_box(parse(EXPR, {slab.clear(); &mut slab.ps})?.from(&slab.ps).eval(&slab, &mut ns)?);
            }
            Ok(())
        })();
    });
}

#[bench]
fn preparse_eval_1000x(b:&mut Bencher) {
    let mut slab = Slab::new();
    let mut ns = Namespace!();
    let expr_ref = match parse(EXPR, &mut slab.ps) {
        Ok(expr_i) => expr_i.from(&slab.ps),
        Err(_) => return,
    };

    b.iter(|| {
        let _ = (|| -> Result<(),al::Error> {
            for _ in 0..1000 {
                black_box( expr_ref.eval(&slab, &mut ns)? );
            }
            Ok(())
        })();
    });
}

#[bench]
fn parse_compile_eval_1000x(b:&mut Bencher) {
    let mut slab = Slab::new();
    let mut ns = Namespace!();

    b.iter(|| {
        let _ = (|| -> Result<(),al::Error> {
            for _ in 0..1000 {
                let instr = parse(EXPR, {slab.clear(); &mut slab.ps})?.from(&slab.ps).compile(&slab.ps, &mut slab.cs);
                black_box(eval_compiled!(instr, &slab, &mut ns));
            }
            Ok(())
        })();
    });
}

#[bench]
fn preparse_precompile_eval_1000x(b:&mut Bencher) {
    let mut slab = Slab::new();
    let mut ns = Namespace!();
    let instr = match parse(EXPR, &mut slab.ps) {
        Ok(expr_i) => expr_i.from(&slab.ps).compile(&slab.ps, &mut slab.cs),
        Err(_) => return,
    };

    b.iter(|| {
        let _ = (|| -> Result<(),al::Error> {
            let (instr_ref, slab_ref, ns_mut) = (&instr, &slab, &mut ns);  // Localize (doesn't help much)
            for _ in 0..1000 {
                black_box( eval_compiled_ref!(instr_ref, slab_ref, ns_mut));
            }
            Ok(())
        })();
    });

    //// Produces basically the same results, proving that the --emit=asm performanace boost is not coming from this test function -- it's coming from the evaluation, and I'm not able to replicate it.
    // let _ = (|| -> Result<(),al::Error> {
    //     let mut slab = Slab::new();
    //     let mut ns = Namespace!();
    //     let instr = match parse(EXPR, &mut slab.ps) {
    //         Ok(expr_i) => expr_i.from(&slab.ps).compile(&slab.ps, &mut slab.cs),
    //         Err(e) => return Err(e),
    //     };
    //
    //     let start = std::time::Instant::now();
    //     for _ in 0..1_000_000 {
    //         black_box( eval_compiled_ref!(&instr, &slab, &mut ns) );
    //     }
    //     eprintln!("bench time: {}", start.elapsed().as_secs_f64());
    //
    //     Ok(())
    // })();
}

#[bench]
fn preparse_precompile_nsbubble_eval_1000x(b:&mut Bencher) {
    let mut slab = Slab::new();
    let mut ns = ScopedNamespace::new(evalcb);
    let instr = match parse(EXPR, &mut slab.ps) {
        Ok(expr_i) => expr_i.from(&slab.ps).compile(&slab.ps, &mut slab.cs),
        Err(_) => return,
    };

    b.iter(|| {
        let _ = (|| -> Result<(),al::Error> {
            for _ in 0..1000 {
                let mut bub = Bubble::new(&mut ns);  bub.push();
                black_box( eval_compiled_ref!(&instr, &slab, &mut bub) );
            }
            Ok(())
        })();
    });
}

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

    let mut ns = EmptyNamespace;
    let instr = parse(EXPR, &mut slab.ps).unwrap().from(&slab.ps).compile(&slab.ps, &mut slab.cs);

    b.iter(|| {
        (|| -> Result<(),al::Error> {
            for _ in 0..1000 {
                black_box(eval_compiled_ref!(&instr, &slab, &mut ns));
            }
            Ok(())
        })().unwrap();
    });
}

#[bench]
#[cfg(feature="unsafe-vars")]
#[allow(non_snake_case)]
fn preparse_precompile_eval_unsafe_100B(_:&mut Bencher) {
    let _ = (|| -> Result<(),al::Error> {
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

        let mut ns = EmptyNamespace;
        let instr = parse(EXPR, &mut slab.ps).unwrap().from(&slab.ps).compile(&slab.ps, &mut slab.cs);
        eprintln!("slab: {:?}  instr: {:?}", slab, instr);

        let start = std::time::Instant::now();
        //for _ in 0..100 {
            for _ in 0..1_000_000_000 {
                black_box(eval_compiled_ref!(&instr, &slab, &mut ns));
            }
        //}
        eprintln!("bench time: {}", start.elapsed().as_secs_f64());

        Ok(())
    })();
}

// #[bench]
// #[allow(non_snake_case)]
// fn preparse_compile_100M(_:&mut Bencher) {
//     let mut slab = Slab::new();
//     let expr_ref = parse(EXPR, &mut slab.ps).unwrap().from(&slab.ps);
//
//
//     let start = std::time::Instant::now();
//     for _ in 0..100 {
//         for _ in 0..1_000_000 {
//             slab.cs.clear();
//             black_box( expr_ref.compile(&slab.ps, &mut slab.cs) );
//         }
//     }
//     eprintln!("bench time: {}", start.elapsed().as_secs_f64());
// }

