// usage: cargo run --release --example repl

use al::{Parser, Slab, EvalNS};

use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::cell::RefCell;

fn main() {
    repl();
}

fn repl() {
    let mut parser = Parser::new();
    let mut slab = Slab::new();
    let ns_map = RefCell::new(HashMap::new());
    let mut ns = EvalNS::new(|name, _args| {
        ns_map.borrow().get(name).cloned()
    });

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut ans_i = 0usize;
    loop {
        print!(">>> ");  io::stdout().flush().unwrap();

        let mut ans_key = "ans_".to_string();  ans_key.push_str(&ans_i.to_string());  let mut ans_key_is_orig = true;

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

            ans_key = pieces[1].to_string();  ans_key_is_orig = false;
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

        println!("let {} = {}", ans_key, ans);
        {
            let mut ns_map = ns_map.borrow_mut();
            ns_map.insert(ans_key, ans);
            ns_map.insert("_".to_string(), ans);
        }

        if ans_key_is_orig { ans_i += 1; }
    }
    println!();
}

