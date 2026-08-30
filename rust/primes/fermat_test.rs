//! Fermat's Primality Test
//! Can produce false positives for pseudoprimes 

fn square_and_multiply(mut base: u128, mut exponent: u128, n: u128) -> u128 {
    let mut result: u128 = 1;
    
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = (result * base) % n;
        }

        base = (base * base) % n;
        exponent >>= 1;
    }

    result
}

fn gcd(mut num1: u128, mut num2: u128) -> u128 {
    while num2 != 0 {
        let temp = num2;
        num2 = num1 % num2;
        num1 = temp;
    } 
    num1
}

fn fermat_test(n: u128) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    for a in [2, 3, 5, 7, 11, 13, 17] {
        if a >= n {
            continue;
        }
        if gcd(a, n) != 1 {
            return false;
        }
        if square_and_multiply(a, n-1, n) != 1 {
            return false;
        }   
    }
    true
}

fn main() {
    println!("{}", fermat_test(2305843009213693951));
    println!("{}", fermat_test(18446744073709551557));
    println!("{}", fermat_test(9223372036854775783));
    println!("{}", fermat_test(4611686018427387847));
    println!("{}", fermat_test(1152921504606846883));
}
