use std::fmt;
use std::rc::Rc;

pub struct Error {
    err   : String,
    chain : LList<String>,
}

impl Error {
    pub fn new(err:String) -> Self {
        Error{err:err, chain:LList(None)}
    }
    pub fn add(mut self, info:String) -> Self {
        self.chain = self.chain.prepend(info);
        self
    }
}

// An immutable linked list, perfectly designed for our error chain:
struct LList<T>(Option<Rc<LLNode<T>>>) where T:Clone;
struct LLNode<T> where T:Clone {
    el  : T,
    next: LList<T>,
}

impl<T> LList<T> where T:Clone {
    fn new() -> Self { LList(None) }
    fn prepend(&self, el:T) -> Self {
        LList(Some(Rc::new(LLNode{el:el,
                                  next:match self.0 {
                                           Some(rc) => LList(Some(Rc::clone(&rc))),
                                           None => LList(None),
                                       }})))
    }
}

struct Iter<T>(LList<T>) where T:Clone;

impl<T> Iterator for Iter<T> where T:Clone {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        (self.0).0.map(|rc:Rc<LLNode<T>>| {
            self.0 = rc.next;
            rc.el.clone()
        })
    }
}

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
