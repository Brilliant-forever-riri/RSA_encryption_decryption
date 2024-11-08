# RSA Encryption and Decryption in Rust

This project implements RSA encryption and decryption in Rust, showcasing the fundamentals of the RSA algorithm. The code includes functions for calculating the greatest common divisor (GCD), modular inverse, modular exponentiation, and encryption/decryption using public and private keys. It utilizes the `num-bigint` crate to handle large integer operations.

## Table of Contents

- [Requirements](#requirements)
- [Project Structure](#project-structure)
- [Code Explanation](#code-explanation)
  - [Functions](#functions)
  - [Main Workflow](#main-workflow)
- [Usage](#usage)
- [Example Output](#example-output)
- [Notes](#notes)
- [License](#license)

## Requirements

To run this code, ensure that you have Rust installed. Additionally, add the required dependencies by adding them to your `Cargo.toml` file:

```toml
[dependencies]
num-bigint = "0.4"
num-traits = "0.2"
```

### Install the dependencies by running:

```sh
cargo build
```

### Project Structure
- gcd(a, b): Computes the greatest common divisor of two large integers.
- mod_inverse(e, phi): Computes the modular inverse of e modulo phi using the Extended Euclidean Algorithm.
- mod_exp(base, exp, modulus): Efficiently calculates (base^exp) % modulus.
- encrypt(message, e, n): Encrypts a plaintext message using the RSA public key (e, n).
- decrypt(ciphertext, d, n): Decrypts an encrypted message using the RSA private key (d, n).

### Code Explanation

This section explains the purpose and logic of each function in the code.

### Functions

- gcd(a: &BigInt, b: &BigInt) -> BigInt: Calculates the greatest common divisor of a and b recursively using the Euclidean algorithm.
- mod_inverse(e: &BigInt, phi: &BigInt) -> BigInt: Computes the modular inverse of e modulo phi, which is necessary for finding the RSA private key. Uses the Extended Euclidean Algorithm.
- mod_exp(base: &BigInt, exp: &BigInt, modulus: &BigInt) -> BigInt: Efficiently computes modular exponentiation, which is a key operation in both RSA encryption and decryption.
- encrypt(message: &BigInt, e: &BigInt, n: &BigInt) -> BigInt: Encrypts a message by computing message^e % n, where e is the public exponent and n is the modulus.
- decrypt(ciphertext: &BigInt, d: &BigInt, n: &BigInt) -> BigInt: Decrypts an encrypted message (ciphertext) by computing ciphertext^d % n, where d is the private exponent.


### Main Workflow

- Prime Number Initialization: Defines two prime numbers p and q for generating RSA keys.
- Modulus Calculation: Calculates n = p * q, which is the public modulus.
- Euler's Totient Calculation: Computes phi(n) = (p-1)(q-1).
- Public Exponent: Chooses a public exponent e that is coprime with phi(n). For simplicity, e is set to 7 in this example.
- Private Key Calculation: Computes the private key d by finding the modular inverse of e modulo phi(n).
- Encryption: Encrypts a sample message using the public key (e, n).
- Decryption: Decrypts the resulting ciphertext using the private key (d, n).

### Usage

To run this code:

- Copy codeAdd the required dependencies to your Cargo.toml.

Compile and run the code with:

```sh
cargo run
```

The code will execute the RSA encryption and decryption process for a sample message and print the results to the console.

Example Output
After running the program, you should see output similar to the following:

```plaintext

Encrypted message: <ciphertext>
Decrypted message: <original message>

Here, <ciphertext> represents the encrypted form of the sample message, and <original message> is the decrypted message, confirming that the RSA encryption and decryption work correctly.
```

### Notes

- Prime Numbers: The primes p and q used in this code are small for simplicity. For secure RSA encryption, much larger prime numbers are required.

- Public Exponent: The value of e should be chosen such that it is coprime with phi(n). Common values for e are 3, 5, or 65537.
Error Handling: In real-world applications, you may want to add more error handling, particularly for invalid keys or messages that are out of range.

### License

This project is licensed under the MIT License. See the LICENSE file for details.
