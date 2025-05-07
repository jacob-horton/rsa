use crate::bits;

// Find the multiplicative inverse of `n` (mod `modulus`)
pub fn multiplicative_inverse(n: u32, modulus: u32) -> u32 {
    // Extended Euclid Algorithm
    let (mut old_r, mut r): (i64, i64) = (n as i64, modulus as i64);
    let (mut old_s, mut s): (i64, i64) = (1, 0);
    let (mut old_t, mut t): (i64, i64) = (0, 1);

    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
        (old_t, t) = (t, old_t - q * t);
    }

    return old_s.rem_euclid(modulus as i64) as u32;
}

// Raise `n` to the power `p` (mod `modulus`)
pub fn modulo_power(n: u32, p: u32, modulus: u32) -> u32 {
    let m = modulus as u64;

    // Need u64 to prevent overflow
    let mut result: u64 = 1;

    for bit in bits::get_bits(p) {
        // Always square
        result = result.pow(2) % m;

        // Multiply if bit is a `1`
        if bit {
            result = result * (n as u64) % m;
        }
    }

    // Guaranteed to be in u32 as m is u32
    return result as u32;
}
