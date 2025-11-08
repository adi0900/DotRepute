//! Mathematical helper functions

/// Fast exponentiation
pub fn pow(base: u64, exp: u32) -> u64 {
    let mut result = 1u64;
    let mut b = base;
    let mut e = exp;
    
    while e > 0 {
        if e & 1 == 1 {
            result = result.saturating_mul(b);
        }
        b = b.saturating_mul(b);
        e >>= 1;
    }
    
    result
}

/// Integer square root
pub fn sqrt(n: u64) -> u64 {
    if n < 2 {
        return n;
    }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

/// Greatest common divisor
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow() {
        assert_eq!(pow(2, 10), 1024);
    }

    #[test]
    fn test_sqrt() {
        assert_eq!(sqrt(16), 4);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
    }
}
