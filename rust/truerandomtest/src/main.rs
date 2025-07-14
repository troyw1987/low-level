use rand::Rng;
use std::collections::HashMap;

fn rbool() -> bool {
    let mut random = rand::rng();
    random.random_bool(0.5)
}

fn main() {
    let sample_size: u32 = 200;
    let mut myhashmap = HashMap::new();

    for i in 0..sample_size {
        let myrandom: bool = rbool();
        myhashmap.insert(i, myrandom);
    }

    let mut trues: i32 = 0;
    let mut falses: i32 = 0;

    for (_, value) in myhashmap {
        if value {
            trues += 1;
            continue;
        }
        falses += 1;
    }

    let difference: u32 = (i32::abs(trues - falses)) as u32;
    let randomness: f64 = ((difference as f64 / sample_size as f64) / 2.0) * 100.0;
    println!("T => {trues}\nF => {falses}");
    println!("{randomness}% imbalance in randomness given {sample_size} instances");
}
