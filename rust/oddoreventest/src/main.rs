use clap::Parser;
use std::time::{Duration, Instant};

#[derive(Parser, Debug)]
#[command(
    author = "Troy",
    version = "version 0",
    about = "Odd or even benchmarking",
    long_about = "Uses both modulus and binary methods of determining if the provided number is odd or even, and returns nanoseconds of the operation."
)]
struct Args {
    #[arg(default_value_t = 8)]
    number: u128,
}

fn modulus_iseven(number: u128) -> bool {
    if (number % 2) == 1 {
        return false;
    }

    true
}

fn modulus_getresults(number: u128) -> (Duration, bool) {
    let start_time: Instant = Instant::now();
    let iseven: bool = modulus_iseven(number);
    (start_time.elapsed(), iseven)
}

fn binary_iseven(number: u128) -> bool {
    let last_bit: u8 = (number & 1) as u8;

    match last_bit {
        0 => true,
        1 => false,
        _ => panic!("Expected u8 of 0 or 1, got {last_bit} instead"),
    }
}

fn binary_getresults(number: u128) -> (Duration, bool) {
    let start_time: Instant = Instant::now();
    let iseven: bool = binary_iseven(number);
    (start_time.elapsed(), iseven)
}

fn main() {
    let args = Args::parse();

    let x: u128 = args.number;

    let modulusresults: (Duration, bool) = modulus_getresults(x);

    println!(
        "modulus is-odd - {}, within {:?} nanoseconds",
        modulusresults.1,
        modulusresults.0.as_nanos()
    );

    let binaryresults: (Duration, bool) = binary_getresults(x);
    println!(
        "binary  is-odd - {}, within {:?} nanoseconds",
        binaryresults.1,
        binaryresults.0.as_nanos()
    );
}
