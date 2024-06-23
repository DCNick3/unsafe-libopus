//! A betterrer version of the `run_vectors.sh` test from the upstream repo.
//!
//! Unlike the original, it uses the `unsafe-libopus` as a library, instead of calling the `opus_demo` binary.
//!
//! Also, instead of testing encoder-decoder roundtrip quality, it tests the encoded/decoded results against those of the upstream library.
//! This is a much stricter test, which prevents divergence of behavior from the upstream instead of testing that the encoder-decoder pair works "somehow".

use clap::Parser;
use itertools::iproduct;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use indicatif::ParallelProgressIterator;
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::time::Instant;
use unsafe_libopus_tools::demo::{
    opus_demo_decode, opus_demo_encode, Application, Channels, DecodeArgs, EncodeArgs,
    RustLibopusBackend, SampleRate, UpstreamLibopusBackend,
};
use unsafe_libopus_tools::CompareResult;

#[derive(Parser)]
struct Cli {
    /// Path to opus test vectors
    vector_path: PathBuf,
    /// Directory to save intermediate files to
    #[clap(long)]
    dump_dir: Option<PathBuf>,
}

struct TestVector {
    name: String,
    encoded: Vec<u8>,
    decoded_stereo: Vec<u8>,
    #[allow(unused)]
    decoded_mono: Vec<u8>,
}

#[derive(Debug, Copy, Clone)]
enum TestKind {
    RustDecode {
        channels: Channels,
        sample_rate: SampleRate,
    },
    RustEncode {
        bitrate: u32,
    },
}

fn load_test_vectors(vector_dir: &Path) -> Vec<TestVector> {
    let mut output = BTreeMap::new();

    for entry in std::fs::read_dir(vector_dir).expect("Reading test vectors directory") {
        let entry = entry.expect("Reading test vectors directory");

        if entry
            .file_type()
            .expect("Reading test vectors directory")
            .is_dir()
        {
            continue;
        }

        let path = entry.path();
        let Some(ext) = path.extension() else {
            continue;
        };
        if ext != "bit" && ext != "dec" {
            continue;
        }
        let Some(stem) = path.file_stem() else {
            continue;
        };
        let Some(stem) = stem.to_str() else {
            continue;
        };
        let stem = stem.trim_end_matches('m'); // testvector01m -> testvector01

        let Entry::Vacant(entry) = output.entry(stem.to_string()) else {
            continue;
        };

        let encoded_path = path.with_file_name(format!("{}.bit", stem));
        let decoded_stereo_path = path.with_file_name(format!("{}.dec", stem));
        let decoded_mono_path = path.with_file_name(format!("{}m.dec", stem));

        // if any of the files is missing, skip the test vector
        if !encoded_path.exists() || !decoded_stereo_path.exists() || !decoded_mono_path.exists() {
            continue;
        }

        entry.insert(TestVector {
            name: stem.to_string(),
            encoded: std::fs::read(encoded_path).expect("Reading encoded file"),
            decoded_stereo: std::fs::read(decoded_stereo_path)
                .expect("Reading decoded stereo file"),
            decoded_mono: std::fs::read(decoded_mono_path).expect("Reading decoded mono file"),
        });
    }

    output.into_values().collect()
}

#[derive(Debug, Copy, Clone)]
enum TestResult {
    #[allow(unused)]
    FreqCompare(CompareResult),
    BitstreamCompare(bool),
    DecodedCompare(bool),
}

impl TestResult {
    pub fn is_success(&self) -> bool {
        match self {
            TestResult::FreqCompare(freq) => freq.is_success(),
            &TestResult::BitstreamCompare(result) => result,
            &TestResult::DecodedCompare(result) => result,
        }
    }
}

impl Display for TestResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestResult::FreqCompare(compare) => Display::fmt(compare, f),
            TestResult::BitstreamCompare(true) => {
                write!(f, "PASS: exact bitstream")
            }
            TestResult::BitstreamCompare(false) => {
                write!(f, "FAIL: different bitstream")
            }
            TestResult::DecodedCompare(true) => {
                write!(f, "PASS: exact decoded audio")
            }
            TestResult::DecodedCompare(false) => {
                write!(f, "FAIL: different decoded audio")
            }
        }
    }
}

fn run_test(
    test_vector: &TestVector,
    test_kind: TestKind,
    dump_directory: Option<&Path>,
) -> TestResult {
    match test_kind {
        TestKind::RustDecode {
            sample_rate,
            channels,
        } => {
            let decode_args = DecodeArgs {
                sample_rate,
                channels,
                options: Default::default(),
            };

            let upstream_decoded =
                opus_demo_decode(&UpstreamLibopusBackend, &test_vector.encoded, decode_args);
            let rust_decoded =
                opus_demo_decode(&RustLibopusBackend, &test_vector.encoded, decode_args);

            if let Some(dump_directory) = dump_directory {
                let name_base = format!(
                    "dec_{}_{}_{}",
                    test_vector.name,
                    usize::from(sample_rate),
                    match channels {
                        Channels::Mono => "mono",
                        Channels::Stereo => "stereo",
                    }
                );

                std::fs::write(
                    dump_directory.join(format!("{}_upstream.dec", &name_base)),
                    &upstream_decoded,
                )
                .unwrap();
                std::fs::write(
                    dump_directory.join(format!("{}_rust.dec", &name_base)),
                    &rust_decoded,
                )
                .unwrap();
            }

            TestResult::DecodedCompare(upstream_decoded == rust_decoded)
        }
        TestKind::RustEncode { bitrate } => {
            let true_decoded = &test_vector.decoded_stereo;

            let encode_args = EncodeArgs {
                // NOTE: the test vectors we are using are in 48khz
                // if we want to test encoding at different sample rates or channels we would need to resample
                application: Application::Audio,
                sample_rate: SampleRate::R48000,
                channels: Channels::Stereo,
                bitrate,
                options: Default::default(),
            };
            let decode_args = DecodeArgs {
                sample_rate: SampleRate::R48000,
                channels: Channels::Stereo,
                options: Default::default(),
            };

            let (upstream_encoded, pre_skip) =
                opus_demo_encode(&UpstreamLibopusBackend, &true_decoded, encode_args);
            let (rust_encoded, rust_pre_skip) =
                opus_demo_encode(&RustLibopusBackend, &true_decoded, encode_args);
            assert_eq!(rust_pre_skip, pre_skip);

            if let Some(dump_directory) = dump_directory {
                // save encoded files
                std::fs::write(
                    dump_directory
                        .join(format!("enc_{}_{}_upstream.enc", test_vector.name, bitrate)),
                    &upstream_encoded,
                )
                .unwrap();
                std::fs::write(
                    dump_directory.join(format!("enc_{}_{}_rust.enc", test_vector.name, bitrate)),
                    &rust_encoded,
                )
                .unwrap();

                // decode & save decoded files
                let upstream_decoded =
                    opus_demo_decode(&UpstreamLibopusBackend, &upstream_encoded, decode_args);
                let rust_decoded =
                    opus_demo_decode(&UpstreamLibopusBackend, &rust_encoded, decode_args);

                std::fs::write(
                    dump_directory
                        .join(format!("enc_{}_{}_upstream.dec", test_vector.name, bitrate)),
                    &upstream_decoded,
                )
                .unwrap();
                std::fs::write(
                    dump_directory.join(format!("enc_{}_{}_rust.dec", test_vector.name, bitrate)),
                    &rust_decoded,
                )
                .unwrap();
            }

            TestResult::BitstreamCompare(upstream_encoded == rust_encoded)
        }
    }
}

fn main() {
    let args = Cli::parse();

    let test_vectors = load_test_vectors(&args.vector_path);

    if let Some(ref dump_dir) = args.dump_dir {
        // remove existing files and create the directory
        std::fs::remove_dir_all(dump_dir)
            // ignore the "not exists" error
            .or_else(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    Ok(())
                } else {
                    Err(e)
                }
            })
            .expect("Removing dump directory");
        std::fs::create_dir(dump_dir).expect("Creating dump directory");
    }
    // TODO: test more configurations for the encoder/decoder
    let test_kinds = iproduct!(
        [
            SampleRate::R48000,
            SampleRate::R24000,
            SampleRate::R16000,
            SampleRate::R12000,
            SampleRate::R8000,
        ]
        .iter(),
        [Channels::Mono, Channels::Stereo].iter()
    )
    .map(|(&sample_rate, &channels)| TestKind::RustDecode {
        sample_rate,
        channels,
    })
    .chain([
        TestKind::RustEncode { bitrate: 10_000 },
        TestKind::RustEncode { bitrate: 20_000 },
        TestKind::RustEncode { bitrate: 30_000 },
        TestKind::RustEncode { bitrate: 45_000 },
        TestKind::RustEncode { bitrate: 60_000 },
        TestKind::RustEncode { bitrate: 90_000 },
        TestKind::RustEncode { bitrate: 120_000 },
        TestKind::RustEncode { bitrate: 180_000 },
        TestKind::RustEncode { bitrate: 240_000 },
    ]);

    let tests = iproduct!(test_vectors.iter(), test_kinds).collect::<Vec<_>>();

    println!("Running {} tests in parallel", tests.len());

    let start_time = Instant::now();

    let results = tests
        .into_par_iter()
        .progress()
        // .into_iter()
        .map(|(test_vector, test_kind)| {
            (
                test_vector.name.as_str(),
                test_kind,
                run_test(test_vector, test_kind, args.dump_dir.as_deref()),
            )
        })
        .collect::<Vec<_>>();

    let elapsed = start_time.elapsed();
    println!("Ran {} tests in {:?}", results.len(), elapsed);

    for &(vector, kind, result) in &results {
        let kind = match kind {
            TestKind::RustDecode {
                channels,
                sample_rate,
            } => {
                let channels = match channels {
                    Channels::Mono => "M",
                    Channels::Stereo => "S",
                };
                let sample_rate = format!("{:02}k", usize::from(sample_rate) / 1000);

                format!("DEC {} {}    ", channels, sample_rate)
            }
            TestKind::RustEncode { bitrate } => {
                format!("ENC @ {:03}kbps", bitrate / 1000)
            }
        };

        println!("{}: {} -> {}", vector, kind, result);
    }

    let passed = results.iter().filter(|(_, _, r)| r.is_success()).count();

    println!("{}/{} passed", passed, results.len());

    if passed != results.len() {
        std::process::exit(1);
    }
}
