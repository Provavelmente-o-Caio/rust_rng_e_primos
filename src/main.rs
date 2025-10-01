use num::integer::gcd;
use rand::Rng;

fn main() {
    println!("Testando valores de prime");
    let pseudoprimes = [
        561,
        41041,
        825265,
        321197185,
        5394826801,
        232250619601,
        9746347772161,
        1436697831295441,
        60977817398996785,
        7156857700403137441,
        1791562810662585767521,
    ];
    for pseudoprime in pseudoprimes.iter() {
        let is_prime = fermat(*pseudoprime, 10);
        println!("{} é primo? {}", pseudoprime, is_prime);
    }
}

fn fermat(n: u128, k: u8) -> bool {
    if n < 2 {
        return false;
    } else if n == 2 {
        return true;
    } else if n % 2 == 0 {
        return false;
    }

    let mut generator = rand::rng();
    for _ in 0..k {
        let mut a: u128 = generator.random_range(2..n);
        while gcd(a, n) != 1 {
            a = generator.random_range(2..n);
        }
        if mod_pow(a, n - 1, n) != 1 {
            return false;
        }
    }
    true
}

fn mod_pow(mut base: u128, mut exponent: u128, modulus: u128) -> u128 {
    if modulus == 1 {
        0
    } else {
        let mut result = 0;
        base = base % modulus;
        while exponent > 0 {
            if exponent % 2 == 1 {
                result = (result * base) % modulus;
            }
            exponent = exponent >> 1;
            base = (base * base) % modulus;
        }
        result
    }
}
