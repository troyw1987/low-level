use clap::Parser;
use std::time::Instant;
#[derive(Parser, Debug)]
#[command(
    author = "Troy",
    version = "0",
    about = "Randomness tester",
    long_about = None
)]
struct Args {
    #[arg(default_value_t = 100)]
    samplesize: u128,

    #[arg(short, long)]
    debug: bool,
}

fn is_random(trues: u32, sample_size: u128) -> bool {
    let 
}

fn calculations(sample_size: u128) -> (u32, u32, f64) {
    let mut trues: u32 = 0;
    let mut falses: u32 = 0;

    for _ in 0..sample_size {
        let myrandom: bool = true; //rng().random_bool(0.5);
        if myrandom {
            trues += 1;
        } else {
            falses += 1;
        }
    }

    let difference: u32 = trues.max(falses) - trues.min(falses);
    let randomness: f64 = ((difference as f64 / sample_size as f64) / 2.0) * 100.0;
    (trues, falses, randomness)
}

fn main() {
    let programstart: Instant = Instant::now();
    let args = Args::parse();
    if args.debug {
        println!("Wassup, Debug!");
    }

    let sample_size: u128 = args.samplesize;

    if sample_size == 0 {
        println!("Sample size must be more than 0, dummy!");
        return;
    }

    let calcstart: Instant = Instant::now();
    let (trues, falses, observedrandomnessdeviation) = calculations(sample_size);

    let calcdurationus: u128 = calcstart.elapsed().as_micros();
    let totaldurationus: u128 = programstart.elapsed().as_micros();

    let mspersample: f64 = calcdurationus as f64 / sample_size as f64;

    println!(
        "µs total = {totaldurationus} \nµs calc = {calcdurationus} µs ({mspersample}µs / sample)"
    );
    println!("T => {trues}\nF => {falses}");
    println!("random imbalance of {observedrandomnessdeviation}% given {sample_size} instances");
}
