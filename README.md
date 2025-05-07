# RSA

This is a basic implementation of the RSA cryptosystem as a learning exercise

It should not be used in practice, as there are security flaws with this implementation

## Features

- Generate a random RSA key pair
- Encrypt data with the public key
- Decrypt data with the private key

## Limitations

- Only works with 32-bit keys
- Can only encrypt 32-bit numbers
- The square and multiply algorithm is vulnerable to power analysis and timing attacks
- Primes are generated randomly, so may not be the most secure selection
- No padding is used - not semantically secure
