use crate::slab::Slab;
use crate::grammar::{ExpressionI, ValueI,
                     Expression,
                     ExprPair,
                     Value::{self, EConstant, EVariable, EUnaryOp, ECallable},
                     Constant,
                     Variable,
                     UnaryOp::{self, EPos, ENeg, EParens, ENot},
                     BinaryOp::{self, EPlus, EMinus, EMul, EDiv, EMod, EExp, ELT, ELTE, EEQ, ENE, EGTE, EGT, EOR, EAND},
                     Callable::{self, EFunc, EPrintFunc, EEvalFunc},
                     Func::{self, EFuncInt, EFuncCeil, EFuncFloor, EFuncAbs, EFuncLog, EFuncRound, EFuncMin, EFuncMax, EFuncE, EFuncPi, EFuncSin, EFuncCos, EFuncTan, EFuncASin, EFuncACos, EFuncATan, EFuncSinH, EFuncCosH, EFuncTanH},
                     PrintFunc,
                     ExpressionOrString::{self, EExpr, EStr},
                     EvalFunc,
                     KWArg};

use kerr::KErr;
use stacked::{SVec, SVec8, SVec16, SString32, SString256};


// Vec seems really inefficient to me because remove() does not just increment the internal pointer -- it shifts data all around.  There's also split_* methods but they seem to be designed to return new Vecs, not modify self.
// Just use slices instead, which I know will be very efficient:
fn peek(bs:&[u8], skip:usize) -> Option<u8> {
    if bs.len() > skip { Some(bs[skip])
    } else { None }
}
fn is_at_eof(bs:&[u8]) -> bool { bs.len() == 0 }
fn peek_is(bs:&[u8], skip:usize, val:u8) -> bool {
    match peek(bs,skip) {
        None => false,
        Some(b) => b==val,
    }
}
fn peek_word_ci(bs:&[u8], skip:usize, word:&[u8]) -> bool {  // 'ci' = case insensitive
    #[allow(non_snake_case)]
    for (i,B) in word.iter().enumerate() {
        #[allow(non_snake_case)]
        let B = B.to_ascii_lowercase();
        match peek(bs,skip+i) {
            Some(b) => {
                let b = b.to_ascii_lowercase();
                if b!=B { return false; }
            }
            None => return false,
        }
    }
    true
}
fn peek_func(bs:&[u8], skip:usize, name:&[u8]) -> bool {
    if peek_word_ci(bs,skip,name) {
        let mut post_name_spaces = 0;
        while let Some(b) = peek(bs,skip+name.len()+post_name_spaces) {
            if !is_space(b) { break; }
            post_name_spaces+=1;
        }
        name.len()>0 && peek(bs,skip+name.len()+post_name_spaces)==Some(b'(')
    } else { false }
}

fn read(bs:&mut &[u8]) -> Result<u8, KErr> {
    if bs.len() > 0 {
        let b = bs[0];
        *bs = &bs[1..];
        Ok(b)
    } else { Err(KErr::new("EOF")) }
}
fn read_word_ci(bs:&mut &[u8], word:&[u8]) -> Result<(), KErr> {
    #[allow(non_snake_case)]
    for B in word.iter() {
        #[allow(non_snake_case)]
        let B = B.to_ascii_lowercase();
        match read(bs) {
            Ok(b) => {
                let bl = b.to_ascii_lowercase();
                if bl!=B { return Err(KErr::new(&format!("unexpected '{}' when reading '{}'",b as char,std::str::from_utf8(word).map_err(|_| KErr::new("Utf8Error"))?))) }
            }
            Err(e) => { return Err(e.pre(&format!("read_word_ci({})",std::str::from_utf8(word).map_err(|_| KErr::new("Utf8Error"))?))) }
        }
    }
    Ok(())
}
fn read_func(bs:&mut &[u8], name:&[u8]) -> Result<(), KErr> {
    read_word_ci(bs,name)?;
    space(bs);
    if read(bs)?==b'(' { Ok(())
    } else { Err(KErr::new("expected '('")) }
}

fn is_space(b:u8) -> bool {
    match b {
    b' ' | b'\t' | b'\r' | b'\n' => true,
    _ => false,
    }
}
fn space(bs:&mut &[u8]) {
    while let Some(b) = peek(bs,0) {
        if !is_space(b) { break }
        let _ = read(bs);
    }
}



pub struct Parser<'a> {
    pub is_const_byte:Option<&'a dyn Fn(u8,usize)->bool>,
    pub is_var_byte  :Option<&'a dyn Fn(u8,usize)->bool>,  // Until proven otherwise, assume that function names follow the same rules as vars.
}

impl<'a> Parser<'a> {
    fn default_is_const_byte(b:u8, i:usize) -> bool {
        if b'0'<=b && b<=b'9' || b==b'.' { return true }
        if i>0 && ( b==b'k' || b==b'K' || b==b'M' || b==b'G' || b==b'T' ) { return true }
        return false
    }
    fn default_is_var_byte(b:u8, i:usize) -> bool {
        (b'A'<=b && b<=b'Z') || (b'a'<=b && b<=b'z') || b==b'_' || (i>0 && b'0'<=b && b<=b'9')
    }

    fn call_is_const_byte(&self, bo:Option<u8>, i:usize) -> bool {
        match bo {
            Some(b) => match self.is_const_byte {
                Some(f) => f(b,i),
                None => Parser::default_is_const_byte(b,i),
            }
            None => false
        }
    }
    fn call_is_var_byte(&self, bo:Option<u8>, i:usize) -> bool {
        match bo {
            Some(b) => match self.is_var_byte {
                Some(f) => f(b,i),
                None => Parser::default_is_var_byte(b,i),
            }
            None => false
        }
    }
    // Re-use var logic until proven otherwise:
    fn call_is_func_byte(&self, bo:Option<u8>, i:usize) -> bool {
        self.call_is_var_byte(bo,i)
    }

    pub fn parse<'b>(&self, slab:&'b Slab, s:&str) -> Result<&'b Expression, KErr> {
        let mut bs = s.as_bytes();
        let expr_i = self.read_expression(slab, &mut bs, true)?;
        Ok(slab.get_expr(expr_i))
    }

    fn read_expression(&self, slab:&Slab, bs:&mut &[u8], expect_eof:bool) -> Result<ExpressionI, KErr> {
        let first = self.read_value(slab,bs).map_err(|e| e.pre("read_value"))?;
        let pairs = SVec16::<ExprPair>::new();
        while self.peek_binaryop(bs) {
            let bop = self.read_binaryop(bs).map_err(|e| e.pre("read_binaryop"))?;
            let val = self.read_value(slab,bs).map_err(|e| e.pre("read_value"))?;
            pairs.push(ExprPair(bop,val))?;
        }
        space(bs);
        if expect_eof && !is_at_eof(bs) { return Err(KErr::new("unparsed tokens remaining")); }
        Ok(slab.push_expr(Expression{first, pairs})?)
    }

    fn read_value(&self, slab:&Slab, bs:&mut &[u8]) -> Result<Value, KErr> {
        if self.peek_const(bs) {
            return self.read_const(bs).map(|c| EConstant(c));
        }
        if self.peek_unaryop(bs) {
            return self.read_unaryop(slab,bs).map(|u| EUnaryOp(u));
        }
        if self.peek_callable(bs) {
            return self.read_callable(slab,bs).map(|c| ECallable(c));
        }
        if self.peek_var(bs) {  // Should go last -- don't mask callables.
            return self.read_var(bs).map(|v| EVariable(v));
        }
        Err(KErr::new("invalid value"))
    }

    fn peek_const(&self, bs:&mut &[u8]) -> bool {
        space(bs);
        self.call_is_const_byte(peek(bs,0),0)
    }
    fn read_const(&self, bs:&mut &[u8]) -> Result<Constant, KErr> {
        space(bs);

        let mut buf = SString256::new();
        while self.call_is_const_byte(peek(bs,0),buf.len()) {
            buf.push(read(bs)?)?;
        }

        let mut multiple = 1.0;
        let buflen = buf.len();
        if buflen>0 {
            match buf[buflen-1] {
                b'k' | b'K' => {      multiple=1_000.0; buf.pop(); }
                b'M' => {         multiple=1_000_000.0; buf.pop(); }
                b'G' => {     multiple=1_000_000_000.0; buf.pop(); }
                b'T' => { multiple=1_000_000_000_000.0; buf.pop(); }
                _ => {}
            }
        }

        let bufstr = buf.as_str()?;
        let val = bufstr.parse::<f64>().map_err(|_| {
            KErr::new("parse<f64> error").pre(bufstr)
        })?;
        Ok(Constant(val*multiple))
    }

    fn peek_var(&self, bs:&mut &[u8]) -> bool {
        space(bs);
        self.call_is_var_byte(peek(bs,0),0)
    }
    fn read_var(&self, bs:&mut &[u8]) -> Result<Variable, KErr> {
        space(bs);

        let buf = SString32::new();
        while self.call_is_var_byte(peek(bs,0),buf.len()) {
            buf.push(read(bs)?)?;
        }

        Ok(Variable(buf))
    }

    fn peek_unaryop(&self, bs:&mut &[u8]) -> bool {
        space(bs);
        match peek(bs,0) {
            Some(b'+') | Some(b'-') | Some(b'(') | Some(b'!') => true,
            _ => false,
        }
    }
    fn read_unaryop(&self, slab:&Slab, bs:&mut &[u8]) -> Result<UnaryOp, KErr> {
        space(bs);
        match read(bs)? {
            b'+' => {
                let v = self.read_value(slab,bs)?;
                Ok(EPos(slab.push_val(v)?))
            }
            b'-' => {
                let v = self.read_value(slab,bs)?;
                Ok(ENeg(slab.push_val(v)?))
            }
            b'!' => {
                let v = self.read_value(slab,bs)?;
                Ok(ENot(slab.push_val(v)?))
            }
            b'(' => {
                let xi = self.read_expression(slab,bs,false)?;
                space(bs);
                if read(bs)? != b')' { return Err(KErr::new("Expected ')'")) }
                Ok(EParens(xi))
            }
            _ => Err(KErr::new("invalid unaryop")),
        }
    }

    fn peek_binaryop(&self, bs:&mut &[u8]) -> bool {
        space(bs);
        match peek(bs,0) {
            None => false,
            Some(b) => match b {
                b'+'|b'-'|b'*'|b'/'|b'%'|b'^'|b'<'|b'>' => true,
                b'=' => peek_is(bs,1,b'='),
                b'!' => peek_is(bs,1,b'='),
                b'o' => peek_is(bs,1,b'r'),
                b'a' => peek_is(bs,1,b'n') && peek_is(bs,2,b'd'),
                _ => false,
            }
        }
    }
    fn read_binaryop(&self, bs:&mut &[u8]) -> Result<BinaryOp, KErr> {
        let err = KErr::new("illegal binaryop");
        space(bs);
        match read(bs)? {
            b'+' => Ok(EPlus),
            b'-' => Ok(EMinus),
            b'*' => Ok(EMul),
            b'/' => Ok(EDiv),
            b'%' => Ok(EMod),
            b'^' => Ok(EExp),
            b'<' => if peek_is(bs,0,b'=') { read(bs)?; Ok(ELTE)
                    } else { Ok(ELT) },
            b'>' => if peek_is(bs,0,b'=') { read(bs)?; Ok(EGTE)
                    } else { Ok(EGT) },
            b'=' => if peek_is(bs,0,b'=') { read(bs)?; Ok(EEQ)
                    } else { Err(err) },
            b'!' => if peek_is(bs,0,b'=') { read(bs)?; Ok(ENE)
                    } else { Err(err) },
            b'o' => if peek_is(bs,0,b'r') { read(bs)?; Ok(EOR)
                    } else { Err(err) },
            b'a' => if peek_is(bs,0,b'n') && peek_is(bs,1,b'd') { read(bs)?; read(bs)?; Ok(EAND)
                    } else { Err(err) },
            _ => Err(err),
        }
    }

    fn peek_callable(&self, bs:&mut &[u8]) -> bool {
        self.peek_func(bs) || self.peek_printfunc(bs) || self.peek_evalfunc(bs)
    }
    fn read_callable(&self, slab:&Slab, bs:&mut &[u8]) -> Result<Callable, KErr> {
        if self.peek_printfunc(bs) {
            return self.read_printfunc(slab,bs).map(|f| EPrintFunc(f));
        }
        if self.peek_evalfunc(bs) {
            return self.read_evalfunc(slab,bs).map(|f| EEvalFunc(f));
        }
        if self.peek_func(bs) {
            return self.read_func(slab,bs).map(|f| EFunc(f));
        }
        Err(KErr::new("invalid callable"))
    }

    fn peek_func(&self, bs:&mut &[u8]) -> bool {
        space(bs);

        let mut name_len=0; let mut post_name_spaces=0;
        while self.call_is_func_byte(peek(bs,name_len),name_len) { name_len+=1; }
        while let Some(b) = peek(bs,name_len+post_name_spaces) {
            if !is_space(b) { break; }
            post_name_spaces+=1;
        }
        name_len>0 && peek(bs,name_len+post_name_spaces)==Some(b'(')
    }
    fn read_func(&self, slab:&Slab, bs:&mut &[u8]) -> Result<Func, KErr> {
        space(bs);

        let fname = SString32::new();
        while self.call_is_func_byte(peek(bs,0),fname.len()) {
            fname.push(read(bs)?.to_ascii_lowercase())?;
        }

        space(bs);

        if let Ok(b'(') = read(bs) {}
        else { return Err(KErr::new("expected '('")); }

        let mut args = SVec8::<ExpressionI>::new();

        loop {
            space(bs);
            match peek(bs,0) {
                Some(b) => {
                    if b==b')' {
                        read(bs)?;
                        break;
                    }
                }
                None => return Err(KErr::new("Reached end of input while parsing function")),
            }
            if args.len()>0 {
                match read(bs) {
                    Ok(b',') | Ok(b';') => {
                        // I accept ',' or ';' because the TV API disallows the ',' char in symbols... so I'm using ';' as a compromise.
                    }
                    _ => return Err(KErr::new("expected ',' or ';'")),
                }
            }
            args.push(self.read_expression(slab,bs,false).map_err(|e| e.pre("read_expression"))?)?;
        }

        match fname.as_str() {
            Ok("int") => {
                if args.len()==1 { Ok(EFuncInt(args.pop()))
                } else { Err(KErr::new("expected one arg")) }
            }
            Ok("ceil") => {
                if args.len()==1 { Ok(EFuncCeil(args.pop()))
                } else { Err(KErr::new("expected one arg")) }
            }
            Ok("floor") => {
                if args.len()==1 { Ok(EFuncFloor(args.pop()))
                } else { Err(KErr::new("expected one arg")) }
            }
            Ok("abs") => {
                if args.len()==1 { Ok(EFuncAbs(args.pop()))
                } else { Err(KErr::new("expected one arg")) }
            }
            Ok("log") => {
                if args.len()==1 { Ok(EFuncLog{base:None, expr:args.pop()})
                } else if args.len()==2 {
                    let expr = args.pop();
                    Ok(EFuncLog{base:Some(args.pop()), expr:expr})
                } else { Err(KErr::new("expected log(x) or log(base,x)")) }
            }
            Ok("round") => {
                if args.len()==1 { Ok(EFuncRound{modulus:None, expr:args.pop()})
                } else if args.len()==2 {
                    let expr = args.pop();
                    Ok(EFuncRound{modulus:Some(args.pop()), expr:expr})
                } else { Err(KErr::new("expected round(x) or round(modulus,x)")) }
            }
            Ok("min") => {
                if args.len()>0 {
                    let first = args.remove(0);
                    Ok(EFuncMin{first:first, rest:args})
                } else { Err(KErr::new("expected one or more args")) }
            }
            Ok("max") => {
                if args.len()>0 {
                    let first = args.remove(0);
                    Ok(EFuncMax{first:first, rest:args})
                } else { Err(KErr::new("expected one or more args")) }
            }

            Ok("e") => {
                if args.len()==0 { Ok(EFuncE)
                } else { Err(KErr::new("expected no args")) }
            }
            Ok("pi") => {
                if args.len()==0 { Ok(EFuncPi)
                } else { Err(KErr::new("expected no args")) }
            }

            Ok("sin") => {
                if args.len()==1 { Ok(EFuncSin(args.pop()))
                } else { Err(KErr::new("expected one arg")) }
            }
            Ok("cos") => {
                if args.len()==1 { Ok(EFuncCos(args.pop()))
                } else { Err(KErr::new("expected one arg")) }
            }
            Ok("tan") => {
                if args.len()==1 { Ok(EFuncTan(args.pop()))
                } else { Err(KErr::new("expected one arg")) }
            }
            Ok("asin") => {
                if args.len()==1 { Ok(EFuncASin(args.pop()))
                } else { Err(KErr::new("expected one arg")) }
            }
            Ok("acos") => {
                if args.len()==1 { Ok(EFuncACos(args.pop()))
                } else { Err(KErr::new("expected one arg")) }
            }
            Ok("atan") => {
                if args.len()==1 { Ok(EFuncATan(args.pop()))
                } else { Err(KErr::new("expected one arg")) }
            }
            Ok("sinh") => {
                if args.len()==1 { Ok(EFuncSinH(args.pop()))
                } else { Err(KErr::new("expected one arg")) }
            }
            Ok("cosh") => {
                if args.len()==1 { Ok(EFuncCosH(args.pop()))
                } else { Err(KErr::new("expected one arg")) }
            }
            Ok("tanh") => {
                if args.len()==1 { Ok(EFuncTanH(args.pop()))
                } else { Err(KErr::new("expected one arg")) }
            }

            Ok(_) => Err(KErr::new(&format!("undefined function: {}",fname))),
            Err(e) => Err(e.pre("invalid function name")),
        }
    }

    fn peek_printfunc(&self, bs:&mut &[u8]) -> bool { peek_func(bs, 0, b"print") }
    fn read_printfunc(&self, slab:&Slab, bs:&mut &[u8]) -> Result<PrintFunc, KErr> {
        read_func(bs, b"print")?;

        let args = SVec16::<ExpressionOrString>::new();
        loop {
            space(bs);
            match peek(bs,0) {
                Some(b) => {
                    if b==b')' {
                        read(bs)?;
                        break;
                    }
                }
                None => { return Err(KErr::new("reached end of inupt while parsing printfunc")) }
            }
            if args.len()>0 {
                match read(bs) {
                    Ok(b',') | Ok(b';') => {}
                    _ => { return Err(KErr::new("expected ',' or ';'")) }
                }
            }
            args.push(self.read_expressionorstring(slab,bs)?)?;
        }

        Ok(PrintFunc(args))
    }

    fn peek_evalfunc(&self, bs:&mut &[u8]) -> bool { peek_func(bs, 0, b"eval") }
    fn read_evalfunc(&self, slab:&Slab, bs:&mut &[u8]) -> Result<EvalFunc, KErr> {
        read_func(bs, b"eval")?;

        let eval_expr = self.read_expression(slab,bs,false)?;
        let kwargs = SVec16::<KWArg>::new();
        fn kwargs_has(kwargs:&SVec16<KWArg>, name:&Variable) -> bool {
            for kwarg in kwargs {
                if kwarg.name==*name { return true; }
            }
            false
        }

        loop {
            space(bs);
            match peek(bs,0) {
                Some(b) => {
                    if b==b')' {
                        read(bs)?;   
                        break;
                    }
                }
                None => { return Err(KErr::new("reached end of input while parsing evalfunc")) }
            }
            match read(bs) {
                Ok(b',') | Ok(b';') => {}
                _ => { return Err(KErr::new("expected ',' or ';'")) }
            }
            let name = self.read_var(bs)?;
            space(bs);
            if let Ok(b'=') = read(bs) {
            } else { return Err(KErr::new("expected '='")) }
            let expr = self.read_expression(slab,bs,false)?;

            if kwargs_has(&kwargs,&name) { return Err(KErr::new(&format!("already defined: {}",name.0))) }
            kwargs.push(KWArg{name, expr})?;
        }

        Ok(EvalFunc{expr:eval_expr, kwargs:kwargs})
    }

    fn read_expressionorstring(&self, slab:&Slab, bs:&mut &[u8]) -> Result<ExpressionOrString, KErr> {
        if self.peek_string(bs) { Ok(EStr(self.read_string(bs)?))
        } else { Ok(EExpr(self.read_expression(slab,bs,false)?)) }
    }

    fn peek_string(&self, bs:&mut &[u8]) -> bool {
        space(bs);
        peek_is(bs,0,b'"')
    }
    fn read_string(&self, bs:&mut &[u8]) -> Result<SString256, KErr> {
        space(bs);

        match read(bs) {
            Ok(b) => {
                if b!=b'"' { return Err(KErr::new(r#"expected '"'"#)) }
            }
            Err(e) => { return Err(e.pre("read_string")) }
        }

        let buf = SString256::new();
        loop {
            let b = read(bs)?;
            if b==b'"' { break; }
            buf.push(b)?;
        }

        //String::from_utf8(buf).map_err(|_| KErr::new("Utf8Error"))
        Ok(buf)
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn util() {
        match (|| -> Result<(),KErr> {
            let bsarr = [1,2,3];
            let bs = &mut &bsarr[..];

            assert_eq!(peek(bs,0), Some(1));
            assert_eq!(peek(bs,1), Some(2));
            assert_eq!(peek(bs,2), Some(3));
            assert_eq!(peek(bs,3), None);

            assert_eq!(read(bs)?, 1);
            assert_eq!(read(bs)?, 2);
            assert_eq!(read(bs)?, 3);
            match read(bs).err() {
                Some(KErr{..}) => {}  // Can I improve this so I can match the "EOF" ?
                None => panic!("I expected an EOF")
            }

            Ok(())
        })() {
            Ok(_) => {}
            Err(_) => {
                unimplemented!();
            }
        }

        assert!(is_at_eof(&[]));
        assert!(!is_at_eof(&[1]));
        assert!(is_at_eof(b""));
        assert!(!is_at_eof(b"x"));

        assert!(is_space(b' '));
        assert!(is_space(b'\t'));
        assert!(is_space(b'\r'));
        assert!(is_space(b'\n'));
        assert!(!is_space(b'a'));
        assert!(!is_space(b'1'));
        assert!(!is_space(b'.'));

        {
            let bsarr = b"  abc 123   ";
            let bs = &mut &bsarr[..];
            space(bs);
            assert_eq!(bs, b"abc 123   ");
        }
    }
}

