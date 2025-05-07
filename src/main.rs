mod bits;
mod keys;
mod math;
mod prime;

use std::u32;

use keys::{PrivateKey, PublicKey};

// Generate a random RSA key pair
fn gen_keys() -> (PrivateKey, PublicKey) {
    // 1. Generate p and q (primes, not equal)
    let p = prime::random_prime(u32::MAX.isqrt());
    let q = prime::random_prime(u32::MAX.isqrt());
    assert_ne!(p, q);

    // 2. Calculate n = pq, phi(n) = (p-1)(q-1)
    let n = p * q;
    let phi_n = (p - 1) * (q - 1);

    // 3. Pick 1 < b < phi(n), gcd(b, phi(n)) == 1
    let b = 65537;

    // 4. Calculate a = b^(-1) (mod phi(n))
    let a = math::multiplicative_inverse(b, phi_n);

    return (PrivateKey::init(p, q, a), PublicKey::init(n, b));
}

fn main() {
    let (private, public) = gen_keys();

    let plaintext = 42;
    let ciphertext = public.encrypt(plaintext);
    println!("Ciphertext:\t{ciphertext}");

    let decrypted = private.decrypt(ciphertext);
    println!("Plaintext:\t{decrypted}");
}
