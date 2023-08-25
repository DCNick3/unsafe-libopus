#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

mod cli;

use crate::cli::{Cli, Mode};
use unsafe_libopus::opus_get_version_string;
use unsafe_libopus::test::demo::{opus_demo_decode, opus_demo_encode, DecodeArgs};

pub fn main() {
    let cli = Cli::parse();

    eprintln!("{}", opus_get_version_string());

    match cli.mode {
        Mode::EncodeDecode(args) => {
            let fin = std::fs::read(&cli.input).expect("failed to read input file");
            let encoded = opus_demo_encode(&fin, args);
            let decoded = opus_demo_decode(
                &encoded,
                DecodeArgs {
                    sampling_rate: args.sampling_rate,
                    channels: args.channels,
                    options: args.options.common,
                },
            );
            std::fs::write(&cli.output, &decoded).expect("failed to write output file");
            // TODO: write statistics
        }
        Mode::EncodeOnly(args) => {
            let fin = std::fs::read(&cli.input).expect("failed to read input file");
            let output = opus_demo_encode(&fin, args);
            std::fs::write(&cli.output, &output).expect("failed to write output file");
            // TODO: write statistics
        }
        Mode::DecodeOnly(args) => {
            let fin = std::fs::read(&cli.input).expect("failed to read input file");
            let output = opus_demo_decode(&fin, args);
            std::fs::write(&cli.output, &output).expect("failed to write output file");
        }
    }
}
