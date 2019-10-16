use crate::grammar::{Expression, Value::{self, EConstant}, Constant};
//use crate::evaler::Evaler;
use crate::error::Error;



// Vec seems really inefficient to me because remove() does not just increment the internal pointer -- it shifts data all around.  There's also split_* methods but they seem to be designed to return new Vecs, not modify self.
// Just use slices instead, which I know will be very efficient:
pub fn read_byte(bs:&mut &[u8]) -> Result<u8, Error> {
    if bs.len() > 0 {
        let b = bs[0];
        *bs = &bs[1..];
        Ok(b)
    } else { Err(Error::new("EOF".to_string())) }
}
pub fn peek_byte(bs:&[u8], skip:usize) -> Option<u8> {
    if bs.len() > skip { Some(bs[skip])
    } else { None }
}
fn is_at_eof(bs:&[u8]) -> bool { bs.len() == 0 }

fn is_space(b:u8) -> bool {
    match b {
    b' ' | b'\t' | b'\r' | b'\n' => true,
    _ => false,
    }
}
pub fn space(bs:&mut &[u8]) {
    while let Some(b) = peek_byte(bs,0) {
        if !is_space(b) { break }
        let _ = read_byte(bs);
    }
}



struct Parser<'a> {
    is_const_byte:Option<&'a dyn Fn(u8,usize)->bool>,
    is_func_byte :Option<&'a dyn Fn(u8,usize)->bool>,
    is_var_byte  :Option<&'a dyn Fn(u8,usize)->bool>,
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
    fn call_is_func_byte(&self, bo:Option<u8>, i:usize) -> bool {
        match bo {
            Some(b) => match self.is_func_byte {
                Some(f) => f(b,i),
                None => Parser::default_is_var_byte(b,i),
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

    pub fn parse(&self, s:&str) -> Result<Expression, Error> {
        let bs = &mut s.as_bytes();
        self.read_expression(bs, true)
    }

    fn read_expression(&self, bs:&mut &[u8], expect_eof:bool) -> Result<Expression, Error> {
        unimplemented!();
    }

    fn read_value(&self, bs:&mut &[u8]) -> Result<Value, Error> {
        if self.peek_const(bs) {
            return match self.read_const(bs) {
                Ok(constant) => Ok(EConstant(constant)),
                Err(err) => Err(err),
            }
        }
        //if self.peek_unaryop(bs) { return self.read_unaryop(bs) }
        //if self.peek_callable(bs) { return self.read_callable(bs) }
        //if self.peek_var(bs) { return self.read_var(bs) }
        Err(Error::new("InvalidValue".to_string()))
    }

    fn peek_const(&self, bs:&mut &[u8]) -> bool {
        space(bs);
        self.call_is_const_byte(peek_byte(bs,0),0)
    }
    fn read_const(&self, bs:&mut &[u8]) -> Result<Constant, Error> {
        space(bs);
        let mut buf : Vec<u8> = Vec::with_capacity(16);
        while self.call_is_const_byte(peek_byte(bs,0),buf.len()) { buf.push(read_byte(bs).map_err(|err| err.add("read_byte".to_string()))?); }
        let multiple = 1.0;
        if buf.len()>0 {
            match buf.last().unwrap() {
                b'k' | b'K' => {   multiple=1000.0; buf.pop(); }
                b'M' => {       multiple=1000000.0; buf.pop(); }
                b'G' => {    multiple=1000000000.0; buf.pop(); }
                b'T' => { multiple=1000000000000.0; buf.pop(); }
                _ => {}
            }
        }
unimplemented!();
    }
}



//---- Tests:

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn util() {
        match (|| -> Result<(),Error> {
            let bsarr = [1,2,3];
            let bs = &mut &bsarr[..];

            assert_eq!(peek_byte(bs,0), Some(1));
            assert_eq!(peek_byte(bs,1), Some(2));
            assert_eq!(peek_byte(bs,2), Some(3));
            assert_eq!(peek_byte(bs,3), None);

            assert_eq!(read_byte(bs)?, 1);
            assert_eq!(read_byte(bs)?, 2);
            assert_eq!(read_byte(bs)?, 3);
            match read_byte(bs).err() {
                Some(Error{}) => {}
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
    fn parser() {
        let p = Parser{
            is_const_byte:None,
            is_func_byte:None,
            is_var_byte:None,
        };
        assert!(p.call_is_func_byte(Some(b'a'),0));
        assert!(p.call_is_var_byte(Some(b'a'),0));
        assert!(!p.call_is_const_byte(Some(b'a'),0));

        let p = Parser{
            is_const_byte:Some(&|_:u8, _:usize| true),
            is_func_byte:None,
            is_var_byte:None,
        };
        assert!(p.call_is_const_byte(Some(b'a'),0));
        
    }
}

