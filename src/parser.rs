
struct Parser<'a> {
    is_const_byte:Option<&'a Fn(&str)->Option<f64>>,
    is_func_byte :Option<&'a Fn(&str)->Option<f64>>,
    is_var_byte  :Option<&'a Fn(&str)->Option<f64>>,
}








// impl<F> Parser<F> where F:Fn(u8,usize)->bool {
//     fn default_is_const_byte(b:u8, i:usize) -> bool {
//         if b'0'<=b && b<=b'9' || b==b'.' { return true }
//         if i>0 && ( b==b'k' || b==b'K' || b==b'M' || b==b'G' || b==b'T' ) { return true }
//         return false
//     }
//     fn default_is_var_byte(b:u8, i:usize) -> bool {
//         (b'A'<=b && b<=b'Z') || (b'a'<=b && b<=b'z') || b==b'_' || (i>0 && b'0'<=b && b<=b'9')
//     }
// 
//     fn call_is_const_byte(&self, b:u8, i:usize) -> bool {
//         match self.is_const_byte {
//             Some(f) => f(b,i),
//             None => Parser::default_is_const_byte(b,i),
//         }
//     }
// //    fn call_is_func_byte(&self, b:u8, i:usize) -> bool {
// //        match self.is_func_byte {
// //            Some(f) => f(b,i),
// //            None => Parser::default_is_var_byte(b,i),
// //        }
// //    }
// //    fn call_is_var_byte(&self, b:u8, i:usize) -> bool {
// //        match self.is_var_byte {
// //            Some(f) => f(b,i),
// //            None => Parser::default_is_var_byte(b,i),
// //        }
// //    }
// }



//---- Tests:

#[cfg(test)]
mod tests {
//     #[test]
//     fn parser() {
//         let p = Parser{
//             is_const_byte:None,
// //            is_func_byte:None,
// //            is_var_byte:None,
//         };
// //        assert!(p.call_is_func_byte(b'a',0));
// //        assert!(p.call_is_var_byte(b'a',0));
//         assert!(!p.call_is_const_byte(b'a',0));
// 
//         let p = Parser{
//             is_const_byte:Some(|b:u8, i:usize| true),
// //            is_func_byte:None,
// //            is_var_byte:None,
//         };
//         assert!(p.call_is_const_byte(b'a',0));
//         
//     }
}

