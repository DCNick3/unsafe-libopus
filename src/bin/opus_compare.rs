//! A utility to compare PCM data
//!
//! It is used to run opus test vectors

#![forbid(unsafe_code)]

use clap::Parser;
use std::str::FromStr;
use unsafe_libopus::test::{opus_compare, CompareParams, CompareResult};

#[derive(Debug, Default, Copy, Clone)]
enum SampleRate {
    #[default]
    R48000,
    R24000,
    R16000,
    R12000,
    R8000,
}

impl FromStr for SampleRate {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .parse()
            .map_err(|_| "Cannot parse the sample rate as number")?;

        match s {
            48000 => Ok(SampleRate::R48000),
            24000 => Ok(SampleRate::R24000),
            16000 => Ok(SampleRate::R16000),
            12000 => Ok(SampleRate::R12000),
            8000 => Ok(SampleRate::R8000),
            _ => Err("Unsupported sample rate"),
        }
    }
}

impl From<SampleRate> for usize {
    fn from(value: SampleRate) -> Self {
        match value {
            SampleRate::R48000 => 48000,
            SampleRate::R24000 => 24000,
            SampleRate::R16000 => 16000,
            SampleRate::R12000 => 12000,
            SampleRate::R8000 => 8000,
        }
    }
}

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
        sample_rate: args.sample_rate.into(),
        stereo: args.stereo,
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
