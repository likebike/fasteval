use std::fmt;

pub struct Error {
    err   : String,
    chain : LList<String>,
}

impl Error {
    pub fn new(err:String) -> Self {
        Error{err:err, chain:Vec::new()}
    }
    pub fn add(el:String) -> Self {
        Mutating an error is just a bad idea.  I need a mutation-free implementation.
    }
}

// An immutable linked list, perfectly designed for our error chain:
struct LList<T>(Option<Rc<LLNode<T>>>)
struct LLNode<T> {
    elem: T,
    next: LList<T>,
}

impl<T> LList<T> {
    fn new() -> Self { LList(None) }
    fn prepend(&self, elem:T) -> Self {
        LList(Some(Rc::new(LLNode{elem:elem,
                                  next:match self {
                                           Some(rc) => Some(Rc::clone(rc)),
                                           None => None,
                                       }})))
    }
}

HERE I AM, implement Iterator.

// #[macro_export]
// macro_rules! errf {
//     ( $msg:expr ) => {
//         |err| { format!($msg, err) }
//     };
// }

// pub enum Error {
//     EOF,
//     AlreadyExists,
// 
//     InvalidValue,
// }
// 
// impl fmt::Display for Error {
//     fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             EOF           => write!(f, "EOF"),
//             AlreadyExists => write!(f, "AlreadyExists"),
//             InvalidValue  => write!(f, "InvalidValue"),
//         }
//     }
// }
