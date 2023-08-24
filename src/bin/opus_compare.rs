//! A utility to compare PCM data
//!
//! It is used to run opus test vectors

#![forbid(unsafe_code)]

use unsafe_libopus::test::{opus_compare, CompareParams, CompareResult};

fn main() {
    let mut args = std::env::args().peekable();
    let argc = args.len();
    let argv0 = args.next().unwrap();

    if argc < 3 || argc > 6 {
        eprintln!("Usage: {} [-s] [-r rate2] <file1.sw> <file2.sw>", argv0);
        std::process::exit(1);
    }

    let mut params = CompareParams::default();

    if args.peek().unwrap() == "-s" {
        params.stereo = true;
    }

    if args.peek().unwrap() == "-r" {
        args.next().unwrap();
        let rate = args.next().unwrap().parse().expect("Could not parse rate");
        if rate != 8000 && rate != 12000 && rate != 16000 && rate != 24000 && rate != 48000 {
            eprintln!("Sampling rate must be 8000, 12000, 16000, 24000, or 48000");
            std::process::exit(1);
        }
        params.sample_rate = rate;
    }
    let fin1 = std::fs::read(args.next().unwrap()).expect("Could not read true file");
    let fin2 = std::fs::read(args.next().unwrap()).expect("Could not read comparing file");

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
