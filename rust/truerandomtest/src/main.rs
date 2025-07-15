use clap::Parser;
use colored::*;
use rand::{Rng, rng};
use std::time::Instant;
#[derive(Parser, Debug)]
#[command(
    author = "Troy",
    version = "version 0",
    about = "Randomness tester",
    long_about = None
)]
struct Args {
    #[arg(default_value_t = 100000)]
    samplesize: u128,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    #[arg(short, long, default_value_t = false)]
    bypass: bool,
}

fn calculations(sample_size: u128) -> (u32, u32, f64) {
    let mut trues: u32 = 0;
    let mut falses: u32 = 0;

    for _ in 0..sample_size {
        let myrandom: bool = rng().random_bool(0.6);
        if myrandom {
            trues += 1;
        } else {
            falses += 1;
        }
    }

    let difference: u32 = trues.max(falses) - trues.min(falses);
    let randomness: f64 = (difference as f64 / sample_size as f64) / 2.0;
    let zscore = randomness * 10.0;
    (trues, falses, zscore)
}

fn main() {
    let args = Args::parse();

    let start_time: Option<Instant> = if args.verbose {
        Some(Instant::now())
    } else {
        None
    };

    let sample_size: u128 = args.samplesize;

    if !args.bypass {
        if sample_size == 0 {
            println!(
                "{}",
                "Sample size must be more than 0 dummy!\nUse --bypass to bypass".bright_magenta()
            );
            return;
        } else if sample_size < 100 {
            println!(
                "{}",
                "Below 100 will very often cause discrepancies.\nUse --bypass for bad readings."
                    .bright_magenta()
            );
            return;
        }
    }

    let (trues, falses, zscore) = calculations(sample_size);

    if let Some(start_instant) = start_time
        && args.verbose
    {
        let calcduration: u128 = start_instant.elapsed().as_micros();
        let mspersample: f64 = calcduration as f64 / sample_size as f64;

        println!("TIME:");
        println!("calculations time - {calcduration:.2}µs (~{mspersample}µs / sample)\n");

        println!("CALCULATIONS:");
        println!("{sample_size} instances");
        println!("T => {trues}\nF => {falses}");
        println!("z = {zscore:.2}");
    }
    if zscore <= 0.00 {
        println!(
            "{}",
            "100% Perfectly Random :))\n(almost too random)".bright_purple()
        );
    } else if zscore <= 1.96 {
        println!("{}", "Within 95% random :)".green());
    } else if zscore < 2.58 {
        println!("{}", "Within 5% random ಠಿ_ಠ".bright_yellow());
    } else if zscore >= 2.58 {
        println!(
            "{}",
            "0.009% random - Horribly not random! :| \n(or very very unlucky)".bright_red()
        );
    } else if zscore.is_nan() {
        println!(
            "{}",
            "congrats, you broke it. Here's your cookie. \n🍪".bright_red()
        );
    }
}
