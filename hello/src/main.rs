fn main() {
    println!("Hello, world!");
}
use std::str::FromStr;
use std::env;

fn main() {
    let mut numbers = Vec::new();
    let mut primes : Vec<u32> = vec![2, 3, 5, 7];

    for arg in env::args().skip(1) {
        numbers.push(u32::from_str(&arg)
                     .expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: NUMBER ...");
        std::process::exit(1);
    }

    primes.push(11);
    primes.push(13);

    numbers.sort();

    println!("The sorted numbers are {:?}", numbers);
    println!("The prime numbers are {:?}", primes);
}

#[test]
fn test_gcd() {

}