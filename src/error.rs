use std::fmt;
use std::rc::Rc;
use std::mem;

#[derive(Debug)]
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
struct LList<T>(Option<Rc<LLNode<T>>>);
struct LLNode<T> {
    el  : T,
    next: LList<T>,
}

impl<T> LList<T> {
    fn new() -> Self { LList(None) }
    fn prepend(&self, el:T) -> Self {
        LList(Some(Rc::new(LLNode{el:el,
                                  next:LList::clone(self)})))
    }
    fn head<'a>(&'a self) -> Option<&'a T> {
        self.0.as_ref().map(|rc| &rc.el)
    }

    fn iter(&self) -> Iter<T> { Iter(LList::clone(self)) }
}

impl<T> Clone for LList<T> {
    fn clone(&self) -> Self {
        match self.0 {
            Some(ref rc) => LList(Some(Rc::clone(rc))),
            None => LList(None),
        }
    }
}

impl<T> fmt::Display for LList<T> where T:fmt::Display {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "[")?;

        let mut nonempty = false;
        for node in self.iter() {
            nonempty = true;
            write!(f, " {}", node.head().unwrap())?;
        }

        if nonempty { write!(f, " ")?; }
        write!(f, "]")?;
        Ok(())
    }
}
impl<T> fmt::Debug for LList<T> where T:fmt::Debug {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "LList[")?;

        let mut nonempty = false;
        for node in self.iter() {
            nonempty = true;
            write!(f, " {:?}", node.head().unwrap())?;
        }

        if nonempty { write!(f, " ")?; }
        write!(f, "]")?;
        Ok(())
    }
}

struct Iter<T>(LList<T>);

impl<T> Iterator for Iter<T> {
    type Item = LList<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref rc) = (self.0).0 {
            let next = LList::clone(&rc.next);
            Some(mem::replace(&mut self.0, next))
        } else {
            None
        }
    }
}

//---- Tests:

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn llist() {
        let l : LList<String> = LList::new();
        eprintln!("list #1: {}", l);
        eprintln!("list #1: {:?}", l);
        let l = l.prepend("a".to_string());
        eprintln!("list #2: {}", l);
        eprintln!("list #2: {:?}", l);
        let l = l.prepend("b".to_string());
        eprintln!("list #3: {}", l);
        eprintln!("list #3: {:?}", l);
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
