// I would very much like to use clap here, but the opus_demo CLI is too weird for it to support:
// - different options depending on encode-only/decode-only/both mode (possible to implement but clunky)
// - single-hyphen options (not supported https://github.com/clap-rs/clap/issues/2468)
// so, yay, hand-writing CLI parsers yet again

use std::fmt::Debug;
use std::iter::Peekable;
use std::path::PathBuf;
use std::str::FromStr;
use unsafe_libopus::test::demo::{
    CommonOptions, DecodeArgs, EncodeArgs, EncoderOptions, MAX_PACKET,
};

#[rustfmt::skip]
fn usage(argv0: &str) {
    eprintln!("Usage: {} [-b <backend>]  [-e] <application> <sampling rate (Hz)> <channels (1/2)> <bits per second>  [options] <input> <output>", argv0);
    eprintln!("       {} [-b <backend>] -d <sampling rate (Hz)> <channels (1/2)> [options] <input> <output>", argv0);
    eprintln!("backend: unsafe | upstream");
    eprintln!("application: voip | audio | restricted-lowdelay");
    eprintln!("options:");
    eprintln!("-e                   : only runs the encoder (output the bit-stream)");
    eprintln!("-d                   : only runs the decoder (reads the bit-stream as input)");
    eprintln!("-cbr                 : use constant bit-rate; default: variable bitrate");
    eprintln!("-cvbr                : enable constrained variable bitrate; default: unconstrained");
    eprintln!("-delayed-decision    : use look-ahead for speech/music detection (experts only); default: disabled");
    eprintln!("-bandwidth <NB|MB|WB|SWB|FB> : audio bandwidth (from narrowband to fullband); default: sampling rate");
    eprintln!("-framesize <2.5|5|10|20|40|60|80|100|120> : frame size in ms; default: 20 ");
    eprintln!("-max_payload <bytes> : maximum payload size in bytes, default: 1024");
    eprintln!("-complexity <comp>   : complexity, 0 (lowest) ... 10 (highest); default: 10");
    eprintln!("-inbandfec           : enable SILK inband FEC");
    eprintln!("-forcemono           : force mono encoding, even for stereo input");
    eprintln!("-dtx                 : enable SILK DTX");
    eprintln!("-loss <perc>         : simulate packet loss, in percent (0-100); default: 0");
}

type Result<T, E = String> = std::result::Result<T, E>;

struct ArgsCursor<I: Iterator<Item = String>> {
    inner: Peekable<I>,
}

impl<I: Iterator<Item = String>> ArgsCursor<I> {
    pub fn new(iter: I) -> Self {
        Self {
            inner: iter.peekable(),
        }
    }

    pub fn peek(&mut self) -> Result<&str> {
        self.inner
            .peek()
            .ok_or_else(|| "Unexpected end of arguments".to_string())
            .map(String::as_str)
    }

    pub fn next<T: FromStr<Err = E>, E: Debug>(&mut self) -> Result<T> {
        self.inner
            .next()
            .ok_or_else(|| "Unexpected end of arguments".to_string())
            .and_then(|s| {
                s.parse()
                    .map_err(|e| format!("Parsing `{}` failed: {:?}", s, e))
            })
    }

    pub fn poke(&mut self, expected: &str) -> Result<bool> {
        let value = self.peek()?;
        if value == expected {
            self.next::<String, _>()
                .expect("BUG: peek() returned Some(...), but next() returned None");
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn finish(mut self) -> Result<()> {
        match self.inner.next() {
            None => Ok(()),
            Some(arg) => Err(format!("Unexpected argument: {}", arg)),
        }
    }
}

fn parse_common_options<I: Iterator<Item = String>>(
    args: &mut ArgsCursor<I>,
) -> Result<CommonOptions> {
    let mut result = CommonOptions::default();

    loop {
        if args.poke("-inbandfec")? {
            result.inbandfec = true;
        } else if args.poke("-loss")? {
            result.loss = args.next()?;
            if result.loss > 100 {
                return Err("Loss percentage must be between 0 and 100".to_string());
            }
        } else {
            break;
        }
    }

    Ok(result)
}

fn parse_encoder_options<I: Iterator<Item = String>>(
    args: &mut ArgsCursor<I>,
) -> Result<EncoderOptions> {
    let mut result = EncoderOptions::default();

    loop {
        if args.poke("-cbr")? {
            result.cbr = true;
        } else if args.poke("-bandwidth")? {
            result.bandwidth = Some(args.next()?);
        } else if args.poke("-framesize")? {
            result.framesize = args.next()?;
        } else if args.poke("-max_payload")? {
            result.max_payload = args.next()?;
            if result.max_payload > MAX_PACKET {
                return Err(format!("max_payload must be <= {}", MAX_PACKET));
            }
        } else if args.poke("-complexity")? {
            result.complexity = args.next()?;
        } else if args.poke("-forcemono")? {
            result.forcemono = true;
        } else if args.poke("-cvbr")? {
            result.cvbr = true;
        } else if args.poke("-delayed_decision")? {
            result.delayed_decision = true;
        } else if args.poke("-dtx")? {
            result.dtx = true;
        } else if args.poke("-inbandfec")? {
            result.common.inbandfec = true;
        } else if args.poke("-loss")? {
            result.common.loss = args.next()?;
            if result.common.loss > 100 {
                return Err("Loss percentage must be between 0 and 100".to_string());
            }
        } else {
            break;
        }
    }

    Ok(result)
}

fn parse_encode_args<I: Iterator<Item = String>>(args: &mut ArgsCursor<I>) -> Result<EncodeArgs> {
    let application = args.next()?;
    let sampling_rate = args.next()?;
    let channels = args.next()?;
    let bitrate = args.next()?;

    let options = parse_encoder_options(args)?;

    Ok(EncodeArgs {
        application,
        sample_rate: sampling_rate,
        channels,
        bitrate,
        options,
    })
}

fn parse_decode_args<I: Iterator<Item = String>>(args: &mut ArgsCursor<I>) -> Result<DecodeArgs> {
    let sampling_rate = args.next()?;
    let channels = args.next()?;

    let options = parse_common_options(args)?;

    Ok(DecodeArgs {
        sample_rate: sampling_rate,
        channels,
        options,
    })
}

#[derive(Debug, Copy, Clone, Default)]
pub enum Backend {
    #[default]
    RustLibOpus,
    UpstreamLibOpus,
}

impl FromStr for Backend {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "unsafe" => Ok(Backend::RustLibOpus),
            "upstream" => Ok(Backend::UpstreamLibOpus),
            _ => Err("Invalid backend"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Mode {
    EncodeDecode(EncodeArgs),
    EncodeOnly(EncodeArgs),
    DecodeOnly(DecodeArgs),
}

#[derive(Debug, Clone)]
pub struct Cli {
    pub backend: Backend,
    pub mode: Mode,
    pub input: PathBuf,
    pub output: PathBuf,
}

impl Cli {
    pub fn parse() -> Self {
        let mut args = std::env::args().peekable();
        let argv0 = args.peek().unwrap().clone();
        match Self::try_parse_from(args) {
            Ok(cli) => cli,
            Err(err) => {
                eprintln!("Could not parse arguments: {}", err);
                usage(&argv0);
                std::process::exit(1);
            }
        }
    }

    pub fn try_parse_from<I, T>(iter: I) -> Result<Self>
    where
        I: IntoIterator<Item = T>,
        T: Into<String> + Clone,
    {
        let mut args = ArgsCursor::new(iter.into_iter().map(Into::into));
        let _app = args
            .next::<String, _>()
            .map_err(|_| "Missing application".to_string())?;

        let backend = if args.poke("-b")? {
            args.next().map_err(|_| "Missing backend".to_string())?
        } else {
            Backend::default()
        };

        let mode = if args.poke("-e")? {
            Mode::EncodeOnly(parse_encode_args(&mut args)?)
        } else if args.poke("-d")? {
            Mode::DecodeOnly(parse_decode_args(&mut args)?)
        } else {
            Mode::EncodeDecode(parse_encode_args(&mut args)?)
        };

        let input = args.next().map_err(|_| "Missing input file")?;
        let output = args.next().map_err(|_| "Missing output file")?;

        args.finish()?;

        Ok(Self {
            backend,
            mode,
            input,
            output,
        })
    }
}
