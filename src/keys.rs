use crate::math;

// RSA private key
#[derive(Debug, Clone)]
pub struct PrivateKey {
    p: u32,
    q: u32,
    a: u32,
}

// RSA public key
#[derive(Debug, Clone)]
pub struct PublicKey {
    n: u32,
    b: u32,
}

impl PrivateKey {
    pub fn init(p: u32, q: u32, a: u32) -> Self {
        Self { p, q, a }
    }

    pub fn decrypt(&self, ciphertext: u32) -> u32 {
        math::modulo_power(ciphertext, self.a, self.p * self.q)
    }
}

impl PublicKey {
    pub fn init(n: u32, b: u32) -> Self {
        Self { n, b }
    }

    pub fn encrypt(&self, plaintext: u32) -> u32 {
        math::modulo_power(plaintext, self.b, self.n)
    }
}
