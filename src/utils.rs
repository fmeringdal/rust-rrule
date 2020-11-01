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
