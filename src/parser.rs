use crate::slab::ParseSlab;
use kerr::KErr;



// === Algebra Grammar ===
//
// Expression: Value (BinaryOp Value)*
//
// Value: Constant || UnaryOp || Callable || Variable
// #^^^ Variable must be last to avoid masking.
//
// Constant: (\.[0-9])+(k || K || M || G || T)?
//
// Variable: [a-zA-Z_][a-zA-Z_0-9]*
//
// UnaryOp: +Value || -Value || (Expression) || !Value
//
// BinaryOp: + || - || * || / || % || ^ || < || <= || == || != || >= || > || or || and
//
// Callable: PrintFunc || EvalFunc || Function
// #^^^ Function must be last to avoid masking.
//
// Function: Variable(Expression(,Expression)*)
//
// PrintFunc: print(ExpressionOrString,*)
//
// ExpressionOrString: Expression || String
//
// String: ".*"
//
// EvalFunc: eval(Expression(,Variable=Expression)*)



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
    EVariable(Variable),
    EUnaryOp(UnaryOp),
    ECallable(Callable),
}
use Value::{EConstant, EVariable, EUnaryOp, ECallable};

#[derive(Debug, PartialEq)]
pub struct Constant(pub f64);

#[derive(PartialEq)]
pub struct Variable(pub String);  // cap=16

#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    EPos(ValueI),
    ENeg(ValueI),
    ENot(ValueI),
    EParens(ExpressionI),
}
use UnaryOp::{EPos, ENeg, ENot, EParens};

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
    EPlus  =  9,
    EMinus = 10,
    EMul   = 11,
    EDiv   = 12,
    EMod   = 13,
    EExp   = 14,  // Highest Priority
}
use BinaryOp::{EPlus, EMinus, EMul, EDiv, EMod, EExp, ELT, ELTE, EEQ, ENE, EGTE, EGT, EOR, EAND};

#[derive(Debug, PartialEq)]
pub enum Callable {
    EFunc(Func),
    EPrintFunc(PrintFunc),
    EEvalFunc(EvalFunc),
}
use Callable::{EFunc, EPrintFunc, EEvalFunc};

#[derive(Debug, PartialEq)]
pub enum Func {
    EFuncInt(ExpressionI),
    EFuncCeil(ExpressionI),
    EFuncFloor(ExpressionI),
    EFuncAbs(ExpressionI),
    EFuncLog{     base:Option<ExpressionI>, expr:ExpressionI},
    EFuncRound{modulus:Option<ExpressionI>, expr:ExpressionI},
    EFuncMin{first:ExpressionI, rest:Vec<ExpressionI>},  // cap=8
    EFuncMax{first:ExpressionI, rest:Vec<ExpressionI>},  // cap=8
    //
    EFuncE,
    EFuncPi,
    //
    EFuncSin(ExpressionI),
    EFuncCos(ExpressionI),
    EFuncTan(ExpressionI),
    EFuncASin(ExpressionI),
    EFuncACos(ExpressionI),
    EFuncATan(ExpressionI),
    EFuncSinH(ExpressionI),
    EFuncCosH(ExpressionI),
    EFuncTanH(ExpressionI),
}
use Func::{EFuncInt, EFuncCeil, EFuncFloor, EFuncAbs, EFuncLog, EFuncRound, EFuncMin, EFuncMax, EFuncE, EFuncPi, EFuncSin, EFuncCos, EFuncTan, EFuncASin, EFuncACos, EFuncATan, EFuncSinH, EFuncCosH, EFuncTanH};

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
    pub kwargs: Vec<KWArg>,  // cap=16
}

#[derive(Debug, PartialEq)]
pub struct KWArg {
    pub name: Variable,
    pub expr: ExpressionI,
}



impl Clone for PrintFunc {
    fn clone(&self) -> Self {
        let mut vec = Vec::<ExpressionOrString>::with_capacity(self.0.len());
        for xors in self.0.iter() {
            vec.push(match xors {
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
            let name = Variable(kw.name.0.clone());
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
fn is_at_eof(bs:&[u8]) -> bool { bs.len() == 0 }
fn peek_is(bs:&[u8], skip:usize, val:u8) -> bool {
    match peek(bs,skip) {
        Some(b) => b==val,
        None => false,
    }
}

fn read(bs:&mut &[u8]) -> Result<u8, KErr> {
    if bs.len() > 0 {
        let b = bs[0];
        *bs = &bs[1..];
        Ok(b)
    } else { Err(KErr::new("EOF")) }
}

fn is_space(b:u8) -> bool {
    if b>b' ' { return false }  // Try to improve performance of the common case.
    return b==b' ' || b==b'\n' || b==b'\t' || b==b'\r'
}
fn space(bs:&mut &[u8]) {
    while let Some(b) = peek(bs,0) {
        if !is_space(b) { break }
        let _ = read(bs);
    }
}



pub struct Parser<'a> {
    is_const_byte:&'a dyn Fn(u8,usize)->bool,
    is_var_byte  :&'a dyn Fn(u8,usize)->bool,  // Until proven otherwise, assume that function names follow the same rules as vars.
}

impl Parser<'_> {
    pub fn new<'b>(is_const_byte:Option<&'b dyn Fn(u8,usize)->bool>,
               is_var_byte:Option<&'b dyn Fn(u8,usize)->bool>) -> Parser<'b> {
        Parser{
            is_const_byte:is_const_byte.unwrap_or(&Parser::default_is_const_byte),
            is_var_byte:is_var_byte.unwrap_or(&Parser::default_is_var_byte),
        }
    }
    fn default_is_const_byte(b:u8, i:usize) -> bool {
        if b'0'<=b && b<=b'9' || b==b'.' { return true }
        if i>0 && ( b==b'k' || b==b'K' || b==b'M' || b==b'G' || b==b'T' ) { return true }
        return false
    }
    fn default_is_var_byte(b:u8, i:usize) -> bool {
        (b'A'<=b && b<=b'Z') || (b'a'<=b && b<=b'z') || b==b'_' || (i>0 && b'0'<=b && b<=b'9')
    }

    #[inline]
    fn call_is_const_byte(&self, bo:Option<u8>, i:usize) -> bool {
        match bo {
            Some(b) => (self.is_const_byte)(b,i),
            None => false,
        }
    }
    #[inline]
    fn call_is_var_byte(&self, bo:Option<u8>, i:usize) -> bool {
        match bo {
            Some(b) => (self.is_var_byte)(b,i),
            None => false,
        }
    }
    // Re-use var logic until proven otherwise:
    #[inline]
    fn call_is_func_byte(&self, bo:Option<u8>, i:usize) -> bool {
        self.call_is_var_byte(bo,i)
    }

    // I cannot return Result<&Expression> because it would prolong the mut:
    pub fn parse(&self, slab:&mut ParseSlab, s:&str) -> Result<ExpressionI, KErr> {
        if s.len()>4096 { return Err(KErr::new("expression string is too long")); }  // Restrict length for safety
        let mut bs = s.as_bytes();
        self.read_expression(slab, &mut bs, true)
    }

    fn read_expression(&self, slab:&mut ParseSlab, bs:&mut &[u8], expect_eof:bool) -> Result<ExpressionI, KErr> {
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
        if expect_eof && !is_at_eof(bs) { return Err(KErr::new("unparsed tokens remaining")); }
        Ok(slab.push_expr(Expression{first, pairs})?)
    }

    fn read_value(&self, slab:&mut ParseSlab, bs:&mut &[u8]) -> Result<Value, KErr> {
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
        match self.read_var(bs)? {  // Should go last -- don't mask callables.
            Pass => {}
            Bite(v) => return Ok(EVariable(v)),
        }
        Err(KErr::new("invalid value"))
    }

    fn read_const(&self, bs:&mut &[u8]) -> Result<Tok<Constant>, KErr> {
        space(bs);

        let mut buf = String::with_capacity(64);
        while self.call_is_const_byte(peek(bs,0),buf.len()) {
            buf.push(read(bs)? as char);
        }

        let buflen = buf.len();
        if buflen==0 { return Ok(Pass); }

        let mut multiple = 1.0;
        match buf.as_bytes()[buflen-1] {
            b'k' | b'K' => {      multiple=1_000.0; buf.pop(); }
            b'M' => {         multiple=1_000_000.0; buf.pop(); }
            b'G' => {     multiple=1_000_000_000.0; buf.pop(); }
            b'T' => { multiple=1_000_000_000_000.0; buf.pop(); }
            _ => {}
        }

        let val = buf.parse::<f64>().map_err(|_| {
            KErr::new("parse<f64> error").pre(&buf)
        })?;
        Ok(Bite(Constant(val*multiple)))
    }

    fn read_var(&self, bs:&mut &[u8]) -> Result<Tok<Variable>, KErr> {
        space(bs);

        let mut buf = String::with_capacity(16);
        while self.call_is_var_byte(peek(bs,0),buf.len()) {
            buf.push(read(bs)? as char);
        }

        if buf.len()==0 { return Ok(Pass); }  // This is NOT a Pass after a read() -- len=0 so no read occurred.

        Ok(Bite(Variable(buf)))
    }

    fn read_unaryop(&self, slab:&mut ParseSlab, bs:&mut &[u8]) -> Result<Tok<UnaryOp>, KErr> {
        space(bs);
        match peek(bs,0) {
            None => Err(KErr::new("EOF")),
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
                    Ok(Bite(EParens(xi)))
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
                b'+' => { read(bs)?; Ok(Bite(EPlus)) }
                b'-' => { read(bs)?; Ok(Bite(EMinus)) }
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

    fn read_callable(&self, slab:&mut ParseSlab, bs:&mut &[u8]) -> Result<Tok<Callable>, KErr> {
        match self.read_printfunc(slab,bs)? {
            Pass => {}
            Bite(f) => return Ok(Bite(EPrintFunc(f))),
        }
        match self.read_evalfunc(slab,bs)? {
            Pass => {}
            Bite(f) => return Ok(Bite(EEvalFunc(f))),
        }
        match self.read_func(slab,bs)? {
            Pass => {}
            Bite(f) => return Ok(Bite(EFunc(f))),
        }
        Ok(Pass)
    }

    fn read_func_start(&self, bs:&mut &[u8], expected_name:Option<&str>) -> Result<Tok<String>, KErr> {
        space(bs);

        let mut name = String::with_capacity(16);  // TODO: Avoid allocation here.
        loop {
            match peek(bs,name.len()) {
                None => break,
                Some(b) => {
                    if self.call_is_func_byte(Some(b),name.len()) { name.push(b.to_ascii_lowercase() as char); }
                    else { break; }
                }
            }
        }
        if name.len()==0 { return Ok(Pass) }
        if let Some(xn) = expected_name {
            if name!=xn { return Ok(Pass) }
        }

        let mut post_name_spaces=0;
        while let Some(b) = peek(bs,name.len()+post_name_spaces) {
            if !is_space(b) { break; }
            post_name_spaces+=1;
        }
        if peek(bs,name.len()+post_name_spaces) != Some(b'(') { return Ok(Pass) }

        // Begin 'Bite':
        for _ in 0..name.len()+post_name_spaces { read(bs)?; }
        if read(bs)? != b'(' { return Err(KErr::new("expected '('")) }

        Ok(Bite(name))
    }
    fn read_func(&self, slab:&mut ParseSlab, bs:&mut &[u8]) -> Result<Tok<Func>, KErr> {
        let fname : String;
        match self.read_func_start(bs,None)? {
            Pass => return Ok(Pass),
            Bite(n) => fname=n,
        }

        let mut args = Vec::<ExpressionI>::with_capacity(8);

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
            args.push(self.read_expression(slab,bs,false).map_err(|e| e.pre("read_expression"))?);
        }

        match fname.as_str() {
            "int" => {
                if args.len()==1 { Ok(Bite(EFuncInt(args.pop().unwrap())))
                } else { Err(KErr::new("expected one arg")) }
            }
            "ceil" => {
                if args.len()==1 { Ok(Bite(EFuncCeil(args.pop().unwrap())))
                } else { Err(KErr::new("expected one arg")) }
            }
            "floor" => {
                if args.len()==1 { Ok(Bite(EFuncFloor(args.pop().unwrap())))
                } else { Err(KErr::new("expected one arg")) }
            }
            "abs" => {
                if args.len()==1 { Ok(Bite(EFuncAbs(args.pop().unwrap())))
                } else { Err(KErr::new("expected one arg")) }
            }
            "log" => {
                if args.len()==1 { Ok(Bite(EFuncLog{base:None, expr:args.pop().unwrap()}))
                } else if args.len()==2 {
                    let expr = args.pop().unwrap();
                    Ok(Bite(EFuncLog{base:Some(args.pop().unwrap()), expr:expr}))
                } else { Err(KErr::new("expected log(x) or log(base,x)")) }
            }
            "round" => {
                if args.len()==1 { Ok(Bite(EFuncRound{modulus:None, expr:args.pop().unwrap()}))
                } else if args.len()==2 {
                    let expr = args.pop().unwrap();
                    Ok(Bite(EFuncRound{modulus:Some(args.pop().unwrap()), expr:expr}))
                } else { Err(KErr::new("expected round(x) or round(modulus,x)")) }
            }
            "min" => {
                if args.len()>0 {
                    let first = args.remove(0);
                    Ok(Bite(EFuncMin{first:first, rest:args}))
                } else { Err(KErr::new("expected one or more args")) }
            }
            "max" => {
                if args.len()>0 {
                    let first = args.remove(0);
                    Ok(Bite(EFuncMax{first:first, rest:args}))
                } else { Err(KErr::new("expected one or more args")) }
            }

            "e" => {
                if args.len()==0 { Ok(Bite(EFuncE))
                } else { Err(KErr::new("expected no args")) }
            }
            "pi" => {
                if args.len()==0 { Ok(Bite(EFuncPi))
                } else { Err(KErr::new("expected no args")) }
            }

            "sin" => {
                if args.len()==1 { Ok(Bite(EFuncSin(args.pop().unwrap())))
                } else { Err(KErr::new("expected one arg")) }
            }
            "cos" => {
                if args.len()==1 { Ok(Bite(EFuncCos(args.pop().unwrap())))
                } else { Err(KErr::new("expected one arg")) }
            }
            "tan" => {
                if args.len()==1 { Ok(Bite(EFuncTan(args.pop().unwrap())))
                } else { Err(KErr::new("expected one arg")) }
            }
            "asin" => {
                if args.len()==1 { Ok(Bite(EFuncASin(args.pop().unwrap())))
                } else { Err(KErr::new("expected one arg")) }
            }
            "acos" => {
                if args.len()==1 { Ok(Bite(EFuncACos(args.pop().unwrap())))
                } else { Err(KErr::new("expected one arg")) }
            }
            "atan" => {
                if args.len()==1 { Ok(Bite(EFuncATan(args.pop().unwrap())))
                } else { Err(KErr::new("expected one arg")) }
            }
            "sinh" => {
                if args.len()==1 { Ok(Bite(EFuncSinH(args.pop().unwrap())))
                } else { Err(KErr::new("expected one arg")) }
            }
            "cosh" => {
                if args.len()==1 { Ok(Bite(EFuncCosH(args.pop().unwrap())))
                } else { Err(KErr::new("expected one arg")) }
            }
            "tanh" => {
                if args.len()==1 { Ok(Bite(EFuncTanH(args.pop().unwrap())))
                } else { Err(KErr::new("expected one arg")) }
            }

            _ => Err(KErr::new(&format!("undefined function: {}",fname))),
        }
    }

    fn read_printfunc(&self, slab:&mut ParseSlab, bs:&mut &[u8]) -> Result<Tok<PrintFunc>, KErr> {
        match self.read_func_start(bs,Some("print"))? {
            Pass => return Ok(Pass),
            Bite(_) => {}  // We already know this is 'print'.
        }

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
            if args.len()>0 {
                match read(bs) {
                    Ok(b',') | Ok(b';') => {}
                    _ => { return Err(KErr::new("expected ',' or ';'")) }
                }
            }
            args.push(self.read_expressionorstring(slab,bs)?);
        }

        Ok(Bite(PrintFunc(args)))
    }

    fn read_evalfunc(&self, slab:&mut ParseSlab, bs:&mut &[u8]) -> Result<Tok<EvalFunc>, KErr> {
        match self.read_func_start(bs,Some("eval"))? {
            Pass => return Ok(Pass),
            Bite(_) => {}  // We already know this is 'eval'.
        }

        let eval_expr = self.read_expression(slab,bs,false)?;
        let mut kwargs = Vec::<KWArg>::with_capacity(16);
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
                None => { return Err(KErr::new("reached end of input while parsing evalfunc")) }
            }
            match read(bs) {
                Ok(b',') | Ok(b';') => {}
                _ => { return Err(KErr::new("expected ',' or ';'")) }
            }
            let name : Variable;
            match self.read_var(bs) {
                Ok(Pass) => return Err(KErr::new("unexpected read_var pass")),
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

        Ok(Bite(EvalFunc{expr:eval_expr, kwargs:kwargs}))
    }

    fn read_expressionorstring(&self, slab:&mut ParseSlab, bs:&mut &[u8]) -> Result<ExpressionOrString, KErr> {
        match self.read_string(bs)? {
            Pass => {}
            Bite(s) => return Ok(EStr(s)),
        }
        Ok(EExpr(self.read_expression(slab,bs,false)?))
    }

    fn read_string(&self, bs:&mut &[u8]) -> Result<Tok<String>,KErr> {
        space(bs);

        match peek(bs,0) {
            None => return Err(KErr::new("EOF while reading opening quote of string")),
            Some(b'"') => { read(bs)?; }
            Some(_) => { return Ok(Pass) }
        }

        let mut buf = String::with_capacity(64);
        loop {
            let b = read(bs)?;
            if b==b'"' { break; }
            buf.push(b as char);
        }

        Ok(Bite(buf))
    }
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
        let p = Parser::new(None,None);
        assert!(p.call_is_var_byte(Some(b'a'),0));
        assert!(!p.call_is_const_byte(Some(b'a'),0));

        let p = Parser::new(Some(&|_:u8, _:usize| true), None);
        assert!(p.call_is_const_byte(Some(b'a'),0));

        let p = Parser::new(None,None);

        let mut slab = Slab::new();
        
        {
            let bsarr = b"12.34";
            let bs = &mut &bsarr[..];
            assert_eq!(p.read_value(&mut slab.ps, bs), Ok(EConstant(Constant(12.34))));
        }
    }
}

