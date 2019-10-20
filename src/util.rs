
pub fn bool_to_f64(b:bool) -> f64 {
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
    }
}

