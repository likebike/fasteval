use crate::grammar::{Expression,
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
use crate::error::Error;
use crate::stackvec::{StackVec, StackVec16};
use crate::slab::Slab;


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

fn read(bs:&mut &[u8]) -> Result<u8, Error> {
    if bs.len() > 0 {
        let b = bs[0];
        *bs = &bs[1..];
        Ok(b)
    } else { Err(Error::new("EOF")) }
}
fn read_word_ci(bs:&mut &[u8], word:&[u8]) -> Result<(), Error> {
    #[allow(non_snake_case)]
    for B in word.iter() {
        #[allow(non_snake_case)]
        let B = B.to_ascii_lowercase();
        match read(bs) {
            Ok(b) => {
                let bl = b.to_ascii_lowercase();
                if bl!=B { return Err(Error::new(&format!("unexpected '{}' when reading '{}'",b as char,std::str::from_utf8(word).map_err(|_| Error::new("Utf8Error"))?))) }
            }
            Err(e) => { return Err(e.pre(&format!("read_word_ci({})",std::str::from_utf8(word).map_err(|_| Error::new("Utf8Error"))?))) }
        }
    }
    Ok(())
}
fn read_func(bs:&mut &[u8], name:&[u8]) -> Result<(), Error> {
    read_word_ci(bs,name)?;
    space(bs);
    if read(bs)?==b'(' { Ok(())
    } else { Err(Error::new("expected '('")) }
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

    pub fn parse(&self, slab:&Slab, s:&str) -> Result<Expression, Error> {
        let bs = &mut s.as_bytes();
        self.read_expression(slab, bs, true)
    }

    fn read_expression(&self, slab:&Slab, bs:&mut &[u8], expect_eof:bool) -> Result<Expression, Error> {
        let first = self.read_value(slab,bs).map_err(|e| e.pre("read_value"))?;
        let pairs = StackVec16::<ExprPair>::new();
        while self.peek_binaryop(bs) {
            let bop = self.read_binaryop(bs).map_err(|e| e.pre("read_binaryop"))?;
            let val = self.read_value(slab,bs).map_err(|e| e.pre("read_value"))?;
            pairs.push(ExprPair(bop,val));
        }
        space(bs);
        if expect_eof && !is_at_eof(bs) { return Err(Error::new("unparsed tokens remaining")); }
        Ok(Expression{first, pairs})
    }

    fn read_value(&self, slab:&Slab, bs:&mut &[u8]) -> Result<Value, Error> {
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
        Err(Error::new("invalid value"))
    }

    fn peek_const(&self, bs:&mut &[u8]) -> bool {
        space(bs);
        self.call_is_const_byte(peek(bs,0),0)
    }
    fn read_const(&self, bs:&mut &[u8]) -> Result<Constant, Error> {
        space(bs);

        let mut buf = StackString256::new();
        while self.call_is_const_byte(peek(bs,0),buf.len()) {
            buf.push(read(bs)?)?;
        }

        let mut multiple = 1.0;
        if buf.len()>0 {
            match buf.last().unwrap() {
                b'k' | b'K' => {   multiple=1000.0; buf.pop(); }
                b'M' => {       multiple=1000000.0; buf.pop(); }
                b'G' => {    multiple=1000000000.0; buf.pop(); }
                b'T' => { multiple=1000000000000.0; buf.pop(); }
                _ => {}
            }
        }

        let bufstr = std::str::from_utf8(buf.as_slice()).map_err(|_| Error::new("Utf8Error"))?;
        let val = bufstr.parse::<f64>().map_err(|_| {
            Error::new("parse<f64> error").pre(bufstr)
        })?;
        Ok(Constant(val*multiple))
    }

    fn peek_var(&self, bs:&mut &[u8]) -> bool {
        space(bs);
        self.call_is_var_byte(peek(bs,0),0)
    }
    fn read_var(&self, bs:&mut &[u8]) -> Result<Variable, Error> {
        space(bs);

        let mut buf = StackString32::new();
        while self.call_is_var_byte(peek(bs,0),buf.len()) {
            buf.push(read(bs)?);
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
    fn read_unaryop(&self, slab:&Slab, bs:&mut &[u8]) -> Result<UnaryOp, Error> {
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
                let x = self.read_expression(slab,bs,false)?;
                space(bs);
                if read(bs)? != b')' { return Err(Error::new("Expected ')'")) }
                Ok(EParens(slab.push_expr(x)?))
            }
            _ => Err(Error::new("invalid unaryop")),
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
    fn read_binaryop(&self, bs:&mut &[u8]) -> Result<BinaryOp, Error> {
        let err = Error::new("illegal binaryop");
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
    fn read_callable(&self, slab:&Slab, bs:&mut &[u8]) -> Result<Callable, Error> {
        if self.peek_printfunc(bs) {
            return self.read_printfunc(bs).map(|f| EPrintFunc(f));
        }
        if self.peek_evalfunc(bs) {
            return self.read_evalfunc(bs).map(|f| EEvalFunc(f));
        }
        if self.peek_func(bs) {
            return self.read_func(slab,bs).map(|f| EFunc(f));
        }
        Err(Error::new("invalid callable"))
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
    fn read_func(&self, slab:&Slab, bs:&mut &[u8]) -> Result<Func, Error> {
        space(bs);

        let mut fname = StackString32::new();
        while self.call_is_func_byte(peek(bs,0),fname.len()) {
            fname.push(read(bs)?.to_ascii_lowercase());
        }
        let fname = String::from_utf8(fname).map_err(|_| Error::new("Utf8Error"))?;

        space(bs);

        if let Ok(b'(') = read(bs) {}
        else { return Err(Error::new("expected '('")); }

        let mut args : Vec<Expression> = Vec::new();  HERE I AM, converting Vecs to stack stuff.

        loop {
            space(bs);
            match peek(bs,0) {
                Some(b) => {
                    if b==b')' {
                        read(bs)?;
                        break;
                    }
                }
                None => return Err(Error::new("Reached end of input while parsing function")),
            }
            if args.len()>0 {
                match read(bs) {
                    Ok(b',') | Ok(b';') => {
                        // I accept ',' or ';' because the TV API disallows the ',' char in symbols... so I'm using ';' as a compromise.
                    }
                    _ => return Err(Error::new("expected ',' or ';'")),
                }
            }
            args.push(self.read_expression(slab,bs,false).map_err(|e| e.pre("read_expression"))?);
        }

        match fname.as_ref() {
            "int" => {
                if args.len()==1 { Ok(EFuncInt(Box::new(args.pop().unwrap())))
                } else { Err(Error::new("expected one arg")) }
            }
            "ceil" => {
                if args.len()==1 { Ok(EFuncCeil(Box::new(args.pop().unwrap())))
                } else { Err(Error::new("expected one arg")) }
            }
            "floor" => {
                if args.len()==1 { Ok(EFuncFloor(Box::new(args.pop().unwrap())))
                } else { Err(Error::new("expected one arg")) }
            }
            "abs" => {
                if args.len()==1 { Ok(EFuncAbs(Box::new(args.pop().unwrap())))
                } else { Err(Error::new("expected one arg")) }
            }
            "log" => {
                if args.len()==1 { Ok(EFuncLog{base:None, val:Box::new(args.pop().unwrap())})
                } else if args.len()==2 {
                    let val = args.pop().unwrap();
                    Ok(EFuncLog{base:Some(Box::new(args.pop().unwrap())), val:Box::new(val)})
                } else { Err(Error::new("expected log(x) or log(base,x)")) }
            }
            "round" => {
                if args.len()==1 { Ok(EFuncRound{modulus:None, val:Box::new(args.pop().unwrap())})
                } else if args.len()==2 {
                    let val = args.pop().unwrap();
                    Ok(EFuncRound{modulus:Some(Box::new(args.pop().unwrap())), val:Box::new(val)})
                } else { Err(Error::new("expected round(x) or round(modulus,x)")) }
            }
            "min" => {
                if args.len()>0 {
                    let first = args.remove(0);
                    Ok(EFuncMin{first:Box::new(first), rest:args.into_boxed_slice()})
                } else { Err(Error::new("expected one or more args")) }
            }
            "max" => {
                if args.len()>0 {
                    let first = args.remove(0);
                    Ok(EFuncMax{first:Box::new(first), rest:args.into_boxed_slice()})
                } else { Err(Error::new("expected one or more args")) }
            }

            "e" => {
                if args.len()==0 { Ok(EFuncE)
                } else { Err(Error::new("expected no args")) }
            }
            "pi" => {
                if args.len()==0 { Ok(EFuncPi)
                } else { Err(Error::new("expected no args")) }
            }

            "sin" => {
                if args.len()==1 { Ok(EFuncSin(Box::new(args.pop().unwrap())))
                } else { Err(Error::new("expected one arg")) }
            }
            "cos" => {
                if args.len()==1 { Ok(EFuncCos(Box::new(args.pop().unwrap())))
                } else { Err(Error::new("expected one arg")) }
            }
            "tan" => {
                if args.len()==1 { Ok(EFuncTan(Box::new(args.pop().unwrap())))
                } else { Err(Error::new("expected one arg")) }
            }
            "asin" => {
                if args.len()==1 { Ok(EFuncASin(Box::new(args.pop().unwrap())))
                } else { Err(Error::new("expected one arg")) }
            }
            "acos" => {
                if args.len()==1 { Ok(EFuncACos(Box::new(args.pop().unwrap())))
                } else { Err(Error::new("expected one arg")) }
            }
            "atan" => {
                if args.len()==1 { Ok(EFuncATan(Box::new(args.pop().unwrap())))
                } else { Err(Error::new("expected one arg")) }
            }
            "sinh" => {
                if args.len()==1 { Ok(EFuncSinH(Box::new(args.pop().unwrap())))
                } else { Err(Error::new("expected one arg")) }
            }
            "cosh" => {
                if args.len()==1 { Ok(EFuncCosH(Box::new(args.pop().unwrap())))
                } else { Err(Error::new("expected one arg")) }
            }
            "tanh" => {
                if args.len()==1 { Ok(EFuncTanH(Box::new(args.pop().unwrap())))
                } else { Err(Error::new("expected one arg")) }
            }

            _ => Err(Error::new(&format!("undefined function: {}",fname))),
        }
    }

    fn peek_printfunc(&self, bs:&mut &[u8]) -> bool { peek_func(bs, 0, b"print") }
    fn read_printfunc(&self, bs:&mut &[u8]) -> Result<PrintFunc, Error> {
        read_func(bs, b"print")?;

        let mut args : Vec<ExpressionOrString> = Vec::new();
        loop {
            space(bs);
            match peek(bs,0) {
                Some(b) => {
                    if b==b')' {
                        read(bs)?;
                        break;
                    }
                }
                None => { return Err(Error::new("reached end of inupt while parsing printfunc")) }
            }
            if args.len()>0 {
                match read(bs) {
                    Ok(b',') | Ok(b';') => {}
                    _ => { return Err(Error::new("expected ',' or ';'")) }
                }
            }
            args.push(self.read_expressionorstring(bs)?);
        }

        Ok(PrintFunc(args.into_boxed_slice()))
    }

    fn peek_evalfunc(&self, bs:&mut &[u8]) -> bool { peek_func(bs, 0, b"eval") }
    fn read_evalfunc(&self, bs:&mut &[u8]) -> Result<EvalFunc, Error> {
        read_func(bs, b"eval")?;

        let eval_expr = self.read_expression(bs,false)?;
        let mut kwargs : Vec<KWArg> = Vec::with_capacity(4);
        fn kwargs_has(kwargs:&Vec<KWArg>, name:&Variable) -> bool {
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
                None => { return Err(Error::new("reached end of input while parsing evalfunc")) }
            }
            match read(bs) {
                Ok(b',') | Ok(b';') => {}
                _ => { return Err(Error::new("expected ',' or ';'")) }
            }
            let name = self.read_var(bs)?;
            space(bs);
            if let Ok(b'=') = read(bs) {
            } else { return Err(Error::new("expected '='")) }
            let expr = self.read_expression(bs,false)?;

            if kwargs_has(&kwargs,&name) { return Err(Error::new(&format!("already defined: {}",name))) }
            kwargs.push(KWArg{name:name, expr:Box::new(expr)});
        }

        Ok(EvalFunc{expr:Box::new(eval_expr), kwargs:kwargs.into_boxed_slice()})
    }

    fn read_expressionorstring(&self, bs:&mut &[u8]) -> Result<ExpressionOrString, Error> {
        if self.peek_string(bs) { Ok(EStr(self.read_string(bs)?))
        } else { Ok(EExpr(Box::new(self.read_expression(bs,false)?))) }
    }

    fn peek_string(&self, bs:&mut &[u8]) -> bool {
        space(bs);
        peek_is(bs,0,b'"')
    }
    fn read_string(&self, bs:&mut &[u8]) -> Result<String, Error> {
        space(bs);

        match read(bs) {
            Ok(b) => {
                if b!=b'"' { return Err(Error::new(r#"expected '"'"#)) }
            }
            Err(e) => { return Err(e.pre("read_string")) }
        }

        let mut buf : Vec<u8> = Vec::with_capacity(16);
        loop {
            let b = read(bs)?;
            if b==b'"' { break; }
            buf.push(b);
        }

        String::from_utf8(buf).map_err(|_| Error::new("Utf8Error"))
    }
}

//---- Tests:

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aaa_util() {
        match (|| -> Result<(),Error> {
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
                Some(Error{..}) => {}  // Can I improve this so I can match the "EOF" ?
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

    #[test]
    fn aaa_parser() {
        let p = Parser{
            is_const_byte:None,
            is_var_byte:None,
        };
        assert!(p.call_is_var_byte(Some(b'a'),0));
        assert!(!p.call_is_const_byte(Some(b'a'),0));

        let p = Parser{
            is_const_byte:Some(&|_:u8, _:usize| true),
            is_var_byte:None,
        };
        assert!(p.call_is_const_byte(Some(b'a'),0));

        let p = Parser{
            is_const_byte:None,
            is_var_byte:None,
        };
        
        {
            let bsarr = b"12.34";
            let bs = &mut &bsarr[..];
            assert_eq!(p.read_value(bs), Ok(EConstant(Constant(12.34))));
        }

        let mut slab : Slab;
        assert_eq!(p.parse({slab=Slab::new(); &slab}, "12.34 + 43.21 + 11.11"),
                   Ok(Expression{
                        first:EConstant(Constant(12.34)),
                        pairs:Box::new([
                            ExprPair(EPlus, EConstant(Constant(43.21))),
                            ExprPair(EPlus, EConstant(Constant(11.11)))])}));

        assert_eq!(p.parse({slab=Slab::new(); &slab}, "12.34 + abs ( -43 - 0.21 ) + 11.11"),
                   Ok(Expression {
                        first:EConstant(Constant(12.34)),
                        pairs:Box::new([
                            ExprPair(EPlus, ECallable(EFunc(EFuncAbs(Box::new(Expression {
                                first:EUnaryOp(ENeg(Box::new(EConstant(Constant(43.0))))),
                                pairs:Box::new([ExprPair(EMinus, EConstant(Constant(0.21)))]) }))))),
                            ExprPair(EPlus, EConstant(Constant(11.11)))]) }));

        assert_eq!(p.parse({slab=Slab::new(); &slab}, "12.34 + print ( 43.21 ) + 11.11"),
                   Ok(Expression {
                        first:EConstant(Constant(12.34)),
                        pairs:Box::new([
                            ExprPair(EPlus, ECallable(EPrintFunc(PrintFunc(Box::new([
                                EExpr(Box::new(Expression {
                                    first:EConstant(Constant(43.21)),
                                    pairs:Box::new([]) }))]))))),
                            ExprPair(EPlus, EConstant(Constant(11.11)))]) }));

        assert_eq!(p.parse({slab=Slab::new(); &slab}, "12.34 + eval ( x - y , x = 5 , y=4 ) + 11.11"),
                   Ok(Expression {
                        first:EConstant(Constant(12.34)),
                        pairs:Box::new([
                            ExprPair(EPlus, ECallable(EEvalFunc(EvalFunc {
                                expr:Box::new(Expression {
                                    first:EVariable(Variable("x".to_string())),
                                    pairs:Box::new([ExprPair(EMinus, EVariable(Variable("y".to_string())))]) }),
                                kwargs:Box::new([
                                    KWArg { name: Variable("x".to_string()), expr:Box::new(Expression { first: EConstant(Constant(5.0)), pairs:Box::new([]) }) },
                                    KWArg { name: Variable("y".to_string()), expr:Box::new(Expression { first: EConstant(Constant(4.0)), pairs:Box::new([]) }) }]) }))),
                            ExprPair(EPlus, EConstant(Constant(11.11)))]) }));
    }
}

