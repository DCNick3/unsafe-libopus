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
#[c2rust::header_src = "/usr/include/bits/types.h:36"]
pub mod types_h {
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:36"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:36"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:36"]
pub mod opus_types_h {
    #[c2rust::src_loc = "53:4"]
    pub type opus_int16 = int16_t;
    #[c2rust::src_loc = "55:4"]
    pub type opus_int32 = int32_t;
    #[c2rust::src_loc = "56:4"]
    pub type opus_uint32 = uint32_t;
    use super::stdint_intn_h::{int16_t, int32_t};
    use super::stdint_uintn_h::uint32_t;
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stdarg.h:37"]
pub mod stdarg_h {
    #[c2rust::src_loc = "14:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:38"]
pub mod arch_h {
    #[c2rust::src_loc = "179:1"]
    pub type opus_val16 = libc::c_float;
    #[c2rust::src_loc = "180:1"]
    pub type opus_val32 = libc::c_float;
    #[c2rust::src_loc = "183:1"]
    pub type celt_sig = libc::c_float;
    #[c2rust::src_loc = "184:1"]
    pub type celt_norm = libc::c_float;
    #[c2rust::src_loc = "185:1"]
    pub type celt_ener = libc::c_float;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/kiss_fft.h:38"]
pub mod kiss_fft_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "70:9"]
    pub struct kiss_twiddle_cpx {
        pub r: libc::c_float,
        pub i: libc::c_float,
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
    use super::arch_h::opus_val16;
    use super::opus_types_h::opus_int16;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/mdct.h:38"]
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
        #[c2rust::src_loc = "72:1"]
        pub fn clt_mdct_backward_c(
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entcode.h:40"]
pub mod entcode_h {
    #[c2rust::src_loc = "45:1"]
    pub type ec_window = opus_uint32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "62:8"]
    pub struct ec_ctx {
        pub buf: *mut libc::c_uchar,
        pub storage: opus_uint32,
        pub end_offs: opus_uint32,
        pub end_window: ec_window,
        pub nend_bits: libc::c_int,
        pub nbits_total: libc::c_int,
        pub offs: opus_uint32,
        pub rng: opus_uint32,
        pub val: opus_uint32,
        pub ext: opus_uint32,
        pub rem: libc::c_int,
        pub error: libc::c_int,
    }
    #[c2rust::src_loc = "48:1"]
    pub type ec_dec = ec_ctx;
    #[inline]
    #[c2rust::src_loc = "101:1"]
    pub unsafe extern "C" fn ec_get_error(mut _this: *mut ec_ctx) -> libc::c_int {
        return (*_this).error;
    }
    #[inline]
    #[c2rust::src_loc = "111:1"]
    pub unsafe extern "C" fn ec_tell(mut _this: *mut ec_ctx) -> libc::c_int {
        return (*_this).nbits_total
            - (::core::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int
                * 8 as libc::c_int
                - ((*_this).rng).leading_zeros() as i32);
    }
    use super::opus_types_h::opus_uint32;
    extern "C" {
        #[c2rust::src_loc = "121:1"]
        pub fn ec_tell_frac(_this: *mut ec_ctx) -> opus_uint32;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/cpu_support.h:36"]
pub mod cpu_support_h {
    #[inline]
    #[c2rust::src_loc = "65:1"]
    pub unsafe extern "C" fn opus_select_arch() -> libc::c_int {
        return 0 as libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:37"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "43:14"]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "47:14"]
        pub fn memmove(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "61:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/bits/mathcalls.h:38"]
pub mod mathcalls_h {
    extern "C" {
        #[c2rust::src_loc = "143:13"]
        pub fn sqrt(_: libc::c_double) -> libc::c_double;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_custom.h:40"]
pub mod opus_custom_h {
    use super::modes_h::OpusCustomMode;
    use super::opus_types_h::opus_int32;
    extern "C" {
        #[c2rust::src_loc = "121:20"]
        pub fn opus_custom_mode_create(
            Fs: opus_int32,
            frame_size: libc::c_int,
            error: *mut libc::c_int,
        ) -> *mut OpusCustomMode;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entdec.h:40"]
pub mod entdec_h {
    use super::entcode_h::ec_dec;
    use super::opus_types_h::opus_uint32;
    extern "C" {
        #[c2rust::src_loc = "36:1"]
        pub fn ec_dec_init(_this: *mut ec_dec, _buf: *mut libc::c_uchar, _storage: opus_uint32);
        #[c2rust::src_loc = "72:1"]
        pub fn ec_dec_bit_logp(_this: *mut ec_dec, _logp: libc::c_uint) -> libc::c_int;
        #[c2rust::src_loc = "82:1"]
        pub fn ec_dec_icdf(
            _this: *mut ec_dec,
            _icdf: *const libc::c_uchar,
            _ftb: libc::c_uint,
        ) -> libc::c_int;
        #[c2rust::src_loc = "90:1"]
        pub fn ec_dec_uint(_this: *mut ec_dec, _ft: opus_uint32) -> opus_uint32;
        #[c2rust::src_loc = "98:1"]
        pub fn ec_dec_bits(_this: *mut ec_dec, _ftb: libc::c_uint) -> opus_uint32;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/celt.h:40"]
pub mod celt_h {
    #[c2rust::src_loc = "169:28"]
    pub static mut tapset_icdf: [libc::c_uchar; 3] = [
        2 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ];
    #[c2rust::src_loc = "167:28"]
    pub static mut spread_icdf: [libc::c_uchar; 4] = [
        25 as libc::c_int as libc::c_uchar,
        23 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ];
    #[c2rust::src_loc = "165:28"]
    pub static mut trim_icdf: [libc::c_uchar; 11] = [
        126 as libc::c_int as libc::c_uchar,
        124 as libc::c_int as libc::c_uchar,
        119 as libc::c_int as libc::c_uchar,
        109 as libc::c_int as libc::c_uchar,
        87 as libc::c_int as libc::c_uchar,
        41 as libc::c_int as libc::c_uchar,
        19 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ];
    use super::arch_h::{opus_val16, opus_val32};
    use super::modes_h::OpusCustomMode;
    use super::opus_types_h::opus_int32;
    extern "C" {
        #[c2rust::src_loc = "238:1"]
        pub fn init_caps(
            m: *const OpusCustomMode,
            cap: *mut libc::c_int,
            LM: libc::c_int,
            C: libc::c_int,
        );
        #[c2rust::src_loc = "224:1"]
        pub fn comb_filter(
            y: *mut opus_val32,
            x: *mut opus_val32,
            T0: libc::c_int,
            T1: libc::c_int,
            N: libc::c_int,
            g0: opus_val16,
            g1: opus_val16,
            tapset0: libc::c_int,
            tapset1: libc::c_int,
            window: *const opus_val16,
            overlap: libc::c_int,
            arch: libc::c_int,
        );
        #[c2rust::src_loc = "210:26"]
        pub static tf_select_table: [[libc::c_schar; 8]; 4];
        #[c2rust::src_loc = "219:1"]
        pub fn resampling_factor(rate: opus_int32) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/pitch.h:41"]
pub mod pitch_h {
    use super::arch_h::{celt_sig, opus_val16};
    extern "C" {
        #[c2rust::src_loc = "53:1"]
        pub fn pitch_downsample(
            x: *mut *mut celt_sig,
            x_lp: *mut opus_val16,
            len: libc::c_int,
            C: libc::c_int,
            arch: libc::c_int,
        );
        #[c2rust::src_loc = "56:1"]
        pub fn pitch_search(
            x_lp: *const opus_val16,
            y: *mut opus_val16,
            len: libc::c_int,
            max_pitch: libc::c_int,
            pitch: *mut libc::c_int,
            arch: libc::c_int,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/bands.h:42"]
pub mod bands_h {
    use super::arch_h::{celt_ener, celt_norm, celt_sig, opus_val16};
    use super::entcode_h::ec_ctx;
    use super::modes_h::OpusCustomMode;
    use super::opus_types_h::{opus_int32, opus_uint32};
    extern "C" {
        #[c2rust::src_loc = "64:1"]
        pub fn denormalise_bands(
            m: *const OpusCustomMode,
            X: *const celt_norm,
            freq: *mut celt_sig,
            bandE: *const opus_val16,
            start: libc::c_int,
            end: libc::c_int,
            M: libc::c_int,
            downsample: libc::c_int,
            silence: libc::c_int,
        );
        #[c2rust::src_loc = "113:1"]
        pub fn anti_collapse(
            m: *const OpusCustomMode,
            X_: *mut celt_norm,
            collapse_masks: *mut libc::c_uchar,
            LM: libc::c_int,
            C: libc::c_int,
            size: libc::c_int,
            start: libc::c_int,
            end: libc::c_int,
            logE: *const opus_val16,
            prev1logE: *const opus_val16,
            prev2logE: *const opus_val16,
            pulses: *const libc::c_int,
            seed: opus_uint32,
            arch: libc::c_int,
        );
        #[c2rust::src_loc = "106:1"]
        pub fn quant_all_bands(
            encode: libc::c_int,
            m: *const OpusCustomMode,
            start: libc::c_int,
            end: libc::c_int,
            X: *mut celt_norm,
            Y: *mut celt_norm,
            collapse_masks: *mut libc::c_uchar,
            bandE: *const celt_ener,
            pulses: *mut libc::c_int,
            shortBlocks: libc::c_int,
            spread: libc::c_int,
            dual_stereo: libc::c_int,
            intensity: libc::c_int,
            tf_res: *mut libc::c_int,
            total_bits: opus_int32,
            balance: opus_int32,
            ec: *mut ec_ctx,
            M: libc::c_int,
            codedBands: libc::c_int,
            seed: *mut opus_uint32,
            complexity: libc::c_int,
            arch: libc::c_int,
            disable_inv: libc::c_int,
        );
        #[c2rust::src_loc = "119:1"]
        pub fn celt_lcg_rand(seed: opus_uint32) -> opus_uint32;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/rate.h:42"]
pub mod rate_h {
    use super::entcode_h::ec_ctx;
    use super::modes_h::OpusCustomMode;
    use super::opus_types_h::opus_int32;
    extern "C" {
        #[c2rust::src_loc = "98:1"]
        pub fn clt_compute_allocation(
            m: *const OpusCustomMode,
            start: libc::c_int,
            end: libc::c_int,
            offsets: *const libc::c_int,
            cap: *const libc::c_int,
            alloc_trim: libc::c_int,
            intensity: *mut libc::c_int,
            dual_stereo: *mut libc::c_int,
            total: opus_int32,
            balance: *mut opus_int32,
            pulses: *mut libc::c_int,
            ebits: *mut libc::c_int,
            fine_priority: *mut libc::c_int,
            C: libc::c_int,
            LM: libc::c_int,
            ec: *mut ec_ctx,
            encode: libc::c_int,
            prev: libc::c_int,
            signalBandwidth: libc::c_int,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/quant_bands.h:45"]
pub mod quant_bands_h {
    use super::arch_h::opus_val16;
    use super::entcode_h::ec_dec;
    use super::modes_h::OpusCustomMode;
    extern "C" {
        #[c2rust::src_loc = "62:1"]
        pub fn unquant_fine_energy(
            m: *const OpusCustomMode,
            start: libc::c_int,
            end: libc::c_int,
            oldEBands: *mut opus_val16,
            fine_quant: *mut libc::c_int,
            dec: *mut ec_dec,
            C: libc::c_int,
        );
        #[c2rust::src_loc = "64:1"]
        pub fn unquant_energy_finalise(
            m: *const OpusCustomMode,
            start: libc::c_int,
            end: libc::c_int,
            oldEBands: *mut opus_val16,
            fine_quant: *mut libc::c_int,
            fine_priority: *mut libc::c_int,
            bits_left: libc::c_int,
            dec: *mut ec_dec,
            C: libc::c_int,
        );
        #[c2rust::src_loc = "60:1"]
        pub fn unquant_coarse_energy(
            m: *const OpusCustomMode,
            start: libc::c_int,
            end: libc::c_int,
            oldEBands: *mut opus_val16,
            intra: libc::c_int,
            dec: *mut ec_dec,
            C: libc::c_int,
            LM: libc::c_int,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/celt_lpc.h:51"]
pub mod celt_lpc_h {
    use super::arch_h::{opus_val16, opus_val32};
    extern "C" {
        #[c2rust::src_loc = "40:1"]
        pub fn _celt_lpc(_lpc: *mut opus_val16, ac: *const opus_val32, p: libc::c_int);
        #[c2rust::src_loc = "42:1"]
        pub fn celt_fir_c(
            x: *const opus_val16,
            num: *const opus_val16,
            y: *mut opus_val16,
            N: libc::c_int,
            ord: libc::c_int,
            arch: libc::c_int,
        );
        #[c2rust::src_loc = "55:1"]
        pub fn celt_iir(
            x: *const opus_val32,
            den: *const opus_val16,
            y: *mut opus_val32,
            N: libc::c_int,
            ord: libc::c_int,
            mem: *mut opus_val16,
            arch: libc::c_int,
        );
        #[c2rust::src_loc = "63:1"]
        pub fn _celt_autocorr(
            x: *const opus_val16,
            ac: *mut opus_val32,
            window: *const opus_val16,
            overlap: libc::c_int,
            lag: libc::c_int,
            n: libc::c_int,
            arch: libc::c_int,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/vq.h:52"]
pub mod vq_h {
    use super::arch_h::{celt_norm, opus_val16};
    extern "C" {
        #[c2rust::src_loc = "75:1"]
        pub fn renormalise_vector(
            X: *mut celt_norm,
            N: libc::c_int,
            gain: opus_val16,
            arch: libc::c_int,
        );
    }
}
pub use self::arch_h::{celt_ener, celt_norm, celt_sig, opus_val16, opus_val32};
use self::bands_h::{anti_collapse, celt_lcg_rand, denormalise_bands, quant_all_bands};
pub use self::celt_h::{
    comb_filter, init_caps, resampling_factor, spread_icdf, tapset_icdf, tf_select_table, trim_icdf,
};
use self::celt_lpc_h::{_celt_autocorr, _celt_lpc, celt_fir_c, celt_iir};
pub use self::cpu_support_h::opus_select_arch;
pub use self::entcode_h::{ec_ctx, ec_dec, ec_get_error, ec_tell, ec_tell_frac, ec_window};
use self::entdec_h::{ec_dec_bit_logp, ec_dec_bits, ec_dec_icdf, ec_dec_init, ec_dec_uint};
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::kiss_fft_h::{arch_fft_state, kiss_fft_state, kiss_twiddle_cpx};
use self::mathcalls_h::sqrt;
pub use self::mdct_h::{clt_mdct_backward_c, mdct_lookup};
pub use self::modes_h::{OpusCustomMode, PulseCache};
use self::opus_custom_h::opus_custom_mode_create;
pub use self::opus_types_h::{opus_int16, opus_int32, opus_uint32};
use self::pitch_h::{pitch_downsample, pitch_search};
use self::quant_bands_h::{unquant_coarse_energy, unquant_energy_finalise, unquant_fine_energy};
use self::rate_h::clt_compute_allocation;
pub use self::stdarg_h::va_list;
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::uint32_t;
use self::string_h::{memcpy, memmove, memset};
pub use self::types_h::{__int16_t, __int32_t, __uint32_t};
use self::vq_h::renormalise_vector;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "75:8"]
pub struct OpusCustomDecoder {
    pub mode: *const OpusCustomMode,
    pub overlap: libc::c_int,
    pub channels: libc::c_int,
    pub stream_channels: libc::c_int,
    pub downsample: libc::c_int,
    pub start: libc::c_int,
    pub end: libc::c_int,
    pub signalling: libc::c_int,
    pub disable_inv: libc::c_int,
    pub arch: libc::c_int,
    pub rng: opus_uint32,
    pub error: libc::c_int,
    pub last_pitch_index: libc::c_int,
    pub loss_count: libc::c_int,
    pub skip_plc: libc::c_int,
    pub postfilter_period: libc::c_int,
    pub postfilter_period_old: libc::c_int,
    pub postfilter_gain: opus_val16,
    pub postfilter_gain_old: opus_val16,
    pub postfilter_tapset: libc::c_int,
    pub postfilter_tapset_old: libc::c_int,
    pub preemph_memD: [celt_sig; 2],
    pub _decode_mem: [celt_sig; 1],
}
#[no_mangle]
#[c2rust::src_loc = "144:1"]
pub unsafe extern "C" fn celt_decoder_get_size(mut channels: libc::c_int) -> libc::c_int {
    let mut mode: *const OpusCustomMode = opus_custom_mode_create(
        48000 as libc::c_int,
        960 as libc::c_int,
        0 as *mut libc::c_int,
    );
    return opus_custom_decoder_get_size(mode, channels);
}
#[inline]
#[c2rust::src_loc = "150:1"]
unsafe extern "C" fn opus_custom_decoder_get_size(
    mut mode: *const OpusCustomMode,
    mut channels: libc::c_int,
) -> libc::c_int {
    let mut size: libc::c_int = (::core::mem::size_of::<OpusCustomDecoder>() as libc::c_ulong)
        .wrapping_add(
            ((channels * (2048 as libc::c_int + (*mode).overlap) - 1 as libc::c_int)
                as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<celt_sig>() as libc::c_ulong),
        )
        .wrapping_add(
            ((channels * 24 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<opus_val16>() as libc::c_ulong),
        )
        .wrapping_add(
            ((4 as libc::c_int * 2 as libc::c_int * (*mode).nbEBands) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<opus_val16>() as libc::c_ulong),
        ) as libc::c_int;
    return size;
}
#[no_mangle]
#[c2rust::src_loc = "176:1"]
pub unsafe extern "C" fn celt_decoder_init(
    mut st: *mut OpusCustomDecoder,
    mut sampling_rate: opus_int32,
    mut channels: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = opus_custom_decoder_init(
        st,
        opus_custom_mode_create(
            48000 as libc::c_int,
            960 as libc::c_int,
            0 as *mut libc::c_int,
        ),
        channels,
    );
    if ret != 0 as libc::c_int {
        return ret;
    }
    (*st).downsample = resampling_factor(sampling_rate);
    if (*st).downsample == 0 as libc::c_int {
        return -(1 as libc::c_int);
    } else {
        return 0 as libc::c_int;
    };
}
#[inline]
#[c2rust::src_loc = "189:1"]
unsafe extern "C" fn opus_custom_decoder_init(
    mut st: *mut OpusCustomDecoder,
    mut mode: *const OpusCustomMode,
    mut channels: libc::c_int,
) -> libc::c_int {
    if channels < 0 as libc::c_int || channels > 2 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if st.is_null() {
        return -(7 as libc::c_int);
    }
    memset(
        st as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
        (opus_custom_decoder_get_size(mode, channels) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    (*st).mode = mode;
    (*st).overlap = (*mode).overlap;
    (*st).channels = channels;
    (*st).stream_channels = (*st).channels;
    (*st).downsample = 1 as libc::c_int;
    (*st).start = 0 as libc::c_int;
    (*st).end = (*(*st).mode).effEBands;
    (*st).signalling = 1 as libc::c_int;
    (*st).disable_inv = (channels == 1 as libc::c_int) as libc::c_int;
    (*st).arch = opus_select_arch();
    opus_custom_decoder_ctl(st, 4028 as libc::c_int);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "230:1"]
unsafe extern "C" fn deemphasis_stereo_simple(
    mut in_0: *mut *mut celt_sig,
    mut pcm: *mut opus_val16,
    mut N: libc::c_int,
    coef0: opus_val16,
    mut mem: *mut celt_sig,
) {
    let mut x0: *mut celt_sig = 0 as *mut celt_sig;
    let mut x1: *mut celt_sig = 0 as *mut celt_sig;
    let mut m0: celt_sig = 0.;
    let mut m1: celt_sig = 0.;
    let mut j: libc::c_int = 0;
    x0 = *in_0.offset(0 as libc::c_int as isize);
    x1 = *in_0.offset(1 as libc::c_int as isize);
    m0 = *mem.offset(0 as libc::c_int as isize);
    m1 = *mem.offset(1 as libc::c_int as isize);
    j = 0 as libc::c_int;
    while j < N {
        let mut tmp0: celt_sig = 0.;
        let mut tmp1: celt_sig = 0.;
        tmp0 = *x0.offset(j as isize) + 1e-30f32 + m0;
        tmp1 = *x1.offset(j as isize) + 1e-30f32 + m1;
        m0 = coef0 * tmp0;
        m1 = coef0 * tmp1;
        *pcm.offset((2 as libc::c_int * j) as isize) =
            tmp0 * (1 as libc::c_int as libc::c_float / 32768.0f32);
        *pcm.offset((2 as libc::c_int * j + 1 as libc::c_int) as isize) =
            tmp1 * (1 as libc::c_int as libc::c_float / 32768.0f32);
        j += 1;
    }
    *mem.offset(0 as libc::c_int as isize) = m0;
    *mem.offset(1 as libc::c_int as isize) = m1;
}
#[c2rust::src_loc = "258:1"]
unsafe extern "C" fn deemphasis(
    mut in_0: *mut *mut celt_sig,
    mut pcm: *mut opus_val16,
    mut N: libc::c_int,
    mut C: libc::c_int,
    mut downsample: libc::c_int,
    mut coef: *const opus_val16,
    mut mem: *mut celt_sig,
    mut accum: libc::c_int,
) {
    let mut c: libc::c_int = 0;
    let mut Nd: libc::c_int = 0;
    let mut apply_downsampling: libc::c_int = 0 as libc::c_int;
    let mut coef0: opus_val16 = 0.;
    if downsample == 1 as libc::c_int && C == 2 as libc::c_int && accum == 0 {
        deemphasis_stereo_simple(in_0, pcm, N, *coef.offset(0 as libc::c_int as isize), mem);
        return;
    }
    let vla = N as usize;
    let mut scratch: Vec<celt_sig> = ::std::vec::from_elem(0., vla);
    coef0 = *coef.offset(0 as libc::c_int as isize);
    Nd = N / downsample;
    c = 0 as libc::c_int;
    loop {
        let mut j: libc::c_int = 0;
        let mut x: *mut celt_sig = 0 as *mut celt_sig;
        let mut y: *mut opus_val16 = 0 as *mut opus_val16;
        let mut m: celt_sig = *mem.offset(c as isize);
        x = *in_0.offset(c as isize);
        y = pcm.offset(c as isize);
        if downsample > 1 as libc::c_int {
            j = 0 as libc::c_int;
            while j < N {
                let mut tmp: celt_sig = *x.offset(j as isize) + 1e-30f32 + m;
                m = coef0 * tmp;
                *scratch.as_mut_ptr().offset(j as isize) = tmp;
                j += 1;
            }
            apply_downsampling = 1 as libc::c_int;
        } else {
            j = 0 as libc::c_int;
            while j < N {
                let mut tmp_0: celt_sig = *x.offset(j as isize) + 1e-30f32 + m;
                m = coef0 * tmp_0;
                *y.offset((j * C) as isize) =
                    tmp_0 * (1 as libc::c_int as libc::c_float / 32768.0f32);
                j += 1;
            }
        }
        *mem.offset(c as isize) = m;
        if apply_downsampling != 0 {
            j = 0 as libc::c_int;
            while j < Nd {
                *y.offset((j * C) as isize) =
                    *scratch.as_mut_ptr().offset((j * downsample) as isize)
                        * (1 as libc::c_int as libc::c_float / 32768.0f32);
                j += 1;
            }
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
}
#[c2rust::src_loc = "361:1"]
unsafe extern "C" fn celt_synthesis(
    mut mode: *const OpusCustomMode,
    mut X: *mut celt_norm,
    mut out_syn: *mut *mut celt_sig,
    mut oldBandE: *mut opus_val16,
    mut start: libc::c_int,
    mut effEnd: libc::c_int,
    mut C: libc::c_int,
    mut CC: libc::c_int,
    mut isTransient: libc::c_int,
    mut LM: libc::c_int,
    mut downsample: libc::c_int,
    mut silence: libc::c_int,
    mut arch: libc::c_int,
) {
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut M: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut B: libc::c_int = 0;
    let mut N: libc::c_int = 0;
    let mut NB: libc::c_int = 0;
    let mut shift: libc::c_int = 0;
    let mut nbEBands: libc::c_int = 0;
    let mut overlap: libc::c_int = 0;
    overlap = (*mode).overlap;
    nbEBands = (*mode).nbEBands;
    N = (*mode).shortMdctSize << LM;
    let vla = N as usize;
    let mut freq: Vec<celt_sig> = ::std::vec::from_elem(0., vla);
    M = (1 as libc::c_int) << LM;
    if isTransient != 0 {
        B = M;
        NB = (*mode).shortMdctSize;
        shift = (*mode).maxLM;
    } else {
        B = 1 as libc::c_int;
        NB = (*mode).shortMdctSize << LM;
        shift = (*mode).maxLM - LM;
    }
    if CC == 2 as libc::c_int && C == 1 as libc::c_int {
        let mut freq2: *mut celt_sig = 0 as *mut celt_sig;
        denormalise_bands(
            mode,
            X,
            freq.as_mut_ptr(),
            oldBandE,
            start,
            effEnd,
            M,
            downsample,
            silence,
        );
        freq2 = (*out_syn.offset(1 as libc::c_int as isize))
            .offset((overlap / 2 as libc::c_int) as isize);
        memcpy(
            freq2 as *mut libc::c_void,
            freq.as_mut_ptr() as *const libc::c_void,
            (N as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<celt_sig>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * freq2.offset_from(freq.as_mut_ptr()) as libc::c_long)
                        as libc::c_ulong,
                ),
        );
        b = 0 as libc::c_int;
        while b < B {
            clt_mdct_backward_c(
                &(*mode).mdct,
                &mut *freq2.offset(b as isize),
                (*out_syn.offset(0 as libc::c_int as isize)).offset((NB * b) as isize),
                (*mode).window,
                overlap,
                shift,
                B,
                arch,
            );
            b += 1;
        }
        b = 0 as libc::c_int;
        while b < B {
            clt_mdct_backward_c(
                &(*mode).mdct,
                &mut *freq.as_mut_ptr().offset(b as isize),
                (*out_syn.offset(1 as libc::c_int as isize)).offset((NB * b) as isize),
                (*mode).window,
                overlap,
                shift,
                B,
                arch,
            );
            b += 1;
        }
    } else if CC == 1 as libc::c_int && C == 2 as libc::c_int {
        let mut freq2_0: *mut celt_sig = 0 as *mut celt_sig;
        freq2_0 = (*out_syn.offset(0 as libc::c_int as isize))
            .offset((overlap / 2 as libc::c_int) as isize);
        denormalise_bands(
            mode,
            X,
            freq.as_mut_ptr(),
            oldBandE,
            start,
            effEnd,
            M,
            downsample,
            silence,
        );
        denormalise_bands(
            mode,
            X.offset(N as isize),
            freq2_0,
            oldBandE.offset(nbEBands as isize),
            start,
            effEnd,
            M,
            downsample,
            silence,
        );
        i = 0 as libc::c_int;
        while i < N {
            *freq.as_mut_ptr().offset(i as isize) = 0.5f32 * *freq.as_mut_ptr().offset(i as isize)
                + 0.5f32 * *freq2_0.offset(i as isize);
            i += 1;
        }
        b = 0 as libc::c_int;
        while b < B {
            clt_mdct_backward_c(
                &(*mode).mdct,
                &mut *freq.as_mut_ptr().offset(b as isize),
                (*out_syn.offset(0 as libc::c_int as isize)).offset((NB * b) as isize),
                (*mode).window,
                overlap,
                shift,
                B,
                arch,
            );
            b += 1;
        }
    } else {
        c = 0 as libc::c_int;
        loop {
            denormalise_bands(
                mode,
                X.offset((c * N) as isize),
                freq.as_mut_ptr(),
                oldBandE.offset((c * nbEBands) as isize),
                start,
                effEnd,
                M,
                downsample,
                silence,
            );
            b = 0 as libc::c_int;
            while b < B {
                clt_mdct_backward_c(
                    &(*mode).mdct,
                    &mut *freq.as_mut_ptr().offset(b as isize),
                    (*out_syn.offset(c as isize)).offset((NB * b) as isize),
                    (*mode).window,
                    overlap,
                    shift,
                    B,
                    arch,
                );
                b += 1;
            }
            c += 1;
            if !(c < CC) {
                break;
            }
        }
    }
    c = 0 as libc::c_int;
    loop {
        i = 0 as libc::c_int;
        while i < N {
            *(*out_syn.offset(c as isize)).offset(i as isize) =
                *(*out_syn.offset(c as isize)).offset(i as isize);
            i += 1;
        }
        c += 1;
        if !(c < CC) {
            break;
        }
    }
}
#[c2rust::src_loc = "441:1"]
unsafe extern "C" fn tf_decode(
    mut start: libc::c_int,
    mut end: libc::c_int,
    mut isTransient: libc::c_int,
    mut tf_res: *mut libc::c_int,
    mut LM: libc::c_int,
    mut dec: *mut ec_dec,
) {
    let mut i: libc::c_int = 0;
    let mut curr: libc::c_int = 0;
    let mut tf_select: libc::c_int = 0;
    let mut tf_select_rsv: libc::c_int = 0;
    let mut tf_changed: libc::c_int = 0;
    let mut logp: libc::c_int = 0;
    let mut budget: opus_uint32 = 0;
    let mut tell: opus_uint32 = 0;
    budget = ((*dec).storage).wrapping_mul(8 as libc::c_int as libc::c_uint);
    tell = ec_tell(dec) as opus_uint32;
    logp = if isTransient != 0 {
        2 as libc::c_int
    } else {
        4 as libc::c_int
    };
    tf_select_rsv = (LM > 0 as libc::c_int
        && tell
            .wrapping_add(logp as libc::c_uint)
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            <= budget) as libc::c_int;
    budget = (budget as libc::c_uint).wrapping_sub(tf_select_rsv as libc::c_uint) as opus_uint32
        as opus_uint32;
    curr = 0 as libc::c_int;
    tf_changed = curr;
    i = start;
    while i < end {
        if tell.wrapping_add(logp as libc::c_uint) <= budget {
            curr ^= ec_dec_bit_logp(dec, logp as libc::c_uint);
            tell = ec_tell(dec) as opus_uint32;
            tf_changed |= curr;
        }
        *tf_res.offset(i as isize) = curr;
        logp = if isTransient != 0 {
            4 as libc::c_int
        } else {
            5 as libc::c_int
        };
        i += 1;
    }
    tf_select = 0 as libc::c_int;
    if tf_select_rsv != 0
        && tf_select_table[LM as usize]
            [(4 as libc::c_int * isTransient + 0 as libc::c_int + tf_changed) as usize]
            as libc::c_int
            != tf_select_table[LM as usize]
                [(4 as libc::c_int * isTransient + 2 as libc::c_int + tf_changed) as usize]
                as libc::c_int
    {
        tf_select = ec_dec_bit_logp(dec, 1 as libc::c_int as libc::c_uint);
    }
    i = start;
    while i < end {
        *tf_res.offset(i as isize) = tf_select_table[LM as usize][(4 as libc::c_int * isTransient
            + 2 as libc::c_int * tf_select
            + *tf_res.offset(i as isize))
            as usize] as libc::c_int;
        i += 1;
    }
}
#[c2rust::src_loc = "480:1"]
unsafe extern "C" fn celt_plc_pitch_search(
    mut decode_mem: *mut *mut celt_sig,
    mut C: libc::c_int,
    mut arch: libc::c_int,
) -> libc::c_int {
    let mut pitch_index: libc::c_int = 0;
    let mut lp_pitch_buf: [opus_val16; 1024] = [0.; 1024];
    pitch_downsample(
        decode_mem as *mut *mut celt_sig,
        lp_pitch_buf.as_mut_ptr(),
        2048 as libc::c_int,
        C,
        arch,
    );
    pitch_search(
        lp_pitch_buf
            .as_mut_ptr()
            .offset((720 as libc::c_int >> 1 as libc::c_int) as isize),
        lp_pitch_buf.as_mut_ptr(),
        2048 as libc::c_int - 720 as libc::c_int,
        720 as libc::c_int - 100 as libc::c_int,
        &mut pitch_index,
        arch,
    );
    pitch_index = 720 as libc::c_int - pitch_index;
    return pitch_index;
}
#[c2rust::src_loc = "496:1"]
unsafe extern "C" fn celt_decode_lost(
    mut st: *mut OpusCustomDecoder,
    mut N: libc::c_int,
    mut LM: libc::c_int,
) {
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let C: libc::c_int = (*st).channels;
    let mut decode_mem: [*mut celt_sig; 2] = [0 as *mut celt_sig; 2];
    let mut out_syn: [*mut celt_sig; 2] = [0 as *mut celt_sig; 2];
    let mut lpc: *mut opus_val16 = 0 as *mut opus_val16;
    let mut oldBandE: *mut opus_val16 = 0 as *mut opus_val16;
    let mut oldLogE: *mut opus_val16 = 0 as *mut opus_val16;
    let mut oldLogE2: *mut opus_val16 = 0 as *mut opus_val16;
    let mut backgroundLogE: *mut opus_val16 = 0 as *mut opus_val16;
    let mut mode: *const OpusCustomMode = 0 as *const OpusCustomMode;
    let mut nbEBands: libc::c_int = 0;
    let mut overlap: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut loss_count: libc::c_int = 0;
    let mut noise_based: libc::c_int = 0;
    let mut eBands: *const opus_int16 = 0 as *const opus_int16;
    mode = (*st).mode;
    nbEBands = (*mode).nbEBands;
    overlap = (*mode).overlap;
    eBands = (*mode).eBands;
    c = 0 as libc::c_int;
    loop {
        decode_mem[c as usize] = ((*st)._decode_mem)
            .as_mut_ptr()
            .offset((c * (2048 as libc::c_int + overlap)) as isize);
        out_syn[c as usize] = (decode_mem[c as usize])
            .offset(2048 as libc::c_int as isize)
            .offset(-(N as isize));
        c += 1;
        if !(c < C) {
            break;
        }
    }
    lpc = ((*st)._decode_mem)
        .as_mut_ptr()
        .offset(((2048 as libc::c_int + overlap) * C) as isize) as *mut opus_val16;
    oldBandE = lpc.offset((C * 24 as libc::c_int) as isize);
    oldLogE = oldBandE.offset((2 as libc::c_int * nbEBands) as isize);
    oldLogE2 = oldLogE.offset((2 as libc::c_int * nbEBands) as isize);
    backgroundLogE = oldLogE2.offset((2 as libc::c_int * nbEBands) as isize);
    loss_count = (*st).loss_count;
    start = (*st).start;
    noise_based = (loss_count >= 5 as libc::c_int
        || start != 0 as libc::c_int
        || (*st).skip_plc != 0) as libc::c_int;
    if noise_based != 0 {
        let mut seed: opus_uint32 = 0;
        let mut end: libc::c_int = 0;
        let mut effEnd: libc::c_int = 0;
        let mut decay: opus_val16 = 0.;
        end = (*st).end;
        effEnd = if start
            > (if end < (*mode).effEBands {
                end
            } else {
                (*mode).effEBands
            }) {
            start
        } else if end < (*mode).effEBands {
            end
        } else {
            (*mode).effEBands
        };
        let vla = (C * N) as usize;
        let mut X: Vec<celt_norm> = ::std::vec::from_elem(0., vla);
        decay = if loss_count == 0 as libc::c_int {
            1.5f32
        } else {
            0.5f32
        };
        c = 0 as libc::c_int;
        loop {
            i = start;
            while i < end {
                *oldBandE.offset((c * nbEBands + i) as isize) = if *backgroundLogE
                    .offset((c * nbEBands + i) as isize)
                    > *oldBandE.offset((c * nbEBands + i) as isize) - decay
                {
                    *backgroundLogE.offset((c * nbEBands + i) as isize)
                } else {
                    *oldBandE.offset((c * nbEBands + i) as isize) - decay
                };
                i += 1;
            }
            c += 1;
            if !(c < C) {
                break;
            }
        }
        seed = (*st).rng;
        c = 0 as libc::c_int;
        while c < C {
            i = start;
            while i < effEnd {
                let mut j: libc::c_int = 0;
                let mut boffs: libc::c_int = 0;
                let mut blen: libc::c_int = 0;
                boffs = N * c + ((*eBands.offset(i as isize) as libc::c_int) << LM);
                blen = (*eBands.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                    - *eBands.offset(i as isize) as libc::c_int)
                    << LM;
                j = 0 as libc::c_int;
                while j < blen {
                    seed = celt_lcg_rand(seed);
                    *X.as_mut_ptr().offset((boffs + j) as isize) =
                        (seed as opus_int32 >> 20 as libc::c_int) as celt_norm;
                    j += 1;
                }
                renormalise_vector(
                    X.as_mut_ptr().offset(boffs as isize),
                    blen,
                    1.0f32,
                    (*st).arch,
                );
                i += 1;
            }
            c += 1;
        }
        (*st).rng = seed;
        c = 0 as libc::c_int;
        loop {
            memmove(
                decode_mem[c as usize] as *mut libc::c_void,
                (decode_mem[c as usize]).offset(N as isize) as *const libc::c_void,
                ((2048 as libc::c_int - N + (overlap >> 1 as libc::c_int)) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<celt_sig>() as libc::c_ulong)
                    .wrapping_add(
                        (0 as libc::c_int as libc::c_long
                            * (decode_mem[c as usize])
                                .offset_from((decode_mem[c as usize]).offset(N as isize))
                                as libc::c_long) as libc::c_ulong,
                    ),
            );
            c += 1;
            if !(c < C) {
                break;
            }
        }
        celt_synthesis(
            mode,
            X.as_mut_ptr(),
            out_syn.as_mut_ptr(),
            oldBandE,
            start,
            effEnd,
            C,
            C,
            0 as libc::c_int,
            LM,
            (*st).downsample,
            0 as libc::c_int,
            (*st).arch,
        );
    } else {
        let mut exc_length: libc::c_int = 0;
        let mut window: *const opus_val16 = 0 as *const opus_val16;
        let mut exc: *mut opus_val16 = 0 as *mut opus_val16;
        let mut fade: opus_val16 = 1.0f32;
        let mut pitch_index: libc::c_int = 0;
        if loss_count == 0 as libc::c_int {
            pitch_index = celt_plc_pitch_search(decode_mem.as_mut_ptr(), C, (*st).arch);
            (*st).last_pitch_index = pitch_index;
        } else {
            pitch_index = (*st).last_pitch_index;
            fade = 0.8f32;
        }
        exc_length = if 2 as libc::c_int * pitch_index < 1024 as libc::c_int {
            2 as libc::c_int * pitch_index
        } else {
            1024 as libc::c_int
        };
        let vla_0 = overlap as usize;
        let mut etmp: Vec<opus_val32> = ::std::vec::from_elem(0., vla_0);
        let mut _exc: [opus_val16; 1048] = [0.; 1048];
        let vla_1 = exc_length as usize;
        let mut fir_tmp: Vec<opus_val16> = ::std::vec::from_elem(0., vla_1);
        exc = _exc.as_mut_ptr().offset(24 as libc::c_int as isize);
        window = (*mode).window;
        c = 0 as libc::c_int;
        loop {
            let mut decay_0: opus_val16 = 0.;
            let mut attenuation: opus_val16 = 0.;
            let mut S1: opus_val32 = 0 as libc::c_int as opus_val32;
            let mut buf: *mut celt_sig = 0 as *mut celt_sig;
            let mut extrapolation_offset: libc::c_int = 0;
            let mut extrapolation_len: libc::c_int = 0;
            let mut j_0: libc::c_int = 0;
            buf = decode_mem[c as usize];
            i = 0 as libc::c_int;
            while i < 1024 as libc::c_int + 24 as libc::c_int {
                *exc.offset((i - 24 as libc::c_int) as isize) = *buf.offset(
                    (2048 as libc::c_int - 1024 as libc::c_int - 24 as libc::c_int + i) as isize,
                );
                i += 1;
            }
            if loss_count == 0 as libc::c_int {
                let mut ac: [opus_val32; 25] = [0.; 25];
                _celt_autocorr(
                    exc,
                    ac.as_mut_ptr(),
                    window,
                    overlap,
                    24 as libc::c_int,
                    1024 as libc::c_int,
                    (*st).arch,
                );
                ac[0 as libc::c_int as usize] *= 1.0001f32;
                i = 1 as libc::c_int;
                while i <= 24 as libc::c_int {
                    ac[i as usize] -= ac[i as usize]
                        * (0.008f32 * 0.008f32)
                        * i as libc::c_float
                        * i as libc::c_float;
                    i += 1;
                }
                _celt_lpc(
                    lpc.offset((c * 24 as libc::c_int) as isize),
                    ac.as_mut_ptr(),
                    24 as libc::c_int,
                );
            }
            celt_fir_c(
                exc.offset(1024 as libc::c_int as isize)
                    .offset(-(exc_length as isize)),
                lpc.offset((c * 24 as libc::c_int) as isize),
                fir_tmp.as_mut_ptr(),
                exc_length,
                24 as libc::c_int,
                (*st).arch,
            );
            memcpy(
                exc.offset(1024 as libc::c_int as isize)
                    .offset(-(exc_length as isize)) as *mut libc::c_void,
                fir_tmp.as_mut_ptr() as *const libc::c_void,
                (exc_length as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<opus_val16>() as libc::c_ulong)
                    .wrapping_add(
                        (0 as libc::c_int as libc::c_long
                            * exc
                                .offset(1024 as libc::c_int as isize)
                                .offset(-(exc_length as isize))
                                .offset_from(fir_tmp.as_mut_ptr())
                                as libc::c_long) as libc::c_ulong,
                    ),
            );
            let mut E1: opus_val32 = 1 as libc::c_int as opus_val32;
            let mut E2: opus_val32 = 1 as libc::c_int as opus_val32;
            let mut decay_length: libc::c_int = 0;
            decay_length = exc_length >> 1 as libc::c_int;
            i = 0 as libc::c_int;
            while i < decay_length {
                let mut e: opus_val16 = 0.;
                e = *exc.offset((1024 as libc::c_int - decay_length + i) as isize);
                E1 += e * e;
                e = *exc
                    .offset((1024 as libc::c_int - 2 as libc::c_int * decay_length + i) as isize);
                E2 += e * e;
                i += 1;
            }
            E1 = if E1 < E2 { E1 } else { E2 };
            decay_0 = sqrt((E1 / E2) as libc::c_double) as libc::c_float;
            memmove(
                buf as *mut libc::c_void,
                buf.offset(N as isize) as *const libc::c_void,
                ((2048 as libc::c_int - N) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<celt_sig>() as libc::c_ulong)
                    .wrapping_add(
                        (0 as libc::c_int as libc::c_long
                            * buf.offset_from(buf.offset(N as isize)) as libc::c_long)
                            as libc::c_ulong,
                    ),
            );
            extrapolation_offset = 1024 as libc::c_int - pitch_index;
            extrapolation_len = N + overlap;
            attenuation = fade * decay_0;
            j_0 = 0 as libc::c_int;
            i = j_0;
            while i < extrapolation_len {
                let mut tmp: opus_val16 = 0.;
                if j_0 >= pitch_index {
                    j_0 -= pitch_index;
                    attenuation = attenuation * decay_0;
                }
                *buf.offset((2048 as libc::c_int - N + i) as isize) =
                    attenuation * *exc.offset((extrapolation_offset + j_0) as isize);
                tmp = *buf.offset(
                    (2048 as libc::c_int - 1024 as libc::c_int - N + extrapolation_offset + j_0)
                        as isize,
                );
                S1 += tmp * tmp;
                i += 1;
                j_0 += 1;
            }
            let mut lpc_mem: [opus_val16; 24] = [0.; 24];
            i = 0 as libc::c_int;
            while i < 24 as libc::c_int {
                lpc_mem[i as usize] =
                    *buf.offset((2048 as libc::c_int - N - 1 as libc::c_int - i) as isize);
                i += 1;
            }
            celt_iir(
                buf.offset(2048 as libc::c_int as isize)
                    .offset(-(N as isize)),
                lpc.offset((c * 24 as libc::c_int) as isize),
                buf.offset(2048 as libc::c_int as isize)
                    .offset(-(N as isize)),
                extrapolation_len,
                24 as libc::c_int,
                lpc_mem.as_mut_ptr(),
                (*st).arch,
            );
            let mut S2: opus_val32 = 0 as libc::c_int as opus_val32;
            i = 0 as libc::c_int;
            while i < extrapolation_len {
                let mut tmp_0: opus_val16 = *buf.offset((2048 as libc::c_int - N + i) as isize);
                S2 += tmp_0 * tmp_0;
                i += 1;
            }
            if !(S1 > 0.2f32 * S2) {
                i = 0 as libc::c_int;
                while i < extrapolation_len {
                    *buf.offset((2048 as libc::c_int - N + i) as isize) =
                        0 as libc::c_int as celt_sig;
                    i += 1;
                }
            } else if S1 < S2 {
                let mut ratio: opus_val16 = sqrt(
                    ((S1 + 1 as libc::c_int as libc::c_float)
                        / (S2 + 1 as libc::c_int as libc::c_float))
                        as libc::c_double,
                ) as libc::c_float;
                i = 0 as libc::c_int;
                while i < overlap {
                    let mut tmp_g: opus_val16 =
                        1.0f32 - *window.offset(i as isize) * (1.0f32 - ratio);
                    *buf.offset((2048 as libc::c_int - N + i) as isize) =
                        tmp_g * *buf.offset((2048 as libc::c_int - N + i) as isize);
                    i += 1;
                }
                i = overlap;
                while i < extrapolation_len {
                    *buf.offset((2048 as libc::c_int - N + i) as isize) =
                        ratio * *buf.offset((2048 as libc::c_int - N + i) as isize);
                    i += 1;
                }
            }
            comb_filter(
                etmp.as_mut_ptr(),
                buf.offset(2048 as libc::c_int as isize),
                (*st).postfilter_period,
                (*st).postfilter_period,
                overlap,
                -(*st).postfilter_gain,
                -(*st).postfilter_gain,
                (*st).postfilter_tapset,
                (*st).postfilter_tapset,
                0 as *const opus_val16,
                0 as libc::c_int,
                (*st).arch,
            );
            i = 0 as libc::c_int;
            while i < overlap / 2 as libc::c_int {
                *buf.offset((2048 as libc::c_int + i) as isize) = *window.offset(i as isize)
                    * *etmp
                        .as_mut_ptr()
                        .offset((overlap - 1 as libc::c_int - i) as isize)
                    + *window.offset((overlap - i - 1 as libc::c_int) as isize)
                        * *etmp.as_mut_ptr().offset(i as isize);
                i += 1;
            }
            c += 1;
            if !(c < C) {
                break;
            }
        }
    }
    (*st).loss_count = loss_count + 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "814:1"]
pub unsafe extern "C" fn celt_decode_with_ec(
    mut st: *mut OpusCustomDecoder,
    mut data: *const libc::c_uchar,
    mut len: libc::c_int,
    mut pcm: *mut opus_val16,
    mut frame_size: libc::c_int,
    mut dec: *mut ec_dec,
    mut accum: libc::c_int,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut N: libc::c_int = 0;
    let mut spread_decision: libc::c_int = 0;
    let mut bits: opus_int32 = 0;
    let mut _dec: ec_dec = ec_dec {
        buf: 0 as *mut libc::c_uchar,
        storage: 0,
        end_offs: 0,
        end_window: 0,
        nend_bits: 0,
        nbits_total: 0,
        offs: 0,
        rng: 0,
        val: 0,
        ext: 0,
        rem: 0,
        error: 0,
    };
    let mut decode_mem: [*mut celt_sig; 2] = [0 as *mut celt_sig; 2];
    let mut out_syn: [*mut celt_sig; 2] = [0 as *mut celt_sig; 2];
    let mut lpc: *mut opus_val16 = 0 as *mut opus_val16;
    let mut oldBandE: *mut opus_val16 = 0 as *mut opus_val16;
    let mut oldLogE: *mut opus_val16 = 0 as *mut opus_val16;
    let mut oldLogE2: *mut opus_val16 = 0 as *mut opus_val16;
    let mut backgroundLogE: *mut opus_val16 = 0 as *mut opus_val16;
    let mut shortBlocks: libc::c_int = 0;
    let mut isTransient: libc::c_int = 0;
    let mut intra_ener: libc::c_int = 0;
    let CC: libc::c_int = (*st).channels;
    let mut LM: libc::c_int = 0;
    let mut M: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut effEnd: libc::c_int = 0;
    let mut codedBands: libc::c_int = 0;
    let mut alloc_trim: libc::c_int = 0;
    let mut postfilter_pitch: libc::c_int = 0;
    let mut postfilter_gain: opus_val16 = 0.;
    let mut intensity: libc::c_int = 0 as libc::c_int;
    let mut dual_stereo: libc::c_int = 0 as libc::c_int;
    let mut total_bits: opus_int32 = 0;
    let mut balance: opus_int32 = 0;
    let mut tell: opus_int32 = 0;
    let mut dynalloc_logp: libc::c_int = 0;
    let mut postfilter_tapset: libc::c_int = 0;
    let mut anti_collapse_rsv: libc::c_int = 0;
    let mut anti_collapse_on: libc::c_int = 0 as libc::c_int;
    let mut silence: libc::c_int = 0;
    let mut C: libc::c_int = (*st).stream_channels;
    let mut mode: *const OpusCustomMode = 0 as *const OpusCustomMode;
    let mut nbEBands: libc::c_int = 0;
    let mut overlap: libc::c_int = 0;
    let mut eBands: *const opus_int16 = 0 as *const opus_int16;
    mode = (*st).mode;
    nbEBands = (*mode).nbEBands;
    overlap = (*mode).overlap;
    eBands = (*mode).eBands;
    start = (*st).start;
    end = (*st).end;
    frame_size *= (*st).downsample;
    lpc = ((*st)._decode_mem)
        .as_mut_ptr()
        .offset(((2048 as libc::c_int + overlap) * CC) as isize) as *mut opus_val16;
    oldBandE = lpc.offset((CC * 24 as libc::c_int) as isize);
    oldLogE = oldBandE.offset((2 as libc::c_int * nbEBands) as isize);
    oldLogE2 = oldLogE.offset((2 as libc::c_int * nbEBands) as isize);
    backgroundLogE = oldLogE2.offset((2 as libc::c_int * nbEBands) as isize);
    LM = 0 as libc::c_int;
    while LM <= (*mode).maxLM {
        if (*mode).shortMdctSize << LM == frame_size {
            break;
        }
        LM += 1;
    }
    if LM > (*mode).maxLM {
        return -(1 as libc::c_int);
    }
    M = (1 as libc::c_int) << LM;
    if len < 0 as libc::c_int || len > 1275 as libc::c_int || pcm.is_null() {
        return -(1 as libc::c_int);
    }
    N = M * (*mode).shortMdctSize;
    c = 0 as libc::c_int;
    loop {
        decode_mem[c as usize] = ((*st)._decode_mem)
            .as_mut_ptr()
            .offset((c * (2048 as libc::c_int + overlap)) as isize);
        out_syn[c as usize] = (decode_mem[c as usize])
            .offset(2048 as libc::c_int as isize)
            .offset(-(N as isize));
        c += 1;
        if !(c < CC) {
            break;
        }
    }
    effEnd = end;
    if effEnd > (*mode).effEBands {
        effEnd = (*mode).effEBands;
    }
    if data.is_null() || len <= 1 as libc::c_int {
        celt_decode_lost(st, N, LM);
        deemphasis(
            out_syn.as_mut_ptr(),
            pcm,
            N,
            CC,
            (*st).downsample,
            ((*mode).preemph).as_ptr(),
            ((*st).preemph_memD).as_mut_ptr(),
            accum,
        );
        return frame_size / (*st).downsample;
    }
    (*st).skip_plc = ((*st).loss_count != 0 as libc::c_int) as libc::c_int;
    if dec.is_null() {
        ec_dec_init(&mut _dec, data as *mut libc::c_uchar, len as opus_uint32);
        dec = &mut _dec;
    }
    if C == 1 as libc::c_int {
        i = 0 as libc::c_int;
        while i < nbEBands {
            *oldBandE.offset(i as isize) =
                if *oldBandE.offset(i as isize) > *oldBandE.offset((nbEBands + i) as isize) {
                    *oldBandE.offset(i as isize)
                } else {
                    *oldBandE.offset((nbEBands + i) as isize)
                };
            i += 1;
        }
    }
    total_bits = len * 8 as libc::c_int;
    tell = ec_tell(dec);
    if tell >= total_bits {
        silence = 1 as libc::c_int;
    } else if tell == 1 as libc::c_int {
        silence = ec_dec_bit_logp(dec, 15 as libc::c_int as libc::c_uint);
    } else {
        silence = 0 as libc::c_int;
    }
    if silence != 0 {
        tell = len * 8 as libc::c_int;
        (*dec).nbits_total += tell - ec_tell(dec);
    }
    postfilter_gain = 0 as libc::c_int as opus_val16;
    postfilter_pitch = 0 as libc::c_int;
    postfilter_tapset = 0 as libc::c_int;
    if start == 0 as libc::c_int && tell + 16 as libc::c_int <= total_bits {
        if ec_dec_bit_logp(dec, 1 as libc::c_int as libc::c_uint) != 0 {
            let mut qg: libc::c_int = 0;
            let mut octave: libc::c_int = 0;
            octave = ec_dec_uint(dec, 6 as libc::c_int as opus_uint32) as libc::c_int;
            postfilter_pitch = (((16 as libc::c_int) << octave) as libc::c_uint)
                .wrapping_add(ec_dec_bits(
                    dec,
                    (4 as libc::c_int + octave) as libc::c_uint,
                ))
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                as libc::c_int;
            qg = ec_dec_bits(dec, 3 as libc::c_int as libc::c_uint) as libc::c_int;
            if ec_tell(dec) + 2 as libc::c_int <= total_bits {
                postfilter_tapset =
                    ec_dec_icdf(dec, tapset_icdf.as_ptr(), 2 as libc::c_int as libc::c_uint);
            }
            postfilter_gain = 0.09375f32 * (qg + 1 as libc::c_int) as libc::c_float;
        }
        tell = ec_tell(dec);
    }
    if LM > 0 as libc::c_int && tell + 3 as libc::c_int <= total_bits {
        isTransient = ec_dec_bit_logp(dec, 3 as libc::c_int as libc::c_uint);
        tell = ec_tell(dec);
    } else {
        isTransient = 0 as libc::c_int;
    }
    if isTransient != 0 {
        shortBlocks = M;
    } else {
        shortBlocks = 0 as libc::c_int;
    }
    intra_ener = if tell + 3 as libc::c_int <= total_bits {
        ec_dec_bit_logp(dec, 3 as libc::c_int as libc::c_uint)
    } else {
        0 as libc::c_int
    };
    unquant_coarse_energy(mode, start, end, oldBandE, intra_ener, dec, C, LM);
    let vla = nbEBands as usize;
    let mut tf_res: Vec<libc::c_int> = ::std::vec::from_elem(0, vla);
    tf_decode(start, end, isTransient, tf_res.as_mut_ptr(), LM, dec);
    tell = ec_tell(dec);
    spread_decision = 2 as libc::c_int;
    if tell + 4 as libc::c_int <= total_bits {
        spread_decision = ec_dec_icdf(dec, spread_icdf.as_ptr(), 5 as libc::c_int as libc::c_uint);
    }
    let vla_0 = nbEBands as usize;
    let mut cap: Vec<libc::c_int> = ::std::vec::from_elem(0, vla_0);
    init_caps(mode, cap.as_mut_ptr(), LM, C);
    let vla_1 = nbEBands as usize;
    let mut offsets: Vec<libc::c_int> = ::std::vec::from_elem(0, vla_1);
    dynalloc_logp = 6 as libc::c_int;
    total_bits <<= 3 as libc::c_int;
    tell = ec_tell_frac(dec) as opus_int32;
    i = start;
    while i < end {
        let mut width: libc::c_int = 0;
        let mut quanta: libc::c_int = 0;
        let mut dynalloc_loop_logp: libc::c_int = 0;
        let mut boost: libc::c_int = 0;
        width = C
            * (*eBands.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                - *eBands.offset(i as isize) as libc::c_int)
            << LM;
        quanta = if (width << 3 as libc::c_int)
            < (if (6 as libc::c_int) << 3 as libc::c_int > width {
                (6 as libc::c_int) << 3 as libc::c_int
            } else {
                width
            }) {
            width << 3 as libc::c_int
        } else if (6 as libc::c_int) << 3 as libc::c_int > width {
            (6 as libc::c_int) << 3 as libc::c_int
        } else {
            width
        };
        dynalloc_loop_logp = dynalloc_logp;
        boost = 0 as libc::c_int;
        while tell + (dynalloc_loop_logp << 3 as libc::c_int) < total_bits
            && boost < *cap.as_mut_ptr().offset(i as isize)
        {
            let mut flag: libc::c_int = 0;
            flag = ec_dec_bit_logp(dec, dynalloc_loop_logp as libc::c_uint);
            tell = ec_tell_frac(dec) as opus_int32;
            if flag == 0 {
                break;
            }
            boost += quanta;
            total_bits -= quanta;
            dynalloc_loop_logp = 1 as libc::c_int;
        }
        *offsets.as_mut_ptr().offset(i as isize) = boost;
        if boost > 0 as libc::c_int {
            dynalloc_logp = if 2 as libc::c_int > dynalloc_logp - 1 as libc::c_int {
                2 as libc::c_int
            } else {
                dynalloc_logp - 1 as libc::c_int
            };
        }
        i += 1;
    }
    let vla_2 = nbEBands as usize;
    let mut fine_quant: Vec<libc::c_int> = ::std::vec::from_elem(0, vla_2);
    alloc_trim = if tell + ((6 as libc::c_int) << 3 as libc::c_int) <= total_bits {
        ec_dec_icdf(dec, trim_icdf.as_ptr(), 7 as libc::c_int as libc::c_uint)
    } else {
        5 as libc::c_int
    };
    bits = (((len * 8 as libc::c_int) << 3 as libc::c_int) as libc::c_uint)
        .wrapping_sub(ec_tell_frac(dec))
        .wrapping_sub(1 as libc::c_int as libc::c_uint) as opus_int32;
    anti_collapse_rsv = if isTransient != 0
        && LM >= 2 as libc::c_int
        && bits >= (LM + 2 as libc::c_int) << 3 as libc::c_int
    {
        (1 as libc::c_int) << 3 as libc::c_int
    } else {
        0 as libc::c_int
    };
    bits -= anti_collapse_rsv;
    let vla_3 = nbEBands as usize;
    let mut pulses: Vec<libc::c_int> = ::std::vec::from_elem(0, vla_3);
    let vla_4 = nbEBands as usize;
    let mut fine_priority: Vec<libc::c_int> = ::std::vec::from_elem(0, vla_4);
    codedBands = clt_compute_allocation(
        mode,
        start,
        end,
        offsets.as_mut_ptr(),
        cap.as_mut_ptr(),
        alloc_trim,
        &mut intensity,
        &mut dual_stereo,
        bits,
        &mut balance,
        pulses.as_mut_ptr(),
        fine_quant.as_mut_ptr(),
        fine_priority.as_mut_ptr(),
        C,
        LM,
        dec,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    unquant_fine_energy(mode, start, end, oldBandE, fine_quant.as_mut_ptr(), dec, C);
    c = 0 as libc::c_int;
    loop {
        memmove(
            decode_mem[c as usize] as *mut libc::c_void,
            (decode_mem[c as usize]).offset(N as isize) as *const libc::c_void,
            ((2048 as libc::c_int - N + overlap / 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<celt_sig>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * (decode_mem[c as usize])
                            .offset_from((decode_mem[c as usize]).offset(N as isize))
                            as libc::c_long) as libc::c_ulong,
                ),
        );
        c += 1;
        if !(c < CC) {
            break;
        }
    }
    let vla_5 = (C * nbEBands) as usize;
    let mut collapse_masks: Vec<libc::c_uchar> = ::std::vec::from_elem(0, vla_5);
    let vla_6 = (C * N) as usize;
    let mut X: Vec<celt_norm> = ::std::vec::from_elem(0., vla_6);
    quant_all_bands(
        0 as libc::c_int,
        mode,
        start,
        end,
        X.as_mut_ptr(),
        if C == 2 as libc::c_int {
            X.as_mut_ptr().offset(N as isize)
        } else {
            0 as *mut celt_norm
        },
        collapse_masks.as_mut_ptr(),
        0 as *const celt_ener,
        pulses.as_mut_ptr(),
        shortBlocks,
        spread_decision,
        dual_stereo,
        intensity,
        tf_res.as_mut_ptr(),
        len * ((8 as libc::c_int) << 3 as libc::c_int) - anti_collapse_rsv,
        balance,
        dec,
        LM,
        codedBands,
        &mut (*st).rng,
        0 as libc::c_int,
        (*st).arch,
        (*st).disable_inv,
    );
    if anti_collapse_rsv > 0 as libc::c_int {
        anti_collapse_on = ec_dec_bits(dec, 1 as libc::c_int as libc::c_uint) as libc::c_int;
    }
    unquant_energy_finalise(
        mode,
        start,
        end,
        oldBandE,
        fine_quant.as_mut_ptr(),
        fine_priority.as_mut_ptr(),
        len * 8 as libc::c_int - ec_tell(dec),
        dec,
        C,
    );
    if anti_collapse_on != 0 {
        anti_collapse(
            mode,
            X.as_mut_ptr(),
            collapse_masks.as_mut_ptr(),
            LM,
            C,
            N,
            start,
            end,
            oldBandE,
            oldLogE,
            oldLogE2,
            pulses.as_mut_ptr(),
            (*st).rng,
            (*st).arch,
        );
    }
    if silence != 0 {
        i = 0 as libc::c_int;
        while i < C * nbEBands {
            *oldBandE.offset(i as isize) = -28.0f32;
            i += 1;
        }
    }
    celt_synthesis(
        mode,
        X.as_mut_ptr(),
        out_syn.as_mut_ptr(),
        oldBandE,
        start,
        effEnd,
        C,
        CC,
        isTransient,
        LM,
        (*st).downsample,
        silence,
        (*st).arch,
    );
    c = 0 as libc::c_int;
    loop {
        (*st).postfilter_period = if (*st).postfilter_period > 15 as libc::c_int {
            (*st).postfilter_period
        } else {
            15 as libc::c_int
        };
        (*st).postfilter_period_old = if (*st).postfilter_period_old > 15 as libc::c_int {
            (*st).postfilter_period_old
        } else {
            15 as libc::c_int
        };
        comb_filter(
            out_syn[c as usize],
            out_syn[c as usize],
            (*st).postfilter_period_old,
            (*st).postfilter_period,
            (*mode).shortMdctSize,
            (*st).postfilter_gain_old,
            (*st).postfilter_gain,
            (*st).postfilter_tapset_old,
            (*st).postfilter_tapset,
            (*mode).window,
            overlap,
            (*st).arch,
        );
        if LM != 0 as libc::c_int {
            comb_filter(
                (out_syn[c as usize]).offset((*mode).shortMdctSize as isize),
                (out_syn[c as usize]).offset((*mode).shortMdctSize as isize),
                (*st).postfilter_period,
                postfilter_pitch,
                N - (*mode).shortMdctSize,
                (*st).postfilter_gain,
                postfilter_gain,
                (*st).postfilter_tapset,
                postfilter_tapset,
                (*mode).window,
                overlap,
                (*st).arch,
            );
        }
        c += 1;
        if !(c < CC) {
            break;
        }
    }
    (*st).postfilter_period_old = (*st).postfilter_period;
    (*st).postfilter_gain_old = (*st).postfilter_gain;
    (*st).postfilter_tapset_old = (*st).postfilter_tapset;
    (*st).postfilter_period = postfilter_pitch;
    (*st).postfilter_gain = postfilter_gain;
    (*st).postfilter_tapset = postfilter_tapset;
    if LM != 0 as libc::c_int {
        (*st).postfilter_period_old = (*st).postfilter_period;
        (*st).postfilter_gain_old = (*st).postfilter_gain;
        (*st).postfilter_tapset_old = (*st).postfilter_tapset;
    }
    if C == 1 as libc::c_int {
        memcpy(
            &mut *oldBandE.offset(nbEBands as isize) as *mut opus_val16 as *mut libc::c_void,
            oldBandE as *const libc::c_void,
            (nbEBands as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<opus_val16>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * (&mut *oldBandE.offset(nbEBands as isize) as *mut opus_val16)
                            .offset_from(oldBandE) as libc::c_long)
                        as libc::c_ulong,
                ),
        );
    }
    if isTransient == 0 {
        let mut max_background_increase: opus_val16 = 0.;
        memcpy(
            oldLogE2 as *mut libc::c_void,
            oldLogE as *const libc::c_void,
            ((2 as libc::c_int * nbEBands) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<opus_val16>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * oldLogE2.offset_from(oldLogE) as libc::c_long)
                        as libc::c_ulong,
                ),
        );
        memcpy(
            oldLogE as *mut libc::c_void,
            oldBandE as *const libc::c_void,
            ((2 as libc::c_int * nbEBands) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<opus_val16>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * oldLogE.offset_from(oldBandE) as libc::c_long)
                        as libc::c_ulong,
                ),
        );
        if (*st).loss_count < 10 as libc::c_int {
            max_background_increase = M as libc::c_float * 0.001f32;
        } else {
            max_background_increase = 1.0f32;
        }
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int * nbEBands {
            *backgroundLogE.offset(i as isize) = if *backgroundLogE.offset(i as isize)
                + max_background_increase
                < *oldBandE.offset(i as isize)
            {
                *backgroundLogE.offset(i as isize) + max_background_increase
            } else {
                *oldBandE.offset(i as isize)
            };
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int * nbEBands {
            *oldLogE.offset(i as isize) =
                if *oldLogE.offset(i as isize) < *oldBandE.offset(i as isize) {
                    *oldLogE.offset(i as isize)
                } else {
                    *oldBandE.offset(i as isize)
                };
            i += 1;
        }
    }
    c = 0 as libc::c_int;
    loop {
        i = 0 as libc::c_int;
        while i < start {
            *oldBandE.offset((c * nbEBands + i) as isize) = 0 as libc::c_int as opus_val16;
            let ref mut fresh0 = *oldLogE2.offset((c * nbEBands + i) as isize);
            *fresh0 = -28.0f32;
            *oldLogE.offset((c * nbEBands + i) as isize) = *fresh0;
            i += 1;
        }
        i = end;
        while i < nbEBands {
            *oldBandE.offset((c * nbEBands + i) as isize) = 0 as libc::c_int as opus_val16;
            let ref mut fresh1 = *oldLogE2.offset((c * nbEBands + i) as isize);
            *fresh1 = -28.0f32;
            *oldLogE.offset((c * nbEBands + i) as isize) = *fresh1;
            i += 1;
        }
        c += 1;
        if !(c < 2 as libc::c_int) {
            break;
        }
    }
    (*st).rng = (*dec).rng;
    deemphasis(
        out_syn.as_mut_ptr(),
        pcm,
        N,
        CC,
        (*st).downsample,
        ((*mode).preemph).as_ptr(),
        ((*st).preemph_memD).as_mut_ptr(),
        accum,
    );
    (*st).loss_count = 0 as libc::c_int;
    if ec_tell(dec) > 8 as libc::c_int * len {
        return -(3 as libc::c_int);
    }
    if ec_get_error(dec) != 0 {
        (*st).error = 1 as libc::c_int;
    }
    return frame_size / (*st).downsample;
}
#[no_mangle]
#[c2rust::src_loc = "1247:1"]
pub unsafe extern "C" fn opus_custom_decoder_ctl(
    mut st: *mut OpusCustomDecoder,
    mut request: libc::c_int,
    mut args: ...
) -> libc::c_int {
    let mut current_block: u64;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    match request {
        10010 => {
            let mut value: opus_int32 = ap.arg::<opus_int32>();
            if value < 0 as libc::c_int || value >= (*(*st).mode).nbEBands {
                current_block = 7990025728955927862;
            } else {
                (*st).start = value;
                current_block = 3689906465960840878;
            }
        }
        10012 => {
            let mut value_0: opus_int32 = ap.arg::<opus_int32>();
            if value_0 < 1 as libc::c_int || value_0 > (*(*st).mode).nbEBands {
                current_block = 7990025728955927862;
            } else {
                (*st).end = value_0;
                current_block = 3689906465960840878;
            }
        }
        10008 => {
            let mut value_1: opus_int32 = ap.arg::<opus_int32>();
            if value_1 < 1 as libc::c_int || value_1 > 2 as libc::c_int {
                current_block = 7990025728955927862;
            } else {
                (*st).stream_channels = value_1;
                current_block = 3689906465960840878;
            }
        }
        10007 => {
            let mut value_2: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_2.is_null() {
                current_block = 7990025728955927862;
            } else {
                *value_2 = (*st).error;
                (*st).error = 0 as libc::c_int;
                current_block = 3689906465960840878;
            }
        }
        4027 => {
            let mut value_3: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_3.is_null() {
                current_block = 7990025728955927862;
            } else {
                *value_3 = (*st).overlap / (*st).downsample;
                current_block = 3689906465960840878;
            }
        }
        4028 => {
            let mut i: libc::c_int = 0;
            let mut lpc: *mut opus_val16 = 0 as *mut opus_val16;
            let mut oldBandE: *mut opus_val16 = 0 as *mut opus_val16;
            let mut oldLogE: *mut opus_val16 = 0 as *mut opus_val16;
            let mut oldLogE2: *mut opus_val16 = 0 as *mut opus_val16;
            lpc = ((*st)._decode_mem)
                .as_mut_ptr()
                .offset(((2048 as libc::c_int + (*st).overlap) * (*st).channels) as isize)
                as *mut opus_val16;
            oldBandE = lpc.offset(((*st).channels * 24 as libc::c_int) as isize);
            oldLogE = oldBandE.offset((2 as libc::c_int * (*(*st).mode).nbEBands) as isize);
            oldLogE2 = oldLogE.offset((2 as libc::c_int * (*(*st).mode).nbEBands) as isize);
            memset(
                &mut (*st).rng as *mut opus_uint32 as *mut libc::c_char as *mut libc::c_void,
                0 as libc::c_int,
                ((opus_custom_decoder_get_size((*st).mode, (*st).channels) as libc::c_long
                    - (&mut (*st).rng as *mut opus_uint32 as *mut libc::c_char)
                        .offset_from(st as *mut libc::c_char) as libc::c_long)
                    as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
            i = 0 as libc::c_int;
            while i < 2 as libc::c_int * (*(*st).mode).nbEBands {
                let ref mut fresh2 = *oldLogE2.offset(i as isize);
                *fresh2 = -28.0f32;
                *oldLogE.offset(i as isize) = *fresh2;
                i += 1;
            }
            (*st).skip_plc = 1 as libc::c_int;
            current_block = 3689906465960840878;
        }
        4033 => {
            let mut value_4: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_4.is_null() {
                current_block = 7990025728955927862;
            } else {
                *value_4 = (*st).postfilter_period;
                current_block = 3689906465960840878;
            }
        }
        10015 => {
            let mut value_5: *mut *const OpusCustomMode = ap.arg::<*mut *const OpusCustomMode>();
            if value_5.is_null() {
                current_block = 7990025728955927862;
            } else {
                *value_5 = (*st).mode;
                current_block = 3689906465960840878;
            }
        }
        10016 => {
            let mut value_6: opus_int32 = ap.arg::<opus_int32>();
            (*st).signalling = value_6;
            current_block = 3689906465960840878;
        }
        4031 => {
            let mut value_7: *mut opus_uint32 = ap.arg::<*mut opus_uint32>();
            if value_7.is_null() {
                current_block = 7990025728955927862;
            } else {
                *value_7 = (*st).rng;
                current_block = 3689906465960840878;
            }
        }
        4046 => {
            let mut value_8: opus_int32 = ap.arg::<opus_int32>();
            if value_8 < 0 as libc::c_int || value_8 > 1 as libc::c_int {
                current_block = 7990025728955927862;
            } else {
                (*st).disable_inv = value_8;
                current_block = 3689906465960840878;
            }
        }
        4047 => {
            let mut value_9: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_9.is_null() {
                current_block = 7990025728955927862;
            } else {
                *value_9 = (*st).disable_inv;
                current_block = 3689906465960840878;
            }
        }
        _ => return -(5 as libc::c_int),
    }
    match current_block {
        3689906465960840878 => return 0 as libc::c_int,
        _ => return -(1 as libc::c_int),
    };
}
