pub fn pymod(a: isize, b: isize) -> isize {
    let r = a % b;
    // If r and b differ in sign, add b to wrap the result to the correct sign.
    if (r > 0 && b < 0) || (r < 0 && b > 0) {
        return r + b;
    }
    r
}

pub fn is_some_and_not_empty<T>(v: &Option<Vec<T>>) -> bool {
    match v {
        Some(v) => !v.is_empty(),
        None => false,
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn python_mod(){
        assert_eq!(pymod(2, -3), -1);
        assert_eq!(pymod(-2, 3), 1);
        assert_eq!(pymod(-2, -3), -2);
        assert_eq!(pymod(-3, -3), 0);
        assert_eq!(pymod(3, 3), 0);
        assert_eq!(pymod(2, 3), 2);
        assert_eq!(pymod(4, 3), 1);
        assert_eq!(pymod(3, 3), 0);
        assert_eq!(pymod(6, 3), 0);
        assert_eq!(pymod(-6, 3), 0);
        assert_eq!(pymod(-6, -3), 0);
        assert_eq!(pymod(6, -3), 0);
    }
}