// Here is how I run benchmarks:
//     while true; do echo "time: $(date +%s)"; cargo bench; done >bench.out
//     while true; do echo "time: $(date +%s)"; cargo bench --features unsafe-vars; done >bench.out
//     cat bench.out | awk -v "now=$(date +%s)" '$1=="time:"{when=$2}  $3=="..." && $4=="bench:" {gsub(/,/, "", $5); v=$5+0; if (t[$2]=="" || v<t[$2]){t[$2]=v; w[$2]=when;}} END{for (k in t) { printf "%-40s %9d ns/iter    %5ds ago\n",k,t[k],now-w[k] }}' | sort



// ---- Results (2019-12-04 on a 2012 laptop with Intel(R) Core(TM) i7-3610QM CPU @ 2.30GHz) ----
// al:
//     "(3 * (3 + 3) / 3)"
//     BTreeMap:
//     ez                                             728 ns/iter     1139s ago
//     native_1000x                                   333 ns/iter     1085s ago
//     parse_compile_eval                             871 ns/iter      981s ago
//     parse_eval_1000x                            633406 ns/iter     1192s ago
//     parse_eval_1x                                  631 ns/iter     1139s ago
//     parse_nsbubble_eval                            629 ns/iter     1245s ago
//     parser::internal_tests::spaces_1M            14289 ns/iter     1192s ago
//     preparse_eval_1000x                         202631 ns/iter     1139s ago
//     preparse_eval                                  203 ns/iter     1035s ago
//     preparse_precompile_eval                         0 ns/iter     1245s ago
//     preparse_precompile_eval_1000x                 750 ns/iter     1139s ago
//     preparse_precompile_nsbubble_eval                0 ns/iter     1245s ago
//     BTreeMap, --features unsafe-vars:
//     ez                                             739 ns/iter      784s ago
//     native_1000x                                   332 ns/iter     1536s ago
//     parse_compile_eval                             846 ns/iter     1589s ago
//     parse_eval_1000x                            624035 ns/iter     1322s ago
//     parse_eval_1x                                  617 ns/iter      879s ago
//     parse_nsbubble_eval                            620 ns/iter     1376s ago
//     parser::internal_tests::spaces_1M            14343 ns/iter      923s ago
//     preparse_eval_1000x                         201806 ns/iter      978s ago
//     preparse_eval                                  200 ns/iter      978s ago
//     preparse_precompile_eval_1000x                 956 ns/iter     1322s ago
//     preparse_precompile_eval                         1 ns/iter     1939s ago
//     preparse_precompile_eval_unsafe_1000x          958 ns/iter     1122s ago
//     preparse_precompile_nsbubble_eval                1 ns/iter     1939s ago
//
//     "3 * 3 - 3 / 3"
//     BTreeMap:
//     ez                                             501 ns/iter     1407s ago
//     native_1000x                                   332 ns/iter     1308s ago
//     parse_compile_eval                             656 ns/iter     1407s ago
//     parse_eval_1000x                            378329 ns/iter     1360s ago
//     parse_eval_1x                                  380 ns/iter     1360s ago
//     parse_nsbubble_eval                            385 ns/iter     1407s ago
//     parser::internal_tests::spaces_1M            14369 ns/iter      300s ago
//     preparse_eval_1000x                         100260 ns/iter     1407s ago
//     preparse_eval                                  103 ns/iter     1407s ago
//     preparse_precompile_eval                         0 ns/iter     1464s ago
//     preparse_precompile_eval_1000x                 751 ns/iter      500s ago
//     preparse_precompile_nsbubble_eval                0 ns/iter     1464s ago
//     BTreeMap, --features unsafe-vars:
//     ez                                             495 ns/iter      965s ago
//     native_1000x                                   333 ns/iter      318s ago
//     parse_compile_eval                             655 ns/iter      148s ago
//     parse_eval_1000x                            381228 ns/iter      318s ago
//     parse_eval_1x                                  379 ns/iter      207s ago
//     parse_nsbubble_eval                            382 ns/iter      148s ago
//     parser::internal_tests::spaces_1M            14348 ns/iter      260s ago
//     preparse_eval_1000x                         108394 ns/iter      148s ago
//     preparse_eval                                  108 ns/iter     1136s ago
//     preparse_precompile_eval_1000x                 962 ns/iter      207s ago
//     preparse_precompile_eval                         1 ns/iter     1136s ago
//     preparse_precompile_eval_unsafe_1000x          961 ns/iter     1136s ago
//     preparse_precompile_nsbubble_eval                1 ns/iter     1136s ago
//
//     "2 ^ 3 ^ 4"  = 2417851639229258300000000
//     BTreeMap:
//     ez                                             562 ns/iter     9060s ago
//     native_1000x                                   331 ns/iter     9915s ago
//     parse_compile_eval                             556 ns/iter     7902s ago
//     parse_eval_1000x                            394504 ns/iter     5822s ago
//     parse_eval_1x                                  390 ns/iter    11702s ago
//     parse_nsbubble_eval                            405 ns/iter    10551s ago
//     parser::internal_tests::spaces_1M            14243 ns/iter    12757s ago
//     preparse_eval_1000x                         198010 ns/iter      789s ago
//     preparse_eval                                  196 ns/iter    12154s ago
//     preparse_precompile_eval                         0 ns/iter    15141s ago
//     preparse_precompile_eval_1000x                 740 ns/iter     2059s ago
//     preparse_precompile_nsbubble_eval                0 ns/iter    15141s ago
//     BTreeMap, --features unsafe-vars:
//     ez                                             542 ns/iter     2367s ago
//     native_1000x                                   331 ns/iter     2145s ago
//     parse_compile_eval                             565 ns/iter     1819s ago
//     parse_eval_1000x                            393762 ns/iter     1819s ago
//     parse_eval_1x                                  390 ns/iter     2312s ago
//     parse_nsbubble_eval                            403 ns/iter     2479s ago
//     parser::internal_tests::spaces_1M            14666 ns/iter     2312s ago
//     preparse_eval_1000x                         193577 ns/iter     2367s ago
//     preparse_eval                                  193 ns/iter     2312s ago
//     preparse_precompile_eval_1000x                 958 ns/iter     2479s ago
//     preparse_precompile_eval                         1 ns/iter     2592s ago
//     preparse_precompile_eval_unsafe_1000x          956 ns/iter     2367s ago
//     preparse_precompile_nsbubble_eval                1 ns/iter     2592s ago
//
//     "x * 2"
//     BTreeMap:
//     ez                                             391 ns/iter      695s ago
//     native_1000x                                   715 ns/iter      975s ago
//     parse_compile_eval                             398 ns/iter      873s ago
//     parse_eval_1000x                            247199 ns/iter      975s ago
//     parse_eval_1x                                  247 ns/iter      922s ago
//     parse_nsbubble_eval                            360 ns/iter      561s ago
//     parser::internal_tests::spaces_1M            14308 ns/iter      765s ago
//     preparse_eval_1000x                          85273 ns/iter      873s ago
//     preparse_eval                                   84 ns/iter      837s ago
//     preparse_precompile_eval_1000x               27686 ns/iter      114s ago
//     preparse_precompile_eval                        26 ns/iter      532s ago
//     preparse_precompile_nsbubble_eval               26 ns/iter      922s ago
//     BTreeMap, --features unsafe-vars:
//     ez                                             407 ns/iter     2001s ago
//     native_1000x                                   713 ns/iter     2428s ago
//     parse_compile_eval                             383 ns/iter      782s ago
//     parse_eval_1000x                            246380 ns/iter     2587s ago
//     parse_eval_1x                                  246 ns/iter     1266s ago
//     parse_nsbubble_eval                            377 ns/iter     2938s ago
//     parser::internal_tests::spaces_1M            14650 ns/iter     1949s ago
//     preparse_eval_1000x                          85160 ns/iter      223s ago
//     preparse_eval                                   83 ns/iter     2938s ago
//     preparse_precompile_eval_1000x               26853 ns/iter     2587s ago
//     preparse_precompile_eval                        26 ns/iter      920s ago
//     preparse_precompile_eval_unsafe_1000x         7928 ns/iter      881s ago
//     preparse_precompile_nsbubble_eval               26 ns/iter     3241s ago
//
//     "sin(x)"
//     BTreeMap:
//     ez                                             496 ns/iter      721s ago
//     native_1000x                                 17395 ns/iter      916s ago
//     parse_compile_eval                             353 ns/iter     1242s ago
//     parse_eval_1000x                            351741 ns/iter     1844s ago
//     parse_eval_1x                                  351 ns/iter     1844s ago
//     parse_nsbubble_eval                            467 ns/iter     1844s ago
//     parser::internal_tests::spaces_1M            14443 ns/iter     1788s ago
//     preparse_eval_1000x                         115360 ns/iter     1844s ago
//     preparse_eval                                  115 ns/iter     1844s ago
//     preparse_precompile_eval_1000x               51224 ns/iter     1503s ago
//     preparse_precompile_eval                        50 ns/iter      956s ago
//     preparse_precompile_nsbubble_eval               51 ns/iter     1844s ago
//     BTreeMap, --features unsafe-vars:
//     ez                                             501 ns/iter     2303s ago
//     native_1000x                                 17529 ns/iter     1542s ago
//     parse_compile_eval                             361 ns/iter     2103s ago
//     parse_eval_1000x                            356303 ns/iter     1900s ago
//     parse_eval_1x                                  351 ns/iter     1900s ago
//     parse_nsbubble_eval                            461 ns/iter     1289s ago
//     preparse_eval_1000x                         113618 ns/iter      103s ago
//     preparse_eval                                  113 ns/iter     2899s ago
//     preparse_precompile_eval_1000x               51562 ns/iter     2394s ago
//     preparse_precompile_eval                        51 ns/iter     3217s ago
//     preparse_precompile_eval_unsafe_1000x        23039 ns/iter     1003s ago
//     preparse_precompile_nsbubble_eval               51 ns/iter     3173s ago
//
//     "(-z + (z^2 - 4*x*y)^0.5) / (2*x)"
//     BTreeMap:
//     ez                                            1877 ns/iter     1100s ago
//     native_1000x                                  5130 ns/iter      502s ago
//     parse_compile_eval                            2713 ns/iter      451s ago
//     parse_eval_1000x                           1590939 ns/iter      118s ago
//     parse_eval_1x                                 1562 ns/iter     1286s ago
//     parse_nsbubble_eval                           1856 ns/iter      622s ago
//     parser::internal_tests::spaces_1M            14393 ns/iter      402s ago
//     preparse_eval_1000x                         596206 ns/iter     1535s ago
//     preparse_eval                                  611 ns/iter     1286s ago
//     preparse_precompile_eval_1000x              233028 ns/iter      238s ago
//     preparse_precompile_eval                       231 ns/iter      592s ago
//     preparse_precompile_nsbubble_eval              232 ns/iter     1286s ago
//     BTreeMap, --features unsafe-vars:
//     ez                                            1862 ns/iter      383s ago
//     native_1000x                                  5118 ns/iter      435s ago
//     parse_compile_eval                            2751 ns/iter      157s ago
//     parse_eval_1000x                           1616482 ns/iter      435s ago
//     parse_eval_1x                                 1578 ns/iter      157s ago
//     parse_nsbubble_eval                           1866 ns/iter      383s ago
//     preparse_eval_1000x                         596268 ns/iter      383s ago
//     preparse_eval                                  596 ns/iter      256s ago
//     preparse_precompile_eval_1000x              228685 ns/iter      105s ago
//     preparse_precompile_eval                       229 ns/iter      721s ago
//     preparse_precompile_eval_unsafe_1000x       107844 ns/iter      808s ago
//     preparse_precompile_nsbubble_eval              228 ns/iter      383s ago
//
//     "((((87))) - 73) + (97 + (((15 / 55 * ((31)) + 35))) + (15 - (9)) - (39 / 26) / 20 / 91 + 27 / (33 * 26 + 28 - (7) / 10 + 66 * 6) + 60 / 35 - ((29) - (69) / 44 / (92)) / (89) + 2 + 87 / 47 * ((2)) * 83 / 98 * 42 / (((67)) * ((97))) / (34 / 89 + 77) - 29 + 70 * (20)) + ((((((92))) + 23 * (98) / (95) + (((99) * (41))) + (5 + 41) + 10) - (36) / (6 + 80 * 52 + (90))))"
//     BTreeMap:
//     ez                                           10752 ns/iter      664s ago
//     native_1000x                                   323 ns/iter     1636s ago
//     parse_compile_eval                           15124 ns/iter     1154s ago
//     parse_eval_1000x                          10807702 ns/iter      664s ago
//     parse_eval_1x                                10712 ns/iter     1399s ago
//     parse_nsbubble_eval                          10782 ns/iter     1307s ago
//     preparse_eval_1000x                        3204041 ns/iter     1375s ago
//     preparse_eval                                 3150 ns/iter     1555s ago
//     preparse_precompile_eval                         0 ns/iter     1813s ago
//     preparse_precompile_eval_1000x                 726 ns/iter     1576s ago
//     preparse_precompile_nsbubble_eval                0 ns/iter     1813s ago
//     BTreeMap, --features unsafe-vars:
//     ez                                           11079 ns/iter      897s ago
//     native_1000x                                   326 ns/iter      897s ago
//     parse_compile_eval                           15707 ns/iter      863s ago
//     parse_eval_1000x                          11286841 ns/iter      897s ago
//     parse_eval_1x                                11232 ns/iter      897s ago
//     parse_nsbubble_eval                          11394 ns/iter      897s ago
//     preparse_eval_1000x                        3216461 ns/iter      148s ago
//     preparse_eval                                 3238 ns/iter      148s ago
//     preparse_precompile_eval_1000x                 944 ns/iter      897s ago
//     preparse_precompile_eval                         1 ns/iter     1191s ago
//     preparse_precompile_eval_unsafe_1000x          940 ns/iter      925s ago
//     preparse_precompile_nsbubble_eval                1 ns/iter     1191s ago
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

use al::{parse, Compiler, Evaler, Slab, EmptyNamespace, FlatNamespace, ScopedNamespace, ez_eval, eval_instr_ref_or_panic};

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
//static EXPR : &'static str = "(-z + (z^2 - 4*x*y)^0.5) / (2*x)";
static EXPR : &'static str = "((((87))) - 73) + (97 + (((15 / 55 * ((31)) + 35))) + (15 - (9)) - (39 / 26) / 20 / 91 + 27 / (33 * 26 + 28 - (7) / 10 + 66 * 6) + 60 / 35 - ((29) - (69) / 44 / (92)) / (89) + 2 + 87 / 47 * ((2)) * 83 / 98 * 42 / (((67)) * ((97))) / (34 / 89 + 77) - 29 + 70 * (20)) + ((((((92))) + 23 * (98) / (95) + (((99) * (41))) + (5 + 41) + 10) - (36) / (6 + 80 * 52 + (90))))";

#[bench]
fn native_1000x(bencher:&mut Bencher) {
    // Silence compiler warnings about unused imports:
    let _ = EmptyNamespace;  let _ = FlatNamespace::new(|_,_| None);


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
    let mut vars=BTreeMap::new();
    vars.insert("x".to_string(),1.0);
    vars.insert("y".to_string(),2.0);
    vars.insert("z".to_string(),3.0);

    b.iter(|| {
        black_box(ez_eval(EXPR, &mut vars).unwrap());
    });
}

#[bench]
fn parse_eval_1x(b:&mut Bencher) {
    let mut slab = Slab::new();
    let mut ns = Namespace!();

    b.iter(|| {
        black_box(parse({slab.clear(); &mut slab.ps}, EXPR).unwrap().from(&slab.ps).eval(&slab, &mut ns).unwrap());
    });
}

#[bench]
fn parse_nsbubble_eval(b:&mut Bencher) {
    let mut slab = Slab::new();
    let mut ns = ScopedNamespace::new(evalcb);

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
    let mut ns = Namespace!();

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
//     let mut ns = Namespace!();
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
    let mut ns = Namespace!();
    let expr_ref = parse(&mut slab.ps, EXPR).unwrap().from(&slab.ps);

    b.iter(|| {
        black_box(expr_ref.eval(&slab, &mut ns).unwrap());
    });
}

#[bench]
fn preparse_eval_1000x(b:&mut Bencher) {
    let mut slab = Slab::new();
    let mut ns = Namespace!();
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
    let mut ns = Namespace!();

    b.iter(|| {
        black_box(parse({slab.clear(); &mut slab.ps}, EXPR).unwrap().from(&slab.ps).compile(&slab.ps, &mut slab.cs).eval(&slab, &mut ns).unwrap());
    });
}

#[bench]
fn preparse_precompile_eval(b:&mut Bencher) {
    let mut slab = Slab::new();
    let mut ns = Namespace!();
    let instr = parse(&mut slab.ps, EXPR).unwrap().from(&slab.ps).compile(&slab.ps, &mut slab.cs);

    b.iter(|| {
        black_box(eval_instr_ref_or_panic!(&instr, &slab, &mut ns));
    });
}

#[bench]
fn preparse_precompile_nsbubble_eval(b:&mut Bencher) {
    let mut slab = Slab::new();
    let mut ns = Namespace!();
    let instr = parse(&mut slab.ps, EXPR).unwrap().from(&slab.ps).compile(&slab.ps, &mut slab.cs);

    b.iter(|| {
        black_box(eval_instr_ref_or_panic!(&instr, &slab, &mut ns));
    });
}

#[bench]
fn preparse_precompile_eval_1000x(b:&mut Bencher) {
    let mut slab = Slab::new();
    let mut ns = Namespace!();
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
//     let mut ns = Namespace!();
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

    let mut ns = EmptyNamespace;
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
//     let mut ns = EmptyNamespace;
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

