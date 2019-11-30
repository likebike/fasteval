#[derive(Debug)]
pub struct BufStack<T> {
    bufs:Vec<Vec<T>>,
    next:usize,
}
#[derive(Debug, Copy, Clone)]
pub struct BufI(usize);

impl<T> BufStack<T> {
    #[inline]
    pub fn new() -> Self {
        Self {
            bufs:Vec::new(),
            next:0,
        }
    }

    pub fn push_buf(&mut self, cap:usize) -> BufI {
        let len = self.bufs.len();
        if self.next>len { unreachable!(); }
        if self.next==len {
            self.bufs.push(Vec::with_capacity(cap));
        } else {
            self.bufs[self.next].reserve(cap);
        }
        let bufi = BufI(self.next);
        self.next += 1;
        bufi
    }

    pub fn pop_buf(&mut self, bufi:BufI) -> Vec<T> {
        if bufi.0!=self.next-1 { panic!("out-of-order pop"); }
        if self.next==0 { panic!("underflow") };
        let src = &mut self.bufs[self.next-1];
        let mut dst = Vec::with_capacity(src.len());
        src.reverse();
        while src.len()!=0 {
            dst.push(src.pop().unwrap());
        }
        self.next -= 1;
        dst
    }

    #[inline]
    pub fn push(&mut self, bufi:BufI, value:T) {
        self.bufs[bufi.0].push(value)
    }
}

#[cfg(test)]
mod tests {
    use super::BufStack;

    #[test]
    fn basics() {
        let mut bufs = BufStack::<u8>::new();
        let v0i = bufs.push_buf(8);
        bufs.push(v0i,1);
        bufs.push(v0i,2);
        bufs.push(v0i,3);
        let v1i = bufs.push_buf(8);
        bufs.push(v1i,4);
        bufs.push(v1i,5);
        let v1 = bufs.pop_buf(v1i);
        bufs.push(v0i,6);
        bufs.push(v0i,7);
        let v0 = bufs.pop_buf(v0i);
        eprintln!("v0:{:?}  v1:{:?}",v0,v1);
    }
}

