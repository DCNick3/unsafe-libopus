#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]

use ::unsafe_libopus::varargs::VarArgs;
use std::str::FromStr;

pub(crate) trait OpusBackendTrait {
    type Encoder;
    type Decoder;

    unsafe fn opus_encoder_create(
        Fs: i32,
        channels: i32,
        application: i32,
    ) -> Result<Self::Encoder, i32>;
    unsafe fn opus_encoder_ctl_impl(st: &mut Self::Encoder, request: i32, args: VarArgs) -> i32;
    unsafe fn opus_encode(
        st: &mut Self::Encoder,
        pcm: *const i16,
        analysis_frame_size: i32,
        data: *mut u8,
        max_data_bytes: i32,
    ) -> i32;
    unsafe fn opus_encoder_destroy(st: Self::Encoder);

    unsafe fn opus_decoder_create(Fs: i32, channels: i32) -> Result<Self::Decoder, i32>;
    unsafe fn opus_decode(
        st: &mut Self::Decoder,
        data: *const u8,
        len: i32,
        pcm: *mut i16,
        frame_size: i32,
        decode_fec: i32,
    ) -> i32;
    unsafe fn opus_decoder_ctl_impl(st: &mut Self::Decoder, request: i32, args: VarArgs) -> i32;
    unsafe fn opus_decoder_destroy(st: Self::Decoder);
}

mod unsafe_libopus {
    use unsafe_libopus::{
        opus_decode, opus_decoder_ctl_impl, opus_encode, opus_encoder_create,
        opus_encoder_ctl_impl, opus_encoder_destroy, varargs::VarArgs, OpusDecoder, OpusEncoder,
    };

    pub struct RustLibopusBackend;

    impl super::OpusBackendTrait for RustLibopusBackend {
        type Encoder = *mut OpusEncoder;
        type Decoder = Box<OpusDecoder>;

        unsafe fn opus_encoder_create(
            Fs: i32,
            channels: i32,
            application: i32,
        ) -> Result<*mut OpusEncoder, i32> {
            let mut error = 0;

            let res = opus_encoder_create(Fs, channels, application, &mut error);
            if res.is_null() {
                Err(error)
            } else {
                Ok(res)
            }
        }

        unsafe fn opus_encoder_ctl_impl(
            &mut st: &mut *mut OpusEncoder,
            request: i32,
            args: VarArgs,
        ) -> i32 {
            opus_encoder_ctl_impl(st, request, args)
        }

        unsafe fn opus_encode(
            &mut st: &mut *mut OpusEncoder,
            pcm: *const i16,
            analysis_frame_size: i32,
            data: *mut u8,
            max_data_bytes: i32,
        ) -> i32 {
            opus_encode(st, pcm, analysis_frame_size, data, max_data_bytes)
        }

        unsafe fn opus_encoder_destroy(st: *mut OpusEncoder) {
            opus_encoder_destroy(st)
        }

        unsafe fn opus_decoder_create(Fs: i32, channels: i32) -> Result<Box<OpusDecoder>, i32> {
            OpusDecoder::new(Fs, channels as usize).map(Box::new)
        }

        unsafe fn opus_decode(
            st: &mut Box<OpusDecoder>,
            data: *const u8,
            len: i32,
            pcm: *mut i16,
            frame_size: i32,
            decode_fec: i32,
        ) -> i32 {
            opus_decode(st.as_mut(), data, len, pcm, frame_size, decode_fec)
        }

        unsafe fn opus_decoder_ctl_impl(
            st: &mut Box<OpusDecoder>,
            request: i32,
            args: VarArgs,
        ) -> i32 {
            opus_decoder_ctl_impl(st.as_mut(), request, args)
        }

        unsafe fn opus_decoder_destroy(st: Box<OpusDecoder>) {
            drop(st)
        }
    }
}
pub(crate) use unsafe_libopus::RustLibopusBackend;

mod libopus {
    use unsafe_libopus::varargs::{VarArg, VarArgs};
    use upstream_libopus::{
        opus_decode, opus_decoder_create, opus_decoder_ctl, opus_decoder_destroy, opus_encode,
        opus_encoder_create, opus_encoder_ctl, opus_encoder_destroy,
    };
    use upstream_libopus::{OpusDecoder, OpusEncoder};

    pub struct UpstreamLibopusBackend;

    impl super::OpusBackendTrait for UpstreamLibopusBackend {
        type Encoder = *mut OpusEncoder;
        type Decoder = *mut OpusDecoder;

        unsafe fn opus_encoder_create(
            Fs: i32,
            channels: i32,
            application: i32,
        ) -> Result<*mut OpusEncoder, i32> {
            let mut error = 0;

            let res = opus_encoder_create(Fs, channels, application, &mut error);
            if res.is_null() {
                Err(error)
            } else {
                Ok(res)
            }
        }

        unsafe fn opus_encoder_ctl_impl(
            &mut st: &mut *mut OpusEncoder,
            request: i32,
            mut args: VarArgs,
        ) -> i32 {
            match &mut args.0[..] {
                [VarArg::I32(arg)] => opus_encoder_ctl(st, request, *arg),
                [VarArg::I32Out(arg)] => opus_encoder_ctl(st, request, *arg as *mut _),
                [VarArg::U32Out(arg)] => opus_encoder_ctl(st, request, *arg as *mut _),
                // manually match over all required signatures
                _ => todo!("opus_decoder_ctl signature not implemented"),
            }
        }

        unsafe fn opus_encode(
            &mut st: &mut *mut OpusEncoder,
            pcm: *const i16,
            analysis_frame_size: i32,
            data: *mut u8,
            max_data_bytes: i32,
        ) -> i32 {
            opus_encode(st, pcm, analysis_frame_size, data, max_data_bytes)
        }

        unsafe fn opus_encoder_destroy(st: *mut OpusEncoder) {
            opus_encoder_destroy(st)
        }

        unsafe fn opus_decoder_create(Fs: i32, channels: i32) -> Result<*mut OpusDecoder, i32> {
            let mut error = 0;
            let res = opus_decoder_create(Fs, channels, &mut error);
            if res.is_null() {
                Err(error)
            } else {
                Ok(res)
            }
        }

        unsafe fn opus_decode(
            &mut st: &mut *mut OpusDecoder,
            data: *const u8,
            len: i32,
            pcm: *mut i16,
            frame_size: i32,
            decode_fec: i32,
        ) -> i32 {
            opus_decode(st, data, len, pcm, frame_size, decode_fec)
        }

        unsafe fn opus_decoder_ctl_impl(
            &mut st: &mut *mut OpusDecoder,
            request: i32,
            mut args: VarArgs,
        ) -> i32 {
            match &mut args.0[..] {
                // manually match over all required signatures
                [VarArg::U32Out(ptr)] => opus_decoder_ctl(st, request, *ptr as *mut _),
                _ => todo!("opus_decoder_ctl signature not implemented"),
            }
        }

        unsafe fn opus_decoder_destroy(st: *mut OpusDecoder) {
            opus_decoder_destroy(st)
        }
    }
}
pub(crate) use libopus::UpstreamLibopusBackend;

#[derive(Debug, Copy, Clone, Default)]
pub enum OpusBackend {
    #[default]
    Rust,
    Upstream,
}

impl FromStr for OpusBackend {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "unsafe" => Ok(OpusBackend::Rust),
            "upstream" => Ok(OpusBackend::Upstream),
            _ => Err("Invalid backend"),
        }
    }
}
