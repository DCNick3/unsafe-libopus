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

mod backend;
mod input;

pub use self::backend::OpusBackend;
use self::backend::{OpusBackendTrait, RustLibopusBackend, UpstreamLibopusBackend};

pub use input::{
    Application, Bandwidth, Channels, CommonOptions, Complexity, DecodeArgs, EncodeArgs,
    EncoderOptions, FrameSize, SampleRate,
};

use ::unsafe_libopus::{
    opus_strerror, OPUS_AUTO, OPUS_FRAMESIZE_ARG, OPUS_GET_FINAL_RANGE_REQUEST,
    OPUS_GET_LOOKAHEAD_REQUEST, OPUS_SET_BANDWIDTH_REQUEST, OPUS_SET_BITRATE_REQUEST,
    OPUS_SET_COMPLEXITY_REQUEST, OPUS_SET_DTX_REQUEST, OPUS_SET_EXPERT_FRAME_DURATION_REQUEST,
    OPUS_SET_FORCE_CHANNELS_REQUEST, OPUS_SET_LSB_DEPTH_REQUEST, OPUS_SET_VBR_CONSTRAINT_REQUEST,
    OPUS_SET_VBR_REQUEST,
};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Read, Write};

pub const MAX_PACKET: usize = 1500;
const MAX_FRAME_SIZE: usize = 48000 * 2;

fn handle_opus_error(err: i32, call: &str) -> u32 {
    if err < 0 {
        panic!("{} failed: {}", call, opus_strerror(err));
    }

    err as u32
}

macro_rules! checked_opus_encoder_ctl {
    ($B:ident, $st:expr, $request:expr, $($arg:expr),*) => {
        {
            let ret = $B::opus_encoder_ctl_impl(
                $st,
                $request,
                unsafe_libopus::varargs!($($arg),*)
            );

            handle_opus_error(ret, stringify!(opus_encoder_ctl!($st:expr, $request:expr, $($arg:expr),*)));
            ret
        }
    }
}

macro_rules! checked_opus_decoder_ctl {
    ($B:ident, $st:expr, $request:expr, $($arg:expr),*) => {
        {
            let ret = $B::opus_decoder_ctl_impl(
                $st,
                $request,
                unsafe_libopus::varargs!($($arg),*)
            );

            handle_opus_error(ret, stringify!(opus_decoder_ctl!($st:expr, $request:expr, $($arg:expr),*)));
            ret
        }
    }
}

/// Encode an opus stream, like `opus_demo -e`
///
/// See module documentation for the format of input and output data.
pub fn opus_demo_encode(backend: OpusBackend, data: &[u8], args: EncodeArgs) -> (Vec<u8>, usize) {
    match backend {
        OpusBackend::Rust => opus_demo_encode_impl::<RustLibopusBackend>(data, args),
        OpusBackend::Upstream => opus_demo_encode_impl::<UpstreamLibopusBackend>(data, args),
    }
}

fn opus_demo_encode_impl<B: OpusBackendTrait>(
    data: &[u8],
    EncodeArgs {
        sample_rate: sampling_rate,
        channels,
        application,
        bitrate,
        options,
    }: EncodeArgs,
) -> (Vec<u8>, usize) {
    let channels: usize = channels.into();

    let mut samples = Vec::new();
    for data in data.chunks_exact(2) {
        samples.push(i16::from_le_bytes(data.try_into().unwrap()));
    }

    let mut enc = {
        let mut err: i32 = 0;
        let enc = unsafe {
            B::opus_encoder_create(
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
        checked_opus_encoder_ctl!(B, &mut enc, OPUS_SET_BITRATE_REQUEST, bitrate as i32);
        checked_opus_encoder_ctl!(
            B,
            &mut enc,
            OPUS_SET_BANDWIDTH_REQUEST,
            options.bandwidth.map_or(OPUS_AUTO, |v| v.into_opus())
        );
        checked_opus_encoder_ctl!(B, &mut enc, OPUS_SET_VBR_REQUEST, !options.cbr as i32);
        checked_opus_encoder_ctl!(
            B,
            &mut enc,
            OPUS_SET_VBR_CONSTRAINT_REQUEST,
            options.cvbr as i32
        );
        checked_opus_encoder_ctl!(
            B,
            &mut enc,
            OPUS_SET_COMPLEXITY_REQUEST,
            i32::from(options.complexity)
        );
        checked_opus_encoder_ctl!(
            B,
            &mut enc,
            OPUS_SET_FORCE_CHANNELS_REQUEST,
            if options.forcemono { 1 } else { OPUS_AUTO }
        );
        checked_opus_encoder_ctl!(B, &mut enc, OPUS_SET_DTX_REQUEST, options.dtx as i32);
        checked_opus_encoder_ctl!(B, &mut enc, OPUS_GET_LOOKAHEAD_REQUEST, &mut skip);
        checked_opus_encoder_ctl!(B, &mut enc, OPUS_SET_LSB_DEPTH_REQUEST, 16);
        checked_opus_encoder_ctl!(
            B,
            &mut enc,
            OPUS_SET_EXPERT_FRAME_DURATION_REQUEST,
            OPUS_FRAMESIZE_ARG
        );
    }

    let frame_size: usize = options.framesize.samples_for_rate(sampling_rate);

    // pad samples with 0s to make it a multiple of frame_size
    let samples_len = samples.len();
    let samples_len = samples_len + (frame_size * channels - (samples_len % frame_size * channels));
    samples.resize(samples_len, 0);

    let mut output = Vec::<u8>::new();

    let mut buffer = vec![0u8; options.max_payload];
    for frame in samples.chunks_exact(frame_size * channels) {
        #[allow(unused)]
        let fpos = output.len();
        #[cfg(feature = "ent-dump")]
        eprintln!("START encoding packet @ 0x{:x}", fpos);

        let res = handle_opus_error(
            unsafe {
                B::opus_encode(
                    &mut enc,
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
            checked_opus_encoder_ctl!(
                B,
                &mut enc,
                OPUS_GET_FINAL_RANGE_REQUEST,
                &mut enc_final_range
            );
        };
        #[cfg(feature = "ent-dump")]
        eprintln!("END encoding packet @ 0x{:x}", fpos);

        output.write_i32::<BigEndian>(data.len() as i32).unwrap();
        output.write_u32::<BigEndian>(enc_final_range).unwrap();
        output.write_all(data).unwrap();
    }

    unsafe {
        // yes we leak the encoder on panics
        // so what?
        B::opus_encoder_destroy(enc);
    }

    (output, skip as usize)
}

/// Decode an opus stream, like `opus_demo -d`
///
/// See module documentation for the format of input and output data.
pub fn opus_demo_decode(backend: OpusBackend, data: &[u8], args: DecodeArgs) -> Vec<u8> {
    match backend {
        OpusBackend::Rust => opus_demo_decode_impl::<RustLibopusBackend>(data, args),
        OpusBackend::Upstream => opus_demo_decode_impl::<UpstreamLibopusBackend>(data, args),
    }
}

fn opus_demo_decode_impl<B: OpusBackendTrait>(
    data: &[u8],
    DecodeArgs {
        sample_rate,
        channels,
        options,
    }: DecodeArgs,
) -> Vec<u8> {
    let mut cursor = Cursor::new(data);
    let len = cursor.get_ref().len();

    let channels: usize = channels.into();

    let mut dec = {
        let mut err: i32 = 0;
        let dec = unsafe {
            B::opus_decoder_create(usize::from(sample_rate) as i32, channels as i32, &mut err)
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
                B::opus_decode(
                    &mut dec,
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
                checked_opus_decoder_ctl!(
                    B,
                    &mut dec,
                    OPUS_GET_FINAL_RANGE_REQUEST,
                    &mut dec_final_range
                );
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

    unsafe { B::opus_decoder_destroy(dec) };

    output
}

pub fn opus_demo_adjust_length(
    data: &mut Vec<u8>,
    pre_skip_48k: usize,
    orig_bytes_48k: usize,
    sample_rate: SampleRate,
    channels: Channels,
) {
    let sample_rate: usize = sample_rate.into();
    let channels: usize = channels.into();

    let samples_48k_to_current =
        |samples_48k: usize| samples_48k * sample_rate * channels / 48000 / 2;

    data.drain(..2 * samples_48k_to_current(pre_skip_48k));

    let final_len = samples_48k_to_current(orig_bytes_48k);

    // sanity check: the length should not differ more than a one frame of audio
    assert!(
        // two channels & two bytes per sample
        data.len().abs_diff(final_len) < 48000 * 2 * 2 / 50, // the default frame size is 20ms. currently it's the only tested frame size
        "length mismatch: {} vs {}",
        data.len(),
        final_len
    );

    data.resize(final_len, 0);
}
