use crate::error::{Error};


// Vec seems really inefficient to me because remove() does not just increment the internal pointer -- it shifts data all around.  There's also split_* methods but they seem to be designed to return new Vecs, not modify self.
// Just use slices instead, which I know will be very efficient:
fn read_byte(bs:&mut &[u8]) -> Result<u8, Error> {
    if bs.len() > 0 {
        let b = bs[0];
        *bs = &bs[1..];
        Ok(b)
    } else { Err(Error::EOF) }
}
fn peek_byte(bs:&[u8], skip:usize) -> Option<u8> {
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
fn space(bs:&mut &[u8]) {
    while let Some(b) = peek_byte(bs,0) {
        if !is_space(b) { break }
        let _ = read_byte(bs);
    }
}

fn bool_to_f64(b:bool) -> f64 {
    if b { 1.0 }
    else { 0.0 }
}

//---- Tests:

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn util() {
        assert_eq!(bool_to_f64(true), 1.0);
        assert_eq!(bool_to_f64(false), 0.0);

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
            assert_eq!(read_byte(bs).err(), Some(Error::EOF));

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
}

