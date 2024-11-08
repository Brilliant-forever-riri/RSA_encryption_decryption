use num_bigint::{BigInt, ToBigInt};
use num_traits::{One, Zero};
use num_traits::Signed;
/// Function to compute the greatest common divisor (GCD)
fn gcd(a: &BigInt, b: &BigInt) -> BigInt {
    if b.is_zero() {
        return a.clone();
    }
    gcd(b, &(a % b))
}

/// Function to compute the modular inverse using the Extended Euclidean Algorithm
fn mod_inverse(e: &BigInt, phi: &BigInt) -> BigInt {
    let mut t = BigInt::zero();
    let mut newt = BigInt::one();
    let mut r = phi.clone();
    let mut newr = e.clone();
    let mut quotient;

    while !newr.is_zero() {
        quotient = &r / &newr;

        let temp_t = t.clone() - &quotient * &newt;
        t = newt.clone();
        newt = temp_t;

        let temp_r = r.clone() - &quotient * &newr;
        r = newr.clone();
        newr = temp_r;
    }

    if r > BigInt::one() {
        panic!("e is not invertible");
    }

    if t.is_negative() {
        t = t + phi;
    }

    t
}

/// Function to compute modular exponentiation
fn mod_exp(base: &BigInt, exp: &BigInt, modulus: &BigInt) -> BigInt {
    let mut result = BigInt::one();
    let mut base = base.clone() % modulus;
    let mut exp = exp.clone();

    while exp > BigInt::zero() {
        if &exp % 2.to_bigint().unwrap() == BigInt::one() {
            result = (result * &base) % modulus;
        }
        base = (&base * &base) % modulus;
        exp >>= 1;
    }

    result
}

/// RSA encryption function
fn encrypt(message: &BigInt, e: &BigInt, n: &BigInt) -> BigInt {
    mod_exp(message, e, n)
}

/// RSA decryption function
fn decrypt(ciphertext: &BigInt, d: &BigInt, n: &BigInt) -> BigInt {
    mod_exp(ciphertext, d, n)
}

fn main() {
    // Prime numbers 
    let p = 677656567.to_bigint().unwrap();
    let q = 1765425.to_bigint().unwrap();
    
    // Public modulus n = p * q
    let n = &p * &q;
    
    // Euler's Totient Function: phi(n) = (p-1)(q-1)
    let phi_n = (&p - 1) * (&q - 1);
    
    // Public exponent e, coprime with phi(n)
    let e = 7.to_bigint().unwrap();
    
    // Private key d: modular inverse of e mod phi(n)
    let d = mod_inverse(&e, &phi_n);
    
    // plaintext to encrypt
    let message = 99.to_bigint().unwrap(); // Example message in number form
    
    // Encrypt the message using the public key (e, n)
    let ciphertext = encrypt(&message, &e, &n);
    println!("Encrypted message: {}", ciphertext);
    
    // Decrypt the message using the private key (d, n)
    let decrypted_message = decrypt(&ciphertext, &d, &n); return
    println!("Decrypted message: {}", decrypted_message);
}


