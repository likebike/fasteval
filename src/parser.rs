use crate::slab::ParseSlab;
use kerr::KErr;

use std::str::{from_utf8, from_utf8_unchecked};


// === Algebra Grammar ===
//
// Expression: Value (BinaryOp Value)*
//
// Value: Constant || UnaryOp || PrintFunc || StdFunc
//
// Constant: [+-]?[0-9]*(\.[0-9]+)?( ([eE][+-]?[0-9]+) || [pnuµmkKMGT] )?  || [+-]?(NaN || inf)
//
// UnaryOp: +Value || -Value || (Expression) || [Expression] || !Value
//
// BinaryOp: + || - || * || / || % || ^ || < || <= || == || != || >= || > || (or || '||') || (and || '&&')
//
// VarName: [a-zA-Z_][a-zA-Z_0-9]*
//
// StdFunc: VarName((Expression,)*)?  ||  VarName[(Expression,)*]?
//
// PrintFunc: print(ExpressionOrString,*)
//
// ExpressionOrString: Expression || String
//
// String: ".*"



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
    EConstant(f64),
    EUnaryOp(UnaryOp),
    EStdFunc(StdFunc),
    EPrintFunc(PrintFunc),
}
use Value::{EConstant, EUnaryOp, EStdFunc, EPrintFunc};

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
pub enum StdFunc {
    EVar(String),
    #[cfg(feature="unsafe-vars")]
    EUnsafeVar{name:String, ptr:*const f64},
    EFunc{name:String, args:Vec<ExpressionI>},  // cap=4

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
#[cfg(feature="unsafe-vars")]
use StdFunc::EUnsafeVar;

#[derive(Debug, PartialEq)]
pub struct PrintFunc(pub Vec<ExpressionOrString>);  // cap=8

#[derive(Debug, PartialEq)]
pub enum ExpressionOrString {
    EExpr(ExpressionI),
    EStr(String),  // cap=64
}
use ExpressionOrString::{EExpr, EStr};



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



enum Tok<T> {
    Pass,
    Bite(T),
}
use Tok::{Pass, Bite};

fn peek(bs:&[u8]) -> Option<u8> {
    if !bs.is_empty() { Some(bs[0]) }
    else { None }
}
fn peek_n(bs:&[u8], skip:usize) -> Option<u8> {
    // This is slightly different than slice.get(i) because we return a u8, not a ref.
    if bs.len()>skip { Some(bs[skip]) }
    else { None }
}
fn is_at_eof(bs:&[u8]) -> bool { bs.is_empty() }
fn peek_is(bs:&[u8], skip:usize, val:u8) -> bool {
    match peek_n(bs,skip) {
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

fn skip(bs:&mut &[u8]) { *bs = &bs[1..]; }
fn skip_n(bs:&mut &[u8], n:usize) { *bs = &bs[n..]; }

fn is_space(b:u8) -> bool {
    if b>b' ' { return false; }
    b==b' ' || b==b'\n' || b==b'\t' || b==b'\r'
}
fn spaces(bs:&mut &[u8]) {
    while let Some(b) = peek(bs) {
        if !is_space(b) { break }
        skip(bs);  // We normally don't have long strings of whitespace, so it is more efficient to put this single-skip inside this loop rather than a skip_n afterwards.
    }
}


#[inline]
pub fn parse(slab:&mut ParseSlab, s:&str) -> Result<ExpressionI, KErr> {
    Parser.parse(slab, s)
}


pub struct Parser;

impl Parser {
    #[inline]
    pub fn new() -> Self { Self }

    fn is_varname_byte(b:u8, i:usize) -> bool {
        (b'A'<=b && b<=b'Z') || (b'a'<=b && b<=b'z') || b==b'_' || (i>0 && ( b'0'<=b && b<=b'9' ))
    }
    fn is_varname_byte_opt(bo:Option<u8>, i:usize) -> bool {
        match bo {
            Some(b) => Self::is_varname_byte(b,i),
            None => false,
        }
    }


    // I cannot return Result<&Expression> because it would prolong the mut:
    #[inline]
    pub fn parse(&self, slab:&mut ParseSlab, s:&str) -> Result<ExpressionI, KErr> {
        if s.len()>4096 { return Err(KErr::new("expression string is too long")); }  // Restrict length for safety
        let mut bs = s.as_bytes();
        Self::read_expression(slab, &mut bs, 0, true)
    }

    fn read_expression(slab:&mut ParseSlab, bs:&mut &[u8], depth:usize, expect_eof:bool) -> Result<ExpressionI, KErr> {
        if depth>=32 { return Err(KErr::new("too deep")) }

        let first = Self::read_value(slab,bs,depth).map_err(|e| e.pre("read_value"))?;
        let mut pairs = Vec::<ExprPair>::with_capacity(8);
        loop {
            match Self::read_binaryop(bs).map_err(|e| e.pre("read_binaryop"))? {
                Pass => break,
                Bite(bop) => {
                    let val = Self::read_value(slab,bs,depth).map_err(|e| e.pre("read_value"))?;
                    pairs.push(ExprPair(bop,val));
                }
            }
        }
        spaces(bs);
        if expect_eof && !is_at_eof(bs) {
            let bs_str = from_utf8(bs).map_err(|_| KErr::new("Utf8Error while handling 'unparsed tokens remaining' error."))?;
            return Err(KErr::new("unparsed tokens remaining").pre(bs_str));
        }
        Ok(slab.push_expr(Expression{first, pairs})?)
    }

    fn read_value(slab:&mut ParseSlab, bs:&mut &[u8], depth:usize) -> Result<Value, KErr> {
        if depth>=32 { return Err(KErr::new("too deep")) }

        match Self::read_const(slab,bs)? {
            Pass => {}
            Bite(c) => return Ok(EConstant(c)),
        }
        match Self::read_unaryop(slab,bs,depth)? {
            Pass => {}
            Bite(u) => return Ok(EUnaryOp(u)),
        }
        match Self::read_callable(slab,bs,depth)? {
            Pass => {}
            Bite(c) => return Ok(c),
        }
        Err(KErr::new("invalid value"))
    }

    fn read_const(slab:&mut ParseSlab, bs:&mut &[u8]) -> Result<Tok<f64>, KErr> {
        spaces(bs);

        let mut toklen=0;  let mut sign_ok=true;  let mut specials_ok=true;  let mut suffix_ok=true;  let mut saw_val=false;
        while toklen<bs.len() {
            let b = bs[toklen];
            if b'0'<=b && b<=b'9' || b==b'.' {
                saw_val = true;
                sign_ok=false; specials_ok=false;
                toklen += 1;
            } else if sign_ok && (b==b'-' || b==b'+') {
                sign_ok = false;
                toklen += 1;
            } else if saw_val && (b==b'e' || b==b'E') {
                suffix_ok = false;
                sign_ok = true;
                toklen += 1;
            } else if specials_ok && ( b==b'N' && peek_is(bs,toklen+1,b'a') && peek_is(bs,toklen+2,b'N')  ||  b==b'i' && peek_is(bs,toklen+1,b'n') && peek_is(bs,toklen+2,b'f') ) {
                #[cfg(feature="alpha-keywords")]
                {
                    saw_val = true;
                    suffix_ok = false;
                    toklen += 3;
                }
                break;
            } else {
                break;
            }
        }

        if !saw_val { return Ok(Pass); }

        let mut tok = unsafe { from_utf8_unchecked(&bs[..toklen]) };
        if suffix_ok {
            match peek_n(bs,toklen) {
                None => (),
                Some(b) => {
                    let (exp,suffixlen) = match b {
                        b'k' | b'K' => (3,1),
                        b'M' => (6,1),
                        b'G' => (9,1),
                        b'T' => (12,1),
                        b'm' => (-3,1),
                        b'u' | b'\xb5' => (-6,1),  // ASCII-encoded 'µ'
                        b'\xc2' if peek_is(bs,toklen+1,b'\xb5') => (-6,2),  // UTF8-encoded 'µ'
                        b'n' => (-9,1),
                        b'p' => (-12,1),
                        _ => (0,0),
                    };
                    if exp!=0 {
                        slab.char_buf.clear();
                        slab.char_buf.push_str(tok);
                        slab.char_buf.push('e');
                        slab.char_buf.push_str(&exp.to_string());
                        tok = &slab.char_buf;

                        toklen += suffixlen;
                    }
                }
            }
        }

        let val = tok.parse::<f64>().map_err(|_| { KErr::new("parse<f64> error").pre(tok) })?;
        skip_n(bs,toklen);

        Ok(Bite(val))
    }

    // // This implementation is beautiful and correct, but it is slow due to the fact that I am first parsing everything,
    // // and then I'm calling parse::<f64> which repeats the entire process.
    // // I wish I could just call dec2flt::convert() ( https://doc.rust-lang.org/src/core/num/dec2flt/mod.rs.html#247 )
    // // with all the pieces I already parsed, but, alas, that function is private.
    // //
    // // Also, I have decided that I really do need to support 'NaN' and 'inf'.  I could add them below, but instead,
    // // I think I will just switch back to a drastically-simplified parser which isn't as "correct", but re-uses
    // // more functionality from the stdlib.
    // //
    // // As a side-note, It's surprising how similar these algorithms are (which I created from scratch at 3am with no reference),
    // // compared to the dec2flt::parse module.
    // fn read_const(&mut self, bs:&mut &[u8]) -> Result<Tok<f64>, KErr> {
    //     spaces(bs);
    //
    //     // Grammar: [+-]?[0-9]*(\.[0-9]+)?( ([eE][+-]?[0-9]+) || [pnuµmkKMGT] )?
    //     fn peek_digits(bs:&[u8]) -> usize {
    //         let mut i = 0;
    //         while i<bs.len() && b'0'<=bs[i] && bs[i]<=b'9' { i+=1; }
    //         i
    //     }
    //     fn peek_exp(bs:&[u8]) -> Result<usize, KErr> {
    //         if bs.is_empty() { return Err(KErr::new("peek_exp empty")); }
    //         let mut i = 0;
    //         if bs[i]==b'-' || bs[i]==b'+' { i+=1; }
    //         let digits = peek_digits(&bs[i..]);
    //         if digits==0 { return Err(KErr::new("peek_exp no digits")); }
    //         Ok(i+digits)
    //     }
    //     fn peek_tail(bs:&[u8]) -> Result<(/*read:*/usize, /*skip:*/usize, /*exp:*/i32), KErr> {
    //         if bs.is_empty() { return Ok((0,0,0)); }
    //         match bs[0] {
    //             b'k' | b'K' => Ok((0,1,3)),
    //             b'M' => Ok((0,1,6)),
    //             b'G' => Ok((0,1,9)),
    //             b'T' => Ok((0,1,12)),
    //             b'm' => Ok((0,1,-3)),
    //             b'u' | b'\xb5' => Ok((0,1,-6)),  // ASCII-encoded 'µ'
    //             b'\xc2' if bs.len()>1 && bs[1]==b'\xb5' => Ok((0,2,-6)),  // UTF8-encoded 'µ'
    //             b'n' => Ok((0,1,-9)),
    //             b'p' => Ok((0,1,-12)),
    //             b'e' | b'E' => peek_exp(&bs[1..]).map(|size| (1+size,0,0)),
    //             _ => Ok((0,0,0)),
    //         }
    //     }
    //
    //     let mut toread=0;  let mut toskip=0;  let mut exp=0;
    //
    //     match peek(bs, 0) {
    //         None => return Ok(Pass), 
    //         Some(b) => {
    //             if b==b'-' || b==b'+' { toread+=1; }
    //         }
    //         
    //     }
    //
    //     let predec = peek_digits(&bs[toread..]);
    //     toread+=predec;
    //
    //     match peek(bs, toread) {
    //         None => {
    //             if predec==0 { return Ok(Pass); }
    //         }
    //         Some(b) => {
    //             if b==b'.' {
    //                 toread+=1;
    //                 let postdec = peek_digits(&bs[toread..]);
    //                 if predec==0 && postdec==0 { return Err(KErr::new("decimal without pre- or post-digits")); }
    //                 toread+=postdec;
    //             } else {
    //                 if predec==0 { return Ok(Pass); }
    //             }
    //             let (rd,sk,ex) = peek_tail(&bs[toread..])?;
    //             toread+=rd;  toskip=sk;  exp=ex;
    //         }
    //     }
    //
    //     self.char_buf.clear();
    //     for _ in 0..toread { self.char_buf.push(read(bs)? as char); }
    //     for _ in 0..toskip { read(bs)?; }
    //     if exp!=0 { self.char_buf.push('e'); self.char_buf.push_str(&exp.to_string()); }
    //
    //     let val = self.char_buf.parse::<f64>().map_err(|_| {
    //         KErr::new("parse<f64> error").pre(&self.char_buf)
    //     })?;
    //     Ok(Bite(val))
    // }

    fn read_unaryop(slab:&mut ParseSlab, bs:&mut &[u8], depth:usize) -> Result<Tok<UnaryOp>, KErr> {
        spaces(bs);
        match peek(bs) {
            None => Ok(Pass),  // Err(KErr::new("EOF at UnaryOp position")), -- Instead of erroring, let the higher level decide what to do.
            Some(b) => match b {
                b'+' => {
                    skip(bs);
                    let v = Self::read_value(slab,bs,depth+1)?;
                    Ok(Bite(EPos(slab.push_val(v)?)))
                }
                b'-' => {
                    skip(bs);
                    let v = Self::read_value(slab,bs,depth+1)?;
                    Ok(Bite(ENeg(slab.push_val(v)?)))
                }
                b'(' => {
                    skip(bs);
                    let xi = Self::read_expression(slab,bs,depth+1,false)?;
                    spaces(bs);
                    if read(bs)? != b')' { return Err(KErr::new("Expected ')'")) }
                    Ok(Bite(EParentheses(xi)))
                }
                b'[' => {
                    skip(bs);
                    let xi = Self::read_expression(slab,bs,depth+1,false)?;
                    spaces(bs);
                    if read(bs)? != b']' { return Err(KErr::new("Expected ']'")) }
                    Ok(Bite(EParentheses(xi)))
                }
                b'!' => {
                    skip(bs);
                    let v = Self::read_value(slab,bs,depth+1)?;
                    Ok(Bite(ENot(slab.push_val(v)?)))
                }
                _ => Ok(Pass),
            }
        }
    }

    fn read_binaryop(bs:&mut &[u8]) -> Result<Tok<BinaryOp>, KErr> {
        spaces(bs);
        match peek(bs) {
            None => Ok(Pass), // Err(KErr::new("EOF")), -- EOF is usually OK in a BinaryOp position.
            Some(b) => match b {
                b'+' => { skip(bs); Ok(Bite(EAdd)) }
                b'-' => { skip(bs); Ok(Bite(ESub)) }
                b'*' => { skip(bs); Ok(Bite(EMul)) }
                b'/' => { skip(bs); Ok(Bite(EDiv)) }
                b'%' => { skip(bs); Ok(Bite(EMod)) }
                b'^' => { skip(bs); Ok(Bite(EExp)) }
                b'<' => { skip(bs);
                          if peek_is(bs,0,b'=') { skip(bs); Ok(Bite(ELTE)) }
                          else { Ok(Bite(ELT)) } }
                b'>' => { skip(bs);
                          if peek_is(bs,0,b'=') { skip(bs); Ok(Bite(EGTE)) }
                          else { Ok(Bite(EGT)) } }
                b'=' if peek_is(bs,1,b'=') => { skip_n(bs,2);
                                                Ok(Bite(EEQ)) }
                b'!' if peek_is(bs,1,b'=') => { skip_n(bs,2);
                                                Ok(Bite(ENE)) }
                #[cfg(feature="alpha-keywords")]
                b'o' if peek_is(bs,1,b'r') => { skip_n(bs,2);
                                                Ok(Bite(EOR)) }
                b'|' if peek_is(bs,1,b'|') => { skip_n(bs,2);
                                                Ok(Bite(EOR)) }
                #[cfg(feature="alpha-keywords")]
                b'a' if peek_is(bs,1,b'n') && peek_is(bs,2,b'd') => { skip_n(bs,3);
                                                                      Ok(Bite(EAND)) }
                b'&' if peek_is(bs,1,b'&') => { skip_n(bs,2);
                                                Ok(Bite(EAND)) }
                _ => Ok(Pass),
            }
        }
    }

    fn read_callable(slab:&mut ParseSlab, bs:&mut &[u8], depth:usize) -> Result<Tok<Value>, KErr> {
        match Self::read_varname(bs)? {
            Pass => Ok(Pass),
            Bite(varname) => {
                match Self::read_open_parenthesis(bs)? {
                    Pass => {
                        // VarNames without Parenthesis are always treated as custom 0-arg functions.

                        #[cfg(feature="unsafe-vars")]
                        match slab.unsafe_vars.get(&varname) {
                            None => Ok(Bite(EStdFunc(EVar(varname)))),
                            Some(&ptr) => Ok(Bite(EStdFunc(EUnsafeVar{name:varname, ptr})))
                        }

                        #[cfg(not(feature="unsafe-vars"))]
                        Ok(Bite(EStdFunc(EVar(varname))))
                    }
                    Bite(open_parenth) => {
                        // VarNames with Parenthesis are first matched against builtins, then custom.
                        match varname.as_ref() {
                            "print" => Ok(Bite(EPrintFunc(Self::read_printfunc(slab,bs,depth,open_parenth)?))),
                            _ => Ok(Bite(EStdFunc(Self::read_func(varname,slab,bs,depth,open_parenth)?))),
                        }
                    }
                }
            }
        }
    }

    fn read_varname(bs:&mut &[u8]) -> Result<Tok<String>, KErr> {
        spaces(bs);

        let mut toklen = 0;
        while Self::is_varname_byte_opt(peek_n(bs,toklen),toklen) { toklen+=1; }

        if toklen==0 { return Ok(Pass); }

        let out = unsafe { from_utf8_unchecked(&bs[..toklen]) }.to_string();
        skip_n(bs, toklen);
        Ok(Bite(out))
    }

    fn read_open_parenthesis(bs:&mut &[u8]) -> Result<Tok<u8>, KErr> {
        spaces(bs);

        match peek(bs) {
            Some(b'(') | Some(b'[') => Ok(Bite(read(bs).unwrap())),
            _ => Ok(Pass),
        }
    }

    fn read_func(fname:String, slab:&mut ParseSlab, bs:&mut &[u8], depth:usize, open_parenth:u8) -> Result<StdFunc, KErr> {
        let close_parenth = match open_parenth {
            b'(' => b')',
            b'[' => b']',
            _ => unreachable!(),
        };
        let mut args = Vec::<ExpressionI>::with_capacity(4);
        loop {
            spaces(bs);
            match peek(bs) {
                Some(b) => {
                    if b==close_parenth {
                        skip(bs);
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
            args.push(Self::read_expression(slab,bs,depth+1,false).map_err(|e| e.pre("read_expression"))?);
        }

        let fname_str = fname.as_str();
        match fname_str {
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

            _ => {
                #[cfg(feature="unsafe-vars")]
                match slab.unsafe_vars.get(fname_str) {
                    None => Ok(EFunc{name:fname, args}),
                    Some(&ptr) => Ok(EUnsafeVar{name:fname, ptr}),
                }

                #[cfg(not(feature="unsafe-vars"))]
                Ok(EFunc{name:fname, args})
            }
        }
    }

    fn read_printfunc(slab:&mut ParseSlab, bs:&mut &[u8], depth:usize, open_parenth:u8) -> Result<PrintFunc, KErr> {
        let close_parenth = match open_parenth {
            b'(' => b')',
            b'[' => b']',
            _ => unreachable!(),
        };
        let mut args = Vec::<ExpressionOrString>::with_capacity(8);
        loop {
            spaces(bs);
            match peek(bs) {
                Some(b) => {
                    if b==close_parenth {
                        skip(bs);
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
            args.push(Self::read_expressionorstring(slab,bs,depth+1)?);
        }

        Ok(PrintFunc(args))
    }

    fn read_expressionorstring(slab:&mut ParseSlab, bs:&mut &[u8], depth:usize) -> Result<ExpressionOrString, KErr> {
        match Self::read_string(bs)? {
            Pass => {}
            Bite(s) => return Ok(EStr(s)),
        }
        Ok(EExpr(Self::read_expression(slab,bs,depth+1,false)?))
    }

    fn read_string(bs:&mut &[u8]) -> Result<Tok<String>,KErr> {
        spaces(bs);

        match peek(bs) {
            None => return Err(KErr::new("EOF while reading opening quote of string")),
            Some(b'"') => { skip(bs); }
            Some(_) => { return Ok(Pass) }
        }

        let mut toklen = 0;
        while match peek_n(bs,toklen) {
            None => false,
            Some(b'"') => false,
            Some(_) => true,
        } { toklen+=1; }

        let out = from_utf8(&bs[..toklen]).map_err(|_| KErr::new("Utf8Error while reading string"))?;
        skip_n(bs, toklen);
        match read(bs) {
            Err(e) => Err(e.pre("error while searching for closing double-quote")),
            Ok(b'"') => Ok(Bite(out.to_string())),
            Ok(_) => unreachable!(),
        }
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

            assert_eq!(peek(bs), Some(1));
            assert_eq!(peek_n(bs,1), Some(2));
            assert_eq!(peek_n(bs,2), Some(3));
            assert_eq!(peek_n(bs,3), None);

            assert_eq!(read(bs)?, 1);
            skip(bs);
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
            spaces(bs);
            assert_eq!(bs, b"abc 123   ");
        }
    }

    #[test]
    fn priv_tests() {
        assert!(Parser::is_varname_byte_opt(Some(b'a'),0));

        let mut slab = Slab::new();
        
        {
            let bsarr = b"12.34";
            let bs = &mut &bsarr[..];
            assert_eq!(Parser::read_value(&mut slab.ps, bs, 0), Ok(EConstant(12.34)));
        }
    }

    // #[bench]
    // #[allow(non_snake_case)]
    // fn spaces_1M(bencher:&mut Bencher) {
    //     let zero = "abc".as_bytes();
    //     let one = " abc".as_bytes();
    //     let two = "  abc".as_bytes();
    //     bencher.iter(|| {
    //         for _ in 0..1000 {
    //             let (z1,z2,z3,z4) = (&mut &zero[..], &mut &zero[..], &mut &zero[..], &mut &zero[..]);
    //             let (o1,o2) = (&mut &one[..], &mut &one[..]);
    //             let t1 = &mut &two[..];
    //             spaces(z1);
    //             spaces(z2);
    //             spaces(z3);
    //             spaces(z4);
    //             spaces(o1);
    //             spaces(o2);
    //             spaces(t1);
    //             black_box(z1);
    //             black_box(z2);
    //             black_box(z3);
    //             black_box(z4);
    //             black_box(o1);
    //             black_box(o2);
    //             black_box(t1);
    //         }
    //     });
    // }
}

