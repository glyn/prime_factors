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

fn p_factors(n: i64) -> HashMap<i64, i64> {
    let fac = seive((n as f64).sqrt() as i64);
    let mut factors = HashMap::new();
    let mut rem = n;
    while rem > 1 {
        for i in &fac {
            if rem % i == 0 {
                rem = rem / i;
                *factors.entry(*i).or_insert(0) += 1;
            }
        }
    }
    factors
}

// seive returns all the prime less than or equal to the input value.
fn seive(n: i64) -> Vec<i64> {
    match n {
        0..=1 => vec![],
        2 => vec![2],
        _ => {
            let mut p = seive(n - 1);
            for i in &p {
                if n % i == 0 {
                    return p;
                }
            }
            p.append(&mut vec![n]);
            p
        }
    }
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
    }
}
