// usage: rlwrap cargo run --release --example repl

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
            eprintln!("Entered scope #{}", ns_stack.len());
            continue;
        } else if pieces[0] == "pop" {
            let mut return_value = std::f64::NAN;  let mut has_return_value = false;
            if let Some(v) = ns_stack.last().unwrap().get(&ans_key) {
                return_value = *v;
                has_return_value = true;
            }

            ns_stack.pop();
            eprintln!("Exited scope #{}", ns_stack.len()+1);
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

