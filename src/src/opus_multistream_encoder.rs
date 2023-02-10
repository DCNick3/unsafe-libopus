use ::libc;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __builtin_va_list = [__va_list_tag; 1];
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "0:0"]
    pub struct __va_list_tag {
        pub gp_offset: libc::c_uint,
        pub fp_offset: libc::c_uint,
        pub overflow_arg_area: *mut libc::c_void,
        pub reg_save_area: *mut libc::c_void,
    }
}
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "44:1"]
    pub type __int64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::{__int16_t, __int32_t, __int64_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:32"]
pub mod opus_types_h {
    #[c2rust::src_loc = "53:4"]
    pub type opus_int16 = int16_t;
    #[c2rust::src_loc = "55:4"]
    pub type opus_int32 = int32_t;
    #[c2rust::src_loc = "56:4"]
    pub type opus_uint32 = uint32_t;
    #[c2rust::src_loc = "57:4"]
    pub type opus_int64 = int64_t;
    use super::stdint_intn_h::{int16_t, int32_t, int64_t};
    use super::stdint_uintn_h::uint32_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus.h:32"]
pub mod opus_h {
    use super::opus_private_h::OpusRepacketizer;
    use super::opus_types_h::opus_int32;
    extern "C" {
        #[c2rust::src_loc = "164:16"]
        pub type OpusEncoder;
        #[c2rust::src_loc = "171:1"]
        pub fn opus_encoder_get_size(channels: libc::c_int) -> libc::c_int;
        #[c2rust::src_loc = "228:1"]
        pub fn opus_encoder_init(
            st: *mut OpusEncoder,
            Fs: opus_int32,
            channels: libc::c_int,
            application: libc::c_int,
        ) -> libc::c_int;
        #[c2rust::src_loc = "328:1"]
        pub fn opus_encoder_ctl(st: *mut OpusEncoder, request: libc::c_int, _: ...) -> libc::c_int;
        #[c2rust::src_loc = "838:1"]
        pub fn opus_repacketizer_cat(
            rp: *mut OpusRepacketizer,
            data: *const libc::c_uchar,
            len: opus_int32,
        ) -> libc::c_int;
        #[c2rust::src_loc = "778:1"]
        pub fn opus_repacketizer_init(rp: *mut OpusRepacketizer) -> *mut OpusRepacketizer;
        #[c2rust::src_loc = "884:1"]
        pub fn opus_repacketizer_get_nb_frames(rp: *mut OpusRepacketizer) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/src/opus_private.h:34"]
pub mod opus_private_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "39:8"]
    pub struct OpusRepacketizer {
        pub toc: libc::c_uchar,
        pub nb_frames: libc::c_int,
        pub frames: [*const libc::c_uchar; 48],
        pub len: [opus_int16; 48],
        pub framesize: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "60:8"]
    pub struct OpusMSEncoder {
        pub layout: ChannelLayout,
        pub arch: libc::c_int,
        pub lfe_stream: libc::c_int,
        pub application: libc::c_int,
        pub variable_duration: libc::c_int,
        pub mapping_type: MappingType,
        pub bitrate_bps: opus_int32,
    }
    #[c2rust::src_loc = "54:9"]
    pub type MappingType = libc::c_uint;
    #[c2rust::src_loc = "57:3"]
    pub const MAPPING_TYPE_AMBISONICS: MappingType = 2;
    #[c2rust::src_loc = "56:3"]
    pub const MAPPING_TYPE_SURROUND: MappingType = 1;
    #[c2rust::src_loc = "55:3"]
    pub const MAPPING_TYPE_NONE: MappingType = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "47:16"]
    pub struct ChannelLayout {
        pub nb_channels: libc::c_int,
        pub nb_streams: libc::c_int,
        pub nb_coupled_streams: libc::c_int,
        pub mapping: [libc::c_uchar; 256],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "156:12"]
    pub struct foo {
        pub c: libc::c_char,
        pub u: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "156:25"]
    pub union C2RustUnnamed {
        pub p: *mut libc::c_void,
        pub i: opus_int32,
        pub v: opus_val32,
    }
    #[c2rust::src_loc = "135:1"]
    pub type downmix_func = Option<
        unsafe extern "C" fn(
            *const libc::c_void,
            *mut opus_val32,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >;
    #[c2rust::src_loc = "88:1"]
    pub type opus_copy_channel_in_func = Option<
        unsafe extern "C" fn(
            *mut opus_val16,
            libc::c_int,
            *const libc::c_void,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *mut libc::c_void,
        ) -> (),
    >;
    #[inline]
    #[c2rust::src_loc = "154:1"]
    pub unsafe extern "C" fn align(mut i: libc::c_int) -> libc::c_int {
        let mut alignment: libc::c_uint = 8 as libc::c_ulong as libc::c_uint;
        return (i as libc::c_uint)
            .wrapping_add(alignment)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_div(alignment)
            .wrapping_mul(alignment) as libc::c_int;
    }
    use super::arch_h::{opus_val16, opus_val32};
    use super::opus_h::OpusEncoder;
    use super::opus_types_h::{opus_int16, opus_int32};
    extern "C" {
        #[c2rust::src_loc = "136:1"]
        pub fn downmix_float(
            _x: *const libc::c_void,
            sub: *mut opus_val32,
            subframe: libc::c_int,
            offset: libc::c_int,
            c1: libc::c_int,
            c2: libc::c_int,
            C: libc::c_int,
        );
        #[c2rust::src_loc = "142:1"]
        pub fn frame_size_select(
            frame_size: opus_int32,
            variable_duration: libc::c_int,
            Fs: opus_int32,
        ) -> opus_int32;
        #[c2rust::src_loc = "144:1"]
        pub fn opus_encode_native(
            st: *mut OpusEncoder,
            pcm: *const opus_val16,
            frame_size: libc::c_int,
            data: *mut libc::c_uchar,
            out_data_bytes: opus_int32,
            lsb_depth: libc::c_int,
            analysis_pcm: *const libc::c_void,
            analysis_size: opus_int32,
            c1: libc::c_int,
            c2: libc::c_int,
            analysis_channels: libc::c_int,
            downmix: downmix_func,
            float_api: libc::c_int,
        ) -> opus_int32;
        #[c2rust::src_loc = "170:1"]
        pub fn opus_repacketizer_out_range_impl(
            rp: *mut OpusRepacketizer,
            begin: libc::c_int,
            end: libc::c_int,
            data: *mut libc::c_uchar,
            maxlen: opus_int32,
            self_delimited: libc::c_int,
            pad: libc::c_int,
        ) -> opus_int32;
        #[c2rust::src_loc = "137:1"]
        pub fn downmix_int(
            _x: *const libc::c_void,
            sub: *mut opus_val32,
            subframe: libc::c_int,
            offset: libc::c_int,
            c1: libc::c_int,
            c2: libc::c_int,
            C: libc::c_int,
        );
        #[c2rust::src_loc = "83:1"]
        pub fn validate_layout(layout: *const ChannelLayout) -> libc::c_int;
        #[c2rust::src_loc = "84:1"]
        pub fn get_left_channel(
            layout: *const ChannelLayout,
            stream_id: libc::c_int,
            prev: libc::c_int,
        ) -> libc::c_int;
        #[c2rust::src_loc = "85:1"]
        pub fn get_right_channel(
            layout: *const ChannelLayout,
            stream_id: libc::c_int,
            prev: libc::c_int,
        ) -> libc::c_int;
        #[c2rust::src_loc = "86:1"]
        pub fn get_mono_channel(
            layout: *const ChannelLayout,
            stream_id: libc::c_int,
            prev: libc::c_int,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:34"]
pub mod arch_h {
    #[c2rust::src_loc = "180:1"]
    pub type opus_val32 = libc::c_float;
    #[c2rust::src_loc = "179:1"]
    pub type opus_val16 = libc::c_float;
    #[c2rust::src_loc = "185:1"]
    pub type celt_ener = libc::c_float;
    #[c2rust::src_loc = "183:1"]
    pub type celt_sig = libc::c_float;
    extern "C" {
        #[c2rust::src_loc = "63:1"]
        pub fn celt_fatal(
            str: *const libc::c_char,
            file: *const libc::c_char,
            line: libc::c_int,
        ) -> !;
    }
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:34"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/modes.h:41"]
pub mod modes_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:8"]
    pub struct OpusCustomMode {
        pub Fs: opus_int32,
        pub overlap: libc::c_int,
        pub nbEBands: libc::c_int,
        pub effEBands: libc::c_int,
        pub preemph: [opus_val16; 4],
        pub eBands: *const opus_int16,
        pub maxLM: libc::c_int,
        pub nbShortMdcts: libc::c_int,
        pub shortMdctSize: libc::c_int,
        pub nbAllocVectors: libc::c_int,
        pub allocVectors: *const libc::c_uchar,
        pub logN: *const opus_int16,
        pub window: *const opus_val16,
        pub mdct: mdct_lookup,
        pub cache: PulseCache,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "42:9"]
    pub struct PulseCache {
        pub size: libc::c_int,
        pub index: *const opus_int16,
        pub bits: *const libc::c_uchar,
        pub caps: *const libc::c_uchar,
    }
    use super::arch_h::opus_val16;
    use super::mdct_h::mdct_lookup;
    use super::opus_types_h::{opus_int16, opus_int32};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/mdct.h:40"]
pub mod mdct_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:9"]
    pub struct mdct_lookup {
        pub n: libc::c_int,
        pub maxshift: libc::c_int,
        pub kfft: [*const kiss_fft_state; 4],
        pub trig: *const libc::c_float,
    }
    use super::arch_h::opus_val16;
    use super::kiss_fft_h::kiss_fft_state;
    extern "C" {
        #[c2rust::src_loc = "65:1"]
        pub fn clt_mdct_forward_c(
            l: *const mdct_lookup,
            in_0: *mut libc::c_float,
            out: *mut libc::c_float,
            window: *const opus_val16,
            overlap: libc::c_int,
            shift: libc::c_int,
            stride: libc::c_int,
            arch: libc::c_int,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/kiss_fft.h:40"]
pub mod kiss_fft_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "86:16"]
    pub struct kiss_fft_state {
        pub nfft: libc::c_int,
        pub scale: opus_val16,
        pub shift: libc::c_int,
        pub factors: [opus_int16; 16],
        pub bitrev: *const opus_int16,
        pub twiddles: *const kiss_twiddle_cpx,
        pub arch_fft: *mut arch_fft_state,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "81:16"]
    pub struct arch_fft_state {
        pub is_supported: libc::c_int,
        pub priv_0: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "70:9"]
    pub struct kiss_twiddle_cpx {
        pub r: libc::c_float,
        pub i: libc::c_float,
    }
    use super::arch_h::opus_val16;
    use super::opus_types_h::opus_int16;
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stdarg.h:34"]
pub mod stdarg_h {
    #[c2rust::src_loc = "14:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/usr/include/bits/mathcalls.h:34"]
pub mod mathcalls_h {
    extern "C" {
        #[c2rust::src_loc = "104:17"]
        pub fn log(_: libc::c_double) -> libc::c_double;
        #[c2rust::src_loc = "165:14"]
        pub fn floor(_: libc::c_double) -> libc::c_double;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/celt.h:34"]
pub mod celt_h {
    use super::arch_h::{celt_sig, opus_val16};
    use super::opus_types_h::opus_int32;
    extern "C" {
        #[c2rust::src_loc = "219:1"]
        pub fn resampling_factor(rate: opus_int32) -> libc::c_int;
        #[c2rust::src_loc = "221:1"]
        pub fn celt_preemphasis(
            pcmp: *const opus_val16,
            inp: *mut celt_sig,
            N: libc::c_int,
            CC: libc::c_int,
            upsample: libc::c_int,
            coef: *const opus_val16,
            mem: *mut celt_sig,
            clip: libc::c_int,
        );
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:37"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "568:13"]
        pub fn free(_: *mut libc::c_void);
        #[c2rust::src_loc = "553:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/string.h:38"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "43:14"]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "61:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/os_support.h:38"]
pub mod os_support_h {
    #[inline]
    #[c2rust::src_loc = "64:1"]
    pub unsafe extern "C" fn opus_free(mut ptr: *mut libc::c_void) {
        free(ptr);
    }
    #[inline]
    #[c2rust::src_loc = "47:1"]
    pub unsafe extern "C" fn opus_alloc(mut size: size_t) -> *mut libc::c_void {
        return malloc(size);
    }
    use super::stddef_h::size_t;
    use super::stdlib_h::{free, malloc};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/mathops.h:39"]
pub mod mathops_h {
    use super::opus_types_h::opus_uint32;
    extern "C" {
        #[c2rust::src_loc = "46:1"]
        pub fn isqrt32(_val: opus_uint32) -> libc::c_uint;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/cpu_support.h:40"]
pub mod cpu_support_h {
    #[inline]
    #[c2rust::src_loc = "65:1"]
    pub unsafe extern "C" fn opus_select_arch() -> libc::c_int {
        return 0 as libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/bands.h:42"]
pub mod bands_h {
    use super::arch_h::{celt_ener, celt_sig};
    use super::modes_h::OpusCustomMode;
    extern "C" {
        #[c2rust::src_loc = "47:1"]
        pub fn compute_band_energies(
            m: *const OpusCustomMode,
            X: *const celt_sig,
            bandE: *mut celt_ener,
            end: libc::c_int,
            C: libc::c_int,
            LM: libc::c_int,
            arch: libc::c_int,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/quant_bands.h:43"]
pub mod quant_bands_h {
    use super::arch_h::{celt_ener, opus_val16};
    use super::modes_h::OpusCustomMode;
    extern "C" {
        #[c2rust::src_loc = "44:1"]
        pub fn amp2Log2(
            m: *const OpusCustomMode,
            effEnd: libc::c_int,
            end: libc::c_int,
            bandE: *mut celt_ener,
            bandLogE: *mut opus_val16,
            C: libc::c_int,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/pitch.h:44"]
pub mod pitch_h {
    #[inline]
    #[c2rust::src_loc = "159:1"]
    pub unsafe extern "C" fn celt_inner_prod_c(
        mut x: *const opus_val16,
        mut y: *const opus_val16,
        mut N: libc::c_int,
    ) -> opus_val32 {
        let mut i: libc::c_int = 0;
        let mut xy: opus_val32 = 0 as libc::c_int as opus_val32;
        i = 0 as libc::c_int;
        while i < N {
            xy = xy + *x.offset(i as isize) * *y.offset(i as isize);
            i += 1;
        }
        return xy;
    }
    use super::arch_h::{opus_val16, opus_val32};
}
pub use self::arch_h::{celt_ener, celt_fatal, celt_sig, opus_val16, opus_val32};
use self::bands_h::compute_band_energies;
use self::celt_h::{celt_preemphasis, resampling_factor};
pub use self::cpu_support_h::opus_select_arch;
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::kiss_fft_h::{arch_fft_state, kiss_fft_state, kiss_twiddle_cpx};
use self::mathcalls_h::{floor, log};
use self::mathops_h::isqrt32;
pub use self::mdct_h::{clt_mdct_forward_c, mdct_lookup};
pub use self::modes_h::{OpusCustomMode, PulseCache};
use self::opus_h::{
    opus_encoder_ctl, opus_encoder_get_size, opus_encoder_init, opus_repacketizer_cat,
    opus_repacketizer_get_nb_frames, opus_repacketizer_init, OpusEncoder,
};
pub use self::opus_private_h::{
    align, downmix_float, downmix_func, downmix_int, foo, frame_size_select, get_left_channel,
    get_mono_channel, get_right_channel, opus_copy_channel_in_func, opus_encode_native,
    opus_repacketizer_out_range_impl, validate_layout, C2RustUnnamed, ChannelLayout, MappingType,
    OpusMSEncoder, OpusRepacketizer, MAPPING_TYPE_AMBISONICS, MAPPING_TYPE_NONE,
    MAPPING_TYPE_SURROUND,
};
pub use self::opus_types_h::{opus_int16, opus_int32, opus_int64, opus_uint32};
pub use self::os_support_h::{opus_alloc, opus_free};
pub use self::pitch_h::celt_inner_prod_c;
use self::quant_bands_h::amp2Log2;
pub use self::stdarg_h::va_list;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t};
pub use self::stdint_uintn_h::uint32_t;
use self::stdlib_h::{free, malloc};
use self::string_h::{memcpy, memset};
pub use self::types_h::{__int16_t, __int32_t, __int64_t, __uint32_t};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "46:9"]
pub struct VorbisLayout {
    pub nb_streams: libc::c_int,
    pub nb_coupled_streams: libc::c_int,
    pub mapping: [libc::c_uchar; 8],
}
#[c2rust::src_loc = "53:27"]
static mut vorbis_mappings: [VorbisLayout; 8] = [
    {
        let mut init = VorbisLayout {
            nb_streams: 1 as libc::c_int,
            nb_coupled_streams: 0 as libc::c_int,
            mapping: [0 as libc::c_int as libc::c_uchar, 0, 0, 0, 0, 0, 0, 0],
        };
        init
    },
    {
        let mut init = VorbisLayout {
            nb_streams: 1 as libc::c_int,
            nb_coupled_streams: 1 as libc::c_int,
            mapping: [
                0 as libc::c_int as libc::c_uchar,
                1 as libc::c_int as libc::c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        };
        init
    },
    {
        let mut init = VorbisLayout {
            nb_streams: 2 as libc::c_int,
            nb_coupled_streams: 1 as libc::c_int,
            mapping: [
                0 as libc::c_int as libc::c_uchar,
                2 as libc::c_int as libc::c_uchar,
                1 as libc::c_int as libc::c_uchar,
                0,
                0,
                0,
                0,
                0,
            ],
        };
        init
    },
    {
        let mut init = VorbisLayout {
            nb_streams: 2 as libc::c_int,
            nb_coupled_streams: 2 as libc::c_int,
            mapping: [
                0 as libc::c_int as libc::c_uchar,
                1 as libc::c_int as libc::c_uchar,
                2 as libc::c_int as libc::c_uchar,
                3 as libc::c_int as libc::c_uchar,
                0,
                0,
                0,
                0,
            ],
        };
        init
    },
    {
        let mut init = VorbisLayout {
            nb_streams: 3 as libc::c_int,
            nb_coupled_streams: 2 as libc::c_int,
            mapping: [
                0 as libc::c_int as libc::c_uchar,
                4 as libc::c_int as libc::c_uchar,
                1 as libc::c_int as libc::c_uchar,
                2 as libc::c_int as libc::c_uchar,
                3 as libc::c_int as libc::c_uchar,
                0,
                0,
                0,
            ],
        };
        init
    },
    {
        let mut init = VorbisLayout {
            nb_streams: 4 as libc::c_int,
            nb_coupled_streams: 2 as libc::c_int,
            mapping: [
                0 as libc::c_int as libc::c_uchar,
                4 as libc::c_int as libc::c_uchar,
                1 as libc::c_int as libc::c_uchar,
                2 as libc::c_int as libc::c_uchar,
                3 as libc::c_int as libc::c_uchar,
                5 as libc::c_int as libc::c_uchar,
                0,
                0,
            ],
        };
        init
    },
    {
        let mut init = VorbisLayout {
            nb_streams: 4 as libc::c_int,
            nb_coupled_streams: 3 as libc::c_int,
            mapping: [
                0 as libc::c_int as libc::c_uchar,
                4 as libc::c_int as libc::c_uchar,
                1 as libc::c_int as libc::c_uchar,
                2 as libc::c_int as libc::c_uchar,
                3 as libc::c_int as libc::c_uchar,
                5 as libc::c_int as libc::c_uchar,
                6 as libc::c_int as libc::c_uchar,
                0,
            ],
        };
        init
    },
    {
        let mut init = VorbisLayout {
            nb_streams: 5 as libc::c_int,
            nb_coupled_streams: 3 as libc::c_int,
            mapping: [
                0 as libc::c_int as libc::c_uchar,
                6 as libc::c_int as libc::c_uchar,
                1 as libc::c_int as libc::c_uchar,
                2 as libc::c_int as libc::c_uchar,
                3 as libc::c_int as libc::c_uchar,
                4 as libc::c_int as libc::c_uchar,
                5 as libc::c_int as libc::c_uchar,
                7 as libc::c_int as libc::c_uchar,
            ],
        };
        init
    },
];
#[c2rust::src_loc = "64:1"]
unsafe extern "C" fn ms_get_preemph_mem(mut st: *mut OpusMSEncoder) -> *mut opus_val32 {
    let mut s: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut coupled_size: libc::c_int = 0;
    let mut mono_size: libc::c_int = 0;
    coupled_size = opus_encoder_get_size(2 as libc::c_int);
    mono_size = opus_encoder_get_size(1 as libc::c_int);
    ptr = (st as *mut libc::c_char).offset(align(
        ::core::mem::size_of::<OpusMSEncoder>() as libc::c_ulong as libc::c_int
    ) as isize);
    s = 0 as libc::c_int;
    while s < (*st).layout.nb_streams {
        if s < (*st).layout.nb_coupled_streams {
            ptr = ptr.offset(align(coupled_size) as isize);
        } else {
            ptr = ptr.offset(align(mono_size) as isize);
        }
        s += 1;
    }
    return ptr.offset(
        (((*st).layout.nb_channels * 120 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<opus_val32>() as libc::c_ulong) as isize,
    ) as *mut libc::c_void as *mut opus_val32;
}
#[c2rust::src_loc = "84:1"]
unsafe extern "C" fn ms_get_window_mem(mut st: *mut OpusMSEncoder) -> *mut opus_val32 {
    let mut s: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut coupled_size: libc::c_int = 0;
    let mut mono_size: libc::c_int = 0;
    coupled_size = opus_encoder_get_size(2 as libc::c_int);
    mono_size = opus_encoder_get_size(1 as libc::c_int);
    ptr = (st as *mut libc::c_char).offset(align(
        ::core::mem::size_of::<OpusMSEncoder>() as libc::c_ulong as libc::c_int
    ) as isize);
    s = 0 as libc::c_int;
    while s < (*st).layout.nb_streams {
        if s < (*st).layout.nb_coupled_streams {
            ptr = ptr.offset(align(coupled_size) as isize);
        } else {
            ptr = ptr.offset(align(mono_size) as isize);
        }
        s += 1;
    }
    return ptr as *mut libc::c_void as *mut opus_val32;
}
#[c2rust::src_loc = "104:1"]
unsafe extern "C" fn validate_ambisonics(
    mut nb_channels: libc::c_int,
    mut nb_streams: *mut libc::c_int,
    mut nb_coupled_streams: *mut libc::c_int,
) -> libc::c_int {
    let mut order_plus_one: libc::c_int = 0;
    let mut acn_channels: libc::c_int = 0;
    let mut nondiegetic_channels: libc::c_int = 0;
    if nb_channels < 1 as libc::c_int || nb_channels > 227 as libc::c_int {
        return 0 as libc::c_int;
    }
    order_plus_one = isqrt32(nb_channels as opus_uint32) as libc::c_int;
    acn_channels = order_plus_one * order_plus_one;
    nondiegetic_channels = nb_channels - acn_channels;
    if nondiegetic_channels != 0 as libc::c_int && nondiegetic_channels != 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    if !nb_streams.is_null() {
        *nb_streams = acn_channels + (nondiegetic_channels != 0 as libc::c_int) as libc::c_int;
    }
    if !nb_coupled_streams.is_null() {
        *nb_coupled_streams = (nondiegetic_channels != 0 as libc::c_int) as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "127:1"]
unsafe extern "C" fn validate_encoder_layout(mut layout: *const ChannelLayout) -> libc::c_int {
    let mut s: libc::c_int = 0;
    s = 0 as libc::c_int;
    while s < (*layout).nb_streams {
        if s < (*layout).nb_coupled_streams {
            if get_left_channel(layout, s, -(1 as libc::c_int)) == -(1 as libc::c_int) {
                return 0 as libc::c_int;
            }
            if get_right_channel(layout, s, -(1 as libc::c_int)) == -(1 as libc::c_int) {
                return 0 as libc::c_int;
            }
        } else if get_mono_channel(layout, s, -(1 as libc::c_int)) == -(1 as libc::c_int) {
            return 0 as libc::c_int;
        }
        s += 1;
    }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "146:1"]
unsafe extern "C" fn channel_pos(mut channels: libc::c_int, mut pos: *mut libc::c_int) {
    if channels == 4 as libc::c_int {
        *pos.offset(0 as libc::c_int as isize) = 1 as libc::c_int;
        *pos.offset(1 as libc::c_int as isize) = 3 as libc::c_int;
        *pos.offset(2 as libc::c_int as isize) = 1 as libc::c_int;
        *pos.offset(3 as libc::c_int as isize) = 3 as libc::c_int;
    } else if channels == 3 as libc::c_int
        || channels == 5 as libc::c_int
        || channels == 6 as libc::c_int
    {
        *pos.offset(0 as libc::c_int as isize) = 1 as libc::c_int;
        *pos.offset(1 as libc::c_int as isize) = 2 as libc::c_int;
        *pos.offset(2 as libc::c_int as isize) = 3 as libc::c_int;
        *pos.offset(3 as libc::c_int as isize) = 1 as libc::c_int;
        *pos.offset(4 as libc::c_int as isize) = 3 as libc::c_int;
        *pos.offset(5 as libc::c_int as isize) = 0 as libc::c_int;
    } else if channels == 7 as libc::c_int {
        *pos.offset(0 as libc::c_int as isize) = 1 as libc::c_int;
        *pos.offset(1 as libc::c_int as isize) = 2 as libc::c_int;
        *pos.offset(2 as libc::c_int as isize) = 3 as libc::c_int;
        *pos.offset(3 as libc::c_int as isize) = 1 as libc::c_int;
        *pos.offset(4 as libc::c_int as isize) = 3 as libc::c_int;
        *pos.offset(5 as libc::c_int as isize) = 2 as libc::c_int;
        *pos.offset(6 as libc::c_int as isize) = 0 as libc::c_int;
    } else if channels == 8 as libc::c_int {
        *pos.offset(0 as libc::c_int as isize) = 1 as libc::c_int;
        *pos.offset(1 as libc::c_int as isize) = 2 as libc::c_int;
        *pos.offset(2 as libc::c_int as isize) = 3 as libc::c_int;
        *pos.offset(3 as libc::c_int as isize) = 1 as libc::c_int;
        *pos.offset(4 as libc::c_int as isize) = 3 as libc::c_int;
        *pos.offset(5 as libc::c_int as isize) = 1 as libc::c_int;
        *pos.offset(6 as libc::c_int as isize) = 3 as libc::c_int;
        *pos.offset(7 as libc::c_int as isize) = 0 as libc::c_int;
    }
}
#[c2rust::src_loc = "187:1"]
unsafe extern "C" fn logSum(mut a: opus_val16, mut b: opus_val16) -> opus_val16 {
    let mut max: opus_val16 = 0.;
    let mut diff: opus_val32 = 0.;
    let mut frac: opus_val16 = 0.;
    static mut diff_table: [opus_val16; 17] = [
        0.5000000f32,
        0.2924813f32,
        0.1609640f32,
        0.0849625f32,
        0.0437314f32,
        0.0221971f32,
        0.0111839f32,
        0.0056136f32,
        0.0028123f32,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ];
    let mut low: libc::c_int = 0;
    if a > b {
        max = a;
        diff = a - b;
    } else {
        max = b;
        diff = b - a;
    }
    if !(diff < 8.0f32) {
        return max;
    }
    low = floor((2 as libc::c_int as libc::c_float * diff) as libc::c_double) as libc::c_int;
    frac = 2 as libc::c_int as libc::c_float * diff - low as libc::c_float;
    return max
        + diff_table[low as usize]
        + frac * (diff_table[(low + 1 as libc::c_int) as usize] - diff_table[low as usize]);
}
#[no_mangle]
#[c2rust::src_loc = "224:1"]
pub unsafe extern "C" fn surround_analysis(
    mut celt_mode: *const OpusCustomMode,
    mut pcm: *const libc::c_void,
    mut bandLogE: *mut opus_val16,
    mut mem: *mut opus_val32,
    mut preemph_mem: *mut opus_val32,
    mut len: libc::c_int,
    mut overlap: libc::c_int,
    mut channels: libc::c_int,
    mut rate: libc::c_int,
    mut copy_channel_in: opus_copy_channel_in_func,
    mut arch: libc::c_int,
) {
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut LM: libc::c_int = 0;
    let mut pos: [libc::c_int; 8] = [0 as libc::c_int, 0, 0, 0, 0, 0, 0, 0];
    let mut upsample: libc::c_int = 0;
    let mut frame_size: libc::c_int = 0;
    let mut freq_size: libc::c_int = 0;
    let mut channel_offset: opus_val16 = 0.;
    let mut bandE: [opus_val32; 21] = [0.; 21];
    let mut maskLogE: [[opus_val16; 21]; 3] = [[0.; 21]; 3];
    upsample = resampling_factor(rate);
    frame_size = len * upsample;
    freq_size = if (960 as libc::c_int) < frame_size {
        960 as libc::c_int
    } else {
        frame_size
    };
    LM = 0 as libc::c_int;
    while LM < (*celt_mode).maxLM {
        if (*celt_mode).shortMdctSize << LM == frame_size {
            break;
        }
        LM += 1;
    }
    let vla = (frame_size + overlap) as usize;
    let mut in_0: Vec<opus_val32> = ::std::vec::from_elem(0., vla);
    let vla_0 = len as usize;
    let mut x: Vec<opus_val16> = ::std::vec::from_elem(0., vla_0);
    let vla_1 = freq_size as usize;
    let mut freq: Vec<opus_val32> = ::std::vec::from_elem(0., vla_1);
    channel_pos(channels, pos.as_mut_ptr());
    c = 0 as libc::c_int;
    while c < 3 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 21 as libc::c_int {
            maskLogE[c as usize][i as usize] = -28.0f32;
            i += 1;
        }
        c += 1;
    }
    c = 0 as libc::c_int;
    while c < channels {
        let mut frame: libc::c_int = 0;
        let mut nb_frames: libc::c_int = frame_size / freq_size;
        if !(nb_frames * freq_size == frame_size) {
            celt_fatal(
                b"assertion failed: nb_frames*freq_size == frame_size\0" as *const u8
                    as *const libc::c_char,
                b"src/opus_multistream_encoder.c\0" as *const u8 as *const libc::c_char,
                266 as libc::c_int,
            );
        }
        memcpy(
            in_0.as_mut_ptr() as *mut libc::c_void,
            mem.offset((c * overlap) as isize) as *const libc::c_void,
            (overlap as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<opus_val32>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * in_0
                            .as_mut_ptr()
                            .offset_from(mem.offset((c * overlap) as isize))
                            as libc::c_long) as libc::c_ulong,
                ),
        );
        (Some(copy_channel_in.expect("non-null function pointer")))
            .expect("non-null function pointer")(
            x.as_mut_ptr(),
            1 as libc::c_int,
            pcm,
            channels,
            c,
            len,
            0 as *mut libc::c_void,
        );
        celt_preemphasis(
            x.as_mut_ptr(),
            in_0.as_mut_ptr().offset(overlap as isize),
            frame_size,
            1 as libc::c_int,
            upsample,
            ((*celt_mode).preemph).as_ptr(),
            preemph_mem.offset(c as isize),
            0 as libc::c_int,
        );
        let mut sum: opus_val32 = 0.;
        sum = celt_inner_prod_c(in_0.as_mut_ptr(), in_0.as_mut_ptr(), frame_size + overlap);
        if !(sum < 1e18f32) || sum != sum {
            memset(
                in_0.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ((frame_size + overlap) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<opus_val32>() as libc::c_ulong),
            );
            *preemph_mem.offset(c as isize) = 0 as libc::c_int as opus_val32;
        }
        memset(
            bandE.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            (21 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<opus_val32>() as libc::c_ulong),
        );
        frame = 0 as libc::c_int;
        while frame < nb_frames {
            let mut tmpE: [opus_val32; 21] = [0.; 21];
            clt_mdct_forward_c(
                &(*celt_mode).mdct,
                in_0.as_mut_ptr()
                    .offset((960 as libc::c_int * frame) as isize),
                freq.as_mut_ptr(),
                (*celt_mode).window,
                overlap,
                (*celt_mode).maxLM - LM,
                1 as libc::c_int,
                arch,
            );
            if upsample != 1 as libc::c_int {
                let mut bound: libc::c_int = freq_size / upsample;
                i = 0 as libc::c_int;
                while i < bound {
                    let ref mut fresh0 = *freq.as_mut_ptr().offset(i as isize);
                    *fresh0 *= upsample as libc::c_float;
                    i += 1;
                }
                while i < freq_size {
                    *freq.as_mut_ptr().offset(i as isize) = 0 as libc::c_int as opus_val32;
                    i += 1;
                }
            }
            compute_band_energies(
                celt_mode,
                freq.as_mut_ptr(),
                tmpE.as_mut_ptr(),
                21 as libc::c_int,
                1 as libc::c_int,
                LM,
                arch,
            );
            i = 0 as libc::c_int;
            while i < 21 as libc::c_int {
                bandE[i as usize] = if bandE[i as usize] > tmpE[i as usize] {
                    bandE[i as usize]
                } else {
                    tmpE[i as usize]
                };
                i += 1;
            }
            frame += 1;
        }
        amp2Log2(
            celt_mode,
            21 as libc::c_int,
            21 as libc::c_int,
            bandE.as_mut_ptr(),
            bandLogE.offset((21 as libc::c_int * c) as isize),
            1 as libc::c_int,
        );
        i = 1 as libc::c_int;
        while i < 21 as libc::c_int {
            *bandLogE.offset((21 as libc::c_int * c + i) as isize) = if *bandLogE
                .offset((21 as libc::c_int * c + i) as isize)
                > *bandLogE.offset((21 as libc::c_int * c + i - 1 as libc::c_int) as isize) - 1.0f32
            {
                *bandLogE.offset((21 as libc::c_int * c + i) as isize)
            } else {
                *bandLogE.offset((21 as libc::c_int * c + i - 1 as libc::c_int) as isize) - 1.0f32
            };
            i += 1;
        }
        i = 19 as libc::c_int;
        while i >= 0 as libc::c_int {
            *bandLogE.offset((21 as libc::c_int * c + i) as isize) = if *bandLogE
                .offset((21 as libc::c_int * c + i) as isize)
                > *bandLogE.offset((21 as libc::c_int * c + i + 1 as libc::c_int) as isize) - 2.0f32
            {
                *bandLogE.offset((21 as libc::c_int * c + i) as isize)
            } else {
                *bandLogE.offset((21 as libc::c_int * c + i + 1 as libc::c_int) as isize) - 2.0f32
            };
            i -= 1;
        }
        if pos[c as usize] == 1 as libc::c_int {
            i = 0 as libc::c_int;
            while i < 21 as libc::c_int {
                maskLogE[0 as libc::c_int as usize][i as usize] = logSum(
                    maskLogE[0 as libc::c_int as usize][i as usize],
                    *bandLogE.offset((21 as libc::c_int * c + i) as isize),
                );
                i += 1;
            }
        } else if pos[c as usize] == 3 as libc::c_int {
            i = 0 as libc::c_int;
            while i < 21 as libc::c_int {
                maskLogE[2 as libc::c_int as usize][i as usize] = logSum(
                    maskLogE[2 as libc::c_int as usize][i as usize],
                    *bandLogE.offset((21 as libc::c_int * c + i) as isize),
                );
                i += 1;
            }
        } else if pos[c as usize] == 2 as libc::c_int {
            i = 0 as libc::c_int;
            while i < 21 as libc::c_int {
                maskLogE[0 as libc::c_int as usize][i as usize] = logSum(
                    maskLogE[0 as libc::c_int as usize][i as usize],
                    *bandLogE.offset((21 as libc::c_int * c + i) as isize) - 0.5f32,
                );
                maskLogE[2 as libc::c_int as usize][i as usize] = logSum(
                    maskLogE[2 as libc::c_int as usize][i as usize],
                    *bandLogE.offset((21 as libc::c_int * c + i) as isize) - 0.5f32,
                );
                i += 1;
            }
        }
        memcpy(
            mem.offset((c * overlap) as isize) as *mut libc::c_void,
            in_0.as_mut_ptr().offset(frame_size as isize) as *const libc::c_void,
            (overlap as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<opus_val32>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * mem
                            .offset((c * overlap) as isize)
                            .offset_from(in_0.as_mut_ptr().offset(frame_size as isize))
                            as libc::c_long) as libc::c_ulong,
                ),
        );
        c += 1;
    }
    i = 0 as libc::c_int;
    while i < 21 as libc::c_int {
        maskLogE[1 as libc::c_int as usize][i as usize] = if maskLogE[0 as libc::c_int as usize]
            [i as usize]
            < maskLogE[2 as libc::c_int as usize][i as usize]
        {
            maskLogE[0 as libc::c_int as usize][i as usize]
        } else {
            maskLogE[2 as libc::c_int as usize][i as usize]
        };
        i += 1;
    }
    channel_offset = 0.5f32
        * (1.442695040888963387f64
            * log((2.0f32 / (channels - 1 as libc::c_int) as libc::c_float) as libc::c_double))
            as libc::c_float;
    c = 0 as libc::c_int;
    while c < 3 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 21 as libc::c_int {
            maskLogE[c as usize][i as usize] += channel_offset;
            i += 1;
        }
        c += 1;
    }
    c = 0 as libc::c_int;
    while c < channels {
        let mut mask: *mut opus_val16 = 0 as *mut opus_val16;
        if pos[c as usize] != 0 as libc::c_int {
            mask = &mut *(*maskLogE
                .as_mut_ptr()
                .offset((*pos.as_mut_ptr().offset(c as isize) - 1 as libc::c_int) as isize))
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut opus_val16;
            i = 0 as libc::c_int;
            while i < 21 as libc::c_int {
                *bandLogE.offset((21 as libc::c_int * c + i) as isize) = *bandLogE
                    .offset((21 as libc::c_int * c + i) as isize)
                    - *mask.offset(i as isize);
                i += 1;
            }
        } else {
            i = 0 as libc::c_int;
            while i < 21 as libc::c_int {
                *bandLogE.offset((21 as libc::c_int * c + i) as isize) =
                    0 as libc::c_int as opus_val16;
                i += 1;
            }
        }
        c += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "376:1"]
pub unsafe extern "C" fn opus_multistream_encoder_get_size(
    mut nb_streams: libc::c_int,
    mut nb_coupled_streams: libc::c_int,
) -> opus_int32 {
    let mut coupled_size: libc::c_int = 0;
    let mut mono_size: libc::c_int = 0;
    if nb_streams < 1 as libc::c_int
        || nb_coupled_streams > nb_streams
        || nb_coupled_streams < 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    coupled_size = opus_encoder_get_size(2 as libc::c_int);
    mono_size = opus_encoder_get_size(1 as libc::c_int);
    return align(::core::mem::size_of::<OpusMSEncoder>() as libc::c_ulong as libc::c_int)
        + nb_coupled_streams * align(coupled_size)
        + (nb_streams - nb_coupled_streams) * align(mono_size);
}
#[no_mangle]
#[c2rust::src_loc = "389:1"]
pub unsafe extern "C" fn opus_multistream_surround_encoder_get_size(
    mut channels: libc::c_int,
    mut mapping_family: libc::c_int,
) -> opus_int32 {
    let mut nb_streams: libc::c_int = 0;
    let mut nb_coupled_streams: libc::c_int = 0;
    let mut size: opus_int32 = 0;
    if mapping_family == 0 as libc::c_int {
        if channels == 1 as libc::c_int {
            nb_streams = 1 as libc::c_int;
            nb_coupled_streams = 0 as libc::c_int;
        } else if channels == 2 as libc::c_int {
            nb_streams = 1 as libc::c_int;
            nb_coupled_streams = 1 as libc::c_int;
        } else {
            return 0 as libc::c_int;
        }
    } else if mapping_family == 1 as libc::c_int
        && channels <= 8 as libc::c_int
        && channels >= 1 as libc::c_int
    {
        nb_streams = vorbis_mappings[(channels - 1 as libc::c_int) as usize].nb_streams;
        nb_coupled_streams =
            vorbis_mappings[(channels - 1 as libc::c_int) as usize].nb_coupled_streams;
    } else if mapping_family == 255 as libc::c_int {
        nb_streams = channels;
        nb_coupled_streams = 0 as libc::c_int;
    } else if mapping_family == 2 as libc::c_int {
        if validate_ambisonics(channels, &mut nb_streams, &mut nb_coupled_streams) == 0 {
            return 0 as libc::c_int;
        }
    } else {
        return 0 as libc::c_int;
    }
    size = opus_multistream_encoder_get_size(nb_streams, nb_coupled_streams);
    if channels > 2 as libc::c_int {
        size = (size as libc::c_ulong).wrapping_add(
            (channels as libc::c_ulong).wrapping_mul(
                (120 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<opus_val32>() as libc::c_ulong)
                    .wrapping_add(::core::mem::size_of::<opus_val32>() as libc::c_ulong),
            ),
        ) as opus_int32 as opus_int32;
    }
    return size;
}
#[c2rust::src_loc = "429:1"]
unsafe extern "C" fn opus_multistream_encoder_init_impl(
    mut st: *mut OpusMSEncoder,
    mut Fs: opus_int32,
    mut channels: libc::c_int,
    mut streams: libc::c_int,
    mut coupled_streams: libc::c_int,
    mut mapping: *const libc::c_uchar,
    mut application: libc::c_int,
    mut mapping_type: MappingType,
) -> libc::c_int {
    let mut coupled_size: libc::c_int = 0;
    let mut mono_size: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if channels > 255 as libc::c_int
        || channels < 1 as libc::c_int
        || coupled_streams > streams
        || streams < 1 as libc::c_int
        || coupled_streams < 0 as libc::c_int
        || streams > 255 as libc::c_int - coupled_streams
    {
        return -(1 as libc::c_int);
    }
    (*st).arch = opus_select_arch();
    (*st).layout.nb_channels = channels;
    (*st).layout.nb_streams = streams;
    (*st).layout.nb_coupled_streams = coupled_streams;
    if mapping_type as libc::c_uint != MAPPING_TYPE_SURROUND as libc::c_int as libc::c_uint {
        (*st).lfe_stream = -(1 as libc::c_int);
    }
    (*st).bitrate_bps = -(1000 as libc::c_int);
    (*st).application = application;
    (*st).variable_duration = 5000 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*st).layout.nb_channels {
        (*st).layout.mapping[i as usize] = *mapping.offset(i as isize);
        i += 1;
    }
    if validate_layout(&mut (*st).layout) == 0 {
        return -(1 as libc::c_int);
    }
    if mapping_type as libc::c_uint == MAPPING_TYPE_SURROUND as libc::c_int as libc::c_uint
        && validate_encoder_layout(&mut (*st).layout) == 0
    {
        return -(1 as libc::c_int);
    }
    if mapping_type as libc::c_uint == MAPPING_TYPE_AMBISONICS as libc::c_int as libc::c_uint
        && validate_ambisonics(
            (*st).layout.nb_channels,
            0 as *mut libc::c_int,
            0 as *mut libc::c_int,
        ) == 0
    {
        return -(1 as libc::c_int);
    }
    ptr = (st as *mut libc::c_char).offset(align(
        ::core::mem::size_of::<OpusMSEncoder>() as libc::c_ulong as libc::c_int
    ) as isize);
    coupled_size = opus_encoder_get_size(2 as libc::c_int);
    mono_size = opus_encoder_get_size(1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < (*st).layout.nb_coupled_streams {
        ret = opus_encoder_init(ptr as *mut OpusEncoder, Fs, 2 as libc::c_int, application);
        if ret != 0 as libc::c_int {
            return ret;
        }
        if i == (*st).lfe_stream {
            opus_encoder_ctl(
                ptr as *mut OpusEncoder,
                10024 as libc::c_int,
                1 as libc::c_int,
            );
        }
        ptr = ptr.offset(align(coupled_size) as isize);
        i += 1;
    }
    while i < (*st).layout.nb_streams {
        ret = opus_encoder_init(ptr as *mut OpusEncoder, Fs, 1 as libc::c_int, application);
        if i == (*st).lfe_stream {
            opus_encoder_ctl(
                ptr as *mut OpusEncoder,
                10024 as libc::c_int,
                1 as libc::c_int,
            );
        }
        if ret != 0 as libc::c_int {
            return ret;
        }
        ptr = ptr.offset(align(mono_size) as isize);
        i += 1;
    }
    if mapping_type as libc::c_uint == MAPPING_TYPE_SURROUND as libc::c_int as libc::c_uint {
        memset(
            ms_get_preemph_mem(st) as *mut libc::c_void,
            0 as libc::c_int,
            (channels as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<opus_val32>() as libc::c_ulong),
        );
        memset(
            ms_get_window_mem(st) as *mut libc::c_void,
            0 as libc::c_int,
            ((channels * 120 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<opus_val32>() as libc::c_ulong),
        );
    }
    (*st).mapping_type = mapping_type;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "497:1"]
pub unsafe extern "C" fn opus_multistream_encoder_init(
    mut st: *mut OpusMSEncoder,
    mut Fs: opus_int32,
    mut channels: libc::c_int,
    mut streams: libc::c_int,
    mut coupled_streams: libc::c_int,
    mut mapping: *const libc::c_uchar,
    mut application: libc::c_int,
) -> libc::c_int {
    return opus_multistream_encoder_init_impl(
        st,
        Fs,
        channels,
        streams,
        coupled_streams,
        mapping,
        application,
        MAPPING_TYPE_NONE,
    );
}
#[no_mangle]
#[c2rust::src_loc = "512:1"]
pub unsafe extern "C" fn opus_multistream_surround_encoder_init(
    mut st: *mut OpusMSEncoder,
    mut Fs: opus_int32,
    mut channels: libc::c_int,
    mut mapping_family: libc::c_int,
    mut streams: *mut libc::c_int,
    mut coupled_streams: *mut libc::c_int,
    mut mapping: *mut libc::c_uchar,
    mut application: libc::c_int,
) -> libc::c_int {
    let mut mapping_type: MappingType = MAPPING_TYPE_NONE;
    if channels > 255 as libc::c_int || channels < 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*st).lfe_stream = -(1 as libc::c_int);
    if mapping_family == 0 as libc::c_int {
        if channels == 1 as libc::c_int {
            *streams = 1 as libc::c_int;
            *coupled_streams = 0 as libc::c_int;
            *mapping.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
        } else if channels == 2 as libc::c_int {
            *streams = 1 as libc::c_int;
            *coupled_streams = 1 as libc::c_int;
            *mapping.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
            *mapping.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_uchar;
        } else {
            return -(5 as libc::c_int);
        }
    } else if mapping_family == 1 as libc::c_int
        && channels <= 8 as libc::c_int
        && channels >= 1 as libc::c_int
    {
        let mut i: libc::c_int = 0;
        *streams = vorbis_mappings[(channels - 1 as libc::c_int) as usize].nb_streams;
        *coupled_streams =
            vorbis_mappings[(channels - 1 as libc::c_int) as usize].nb_coupled_streams;
        i = 0 as libc::c_int;
        while i < channels {
            *mapping.offset(i as isize) =
                vorbis_mappings[(channels - 1 as libc::c_int) as usize].mapping[i as usize];
            i += 1;
        }
        if channels >= 6 as libc::c_int {
            (*st).lfe_stream = *streams - 1 as libc::c_int;
        }
    } else if mapping_family == 255 as libc::c_int {
        let mut i_0: libc::c_int = 0;
        *streams = channels;
        *coupled_streams = 0 as libc::c_int;
        i_0 = 0 as libc::c_int;
        while i_0 < channels {
            *mapping.offset(i_0 as isize) = i_0 as libc::c_uchar;
            i_0 += 1;
        }
    } else if mapping_family == 2 as libc::c_int {
        let mut i_1: libc::c_int = 0;
        if validate_ambisonics(channels, streams, coupled_streams) == 0 {
            return -(1 as libc::c_int);
        }
        i_1 = 0 as libc::c_int;
        while i_1 < *streams - *coupled_streams {
            *mapping.offset(i_1 as isize) =
                (i_1 + *coupled_streams * 2 as libc::c_int) as libc::c_uchar;
            i_1 += 1;
        }
        i_1 = 0 as libc::c_int;
        while i_1 < *coupled_streams * 2 as libc::c_int {
            *mapping.offset((i_1 + (*streams - *coupled_streams)) as isize) = i_1 as libc::c_uchar;
            i_1 += 1;
        }
    } else {
        return -(5 as libc::c_int);
    }
    if channels > 2 as libc::c_int && mapping_family == 1 as libc::c_int {
        mapping_type = MAPPING_TYPE_SURROUND;
    } else if mapping_family == 2 as libc::c_int {
        mapping_type = MAPPING_TYPE_AMBISONICS;
    } else {
        mapping_type = MAPPING_TYPE_NONE;
    }
    return opus_multistream_encoder_init_impl(
        st,
        Fs,
        channels,
        *streams,
        *coupled_streams,
        mapping,
        application,
        mapping_type,
    );
}
#[no_mangle]
#[c2rust::src_loc = "585:1"]
pub unsafe extern "C" fn opus_multistream_encoder_create(
    mut Fs: opus_int32,
    mut channels: libc::c_int,
    mut streams: libc::c_int,
    mut coupled_streams: libc::c_int,
    mut mapping: *const libc::c_uchar,
    mut application: libc::c_int,
    mut error: *mut libc::c_int,
) -> *mut OpusMSEncoder {
    let mut ret: libc::c_int = 0;
    let mut st: *mut OpusMSEncoder = 0 as *mut OpusMSEncoder;
    if channels > 255 as libc::c_int
        || channels < 1 as libc::c_int
        || coupled_streams > streams
        || streams < 1 as libc::c_int
        || coupled_streams < 0 as libc::c_int
        || streams > 255 as libc::c_int - coupled_streams
    {
        if !error.is_null() {
            *error = -(1 as libc::c_int);
        }
        return 0 as *mut OpusMSEncoder;
    }
    st = opus_alloc(opus_multistream_encoder_get_size(streams, coupled_streams) as size_t)
        as *mut OpusMSEncoder;
    if st.is_null() {
        if !error.is_null() {
            *error = -(7 as libc::c_int);
        }
        return 0 as *mut OpusMSEncoder;
    }
    ret = opus_multistream_encoder_init(
        st,
        Fs,
        channels,
        streams,
        coupled_streams,
        mapping,
        application,
    );
    if ret != 0 as libc::c_int {
        opus_free(st as *mut libc::c_void);
        st = 0 as *mut OpusMSEncoder;
    }
    if !error.is_null() {
        *error = ret;
    }
    return st;
}
#[no_mangle]
#[c2rust::src_loc = "622:1"]
pub unsafe extern "C" fn opus_multistream_surround_encoder_create(
    mut Fs: opus_int32,
    mut channels: libc::c_int,
    mut mapping_family: libc::c_int,
    mut streams: *mut libc::c_int,
    mut coupled_streams: *mut libc::c_int,
    mut mapping: *mut libc::c_uchar,
    mut application: libc::c_int,
    mut error: *mut libc::c_int,
) -> *mut OpusMSEncoder {
    let mut ret: libc::c_int = 0;
    let mut size: opus_int32 = 0;
    let mut st: *mut OpusMSEncoder = 0 as *mut OpusMSEncoder;
    if channels > 255 as libc::c_int || channels < 1 as libc::c_int {
        if !error.is_null() {
            *error = -(1 as libc::c_int);
        }
        return 0 as *mut OpusMSEncoder;
    }
    size = opus_multistream_surround_encoder_get_size(channels, mapping_family);
    if size == 0 {
        if !error.is_null() {
            *error = -(5 as libc::c_int);
        }
        return 0 as *mut OpusMSEncoder;
    }
    st = opus_alloc(size as size_t) as *mut OpusMSEncoder;
    if st.is_null() {
        if !error.is_null() {
            *error = -(7 as libc::c_int);
        }
        return 0 as *mut OpusMSEncoder;
    }
    ret = opus_multistream_surround_encoder_init(
        st,
        Fs,
        channels,
        mapping_family,
        streams,
        coupled_streams,
        mapping,
        application,
    );
    if ret != 0 as libc::c_int {
        opus_free(st as *mut libc::c_void);
        st = 0 as *mut OpusMSEncoder;
    }
    if !error.is_null() {
        *error = ret;
    }
    return st;
}
#[c2rust::src_loc = "667:1"]
unsafe extern "C" fn surround_rate_allocation(
    mut st: *mut OpusMSEncoder,
    mut rate: *mut opus_int32,
    mut frame_size: libc::c_int,
    mut Fs: opus_int32,
) {
    let mut i: libc::c_int = 0;
    let mut channel_rate: opus_int32 = 0;
    let mut stream_offset: libc::c_int = 0;
    let mut lfe_offset: libc::c_int = 0;
    let mut coupled_ratio: libc::c_int = 0;
    let mut lfe_ratio: libc::c_int = 0;
    let mut nb_lfe: libc::c_int = 0;
    let mut nb_uncoupled: libc::c_int = 0;
    let mut nb_coupled: libc::c_int = 0;
    let mut nb_normal: libc::c_int = 0;
    let mut channel_offset: opus_int32 = 0;
    let mut bitrate: opus_int32 = 0;
    let mut total: libc::c_int = 0;
    nb_lfe = ((*st).lfe_stream != -(1 as libc::c_int)) as libc::c_int;
    nb_coupled = (*st).layout.nb_coupled_streams;
    nb_uncoupled = (*st).layout.nb_streams - nb_coupled - nb_lfe;
    nb_normal = 2 as libc::c_int * nb_coupled + nb_uncoupled;
    channel_offset = 40 as libc::c_int
        * (if 50 as libc::c_int > Fs / frame_size {
            50 as libc::c_int
        } else {
            Fs / frame_size
        });
    if (*st).bitrate_bps == -(1000 as libc::c_int) {
        bitrate =
            nb_normal * (channel_offset + Fs + 10000 as libc::c_int) + 8000 as libc::c_int * nb_lfe;
    } else if (*st).bitrate_bps == -(1 as libc::c_int) {
        bitrate = nb_normal * 300000 as libc::c_int + nb_lfe * 128000 as libc::c_int;
    } else {
        bitrate = (*st).bitrate_bps;
    }
    lfe_offset = (if (bitrate / 20 as libc::c_int) < 3000 as libc::c_int {
        bitrate / 20 as libc::c_int
    } else {
        3000 as libc::c_int
    }) + 15 as libc::c_int
        * (if 50 as libc::c_int > Fs / frame_size {
            50 as libc::c_int
        } else {
            Fs / frame_size
        });
    stream_offset =
        (bitrate - channel_offset * nb_normal - lfe_offset * nb_lfe) / nb_normal / 2 as libc::c_int;
    stream_offset = if 0 as libc::c_int
        > (if (20000 as libc::c_int) < stream_offset {
            20000 as libc::c_int
        } else {
            stream_offset
        }) {
        0 as libc::c_int
    } else if (20000 as libc::c_int) < stream_offset {
        20000 as libc::c_int
    } else {
        stream_offset
    };
    coupled_ratio = 512 as libc::c_int;
    lfe_ratio = 32 as libc::c_int;
    total = (nb_uncoupled << 8 as libc::c_int) + coupled_ratio * nb_coupled + nb_lfe * lfe_ratio;
    channel_rate = (256 as libc::c_int as libc::c_long
        * (bitrate
            - lfe_offset * nb_lfe
            - stream_offset * (nb_coupled + nb_uncoupled)
            - channel_offset * nb_normal) as opus_int64
        / total as libc::c_long) as opus_int32;
    i = 0 as libc::c_int;
    while i < (*st).layout.nb_streams {
        if i < (*st).layout.nb_coupled_streams {
            *rate.offset(i as isize) = 2 as libc::c_int * channel_offset
                + (if 0 as libc::c_int
                    > stream_offset + (channel_rate * coupled_ratio >> 8 as libc::c_int)
                {
                    0 as libc::c_int
                } else {
                    stream_offset + (channel_rate * coupled_ratio >> 8 as libc::c_int)
                });
        } else if i != (*st).lfe_stream {
            *rate.offset(i as isize) = channel_offset
                + (if 0 as libc::c_int > stream_offset + channel_rate {
                    0 as libc::c_int
                } else {
                    stream_offset + channel_rate
                });
        } else {
            *rate.offset(i as isize) =
                if 0 as libc::c_int > lfe_offset + (channel_rate * lfe_ratio >> 8 as libc::c_int) {
                    0 as libc::c_int
                } else {
                    lfe_offset + (channel_rate * lfe_ratio >> 8 as libc::c_int)
                };
        }
        i += 1;
    }
}
#[c2rust::src_loc = "736:1"]
unsafe extern "C" fn ambisonics_rate_allocation(
    mut st: *mut OpusMSEncoder,
    mut rate: *mut opus_int32,
    mut frame_size: libc::c_int,
    mut Fs: opus_int32,
) {
    let mut i: libc::c_int = 0;
    let mut total_rate: opus_int32 = 0;
    let mut per_stream_rate: opus_int32 = 0;
    let nb_channels: libc::c_int = (*st).layout.nb_streams + (*st).layout.nb_coupled_streams;
    if (*st).bitrate_bps == -(1000 as libc::c_int) {
        total_rate = ((*st).layout.nb_coupled_streams + (*st).layout.nb_streams)
            * (Fs + 60 as libc::c_int * Fs / frame_size)
            + (*st).layout.nb_streams * 15000 as libc::c_int;
    } else if (*st).bitrate_bps == -(1 as libc::c_int) {
        total_rate = nb_channels * 320000 as libc::c_int;
    } else {
        total_rate = (*st).bitrate_bps;
    }
    per_stream_rate = total_rate / (*st).layout.nb_streams;
    i = 0 as libc::c_int;
    while i < (*st).layout.nb_streams {
        *rate.offset(i as isize) = per_stream_rate;
        i += 1;
    }
}
#[c2rust::src_loc = "770:1"]
unsafe extern "C" fn rate_allocation(
    mut st: *mut OpusMSEncoder,
    mut rate: *mut opus_int32,
    mut frame_size: libc::c_int,
) -> opus_int32 {
    let mut i: libc::c_int = 0;
    let mut rate_sum: opus_int32 = 0 as libc::c_int;
    let mut Fs: opus_int32 = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    ptr = (st as *mut libc::c_char).offset(align(
        ::core::mem::size_of::<OpusMSEncoder>() as libc::c_ulong as libc::c_int
    ) as isize);
    opus_encoder_ctl(
        ptr as *mut OpusEncoder,
        4029 as libc::c_int,
        (&mut Fs as *mut opus_int32).offset(
            (&mut Fs as *mut opus_int32).offset_from(&mut Fs as *mut opus_int32) as libc::c_long
                as isize,
        ),
    );
    if (*st).mapping_type as libc::c_uint == MAPPING_TYPE_AMBISONICS as libc::c_int as libc::c_uint
    {
        ambisonics_rate_allocation(st, rate, frame_size, Fs);
    } else {
        surround_rate_allocation(st, rate, frame_size, Fs);
    }
    i = 0 as libc::c_int;
    while i < (*st).layout.nb_streams {
        *rate.offset(i as isize) = if *rate.offset(i as isize) > 500 as libc::c_int {
            *rate.offset(i as isize)
        } else {
            500 as libc::c_int
        };
        rate_sum += *rate.offset(i as isize);
        i += 1;
    }
    return rate_sum;
}
#[no_mangle]
#[c2rust::src_loc = "801:1"]
pub unsafe extern "C" fn opus_multistream_encode_native(
    mut st: *mut OpusMSEncoder,
    mut copy_channel_in: opus_copy_channel_in_func,
    mut pcm: *const libc::c_void,
    mut analysis_frame_size: libc::c_int,
    mut data: *mut libc::c_uchar,
    mut max_data_bytes: opus_int32,
    mut lsb_depth: libc::c_int,
    mut downmix: downmix_func,
    mut float_api: libc::c_int,
    mut user_data: *mut libc::c_void,
) -> libc::c_int {
    let mut Fs: opus_int32 = 0;
    let mut coupled_size: libc::c_int = 0;
    let mut mono_size: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tot_size: libc::c_int = 0;
    let mut tmp_data: [libc::c_uchar; 7662] = [0; 7662];
    let mut rp: OpusRepacketizer = OpusRepacketizer {
        toc: 0,
        nb_frames: 0,
        frames: [0 as *const libc::c_uchar; 48],
        len: [0; 48],
        framesize: 0,
    };
    let mut vbr: opus_int32 = 0;
    let mut celt_mode: *const OpusCustomMode = 0 as *const OpusCustomMode;
    let mut bitrates: [opus_int32; 256] = [0; 256];
    let mut bandLogE: [opus_val16; 42] = [0.; 42];
    let mut mem: *mut opus_val32 = 0 as *mut opus_val32;
    let mut preemph_mem: *mut opus_val32 = 0 as *mut opus_val32;
    let mut frame_size: libc::c_int = 0;
    let mut rate_sum: opus_int32 = 0;
    let mut smallest_packet: opus_int32 = 0;
    if (*st).mapping_type as libc::c_uint == MAPPING_TYPE_SURROUND as libc::c_int as libc::c_uint {
        preemph_mem = ms_get_preemph_mem(st);
        mem = ms_get_window_mem(st);
    }
    ptr = (st as *mut libc::c_char).offset(align(
        ::core::mem::size_of::<OpusMSEncoder>() as libc::c_ulong as libc::c_int
    ) as isize);
    opus_encoder_ctl(
        ptr as *mut OpusEncoder,
        4029 as libc::c_int,
        (&mut Fs as *mut opus_int32).offset(
            (&mut Fs as *mut opus_int32).offset_from(&mut Fs as *mut opus_int32) as libc::c_long
                as isize,
        ),
    );
    opus_encoder_ctl(
        ptr as *mut OpusEncoder,
        4007 as libc::c_int,
        (&mut vbr as *mut opus_int32).offset(
            (&mut vbr as *mut opus_int32).offset_from(&mut vbr as *mut opus_int32) as libc::c_long
                as isize,
        ),
    );
    opus_encoder_ctl(
        ptr as *mut OpusEncoder,
        10015 as libc::c_int,
        (&mut celt_mode as *mut *const OpusCustomMode).offset(
            (&mut celt_mode as *mut *const OpusCustomMode)
                .offset_from(&mut celt_mode as *mut *const OpusCustomMode)
                as libc::c_long as isize,
        ),
    );
    frame_size = frame_size_select(analysis_frame_size, (*st).variable_duration, Fs);
    if frame_size <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    smallest_packet = (*st).layout.nb_streams * 2 as libc::c_int - 1 as libc::c_int;
    if Fs / frame_size == 10 as libc::c_int {
        smallest_packet += (*st).layout.nb_streams;
    }
    if max_data_bytes < smallest_packet {
        return -(2 as libc::c_int);
    }
    let vla = (2 as libc::c_int * frame_size) as usize;
    let mut buf: Vec<opus_val16> = ::std::vec::from_elem(0., vla);
    coupled_size = opus_encoder_get_size(2 as libc::c_int);
    mono_size = opus_encoder_get_size(1 as libc::c_int);
    let vla_0 = (21 as libc::c_int * (*st).layout.nb_channels) as usize;
    let mut bandSMR: Vec<opus_val16> = ::std::vec::from_elem(0., vla_0);
    if (*st).mapping_type as libc::c_uint == MAPPING_TYPE_SURROUND as libc::c_int as libc::c_uint {
        surround_analysis(
            celt_mode,
            pcm,
            bandSMR.as_mut_ptr(),
            mem,
            preemph_mem,
            frame_size,
            120 as libc::c_int,
            (*st).layout.nb_channels,
            Fs,
            copy_channel_in,
            (*st).arch,
        );
    }
    rate_sum = rate_allocation(st, bitrates.as_mut_ptr(), frame_size);
    if vbr == 0 {
        if (*st).bitrate_bps == -(1000 as libc::c_int) {
            max_data_bytes = if max_data_bytes
                < 3 as libc::c_int * rate_sum
                    / (3 as libc::c_int * 8 as libc::c_int * Fs / frame_size)
            {
                max_data_bytes
            } else {
                3 as libc::c_int * rate_sum
                    / (3 as libc::c_int * 8 as libc::c_int * Fs / frame_size)
            };
        } else if (*st).bitrate_bps != -(1 as libc::c_int) {
            max_data_bytes = if max_data_bytes
                < (if smallest_packet
                    > 3 as libc::c_int * (*st).bitrate_bps
                        / (3 as libc::c_int * 8 as libc::c_int * Fs / frame_size)
                {
                    smallest_packet
                } else {
                    3 as libc::c_int * (*st).bitrate_bps
                        / (3 as libc::c_int * 8 as libc::c_int * Fs / frame_size)
                }) {
                max_data_bytes
            } else if smallest_packet
                > 3 as libc::c_int * (*st).bitrate_bps
                    / (3 as libc::c_int * 8 as libc::c_int * Fs / frame_size)
            {
                smallest_packet
            } else {
                3 as libc::c_int * (*st).bitrate_bps
                    / (3 as libc::c_int * 8 as libc::c_int * Fs / frame_size)
            };
        }
    }
    ptr = (st as *mut libc::c_char).offset(align(
        ::core::mem::size_of::<OpusMSEncoder>() as libc::c_ulong as libc::c_int
    ) as isize);
    s = 0 as libc::c_int;
    while s < (*st).layout.nb_streams {
        let mut enc: *mut OpusEncoder = 0 as *mut OpusEncoder;
        enc = ptr as *mut OpusEncoder;
        if s < (*st).layout.nb_coupled_streams {
            ptr = ptr.offset(align(coupled_size) as isize);
        } else {
            ptr = ptr.offset(align(mono_size) as isize);
        }
        opus_encoder_ctl(enc, 4002 as libc::c_int, bitrates[s as usize]);
        if (*st).mapping_type as libc::c_uint
            == MAPPING_TYPE_SURROUND as libc::c_int as libc::c_uint
        {
            let mut equiv_rate: opus_int32 = 0;
            equiv_rate = (*st).bitrate_bps;
            if (frame_size * 50 as libc::c_int) < Fs {
                equiv_rate -= 60 as libc::c_int
                    * (Fs / frame_size - 50 as libc::c_int)
                    * (*st).layout.nb_channels;
            }
            if equiv_rate > 10000 as libc::c_int * (*st).layout.nb_channels {
                opus_encoder_ctl(enc, 4008 as libc::c_int, 1105 as libc::c_int);
            } else if equiv_rate > 7000 as libc::c_int * (*st).layout.nb_channels {
                opus_encoder_ctl(enc, 4008 as libc::c_int, 1104 as libc::c_int);
            } else if equiv_rate > 5000 as libc::c_int * (*st).layout.nb_channels {
                opus_encoder_ctl(enc, 4008 as libc::c_int, 1103 as libc::c_int);
            } else {
                opus_encoder_ctl(enc, 4008 as libc::c_int, 1101 as libc::c_int);
            }
            if s < (*st).layout.nb_coupled_streams {
                opus_encoder_ctl(enc, 11002 as libc::c_int, 1002 as libc::c_int);
                opus_encoder_ctl(enc, 4022 as libc::c_int, 2 as libc::c_int);
            }
        } else if (*st).mapping_type as libc::c_uint
            == MAPPING_TYPE_AMBISONICS as libc::c_int as libc::c_uint
        {
            opus_encoder_ctl(enc, 11002 as libc::c_int, 1002 as libc::c_int);
        }
        s += 1;
    }
    ptr = (st as *mut libc::c_char).offset(align(
        ::core::mem::size_of::<OpusMSEncoder>() as libc::c_ulong as libc::c_int
    ) as isize);
    tot_size = 0 as libc::c_int;
    s = 0 as libc::c_int;
    while s < (*st).layout.nb_streams {
        let mut enc_0: *mut OpusEncoder = 0 as *mut OpusEncoder;
        let mut len: libc::c_int = 0;
        let mut curr_max: libc::c_int = 0;
        let mut c1: libc::c_int = 0;
        let mut c2: libc::c_int = 0;
        let mut ret: libc::c_int = 0;
        opus_repacketizer_init(&mut rp);
        enc_0 = ptr as *mut OpusEncoder;
        if s < (*st).layout.nb_coupled_streams {
            let mut i: libc::c_int = 0;
            let mut left: libc::c_int = 0;
            let mut right: libc::c_int = 0;
            left = get_left_channel(&mut (*st).layout, s, -(1 as libc::c_int));
            right = get_right_channel(&mut (*st).layout, s, -(1 as libc::c_int));
            (Some(copy_channel_in.expect("non-null function pointer")))
                .expect("non-null function pointer")(
                buf.as_mut_ptr(),
                2 as libc::c_int,
                pcm,
                (*st).layout.nb_channels,
                left,
                frame_size,
                user_data,
            );
            (Some(copy_channel_in.expect("non-null function pointer")))
                .expect("non-null function pointer")(
                buf.as_mut_ptr().offset(1 as libc::c_int as isize),
                2 as libc::c_int,
                pcm,
                (*st).layout.nb_channels,
                right,
                frame_size,
                user_data,
            );
            ptr = ptr.offset(align(coupled_size) as isize);
            if (*st).mapping_type as libc::c_uint
                == MAPPING_TYPE_SURROUND as libc::c_int as libc::c_uint
            {
                i = 0 as libc::c_int;
                while i < 21 as libc::c_int {
                    bandLogE[i as usize] = *bandSMR
                        .as_mut_ptr()
                        .offset((21 as libc::c_int * left + i) as isize);
                    bandLogE[(21 as libc::c_int + i) as usize] = *bandSMR
                        .as_mut_ptr()
                        .offset((21 as libc::c_int * right + i) as isize);
                    i += 1;
                }
            }
            c1 = left;
            c2 = right;
        } else {
            let mut i_0: libc::c_int = 0;
            let mut chan: libc::c_int = get_mono_channel(&mut (*st).layout, s, -(1 as libc::c_int));
            (Some(copy_channel_in.expect("non-null function pointer")))
                .expect("non-null function pointer")(
                buf.as_mut_ptr(),
                1 as libc::c_int,
                pcm,
                (*st).layout.nb_channels,
                chan,
                frame_size,
                user_data,
            );
            ptr = ptr.offset(align(mono_size) as isize);
            if (*st).mapping_type as libc::c_uint
                == MAPPING_TYPE_SURROUND as libc::c_int as libc::c_uint
            {
                i_0 = 0 as libc::c_int;
                while i_0 < 21 as libc::c_int {
                    bandLogE[i_0 as usize] = *bandSMR
                        .as_mut_ptr()
                        .offset((21 as libc::c_int * chan + i_0) as isize);
                    i_0 += 1;
                }
            }
            c1 = chan;
            c2 = -(1 as libc::c_int);
        }
        if (*st).mapping_type as libc::c_uint
            == MAPPING_TYPE_SURROUND as libc::c_int as libc::c_uint
        {
            opus_encoder_ctl(
                enc_0,
                10026 as libc::c_int,
                bandLogE.as_mut_ptr().offset(
                    bandLogE.as_mut_ptr().offset_from(bandLogE.as_mut_ptr()) as libc::c_long
                        as isize,
                ),
            );
        }
        curr_max = max_data_bytes - tot_size;
        curr_max -= if 0 as libc::c_int
            > 2 as libc::c_int * ((*st).layout.nb_streams - s - 1 as libc::c_int) - 1 as libc::c_int
        {
            0 as libc::c_int
        } else {
            2 as libc::c_int * ((*st).layout.nb_streams - s - 1 as libc::c_int) - 1 as libc::c_int
        };
        if Fs / frame_size == 10 as libc::c_int {
            curr_max -= (*st).layout.nb_streams - s - 1 as libc::c_int;
        }
        curr_max = if curr_max < 6 as libc::c_int * 1275 as libc::c_int + 12 as libc::c_int {
            curr_max
        } else {
            6 as libc::c_int * 1275 as libc::c_int + 12 as libc::c_int
        };
        if s != (*st).layout.nb_streams - 1 as libc::c_int {
            curr_max -= if curr_max > 253 as libc::c_int {
                2 as libc::c_int
            } else {
                1 as libc::c_int
            };
        }
        if vbr == 0 && s == (*st).layout.nb_streams - 1 as libc::c_int {
            opus_encoder_ctl(
                enc_0,
                4002 as libc::c_int,
                curr_max * (8 as libc::c_int * Fs / frame_size),
            );
        }
        len = opus_encode_native(
            enc_0,
            buf.as_mut_ptr(),
            frame_size,
            tmp_data.as_mut_ptr(),
            curr_max,
            lsb_depth,
            pcm,
            analysis_frame_size,
            c1,
            c2,
            (*st).layout.nb_channels,
            downmix,
            float_api,
        );
        if len < 0 as libc::c_int {
            return len;
        }
        ret = opus_repacketizer_cat(&mut rp, tmp_data.as_mut_ptr(), len);
        if ret != 0 as libc::c_int {
            return -(3 as libc::c_int);
        }
        len = opus_repacketizer_out_range_impl(
            &mut rp,
            0 as libc::c_int,
            opus_repacketizer_get_nb_frames(&mut rp),
            data,
            max_data_bytes - tot_size,
            (s != (*st).layout.nb_streams - 1 as libc::c_int) as libc::c_int,
            (vbr == 0 && s == (*st).layout.nb_streams - 1 as libc::c_int) as libc::c_int,
        );
        data = data.offset(len as isize);
        tot_size += len;
        s += 1;
    }
    return tot_size;
}
#[c2rust::src_loc = "1015:1"]
unsafe extern "C" fn opus_copy_channel_in_float(
    mut dst: *mut opus_val16,
    mut dst_stride: libc::c_int,
    mut src: *const libc::c_void,
    mut src_stride: libc::c_int,
    mut src_channel: libc::c_int,
    mut frame_size: libc::c_int,
    mut user_data: *mut libc::c_void,
) {
    let mut float_src: *const libc::c_float = 0 as *const libc::c_float;
    let mut i: opus_int32 = 0;
    float_src = src as *const libc::c_float;
    i = 0 as libc::c_int;
    while i < frame_size {
        *dst.offset((i * dst_stride) as isize) =
            *float_src.offset((i * src_stride + src_channel) as isize);
        i += 1;
    }
}
#[c2rust::src_loc = "1038:1"]
unsafe extern "C" fn opus_copy_channel_in_short(
    mut dst: *mut opus_val16,
    mut dst_stride: libc::c_int,
    mut src: *const libc::c_void,
    mut src_stride: libc::c_int,
    mut src_channel: libc::c_int,
    mut frame_size: libc::c_int,
    mut user_data: *mut libc::c_void,
) {
    let mut short_src: *const opus_int16 = 0 as *const opus_int16;
    let mut i: opus_int32 = 0;
    short_src = src as *const opus_int16;
    i = 0 as libc::c_int;
    while i < frame_size {
        *dst.offset((i * dst_stride) as isize) = 1 as libc::c_int as libc::c_float / 32768.0f32
            * *short_src.offset((i * src_stride + src_channel) as isize) as libc::c_int
                as libc::c_float;
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "1090:1"]
pub unsafe extern "C" fn opus_multistream_encode_float(
    mut st: *mut OpusMSEncoder,
    mut pcm: *const opus_val16,
    mut frame_size: libc::c_int,
    mut data: *mut libc::c_uchar,
    mut max_data_bytes: opus_int32,
) -> libc::c_int {
    return opus_multistream_encode_native(
        st,
        Some(
            opus_copy_channel_in_float
                as unsafe extern "C" fn(
                    *mut opus_val16,
                    libc::c_int,
                    *const libc::c_void,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    *mut libc::c_void,
                ) -> (),
        ),
        pcm as *const libc::c_void,
        frame_size,
        data,
        max_data_bytes,
        24 as libc::c_int,
        Some(
            downmix_float
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *mut opus_val32,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                ) -> (),
        ),
        1 as libc::c_int,
        0 as *mut libc::c_void,
    );
}
#[no_mangle]
#[c2rust::src_loc = "1103:1"]
pub unsafe extern "C" fn opus_multistream_encode(
    mut st: *mut OpusMSEncoder,
    mut pcm: *const opus_int16,
    mut frame_size: libc::c_int,
    mut data: *mut libc::c_uchar,
    mut max_data_bytes: opus_int32,
) -> libc::c_int {
    return opus_multistream_encode_native(
        st,
        Some(
            opus_copy_channel_in_short
                as unsafe extern "C" fn(
                    *mut opus_val16,
                    libc::c_int,
                    *const libc::c_void,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    *mut libc::c_void,
                ) -> (),
        ),
        pcm as *const libc::c_void,
        frame_size,
        data,
        max_data_bytes,
        16 as libc::c_int,
        Some(
            downmix_int
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *mut opus_val32,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                ) -> (),
        ),
        0 as libc::c_int,
        0 as *mut libc::c_void,
    );
}
#[no_mangle]
#[c2rust::src_loc = "1116:1"]
pub unsafe extern "C" fn opus_multistream_encoder_ctl_va_list(
    mut st: *mut OpusMSEncoder,
    mut request: libc::c_int,
    mut ap: ::core::ffi::VaList,
) -> libc::c_int {
    let mut current_block: u64;
    let mut coupled_size: libc::c_int = 0;
    let mut mono_size: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = 0 as libc::c_int;
    coupled_size = opus_encoder_get_size(2 as libc::c_int);
    mono_size = opus_encoder_get_size(1 as libc::c_int);
    ptr = (st as *mut libc::c_char).offset(align(
        ::core::mem::size_of::<OpusMSEncoder>() as libc::c_ulong as libc::c_int
    ) as isize);
    match request {
        4002 => {
            let mut value: opus_int32 = ap.arg::<opus_int32>();
            if value != -(1000 as libc::c_int) && value != -(1 as libc::c_int) {
                if value <= 0 as libc::c_int {
                    current_block = 11382675479785311092;
                } else {
                    value = if 300000 as libc::c_int * (*st).layout.nb_channels
                        < (if 500 as libc::c_int * (*st).layout.nb_channels > value {
                            500 as libc::c_int * (*st).layout.nb_channels
                        } else {
                            value
                        }) {
                        300000 as libc::c_int * (*st).layout.nb_channels
                    } else if 500 as libc::c_int * (*st).layout.nb_channels > value {
                        500 as libc::c_int * (*st).layout.nb_channels
                    } else {
                        value
                    };
                    current_block = 10879442775620481940;
                }
            } else {
                current_block = 10879442775620481940;
            }
            match current_block {
                11382675479785311092 => {}
                _ => {
                    (*st).bitrate_bps = value;
                    current_block = 2616667235040759262;
                }
            }
        }
        4003 => {
            let mut s: libc::c_int = 0;
            let mut value_0: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_0.is_null() {
                current_block = 11382675479785311092;
            } else {
                *value_0 = 0 as libc::c_int;
                s = 0 as libc::c_int;
                while s < (*st).layout.nb_streams {
                    let mut rate: opus_int32 = 0;
                    let mut enc: *mut OpusEncoder = 0 as *mut OpusEncoder;
                    enc = ptr as *mut OpusEncoder;
                    if s < (*st).layout.nb_coupled_streams {
                        ptr = ptr.offset(align(coupled_size) as isize);
                    } else {
                        ptr = ptr.offset(align(mono_size) as isize);
                    }
                    opus_encoder_ctl(enc, request, &mut rate as *mut opus_int32);
                    *value_0 += rate;
                    s += 1;
                }
                current_block = 2616667235040759262;
            }
        }
        4037 | 4007 | 4001 | 4009 | 4011 | 4015 | 4017 | 11019 | 4021 | 4025 | 4027 | 4029
        | 4013 | 4023 | 4043 | 4047 => {
            let mut enc_0: *mut OpusEncoder = 0 as *mut OpusEncoder;
            let mut value_1: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            enc_0 = ptr as *mut OpusEncoder;
            ret = opus_encoder_ctl(enc_0, request, value_1);
            current_block = 2616667235040759262;
        }
        4031 => {
            let mut s_0: libc::c_int = 0;
            let mut value_2: *mut opus_uint32 = ap.arg::<*mut opus_uint32>();
            let mut tmp: opus_uint32 = 0;
            if value_2.is_null() {
                current_block = 11382675479785311092;
            } else {
                *value_2 = 0 as libc::c_int as opus_uint32;
                s_0 = 0 as libc::c_int;
                while s_0 < (*st).layout.nb_streams {
                    let mut enc_1: *mut OpusEncoder = 0 as *mut OpusEncoder;
                    enc_1 = ptr as *mut OpusEncoder;
                    if s_0 < (*st).layout.nb_coupled_streams {
                        ptr = ptr.offset(align(coupled_size) as isize);
                    } else {
                        ptr = ptr.offset(align(mono_size) as isize);
                    }
                    ret = opus_encoder_ctl(enc_1, request, &mut tmp as *mut opus_uint32);
                    if ret != 0 as libc::c_int {
                        break;
                    }
                    *value_2 ^= tmp;
                    s_0 += 1;
                }
                current_block = 2616667235040759262;
            }
        }
        4036 | 4010 | 4006 | 4020 | 4004 | 4008 | 4024 | 4000 | 4012 | 4014 | 4016 | 11002
        | 4022 | 4042 | 4046 => {
            let mut s_1: libc::c_int = 0;
            let mut value_3: opus_int32 = ap.arg::<opus_int32>();
            s_1 = 0 as libc::c_int;
            while s_1 < (*st).layout.nb_streams {
                let mut enc_2: *mut OpusEncoder = 0 as *mut OpusEncoder;
                enc_2 = ptr as *mut OpusEncoder;
                if s_1 < (*st).layout.nb_coupled_streams {
                    ptr = ptr.offset(align(coupled_size) as isize);
                } else {
                    ptr = ptr.offset(align(mono_size) as isize);
                }
                ret = opus_encoder_ctl(enc_2, request, value_3);
                if ret != 0 as libc::c_int {
                    break;
                }
                s_1 += 1;
            }
            current_block = 2616667235040759262;
        }
        5120 => {
            let mut s_2: libc::c_int = 0;
            let mut stream_id: opus_int32 = 0;
            let mut value_4: *mut *mut OpusEncoder = 0 as *mut *mut OpusEncoder;
            stream_id = ap.arg::<opus_int32>();
            if stream_id < 0 as libc::c_int || stream_id >= (*st).layout.nb_streams {
                current_block = 11382675479785311092;
            } else {
                value_4 = ap.arg::<*mut *mut OpusEncoder>();
                if value_4.is_null() {
                    current_block = 11382675479785311092;
                } else {
                    s_2 = 0 as libc::c_int;
                    while s_2 < stream_id {
                        if s_2 < (*st).layout.nb_coupled_streams {
                            ptr = ptr.offset(align(coupled_size) as isize);
                        } else {
                            ptr = ptr.offset(align(mono_size) as isize);
                        }
                        s_2 += 1;
                    }
                    *value_4 = ptr as *mut OpusEncoder;
                    current_block = 2616667235040759262;
                }
            }
        }
        4040 => {
            let mut value_5: opus_int32 = ap.arg::<opus_int32>();
            (*st).variable_duration = value_5;
            current_block = 2616667235040759262;
        }
        4041 => {
            let mut value_6: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_6.is_null() {
                current_block = 11382675479785311092;
            } else {
                *value_6 = (*st).variable_duration;
                current_block = 2616667235040759262;
            }
        }
        4028 => {
            let mut s_3: libc::c_int = 0;
            if (*st).mapping_type as libc::c_uint
                == MAPPING_TYPE_SURROUND as libc::c_int as libc::c_uint
            {
                memset(
                    ms_get_preemph_mem(st) as *mut libc::c_void,
                    0 as libc::c_int,
                    ((*st).layout.nb_channels as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<opus_val32>() as libc::c_ulong),
                );
                memset(
                    ms_get_window_mem(st) as *mut libc::c_void,
                    0 as libc::c_int,
                    (((*st).layout.nb_channels * 120 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<opus_val32>() as libc::c_ulong),
                );
            }
            s_3 = 0 as libc::c_int;
            while s_3 < (*st).layout.nb_streams {
                let mut enc_3: *mut OpusEncoder = 0 as *mut OpusEncoder;
                enc_3 = ptr as *mut OpusEncoder;
                if s_3 < (*st).layout.nb_coupled_streams {
                    ptr = ptr.offset(align(coupled_size) as isize);
                } else {
                    ptr = ptr.offset(align(mono_size) as isize);
                }
                ret = opus_encoder_ctl(enc_3, 4028 as libc::c_int);
                if ret != 0 as libc::c_int {
                    break;
                }
                s_3 += 1;
            }
            current_block = 2616667235040759262;
        }
        _ => {
            ret = -(5 as libc::c_int);
            current_block = 2616667235040759262;
        }
    }
    match current_block {
        2616667235040759262 => return ret,
        _ => return -(1 as libc::c_int),
    };
}
#[no_mangle]
#[c2rust::src_loc = "1315:1"]
pub unsafe extern "C" fn opus_multistream_encoder_ctl(
    mut st: *mut OpusMSEncoder,
    mut request: libc::c_int,
    mut args: ...
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    ret = opus_multistream_encoder_ctl_va_list(st, request, ap.as_va_list());
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "1325:1"]
pub unsafe extern "C" fn opus_multistream_encoder_destroy(mut st: *mut OpusMSEncoder) {
    opus_free(st as *mut libc::c_void);
}
