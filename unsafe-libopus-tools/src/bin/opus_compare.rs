//! A utility to compare PCM data
//!
//! It is used to run opus test vectors

#![forbid(unsafe_code)]

use clap::Parser;
use unsafe_libopus_tools::demo::{Channels, SampleRate};
use unsafe_libopus_tools::{opus_compare, CompareParams, CompareResult};

#[derive(Parser)]
struct Args {
    /// Whether to compare in stereo or mono
    #[arg(short)]
    stereo: bool,

    /// The sampling rate to use
    #[arg(short = 'r')]
    sample_rate: SampleRate,

    /// The true file to compare to
    true_file: std::path::PathBuf,

    /// The tested file to compare
    tested_file: std::path::PathBuf,
}

fn main() {
    let args: Args = Args::parse();

    let params = CompareParams {
        sample_rate: args.sample_rate,
        channels: if args.stereo {
            Channels::Stereo
        } else {
            Channels::Mono
        },
    };

    let fin1 = std::fs::read(args.true_file).expect("Could not read true file");
    let fin2 = std::fs::read(args.tested_file).expect("Could not read comparing file");

    let CompareResult { error, quality } = opus_compare(params, &fin1, &fin2);

    if quality < 0f64 {
        eprintln!("Test vector FAILS");
        eprintln!("Internal weighted error is {}", error);
        std::process::exit(1);
    } else {
        eprintln!("Test vector PASSES");
        eprintln!(
            "Opus quality metric: {} % (internal weighted error is {})",
            quality, error
        );
    };
}
