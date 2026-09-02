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

fn miller_rabin(n: u128) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut s = 0;
    let mut d = n-1;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    for a in [2, 3, 5, 7, 11, 13, 17] {
        if a >= n {
            continue;
        }

        let mut x = square_and_multiply(a, d, n);

        if x == 1 || x == n-1 {
            continue;
        }

        let mut passed = false;
        for _ in 1..s {
            x = (x * x) % n;
            if x == n-1 {
                passed = true;
                break;
            }
            if x == 1 {
                return false;
            }
        }
        if !passed {
            return false;
        }
    }
    true
}

fn main() {
    for n in [2305843009213693951, 18446744073709551557, 9223372036854775783, 4611686018427387847, 1152921504606846883, ] {
        println!("{:?}", miller_rabin(n));
    }
}
