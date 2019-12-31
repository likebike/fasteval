//! usage: rlwrap cargo run --release --example repl
//!
//! Example Session:
//!
//! github.com/fasteval$ rlwrap cargo run --release --example repl
//!     Finished release [optimized] target(s) in 0.01s
//!      Running `target/release/examples/repl`
//! >>> print("Hello fasteval", 1, 2, 3)
//! Hello fasteval 1 2 3
//! 3
//! >>> _ + 1
//! 4
//! >>> _ + 1
//! 5
//! >>> _ * 2
//! 10
//! >>> _ ^ 0.5
//! 3.1622776601683795
//! >>> let a = 1
//! 1
//! >>> let b = a + 1
//! 2
//! >>> let c = a + b * 3
//! 7
//! >>> a + b + c
//! 10
//! >>> push
//! Entered scope[1]
//! >>> let b = b + 10
//! 12
//! >>> a + b + c
//! 20
//! >>> pop
//! Exited scope[1]
//! >>> a + b + c
//! 10
//! >>> 1+2*3/4^5%6 + log(100K) + log(e(),100) + [3*(3-3)/3] + (2<3) && 1.23
//! 1.23
//! >>> 1+2*3/4^5%6 + print("log(100K) =",log(100K)) + log(e(),100) + [3*(3-3)/3] + (2<3) && 1.23
//! log(100K) = 5
//! 1.23



use fasteval::Evaler;  // Import this trait for '.eval()' functionality.
use fasteval::{parse, Slab};

use std::collections::BTreeMap;
use std::io::{self, BufRead, Write};

fn main() {
    repl();
}

fn repl() {
    let mut slab = Slab::new();
    let mut ns_stack = vec![BTreeMap::new()];

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    loop {
        eprint!(">>> ");  io::stderr().flush().unwrap();

        let mut ans_key = "_".to_string();

        let line = match lines.next() {
            Some(res) => res.unwrap(),
            None => break,
        };
        let mut line = line.trim().to_string();
        if line == "" { continue; }

        let pieces : Vec<&str> = line.split_whitespace().collect();
        if pieces[0] == "let" {
            if pieces.len()<4 || pieces[2]!="=" {
                eprintln!("incorrect 'let' syntax.  Should be: let x = ...");
                continue;
            }
            ans_key = pieces[1].to_string();
            line = pieces[3..].join(" ");
        } else if pieces[0] == "push" {
            ns_stack.push(BTreeMap::new());
            eprintln!("Entered scope[{}]", ns_stack.len()-1);
            continue;
        } else if pieces[0] == "pop" {
            let mut return_value = std::f64::NAN;  let mut has_return_value = false;
            if let Some(v) = ns_stack.last().unwrap().get(&ans_key) {
                return_value = *v;
                has_return_value = true;
            }

            ns_stack.pop();
            eprintln!("Exited scope[{}]", ns_stack.len());
            if ns_stack.is_empty() { ns_stack.push(BTreeMap::new()); }  // All scopes have been removed.  Add a new one.

            if has_return_value {
                ns_stack.last_mut().unwrap().insert(ans_key, return_value);
            }

            continue;
        }

        let expr_ref = match parse(&line, &mut slab.ps) {
            Ok(expr_i) => slab.ps.get_expr(expr_i),
            Err(err) => {
                eprintln!("parse error: {}", err);
                continue;
            }
        };

        let ans = match expr_ref.eval(&slab, &mut ns_stack) {
            Ok(val) => val,
            Err(err) => {
                eprintln!("eval error: {}", err);
                continue;
            }
        };

        println!("{}", ans);
        ns_stack.last_mut().unwrap().insert(ans_key, ans);
    }

    println!();
}

