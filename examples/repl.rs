// usage: rlwrap cargo run --release --example repl

use al::{Parser, Slab, EvalNS};

use std::collections::BTreeMap;
use std::io::{self, BufRead, Write};
use std::cell::RefCell;

fn main() {
    repl();
}

fn repl() {
    let mut parser = Parser::new();
    let mut slab = Slab::new();
    let ns_map = RefCell::new(BTreeMap::new());
    let mut ns = EvalNS::new(|name, _args| {
        ns_map.borrow().get(name).cloned()
    });

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
        }

        let expr_ref = match parser.parse(&mut slab.ps, &line) {
            Ok(expr_i) => slab.ps.get_expr(expr_i),
            Err(err) => {
                eprintln!("parse error: {}", err);
                continue;
            }
        };

        let ans = match ns.eval_bubble(&slab, expr_ref) {
            Ok(val) => val,
            Err(err) => {
                eprintln!("eval error: {}", err);
                continue;
            }
        };

        println!("{}", ans);
        {
            let mut ns_map = ns_map.borrow_mut();
            ns_map.insert(ans_key, ans);
        }
    }

    println!();
}

