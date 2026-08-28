//! Sieve of Eratosthenes
//! Generates a boolean lookup table indicating which numbers up to a given range are primes
//! Returns a boolean vector where true indicates a prime number and false indicates a composite number

fn sieve_of_eratosthenes(range: usize) -> Vec<bool>{
    if range < 2 {
        return vec![false; range + 1];
    }
    let mut primes= vec![true; range+1];
    primes[0] = false;
    primes[1] = false;
    for i in 2..primes.len() {
        if i * i > range {
            break;
        }

        if primes[i] {
            for j in (i*i..primes.len()).step_by(i) {
                primes[j] = false;
            }
        }
    }
    primes
}

fn main() {
    let n = 2_000_000;
    let primes = sieve_of_eratosthenes(n);

    for i in 0..primes.len() {
        if primes[i] {
            println!("{i}");
        }
    }
}
