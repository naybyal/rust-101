use std::str::FromStr;
use std::env;
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 &&  m != 0); // handling edge cases
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n   //  return n
}

fn main() {
    let mut numbers = Vec::new();
    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg)
            .expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: GCD NUMBER ....");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The gcd of {:?} is {}", numbers, d);
}