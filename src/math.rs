fn gcd(a : i32, b : i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub(crate) fn fraction(num : i32, den : i32) -> (i32, i32) {
    let divisor = gcd(num.abs(), den.abs());
    (num / divisor, den / divisor)
}
