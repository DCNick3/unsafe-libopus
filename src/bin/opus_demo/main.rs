#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

mod cli;

use crate::cli::{Backend, Cli, Mode};
use unsafe_libopus::opus_get_version_string;
#[cfg(feature = "test-upstream-libopus")]
use unsafe_libopus::test::demo::UpstreamLibopusBackend;
use unsafe_libopus::test::demo::{
    opus_demo_adjust_length, opus_demo_decode, opus_demo_encode, DecodeArgs, OpusBackend,
    RustLibopusBackend,
};

pub fn main() {
    let cli = Cli::parse();

    eprintln!("{}", opus_get_version_string());

    let backend: &dyn OpusBackend = match cli.backend {
        Backend::RustLibOpus => &RustLibopusBackend,
        #[cfg(feature = "test-upstream-libopus")]
        Backend::UpstreamLibOpus => &UpstreamLibopusBackend,
        #[cfg(not(feature = "test-upstream-libopus"))]
        Backend::UpstreamLibOpus => {
            panic!("This build of opus_demo was built without the upstream libopus support")
        }
    };

    match cli.mode {
        Mode::EncodeDecode(args) => {
            let fin = std::fs::read(&cli.input).expect("failed to read input file");
            let (encoded, pre_skip) = opus_demo_encode(backend, &fin, args);
            let mut decoded = opus_demo_decode(
                backend,
                &encoded,
                DecodeArgs {
                    sample_rate: args.sample_rate,
                    channels: args.channels,
                    options: args.options.common,
                },
            );
            opus_demo_adjust_length(
                &mut decoded,
                pre_skip,
                fin.len(),
                args.sample_rate,
                args.channels,
            );
            std::fs::write(&cli.output, &decoded).expect("failed to write output file");
            // TODO: write statistics
        }
        Mode::EncodeOnly(args) => {
            let fin = std::fs::read(&cli.input).expect("failed to read input file");
            let (output, _pre_skip) = opus_demo_encode(backend, &fin, args);
            std::fs::write(&cli.output, &output).expect("failed to write output file");
            // TODO: write statistics
        }
        Mode::DecodeOnly(args) => {
            let fin = std::fs::read(&cli.input).expect("failed to read input file");
            let output = opus_demo_decode(backend, &fin, args);
            std::fs::write(&cli.output, &output).expect("failed to write output file");
        }
    }
}
