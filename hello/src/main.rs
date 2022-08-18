use std::str::FromStr;
use std::env;

fn array_loop() {
    let v = vec![4, 7, 8, 9, 11, 10];

    'outer_loop: for i in 0..10 {
        for n in &v {
            if i + n == 11 {
                break 'outer_loop;
            }
            println!("Hello Vector {} {}",i, n);
        }
    }
}

fn loop_to_10() {
    for n in 0..10 {
        println!("Hello {}", n);
    }
}

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

    array_loop();
    loop_to_10();

    println!("The sorted numbers are {:?}", numbers);
    println!("The prime numbers are {:?}", primes);
}

#[test]
fn test_gcd() {

}