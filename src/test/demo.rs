//! This module implements a subset of the `opus_demo` functionality in a library form
//!
//! It can be used to do end-to-end tests of opus encoding and decoding
//!
//! For PCM files, it expects a raw 16-bit signed PCM stream
//!
//! Use it in `ffmpeg` or `ffplay` with `-f s16le -ar <sample_rate> -ac <channels> -i <file>`
//!
//! For opus-compressed files it uses a simplistic mux format. Each opus packet is prefixed with two 32-bit big-endian integers:
//! - packet length in bytes
//! - range coder state (something internal, used as an additional sanity check)
//!
//! Then the opus packet itself follows
//!
//! I am not aware of any tools (except opus_demo from the official distribution) supporting this format

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Read, Write};

use crate::{
    opus_decode, opus_decoder_create, opus_decoder_ctl, opus_decoder_destroy, opus_encode,
    opus_encoder_create, opus_encoder_ctl, opus_encoder_destroy, opus_strerror,
    OPUS_APPLICATION_AUDIO, OPUS_APPLICATION_RESTRICTED_LOWDELAY, OPUS_APPLICATION_VOIP, OPUS_AUTO,
    OPUS_BANDWIDTH_FULLBAND, OPUS_BANDWIDTH_MEDIUMBAND, OPUS_BANDWIDTH_NARROWBAND,
    OPUS_BANDWIDTH_SUPERWIDEBAND, OPUS_BANDWIDTH_WIDEBAND, OPUS_FRAMESIZE_ARG,
    OPUS_GET_FINAL_RANGE_REQUEST, OPUS_GET_LOOKAHEAD_REQUEST, OPUS_SET_BANDWIDTH_REQUEST,
    OPUS_SET_BITRATE_REQUEST, OPUS_SET_COMPLEXITY_REQUEST, OPUS_SET_DTX_REQUEST,
    OPUS_SET_EXPERT_FRAME_DURATION_REQUEST, OPUS_SET_FORCE_CHANNELS_REQUEST,
    OPUS_SET_LSB_DEPTH_REQUEST, OPUS_SET_VBR_CONSTRAINT_REQUEST, OPUS_SET_VBR_REQUEST,
};
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum Application {
    Voip,
    Audio,
    RestrictedLowDelay,
}

impl Application {
    pub fn into_opus(self) -> i32 {
        match self {
            Application::Voip => OPUS_APPLICATION_VOIP,
            Application::Audio => OPUS_APPLICATION_AUDIO,
            Application::RestrictedLowDelay => OPUS_APPLICATION_RESTRICTED_LOWDELAY,
        }
    }
}

impl FromStr for Application {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "voip" => Ok(Application::Voip),
            "audio" => Ok(Application::Audio),
            "restricted-lowdelay" => Ok(Application::RestrictedLowDelay),
            _ => Err("Unsupported application"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SampleRate {
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

#[derive(Debug, Copy, Clone)]
pub enum Channels {
    Mono,
    Stereo,
}

impl FromStr for Channels {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Channels::Mono),
            "2" => Ok(Channels::Stereo),
            _ => Err("Unsupported number of channels"),
        }
    }
}

impl From<Channels> for usize {
    fn from(value: Channels) -> Self {
        match value {
            Channels::Mono => 1,
            Channels::Stereo => 2,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Bandwidth {
    Narrowband,
    Mediumband,
    Wideband,
    SuperWideband,
    Fullband,
}

impl Bandwidth {
    pub fn into_opus(self) -> i32 {
        match self {
            Bandwidth::Narrowband => OPUS_BANDWIDTH_NARROWBAND,
            Bandwidth::Mediumband => OPUS_BANDWIDTH_MEDIUMBAND,
            Bandwidth::Wideband => OPUS_BANDWIDTH_WIDEBAND,
            Bandwidth::SuperWideband => OPUS_BANDWIDTH_SUPERWIDEBAND,
            Bandwidth::Fullband => OPUS_BANDWIDTH_FULLBAND,
        }
    }
}

impl FromStr for Bandwidth {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NB" => Ok(Bandwidth::Narrowband),
            "MB" => Ok(Bandwidth::Mediumband),
            "WB" => Ok(Bandwidth::Wideband),
            "SWB" => Ok(Bandwidth::SuperWideband),
            "FB" => Ok(Bandwidth::Fullband),
            _ => Err("Unsupported bandwidth"),
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub enum FrameSize {
    Ms2_5,
    Ms5,
    Ms10,
    #[default]
    Ms20,
    Ms40,
    Ms60,
    Ms80,
    Ms100,
    Ms120,
}

impl FrameSize {
    pub fn samples_for_rate(self, rate: SampleRate) -> usize {
        let rate: usize = rate.into();
        match self {
            FrameSize::Ms2_5 => rate / 400,
            FrameSize::Ms5 => rate / 200,
            FrameSize::Ms10 => rate / 100,
            FrameSize::Ms20 => rate / 50,
            FrameSize::Ms40 => rate / 25,
            FrameSize::Ms60 => 3 * rate / 50,
            FrameSize::Ms80 => 4 * rate / 50,
            FrameSize::Ms100 => 5 * rate / 50,
            FrameSize::Ms120 => 6 * rate / 50,
        }
    }
}

impl FromStr for FrameSize {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2.5" => Ok(FrameSize::Ms2_5),
            "5" => Ok(FrameSize::Ms5),
            "10" => Ok(FrameSize::Ms10),
            "20" => Ok(FrameSize::Ms20),
            "40" => Ok(FrameSize::Ms40),
            "60" => Ok(FrameSize::Ms60),
            "80" => Ok(FrameSize::Ms80),
            "100" => Ok(FrameSize::Ms100),
            "120" => Ok(FrameSize::Ms120),
            _ => Err("Unsupported frame size"),
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub enum Complexity {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    #[default]
    C10,
}

impl FromStr for Complexity {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Complexity::C0),
            "1" => Ok(Complexity::C1),
            "2" => Ok(Complexity::C2),
            "3" => Ok(Complexity::C3),
            "4" => Ok(Complexity::C4),
            "5" => Ok(Complexity::C5),
            "6" => Ok(Complexity::C6),
            "7" => Ok(Complexity::C7),
            "8" => Ok(Complexity::C8),
            "9" => Ok(Complexity::C9),
            "10" => Ok(Complexity::C10),
            _ => Err("Unknown complexity"),
        }
    }
}

impl From<Complexity> for i32 {
    fn from(value: Complexity) -> Self {
        match value {
            Complexity::C0 => 0,
            Complexity::C1 => 1,
            Complexity::C2 => 2,
            Complexity::C3 => 3,
            Complexity::C4 => 4,
            Complexity::C5 => 5,
            Complexity::C6 => 6,
            Complexity::C7 => 7,
            Complexity::C8 => 8,
            Complexity::C9 => 9,
            Complexity::C10 => 10,
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct CommonOptions {
    /// Enable SILK inband FEC
    pub inbandfec: bool,
    /// Simulate packet loss, in percent (0-100)
    pub loss: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct EncoderOptions {
    pub cbr: bool,
    pub bandwidth: Option<Bandwidth>,
    pub framesize: FrameSize,
    pub max_payload: usize,
    pub complexity: Complexity,
    pub forcemono: bool,
    pub cvbr: bool,
    pub delayed_decision: bool,
    pub dtx: bool,
    pub common: CommonOptions,
    // TODO: do we need to support undocumented options?
    // there's a bunch of them:
    // -sweep, -random_framesize, -sweep_max, -random_fec
    // -silk8k_test, -silk12k_test, -silk16k_test
    // -hybrid24k_test, -hybrid48k_test, -celt_test, -celt_hq_test
}

impl Default for EncoderOptions {
    fn default() -> Self {
        Self {
            cbr: false,
            bandwidth: None,
            framesize: FrameSize::Ms20,
            max_payload: MAX_PACKET,
            complexity: Complexity::C10,
            forcemono: false,
            cvbr: false,
            delayed_decision: false,
            dtx: false,
            common: CommonOptions::default(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct EncodeArgs {
    pub application: Application,
    pub sampling_rate: SampleRate,
    pub channels: Channels,
    pub bitrate: u32,
    pub options: EncoderOptions,
}

#[derive(Debug, Copy, Clone)]
pub struct DecodeArgs {
    pub sampling_rate: SampleRate,
    pub channels: Channels,
    pub options: CommonOptions,
}

pub const MAX_PACKET: usize = 1500;
const MAX_FRAME_SIZE: usize = 48000 * 2;

fn handle_opus_error(err: i32, call: &str) -> u32 {
    if err < 0 {
        panic!("{} failed: {}", call, opus_strerror(err));
    }

    err as u32
}

macro_rules! checked_opus_encoder_ctl {
    ($($tt:tt)*) => {
        {
            let ret = opus_encoder_ctl!($($tt)*);
            handle_opus_error(ret, stringify!(opus_encoder_ctl!($($tt)*)));
            ret
        }
    }
}

macro_rules! checked_opus_decoder_ctl {
    ($($tt:tt)*) => {
        {
            let ret = opus_decoder_ctl!($($tt)*);
            handle_opus_error(ret, stringify!(opus_decoder_ctl!($($tt)*)));
            ret
        }
    }
}

/// Encode an opus stream, , like `opus_demo -e`
///
/// See module documentation for the format of input and output data.
pub fn opus_demo_encode(
    data: &[u8],
    EncodeArgs {
        sampling_rate,
        channels,
        application,
        bitrate,
        options,
    }: EncodeArgs,
) -> Vec<u8> {
    let channels: usize = channels.into();

    let mut samples = Vec::new();
    for data in data.chunks_exact(2) {
        samples.push(i16::from_le_bytes(data.try_into().unwrap()));
    }

    let enc = {
        let mut err: i32 = 0;
        let enc = unsafe {
            opus_encoder_create(
                usize::from(sampling_rate) as i32,
                channels as i32,
                application.into_opus(),
                &mut err,
            )
        };
        handle_opus_error(err, "opus_encoder_create");
        enc
    };

    let mut skip: i32 = 0;

    if options.common.inbandfec {
        panic!("inbandfec not supported")
    }
    if options.common.loss != 0 {
        panic!("packet loss simulation not supported")
    }

    unsafe {
        checked_opus_encoder_ctl!(enc, OPUS_SET_BITRATE_REQUEST, bitrate as i32,);
        checked_opus_encoder_ctl!(
            enc,
            OPUS_SET_BANDWIDTH_REQUEST,
            options.bandwidth.map_or(OPUS_AUTO, |v| v.into_opus()),
        );
        checked_opus_encoder_ctl!(enc, OPUS_SET_VBR_REQUEST, !options.cbr as i32,);
        checked_opus_encoder_ctl!(enc, OPUS_SET_VBR_CONSTRAINT_REQUEST, options.cvbr as i32,);
        checked_opus_encoder_ctl!(
            enc,
            OPUS_SET_COMPLEXITY_REQUEST,
            i32::from(options.complexity),
        );
        checked_opus_encoder_ctl!(
            enc,
            OPUS_SET_FORCE_CHANNELS_REQUEST,
            if options.forcemono { 1 } else { OPUS_AUTO },
        );
        checked_opus_encoder_ctl!(enc, OPUS_SET_DTX_REQUEST, options.dtx as i32,);
        checked_opus_encoder_ctl!(enc, OPUS_GET_LOOKAHEAD_REQUEST, &mut skip);
        checked_opus_encoder_ctl!(enc, OPUS_SET_LSB_DEPTH_REQUEST, 16,);
        checked_opus_encoder_ctl!(
            enc,
            OPUS_SET_EXPERT_FRAME_DURATION_REQUEST,
            OPUS_FRAMESIZE_ARG,
        );
    }

    let frame_size: usize = options.framesize.samples_for_rate(sampling_rate);

    // pad samples with 0s to make it a multiple of frame_size
    let samples_len = samples.len();
    let samples_len = samples_len + (frame_size - (samples_len % frame_size));
    samples.resize(samples_len, 0);

    let mut output = Vec::<u8>::new();

    let mut buffer = vec![0u8; options.max_payload];
    for frame in samples.chunks_exact(frame_size * channels) {
        let res = handle_opus_error(
            unsafe {
                opus_encode(
                    enc,
                    frame.as_ptr(),
                    // it's not the length of the frame slice!
                    frame_size as i32,
                    buffer.as_mut_ptr(),
                    buffer.len() as i32,
                )
            },
            "opus_encode",
        );
        let data = &buffer[..res as usize];

        let mut enc_final_range: u32 = 0;
        unsafe {
            checked_opus_encoder_ctl!(enc, OPUS_GET_FINAL_RANGE_REQUEST, &mut enc_final_range);
        };

        output.write_i32::<BigEndian>(data.len() as i32).unwrap();
        output.write_u32::<BigEndian>(enc_final_range).unwrap();
        output.write_all(data).unwrap();
    }

    unsafe {
        // yes we leak the encoder on panics
        // so what?
        opus_encoder_destroy(enc);
    }

    output
}

/// Decode an opus stream, like `opus_demo -d`
///
/// See module documentation for the format of input and output data.
pub fn opus_demo_decode(
    data: &[u8],
    DecodeArgs {
        sampling_rate,
        channels,
        options,
    }: DecodeArgs,
) -> Vec<u8> {
    let mut cursor = Cursor::new(data);
    let len = cursor.get_ref().len();

    let channels: usize = channels.into();

    let dec = {
        let mut err: i32 = 0;
        let dec = unsafe {
            opus_decoder_create(usize::from(sampling_rate) as i32, channels as i32, &mut err)
        };
        handle_opus_error(err, "opus_decoder_create()");
        dec
    };

    if options.inbandfec {
        panic!("inbandfec not supported")
    }
    if options.loss != 0 {
        panic!("packet loss simulation not supported")
    }

    let mut frame_idx = 0;

    let mut data = vec![0u8; MAX_PACKET];
    let mut samples = vec![0i16; MAX_FRAME_SIZE * channels];
    let mut output = Vec::<u8>::new();

    while cursor.position() < len as u64 {
        let data_bytes = cursor.read_u32::<BigEndian>().unwrap();
        let enc_final_range = cursor.read_u32::<BigEndian>().unwrap();

        let data = &mut data[..data_bytes as usize];
        cursor.read_exact(data).unwrap();

        let output_samples = handle_opus_error(
            unsafe {
                opus_decode(
                    dec,
                    data.as_ptr(),
                    data.len() as i32,
                    samples.as_mut_ptr(),
                    MAX_FRAME_SIZE as i32,
                    0,
                )
            },
            "opus_decode",
        );
        let samples = &samples[..output_samples as usize * channels];

        let dec_final_range = {
            let mut dec_final_range: u32 = 0;
            unsafe {
                checked_opus_decoder_ctl!(dec, OPUS_GET_FINAL_RANGE_REQUEST, &mut dec_final_range);
            };
            dec_final_range
        };

        assert_eq!(
            enc_final_range, dec_final_range,
            "Range coder state mismatch between encoder and decoder in frame {}",
            frame_idx
        );

        for sample in samples {
            output.extend_from_slice(&sample.to_le_bytes());
        }

        frame_idx += 1;
    }

    unsafe { opus_decoder_destroy(dec) };

    output
}
