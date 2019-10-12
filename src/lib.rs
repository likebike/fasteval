

// === Algebra Grammar ===
//
// Expression: Value (BinaryOp Value)*
//
// Value: Constant || UnaryOp || Callable || Variable
//
// BinaryOp: + || - || * || / || % || ^ || < || <= || == || != || >= || > || or || and
//
// Constant: (\.[0-9])+(k || K || M || G || T)?
//
// UnaryOp: +Value || -Value || (Expression) || !Value
//
// Callable: Function || PrintFunc || EvalFunc
//
// Function: Variable(Expression(,Expression)*)
//
// Variable: [a-zA-Z_][a-zA-Z_0-9]*
//
// PrintFunc: print(ExpressionOrString,*)
//
// ExpressionOrString: Expression || String
//
// String: ".*"
//
// EvalFunc: eval(Expression(,Variable=Expression)*)

struct Expression([ExpressionTok]);

enum ExpressionTok {
    EValue(Value),
    EBinaryOp(BinaryOp),
}

enum Value {
    EConstant(),
//  EUnaryOp,
//  ECallable,
//  EVariable,
}

enum BinaryOp {
    EPlus,
//  EMinus,
//  EMul,
//  EDiv,
//  EMod,
//  EExp,
//  ELT,
//  ELTE,
//  EEQ,
//  ENE,
//  EGTE,
//  EGT,
//  EOR,
//  EAND,
}

struct Constant(String);



#[derive(PartialEq, Debug)]
enum Error {
    EOF
}

fn bool_to_f64(b:bool) -> f64 {
    if b { 1.0 }
    else { 0.0 }
}

// Vec seems really inefficient to me because remove() does not just increment the internal pointer -- it shifts data all around.  There's also split_* methods but they seem to be designed to return new Vecs, not modify self.
// Just use slices instead, which I know will be very efficient:
fn read_byte(bs:&mut &[u8]) -> Result<u8, Error> {
    if bs.len() > 0 {
        let b = bs[0];
        *bs = &bs[1..];
        Ok(b)
    } else { Err(Error::EOF) }
}

// fn PeekByte(in, skip u8) -> u8 {
//     unimplemented!();
// }

// trait Parser {
//     fn is_const_byte(u8, i32) -> bool;
// }


#[cfg(test)]
mod tests {
    use super::{Error, bool_to_f64, read_byte};

    #[test]
    fn basics() {
        assert_eq!(bool_to_f64(true), 1.0);
        assert_eq!(bool_to_f64(false), 0.0);

        match (|| -> Result<(),Error> {
            let bsarr = [1,2,3];
            let bs = &mut &bsarr[..];

            assert_eq!(read_byte(bs)?, 1);
            assert_eq!(read_byte(bs)?, 2);
            assert_eq!(read_byte(bs)?, 3);
            assert_eq!(read_byte(bs).err(), Some(Error::EOF));

            Ok(())
        })() {
            Ok(_) => {}
            Err(_) => {
                unimplemented!();
            }
        }
    }
}

