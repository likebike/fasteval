# al (short for "algebra")
Fast evaluation of algebraic expressions



// TODO:
//   [x] Port all tests
//   [x] NaN, inf, -inf are valid.  problem?  no because my parser thinks they're vars.
//   [x] e() pi() ... or should i prefer variables?  Provide a default layer of variables?  Vars don't work well with TV symbols.
//   [x] Profile, boost critical sections.
//   [x] optimize the peek/read process -- be able to read N bytes if we peek successfully.
//   [x] optimize after parse
//   [x] custom functions  (i.e. Variables With Arguments)
//   [x] REPL Example with Variables
//   [x] Copy smart tests from other libs.
//   [x] Reduce work: Parser obj --> functions.  EvalNS --> BTreeMap.
//   [x] #[inline] last, using profile as a guide.
//   [ ] Review #[inline]s -- can I convert to macros?
//   [ ] More examples:  UnsafeVar.  Callbacks with user-defined vars and functions + mutation with RefCell.
//   [ ] Readme
//   [ ] Documentation
//
//   [ ] sprintf.  Fake it for now: %s, %#.#f
