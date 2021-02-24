use std::collections::HashMap;

fn main() {}

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

fn p_factors<'a>(n: i64) -> HashMap<i64, i64> {
    let fac = new_sieve((n as f64).sqrt() as i64);
    let mut factors = HashMap::new();
    let mut rem = n;
    for i in fac {
        while rem % i == 0 {
            rem = rem / i;
            *factors.entry(i).or_insert(0) += 1;
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

struct Sieve {
    s: Vec<bool>,
    p: usize,
    sqrt_n: usize,
}

// new_sieve returns an iterator of all the prime less than or equal to the input value.
// See: https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes
fn new_sieve(n: i64) -> Sieve {
    let mut v = Sieve {
        s: Vec::with_capacity(n as usize),
        p: 2,
        sqrt_n: (n as f64).sqrt() as usize,
    };
    v.s.push(false); // 1 is not prime
    for _ in 1..n {
        v.s.push(true);
    }
    v
}

impl Iterator for Sieve {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        while self.p <= self.sqrt_n {
            if self.s[self.p - 1] {
                // self.p is prime
                let mut j = self.p * self.p;
                while j <= self.s.len() {
                    self.s[j - 1] = false;
                    j += self.p;
                }
                let r = Some(self.p as i64);
                self.p += 1;
                return r;
            }
            self.p += 1;
        }
        while self.p <= self.s.len() {
            if self.s[self.p - 1] {
                let r = Some(self.p as i64);
                self.p += 1;
                return r;
            }
            self.p += 1;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sieve() {
        assert_eq!(new_sieve(5).collect::<Vec<i64>>(), vec![2, 3, 5]);
    }

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
