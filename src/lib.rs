use std::collections::HashMap;

fn prime_factors(n: i64) -> String {
    let f = p_factors(n);
    let mut res = String::new();
    let mut factors = f.keys().map(|p| *p).collect::<Vec<i64>>();
    factors.sort();
    for p in factors {
        let n = f.get(&p).unwrap();
        if *n == 1 {
            res.push_str(&format!("({})", p));
        } else {
            res.push_str(&format!("({}**{})", p, *n));
        }
    }
    res
}

fn p_factors(n: i64) -> HashMap<i64, i64> {
    let fac = sieve((n as f64).sqrt() as i64);
    let mut factors = HashMap::new();
    let mut rem = n;
    for i in &fac {
        while rem % i == 0 {
            rem = rem / i;
            *factors.entry(*i).or_insert(0) += 1;
        }
        if rem == 1 {
            break;
        }
    }
    if rem > 1 {
        // rem is indivisible by all the primes up to sqrt(n) >= sqrt(rem),
        // so rem is prime
        *factors.entry(rem).or_insert(0) += 1
    }
    factors
}

// sieve returns all the prime less than or equal to the input value.
// See: https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes
fn sieve(n: i64) -> Vec<i64> {
    let mut s: Vec<bool> = Vec::with_capacity(n as usize); // after sieving, true at index i means i+1 is prime
    s.push(false); // 1 is not prime
    for _ in 1..n {
        s.push(true);
    }
    let mut i = 2;
    while i as f64 <= (n as f64).sqrt() {
        if s[i as usize - 1] {
            // if p is prime
            let mut j = i * i;
            while j <= n {
                s[j as usize - 1] = false;
                j += i;
            }
        }
        i += 1
    }
    let mut primes = vec![];
    for i in 0..s.len() {
        if s[i] {
            primes.push((i + 1) as i64)
        }
    }
    primes
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(n: i64, exp: &str) -> () {
        assert_eq!(&prime_factors(n), exp)
    }

    #[test]
    fn basics_prime_factors() {
        testing(7775460, "(2**2)(3**3)(5)(7)(11**2)(17)");
        testing(17 * 17 * 93 * 677, "(3)(17**2)(31)(677)");
        testing(123863, "(123863)");
        testing(933555431, "(7537)(123863)");
        testing(342217392, "(2**4)(3)(11)(43)(15073)");
    }
}
