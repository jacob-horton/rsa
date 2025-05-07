use rand::Rng;

fn is_prime(n: u32) -> bool {
    for i in 2..=(n.isqrt() + 1) {
        if n % i == 0 {
            return false;
        }
    }

    return n > 1;
}

pub fn random_prime(max: u32) -> u32 {
    let mut rng = rand::rng();

    let mut prime = false;
    let mut n: u32 = 0;
    while !prime {
        n = rng.random::<u32>() % max;
        prime = is_prime(n);
    }

    return n;
}
