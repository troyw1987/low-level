use std::time::{Duration, Instant};

fn modulus_iseven(number: u8) -> bool {
    if (number % 2) == 1 {
        return false;
    }

    true
}

fn modulus_getresults(number: u8) -> (Duration, bool) {
    let start_time: Instant = Instant::now();
    let iseven: bool = modulus_iseven(number);
    (start_time.elapsed(), iseven)
}

fn binary_iseven(number: u8) -> bool {
    let last_bit = number & 1;

    match last_bit {
        0 => false,
        1 => true,
        _ => panic!("Expected u8 of 0 or 1, got {last_bit} instead"),
    }
}

fn binary_getresults(number: u8) -> (Duration, bool) {
    let start_time: Instant = Instant::now();
    let iseven: bool = binary_iseven(number);
    (start_time.elapsed(), iseven)
}

fn main() {
    let x: u8 = 5; // 0011

    let modulusresults: (Duration, bool) = modulus_getresults(x);

    println!(
        "MODULUS:\n{}, within {:?} nanoseconds",
        modulusresults.1,
        modulusresults.0.as_nanos()
    );

    let binaryresults: (Duration, bool) = binary_getresults(x);
    println!(
        "BINARY:\n{}, within {:?} nanoseconds",
        binaryresults.1,
        binaryresults.0.as_nanos()
    );
}
