use crate::slab::ParseSlab;
use kerr::KErr;

use std::str::from_utf8;


// === Algebra Grammar ===
//
// Expression: Value (BinaryOp Value)*
//
// Value: Constant || UnaryOp || Callable
//
// Constant: [+-]?[0-9]*(\.[0-9]+)?( ([eE][+-]?[0-9]+) || [pnuµmkKMGT] )?
//
// UnaryOp: +Value || -Value || (Expression) || !Value
//
// BinaryOp: + || - || * || / || % || ^ || < || <= || == || != || >= || > || or || and
//
// Callable: PrintFunc || EvalFunc || StdFunc
//
// VarName: [a-zA-Z_][a-zA-Z_0-9]*
//
// StdFunc: VarName((Expression,)*)?
//
// PrintFunc: print(ExpressionOrString,*)
//
// ExpressionOrString: Expression || String
//
// String: ".*"
//
// EvalFunc: eval(Expression(,VarName=Expression)*)



#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ExpressionI(pub usize);
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ValueI(pub usize);


#[derive(Debug, PartialEq)]
pub struct Expression {
    pub first: Value,
    pub pairs: Vec<ExprPair>,  // cap=8
}

#[derive(Debug, PartialEq)]
pub struct ExprPair(pub BinaryOp, pub Value);

#[derive(Debug, PartialEq)]
pub enum Value {
    EConstant(Constant),
    EUnaryOp(UnaryOp),
    ECallable(Callable),
}
use Value::{EConstant, EUnaryOp, ECallable};

#[derive(Debug, PartialEq)]
pub struct Constant(pub f64);

#[derive(PartialEq)]
pub struct VarName(pub String);

#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    EPos(ValueI),
    ENeg(ValueI),
    ENot(ValueI),
    EParentheses(ExpressionI),
}
use UnaryOp::{EPos, ENeg, ENot, EParentheses};

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum BinaryOp {
    // Sorted in order of precedence (low-priority to high-priority):
    // Keep this order in-sync with evaler.rs.  (Search for 'rtol' and 'ltor'.)
    EOR    =  1,  // Lowest Priority
    EAND   =  2,
    ENE    =  3,
    EEQ    =  4,
    EGTE   =  5,
    ELTE   =  6,
    EGT    =  7,
    ELT    =  8,
    EAdd   =  9,
    ESub   = 10,
    EMul   = 11,
    EDiv   = 12,
    EMod   = 13,
    EExp   = 14,  // Highest Priority
}
use BinaryOp::{EAdd, ESub, EMul, EDiv, EMod, EExp, ELT, ELTE, EEQ, ENE, EGTE, EGT, EOR, EAND};

#[derive(Debug, PartialEq)]
pub enum Callable {
    EStdFunc(StdFunc),
    EPrintFunc(PrintFunc),
    EEvalFunc(EvalFunc),
}
use Callable::{EStdFunc, EPrintFunc, EEvalFunc};

#[derive(Debug, PartialEq)]
pub enum StdFunc {
    EVar(VarName),
    EFunc{name:VarName, args:Vec<ExpressionI>},  // cap=4

    EFuncInt(ExpressionI),
    EFuncCeil(ExpressionI),
    EFuncFloor(ExpressionI),
    EFuncAbs(ExpressionI),
    EFuncSign(ExpressionI),
    EFuncLog{     base:Option<ExpressionI>, expr:ExpressionI},
    EFuncRound{modulus:Option<ExpressionI>, expr:ExpressionI},
    EFuncMin{first:ExpressionI, rest:Vec<ExpressionI>},  // cap=4
    EFuncMax{first:ExpressionI, rest:Vec<ExpressionI>},  // cap=4

    EFuncE,
    EFuncPi,

    EFuncSin(ExpressionI),
    EFuncCos(ExpressionI),
    EFuncTan(ExpressionI),
    EFuncASin(ExpressionI),
    EFuncACos(ExpressionI),
    EFuncATan(ExpressionI),
    EFuncSinH(ExpressionI),
    EFuncCosH(ExpressionI),
    EFuncTanH(ExpressionI),
    EFuncASinH(ExpressionI),
    EFuncACosH(ExpressionI),
    EFuncATanH(ExpressionI),
}
use StdFunc::{EVar, EFunc, EFuncInt, EFuncCeil, EFuncFloor, EFuncAbs, EFuncSign, EFuncLog, EFuncRound, EFuncMin, EFuncMax, EFuncE, EFuncPi, EFuncSin, EFuncCos, EFuncTan, EFuncASin, EFuncACos, EFuncATan, EFuncSinH, EFuncCosH, EFuncTanH, EFuncASinH, EFuncACosH, EFuncATanH};

#[derive(Debug, PartialEq)]
pub struct PrintFunc(pub Vec<ExpressionOrString>);  // cap=8

#[derive(Debug, PartialEq)]
pub enum ExpressionOrString {
    EExpr(ExpressionI),
    EStr(String),  // cap=64
}
use ExpressionOrString::{EExpr, EStr};

#[derive(Debug, PartialEq)]
pub struct EvalFunc {
    pub expr:   ExpressionI,
    pub kwargs: Vec<KWArg>,  // cap=8
}

#[derive(Debug, PartialEq)]
pub struct KWArg {
    pub name: VarName,
    pub expr: ExpressionI,
}



impl Clone for PrintFunc {
    fn clone(&self) -> Self {
        let mut vec = Vec::<ExpressionOrString>::with_capacity(self.0.len());
        for x_or_s in self.0.iter() {
            vec.push(match x_or_s {
                EExpr(i) => EExpr(*i),
                EStr(s) => EStr(s.clone()),
            });
        }
        PrintFunc(vec)
    }
}

impl Clone for EvalFunc {
    fn clone(&self) -> Self {
        let expr = self.expr;
        let mut kwargs = Vec::<KWArg>::with_capacity(self.kwargs.len());
        for kw in self.kwargs.iter() {
            let name = VarName(kw.name.0.clone());
            let expr = kw.expr;
            kwargs.push(KWArg{name, expr});
        }
        EvalFunc{expr, kwargs}
    }
}



enum Tok<T> {
    Pass,
    Bite(T),
}
use Tok::{Pass, Bite};


// Vec seems really inefficient to me because remove() does not just increment the internal pointer -- it shifts data all around.  There's also split_* methods but they seem to be designed to return new Vecs, not modify self.
// Just use slices instead, which I know will be very efficient:
fn peek(bs:&[u8], skip:usize) -> Option<u8> {
    if bs.len()>skip { Some(bs[skip]) }
    else { None }
}
fn is_at_eof(bs:&[u8]) -> bool { bs.is_empty() }
fn peek_is(bs:&[u8], skip:usize, val:u8) -> bool {
    match peek(bs,skip) {
        Some(b) => b==val,
        None => false,
    }
}

fn read(bs:&mut &[u8]) -> Result<u8, KErr> {
    if !bs.is_empty() {
        let b = bs[0];
        *bs = &bs[1..];
        Ok(b)
    } else { Err(KErr::new("EOF")) }
}

fn is_space(b:u8) -> bool {
    if b>b' ' { return false }  // Try to improve performance of the common case.
    b==b' ' || b==b'\n' || b==b'\t' || b==b'\r'
}
fn space(bs:&mut &[u8]) {
    while let Some(b) = peek(bs,0) {
        if !is_space(b) { break }
        let _ = read(bs);
    }
}



pub struct Parser {
    char_buf:String,
}

impl Parser {
    pub fn new() -> Parser {
        Parser{ char_buf:String::with_capacity(64) }
    }

    #[inline]
    fn is_const_byte(b:u8, i:usize) -> bool {
        if b'0'<=b && b<=b'9' || b==b'.' { return true }
        if i>0 && ( b==b'k' || b==b'K' || b==b'M' || b==b'G' || b==b'T' || b==b'm' || b==b'u' || b==b'n' || b==b'p' || b==b'e' || b==b'E' ) { return true }
        false
    }
    #[inline]
    fn is_const_byte_opt(bo:Option<u8>, i:usize) -> bool {
        match bo {
            Some(b) => Self::is_const_byte(b,i),
            None => false,
        }
    }

    #[inline]
    fn is_varname_byte(b:u8, i:usize) -> bool {
        (b'A'<=b && b<=b'Z') || (b'a'<=b && b<=b'z') || b==b'_' || (i>0 && ( b'0'<=b && b<=b'9' ))  // I was considering adding square brackets to the list of var chars so that index operations could be simulated, but I decided not to do that until we have a real-life need for it.
    }
    #[inline]
    fn is_varname_byte_opt(&self, bo:Option<u8>, i:usize) -> bool {
        match bo {
            Some(b) => Self::is_varname_byte(b,i),
            None => false,
        }
    }


    // I cannot return Result<&Expression> because it would prolong the mut:
    pub fn parse(&mut self, slab:&mut ParseSlab, s:&str) -> Result<ExpressionI, KErr> {
        if s.len()>4096 { return Err(KErr::new("expression string is too long")); }  // Restrict length for safety
        let mut bs = s.as_bytes();
        self.read_expression(slab, &mut bs, true)
    }

    fn read_expression(&mut self, slab:&mut ParseSlab, bs:&mut &[u8], expect_eof:bool) -> Result<ExpressionI, KErr> {
        let first = self.read_value(slab,bs).map_err(|e| e.pre("read_value"))?;
        let mut pairs = Vec::<ExprPair>::with_capacity(8);
        loop {
            match self.read_binaryop(bs).map_err(|e| e.pre("read_binaryop"))? {
                Pass => break,
                Bite(bop) => {
                    let val = self.read_value(slab,bs).map_err(|e| e.pre("read_value"))?;
                    pairs.push(ExprPair(bop,val));
                }
            }
        }
        space(bs);
        if expect_eof && !is_at_eof(bs) {
            let bs_str = from_utf8(bs).map_err(|_| KErr::new("Utf8Error while handling 'unparsed tokens remaining' error."))?;
            return Err(KErr::new("unparsed tokens remaining").pre(bs_str));
        }
        Ok(slab.push_expr(Expression{first, pairs})?)
    }

    fn read_value(&mut self, slab:&mut ParseSlab, bs:&mut &[u8]) -> Result<Value, KErr> {
        match self.read_const(bs)? {
            Pass => {}
            Bite(c) => return Ok(EConstant(c)),
        }
        match self.read_unaryop(slab,bs)? {
            Pass => {}
            Bite(u) => return Ok(EUnaryOp(u)),
        }
        match self.read_callable(slab,bs)? {
            Pass => {}
            Bite(c) => return Ok(ECallable(c)),
        }
        Err(KErr::new("invalid value"))
    }

    fn read_const(&mut self, bs:&mut &[u8]) -> Result<Tok<Constant>, KErr> {
        space(bs);

        // Grammar: [+-]?[0-9]*(\.[0-9]+)?( ([eE][+-]?[0-9]+) || [pnuµmkKMGT] )?
        #[inline]
        fn peek_digits(bs:&[u8]) -> usize {
            let mut i = 0;
            while i<bs.len() && b'0'<=bs[i] && bs[i]<=b'9' { i+=1; }
            i
        }
        #[inline]
        fn peek_exp(bs:&[u8]) -> Result<usize, KErr> {
            if bs.is_empty() { return Err(KErr::new("peek_exp empty")); }
            let mut i = 0;
            if bs[i]==b'-' || bs[i]==b'+' { i+=1; }
            let digits = peek_digits(&bs[i..]);
            if digits==0 { return Err(KErr::new("peek_exp no digits")); }
            Ok(i+digits)
        }
        #[inline]
        fn peek_tail(bs:&[u8]) -> Result<(/*read:*/usize, /*skip:*/usize, /*exp:*/i32), KErr> {
            if bs.is_empty() { return Ok((0,0,0)); }
            match bs[0] {
                b'k' | b'K' => Ok((0,1,3)),
                b'M' => Ok((0,1,6)),
                b'G' => Ok((0,1,9)),
                b'T' => Ok((0,1,12)),
                b'm' => Ok((0,1,-3)),
                b'u' | b'\xb5' => Ok((0,1,-6)),  // ASCII-encoded 'µ'
                b'\xc2' if bs.len()>1 && bs[1]==b'\xb5' => Ok((0,2,-6)),  // UTF8-encoded 'µ'
                b'n' => Ok((0,1,-9)),
                b'p' => Ok((0,1,-12)),
                b'e' | b'E' => peek_exp(&bs[1..]).map(|size| (1+size,0,0)),
                _ => Ok((0,0,0)),
            }
        }

        let mut toread=0;  let mut toskip=0;  let mut exp=0;

        match peek(bs,0) {
            None => return Ok(Pass), 
            Some(b) => {
                if b==b'-' || b==b'+' { toread+=1; }
            }
            
        }

        let predec = peek_digits(&bs[toread..]);
        toread+=predec;

        match peek(bs, toread) {
            None => {
                if predec==0 { return Ok(Pass); }
            }
            Some(b) => {
                if b==b'.' {
                    toread+=1;
                    let postdec = peek_digits(&bs[toread..]);
                    if predec==0 && postdec==0 { return Err(KErr::new("decimal without pre- or post-digits")); }
                    toread+=postdec;
                } else {
                    if predec==0 { return Ok(Pass); }
                }
                let (rd,sk,ex) = peek_tail(&bs[toread..])?;
                toread+=rd;  toskip=sk;  exp=ex;
            }
        }

        self.char_buf.clear();
        for _ in 0..toread { self.char_buf.push(read(bs)? as char); }
        for _ in 0..toskip { read(bs)?; }
        if exp!=0 { self.char_buf.push('e'); self.char_buf.push_str(&exp.to_string()); }

        let val = self.char_buf.parse::<f64>().map_err(|_| {
            KErr::new("parse<f64> error").pre(&self.char_buf)
        })?;
        Ok(Bite(Constant(val)))
    }

    fn read_unaryop(&mut self, slab:&mut ParseSlab, bs:&mut &[u8]) -> Result<Tok<UnaryOp>, KErr> {
        space(bs);
        match peek(bs,0) {
            None => Ok(Pass),  // Err(KErr::new("EOF at UnaryOp position")), -- Instead of erroring, let the higher level decide what to do.
            Some(b) => match b {
                b'+' => {
                    read(bs)?;
                    let v = self.read_value(slab,bs)?;
                    Ok(Bite(EPos(slab.push_val(v)?)))
                }
                b'-' => {
                    read(bs)?;
                    let v = self.read_value(slab,bs)?;
                    Ok(Bite(ENeg(slab.push_val(v)?)))
                }
                b'(' => {
                    read(bs)?;
                    let xi = self.read_expression(slab,bs,false)?;
                    space(bs);
                    if read(bs)? != b')' { return Err(KErr::new("Expected ')'")) }
                    Ok(Bite(EParentheses(xi)))
                }
                b'!' => {
                    read(bs)?;
                    let v = self.read_value(slab,bs)?;
                    Ok(Bite(ENot(slab.push_val(v)?)))
                }
                _ => Ok(Pass),
            }
        }
    }

    fn read_binaryop(&self, bs:&mut &[u8]) -> Result<Tok<BinaryOp>, KErr> {
        space(bs);
        match peek(bs,0) {
            None => Ok(Pass), // Err(KErr::new("EOF")), -- EOF is usually OK in a BinaryOp position.
            Some(b) => match b {
                b'+' => { read(bs)?; Ok(Bite(EAdd)) }
                b'-' => { read(bs)?; Ok(Bite(ESub)) }
                b'*' => { read(bs)?; Ok(Bite(EMul)) }
                b'/' => { read(bs)?; Ok(Bite(EDiv)) }
                b'%' => { read(bs)?; Ok(Bite(EMod)) }
                b'^' => { read(bs)?; Ok(Bite(EExp)) }
                b'<' => { read(bs)?;
                          if peek_is(bs,0,b'=') { read(bs)?; Ok(Bite(ELTE)) }
                          else { Ok(Bite(ELT)) } }
                b'>' => { read(bs)?;
                          if peek_is(bs,0,b'=') { read(bs)?; Ok(Bite(EGTE)) }
                          else { Ok(Bite(EGT)) } }
                b'=' if peek_is(bs,1,b'=') => { read(bs)?; read(bs)?;
                                                Ok(Bite(EEQ)) }
                b'!' if peek_is(bs,1,b'=') => { read(bs)?; read(bs)?;
                                                Ok(Bite(ENE)) }
                b'o' if peek_is(bs,1,b'r') => { read(bs)?; read(bs)?;
                                                Ok(Bite(EOR)) }
                b'a' if peek_is(bs,1,b'n') && peek_is(bs,2,b'd') => { read(bs)?; read(bs)?; read(bs)?;
                                                                      Ok(Bite(EAND)) }
                _ => Ok(Pass),
            }
        }
    }

    fn read_callable(&mut self, slab:&mut ParseSlab, bs:&mut &[u8]) -> Result<Tok<Callable>, KErr> {
        match self.read_varname(bs)? {
            Pass => Ok(Pass),
            Bite(varname) => {
                match self.read_open_parenthesis(bs)? {
                    Pass => {
                        // VarNames without Parenthesis are always treated as custom 0-arg functions.
                        Ok(Bite(EStdFunc(EVar(varname))))
                    }
                    Bite(_) => {
                        // VarNames with Parenthesis are first matched against builtins, then custom.
                        match varname.0.as_ref() {
                            "print" => Ok(Bite(EPrintFunc(self.read_printfunc(slab,bs)?))),
                            "eval" => Ok(Bite(EEvalFunc(self.read_evalfunc(slab,bs)?))),
                            _ => Ok(Bite(EStdFunc(self.read_func(varname,slab,bs)?))),
                        }
                    }
                }
            }
        }
    }

    fn read_varname(&mut self, bs:&mut &[u8]) -> Result<Tok<VarName>, KErr> {
        space(bs);

        self.char_buf.clear();
        while self.is_varname_byte_opt(peek(bs,0),self.char_buf.len()) {
            self.char_buf.push(read(bs)? as char);
        }

        if self.char_buf.is_empty() { return Ok(Pass); }  // This is NOT a Pass after a read() -- len=0 so no read occurred.

        Ok(Bite(VarName(self.char_buf.clone())))
    }

    fn read_open_parenthesis(&mut self, bs:&mut &[u8]) -> Result<Tok<()>, KErr> {
        space(bs);

        if peek(bs,0) == Some(b'(') {
            read(bs)?;
            return Ok(Bite(()));
        }
        Ok(Pass)
    }

    fn read_func(&mut self, fname:VarName, slab:&mut ParseSlab, bs:&mut &[u8]) -> Result<StdFunc, KErr> {
        let mut args = Vec::<ExpressionI>::with_capacity(4);
        loop {
            space(bs);
            match peek(bs,0) {
                Some(b) => {
                    if b==b')' {
                        read(bs)?;
                        break;
                    }
                }
                None => return Err(KErr::new(&format!("Reached end of input while parsing function: {}",fname))),
            }
            if !args.is_empty() {
                match read(bs) {
                    Ok(b',') | Ok(b';') => {
                        // I accept ',' or ';' because the TV API disallows the ',' char in symbols... so I'm using ';' as a compromise.
                    }
                    _ => return Err(KErr::new("expected ',' or ';'")),
                }
            }
            args.push(self.read_expression(slab,bs,false).map_err(|e| e.pre("read_expression"))?);
        }

        match fname.0.as_str() {
            "int" => {
                if args.len()==1 { Ok(EFuncInt(args.pop().unwrap()))
                } else { Err(KErr::new("expected one arg")) }
            }
            "ceil" => {
                if args.len()==1 { Ok(EFuncCeil(args.pop().unwrap()))
                } else { Err(KErr::new("expected one arg")) }
            }
            "floor" => {
                if args.len()==1 { Ok(EFuncFloor(args.pop().unwrap()))
                } else { Err(KErr::new("expected one arg")) }
            }
            "abs" => {
                if args.len()==1 { Ok(EFuncAbs(args.pop().unwrap()))
                } else { Err(KErr::new("expected one arg")) }
            }
            "sign" => {
                if args.len()==1 { Ok(EFuncSign(args.pop().unwrap()))
                } else { Err(KErr::new("expected one arg")) }
            }
            "log" => {
                if args.len()==1 { Ok(EFuncLog{base:None, expr:args.pop().unwrap()})
                } else if args.len()==2 {
                    let expr = args.pop().unwrap();
                    Ok(EFuncLog{base:Some(args.pop().unwrap()), expr})
                } else { Err(KErr::new("expected log(x) or log(base,x)")) }
            }
            "round" => {
                if args.len()==1 { Ok(EFuncRound{modulus:None, expr:args.pop().unwrap()})
                } else if args.len()==2 {
                    let expr = args.pop().unwrap();
                    Ok(EFuncRound{modulus:Some(args.pop().unwrap()), expr})
                } else { Err(KErr::new("expected round(x) or round(modulus,x)")) }
            }
            "min" => {
                if !args.is_empty() {
                    let first = args.remove(0);
                    Ok(EFuncMin{first, rest:args})
                } else { Err(KErr::new("expected one or more args")) }
            }
            "max" => {
                if !args.is_empty() {
                    let first = args.remove(0);
                    Ok(EFuncMax{first, rest:args})
                } else { Err(KErr::new("expected one or more args")) }
            }

            "e" => {
                if args.is_empty() { Ok(EFuncE)
                } else { Err(KErr::new("expected no args")) }
            }
            "pi" => {
                if args.is_empty() { Ok(EFuncPi)
                } else { Err(KErr::new("expected no args")) }
            }

            "sin" => {
                if args.len()==1 { Ok(EFuncSin(args.pop().unwrap()))
                } else { Err(KErr::new("expected one arg")) }
            }
            "cos" => {
                if args.len()==1 { Ok(EFuncCos(args.pop().unwrap()))
                } else { Err(KErr::new("expected one arg")) }
            }
            "tan" => {
                if args.len()==1 { Ok(EFuncTan(args.pop().unwrap()))
                } else { Err(KErr::new("expected one arg")) }
            }
            "asin" => {
                if args.len()==1 { Ok(EFuncASin(args.pop().unwrap()))
                } else { Err(KErr::new("expected one arg")) }
            }
            "acos" => {
                if args.len()==1 { Ok(EFuncACos(args.pop().unwrap()))
                } else { Err(KErr::new("expected one arg")) }
            }
            "atan" => {
                if args.len()==1 { Ok(EFuncATan(args.pop().unwrap()))
                } else { Err(KErr::new("expected one arg")) }
            }
            "sinh" => {
                if args.len()==1 { Ok(EFuncSinH(args.pop().unwrap()))
                } else { Err(KErr::new("expected one arg")) }
            }
            "cosh" => {
                if args.len()==1 { Ok(EFuncCosH(args.pop().unwrap()))
                } else { Err(KErr::new("expected one arg")) }
            }
            "tanh" => {
                if args.len()==1 { Ok(EFuncTanH(args.pop().unwrap()))
                } else { Err(KErr::new("expected one arg")) }
            }
            "asinh" => {
                if args.len()==1 { Ok(EFuncASinH(args.pop().unwrap()))
                } else { Err(KErr::new("expected one arg")) }
            }
            "acosh" => {
                if args.len()==1 { Ok(EFuncACosH(args.pop().unwrap()))
                } else { Err(KErr::new("expected one arg")) }
            }
            "atanh" => {
                if args.len()==1 { Ok(EFuncATanH(args.pop().unwrap()))
                } else { Err(KErr::new("expected one arg")) }
            }

            _ => Ok(EFunc{name:fname, args}),
        }
    }

    fn read_printfunc(&mut self, slab:&mut ParseSlab, bs:&mut &[u8]) -> Result<PrintFunc, KErr> {
        let mut args = Vec::<ExpressionOrString>::with_capacity(8);
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
            if !args.is_empty() {
                match read(bs) {
                    Ok(b',') | Ok(b';') => {}
                    _ => { return Err(KErr::new("expected ',' or ';'")) }
                }
            }
            args.push(self.read_expressionorstring(slab,bs)?);
        }

        Ok(PrintFunc(args))
    }

    fn read_evalfunc(&mut self, slab:&mut ParseSlab, bs:&mut &[u8]) -> Result<EvalFunc, KErr> {
        let eval_expr = self.read_expression(slab,bs,false)?;
        let mut kwargs = Vec::<KWArg>::with_capacity(8);
        fn kwargs_has(kwargs:&[KWArg], name:&VarName) -> bool {
            for kwarg in kwargs {
                if kwarg.name.0==name.0 { return true; }
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
            let name : VarName;
            match self.read_varname(bs) {
                Ok(Pass) => return Err(KErr::new("unexpected read_varname pass")),
                Ok(Bite(v)) => name=v,
                Err(e) => return Err(e),
            }
            space(bs);
            if let Ok(b'=') = read(bs) {
            } else { return Err(KErr::new("expected '='")) }
            let expr = self.read_expression(slab,bs,false)?;

            if kwargs_has(&kwargs,&name) { return Err(KErr::new(&format!("already defined: {}",name))) }
            kwargs.push(KWArg{name, expr});
        }

        Ok(EvalFunc{expr:eval_expr, kwargs})
    }

    fn read_expressionorstring(&mut self, slab:&mut ParseSlab, bs:&mut &[u8]) -> Result<ExpressionOrString, KErr> {
        match self.read_string(bs)? {
            Pass => {}
            Bite(s) => return Ok(EStr(s)),
        }
        Ok(EExpr(self.read_expression(slab,bs,false)?))
    }

    fn read_string(&mut self, bs:&mut &[u8]) -> Result<Tok<String>,KErr> {
        space(bs);

        match peek(bs,0) {
            None => return Err(KErr::new("EOF while reading opening quote of string")),
            Some(b'"') => { read(bs)?; }
            Some(_) => { return Ok(Pass) }
        }

        self.char_buf.clear();
        loop {
            let b = read(bs)?;
            if b==b'"' { break; }
            self.char_buf.push(b as char);
        }

        Ok(Bite(self.char_buf.clone()))
    }
}
impl Default for Parser {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod internal_tests {
    use super::*;
    use crate::slab::Slab;

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

    #[test]
    fn priv_tests() {
        let mut p = Parser::new();
        assert!(p.is_varname_byte_opt(Some(b'a'),0));
        assert!(!Parser::is_const_byte_opt(Some(b'a'),0));

        let mut slab = Slab::new();
        
        {
            let bsarr = b"12.34";
            let bs = &mut &bsarr[..];
            assert_eq!(p.read_value(&mut slab.ps, bs), Ok(EConstant(Constant(12.34))));
        }
    }
}

