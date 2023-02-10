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
    extern "C" {
        #[c2rust::src_loc = "63:1"]
        pub fn celt_fatal(
            str: *const libc::c_char,
            file: *const libc::c_char,
            line: libc::c_int,
        ) -> !;
    }
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/celt.h:40"]
pub mod celt_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "71:9"]
    pub struct SILKInfo {
        pub signalType: libc::c_int,
        pub offset: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "55:9"]
    pub struct AnalysisInfo {
        pub valid: libc::c_int,
        pub tonality: libc::c_float,
        pub tonality_slope: libc::c_float,
        pub noisiness: libc::c_float,
        pub activity: libc::c_float,
        pub music_prob: libc::c_float,
        pub music_prob_min: libc::c_float,
        pub music_prob_max: libc::c_float,
        pub bandwidth: libc::c_int,
        pub activity_probability: libc::c_float,
        pub max_pitch_ratio: libc::c_float,
        pub leak_boost: [libc::c_uchar; 19],
    }
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
    #[c2rust::src_loc = "167:28"]
    pub static mut spread_icdf: [libc::c_uchar; 4] = [
        25 as libc::c_int as libc::c_uchar,
        23 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ];
    #[c2rust::src_loc = "169:28"]
    pub static mut tapset_icdf: [libc::c_uchar; 3] = [
        2 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ];
    use super::arch_h::{opus_val16, opus_val32};
    use super::modes_h::OpusCustomMode;
    use super::opus_types_h::opus_int32;
    extern "C" {
        #[c2rust::src_loc = "210:26"]
        pub static tf_select_table: [[libc::c_schar; 8]; 4];
        #[c2rust::src_loc = "219:1"]
        pub fn resampling_factor(rate: opus_int32) -> libc::c_int;
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
        #[c2rust::src_loc = "238:1"]
        pub fn init_caps(
            m: *const OpusCustomMode,
            cap: *mut libc::c_int,
            LM: libc::c_int,
            C: libc::c_int,
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
    #[c2rust::src_loc = "47:1"]
    pub type ec_enc = ec_ctx;
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
#[c2rust::header_src = "/usr/include/stdlib.h:37"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "861:12"]
        pub fn abs(_: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/mathcalls.h:38"]
pub mod mathcalls_h {
    extern "C" {
        #[c2rust::src_loc = "95:17"]
        pub fn exp(_: libc::c_double) -> libc::c_double;
        #[c2rust::src_loc = "104:17"]
        pub fn log(_: libc::c_double) -> libc::c_double;
        #[c2rust::src_loc = "143:13"]
        pub fn sqrt(_: libc::c_double) -> libc::c_double;
        #[c2rust::src_loc = "162:14"]
        pub fn fabs(_: libc::c_double) -> libc::c_double;
        #[c2rust::src_loc = "165:14"]
        pub fn floor(_: libc::c_double) -> libc::c_double;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entenc.h:40"]
pub mod entenc_h {
    use super::entcode_h::ec_enc;
    use super::opus_types_h::opus_uint32;
    extern "C" {
        #[c2rust::src_loc = "36:1"]
        pub fn ec_enc_init(_this: *mut ec_enc, _buf: *mut libc::c_uchar, _size: opus_uint32);
        #[c2rust::src_loc = "56:1"]
        pub fn ec_enc_bit_logp(_this: *mut ec_enc, _val: libc::c_int, _logp: libc::c_uint);
        #[c2rust::src_loc = "65:1"]
        pub fn ec_enc_icdf(
            _this: *mut ec_enc,
            _s: libc::c_int,
            _icdf: *const libc::c_uchar,
            _ftb: libc::c_uint,
        );
        #[c2rust::src_loc = "71:1"]
        pub fn ec_enc_uint(_this: *mut ec_enc, _fl: opus_uint32, _ft: opus_uint32);
        #[c2rust::src_loc = "77:1"]
        pub fn ec_enc_bits(_this: *mut ec_enc, _fl: opus_uint32, _ftb: libc::c_uint);
        #[c2rust::src_loc = "103:1"]
        pub fn ec_enc_shrink(_this: *mut ec_enc, _size: opus_uint32);
        #[c2rust::src_loc = "108:1"]
        pub fn ec_enc_done(_this: *mut ec_enc);
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/pitch.h:41"]
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
    use super::arch_h::{celt_sig, opus_val16, opus_val32};
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
        #[c2rust::src_loc = "59:1"]
        pub fn remove_doubling(
            x: *mut opus_val16,
            maxperiod: libc::c_int,
            minperiod: libc::c_int,
            N: libc::c_int,
            T0: *mut libc::c_int,
            prev_period: libc::c_int,
            prev_gain: opus_val16,
            arch: libc::c_int,
        ) -> opus_val16;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/bands.h:42"]
pub mod bands_h {
    use super::arch_h::{celt_ener, celt_norm, celt_sig, opus_val16};
    use super::entcode_h::ec_ctx;
    use super::modes_h::OpusCustomMode;
    use super::opus_types_h::{opus_int32, opus_uint32};
    extern "C" {
        #[c2rust::src_loc = "121:1"]
        pub fn hysteresis_decision(
            val: opus_val16,
            thresholds: *const opus_val16,
            hysteresis: *const opus_val16,
            N: libc::c_int,
            prev: libc::c_int,
        ) -> libc::c_int;
        #[c2rust::src_loc = "81:1"]
        pub fn haar1(X: *mut celt_norm, N0: libc::c_int, stride: libc::c_int);
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
        #[c2rust::src_loc = "73:1"]
        pub fn spreading_decision(
            m: *const OpusCustomMode,
            X: *const celt_norm,
            average: *mut libc::c_int,
            last_decision: libc::c_int,
            hf_average: *mut libc::c_int,
            tapset_decision: *mut libc::c_int,
            update_hf: libc::c_int,
            end: libc::c_int,
            C: libc::c_int,
            M: libc::c_int,
            spread_weight: *const libc::c_int,
        ) -> libc::c_int;
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
        #[c2rust::src_loc = "57:1"]
        pub fn normalise_bands(
            m: *const OpusCustomMode,
            freq: *const celt_sig,
            X: *mut celt_norm,
            bandE: *const celt_ener,
            end: libc::c_int,
            C: libc::c_int,
            M: libc::c_int,
        );
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
    use super::arch_h::{celt_ener, opus_val16, opus_val32};
    use super::entcode_h::ec_enc;
    use super::modes_h::OpusCustomMode;
    use super::opus_types_h::opus_uint32;
    extern "C" {
        #[c2rust::src_loc = "50:1"]
        pub fn quant_coarse_energy(
            m: *const OpusCustomMode,
            start: libc::c_int,
            end: libc::c_int,
            effEnd: libc::c_int,
            eBands: *const opus_val16,
            oldEBands: *mut opus_val16,
            budget: opus_uint32,
            error: *mut opus_val16,
            enc: *mut ec_enc,
            C: libc::c_int,
            LM: libc::c_int,
            nbAvailableBytes: libc::c_int,
            force_intra: libc::c_int,
            delayedIntra: *mut opus_val32,
            two_pass: libc::c_int,
            loss_rate: libc::c_int,
            lfe: libc::c_int,
        );
        #[c2rust::src_loc = "41:25"]
        pub static eMeans: [opus_val16; 25];
        #[c2rust::src_loc = "44:1"]
        pub fn amp2Log2(
            m: *const OpusCustomMode,
            effEnd: libc::c_int,
            end: libc::c_int,
            bandE: *mut celt_ener,
            bandLogE: *mut opus_val16,
            C: libc::c_int,
        );
        #[c2rust::src_loc = "56:1"]
        pub fn quant_fine_energy(
            m: *const OpusCustomMode,
            start: libc::c_int,
            end: libc::c_int,
            oldEBands: *mut opus_val16,
            error: *mut opus_val16,
            fine_quant: *mut libc::c_int,
            enc: *mut ec_enc,
            C: libc::c_int,
        );
        #[c2rust::src_loc = "58:1"]
        pub fn quant_energy_finalise(
            m: *const OpusCustomMode,
            start: libc::c_int,
            end: libc::c_int,
            oldEBands: *mut opus_val16,
            error: *mut opus_val16,
            fine_quant: *mut libc::c_int,
            fine_priority: *mut libc::c_int,
            bits_left: libc::c_int,
            enc: *mut ec_enc,
            C: libc::c_int,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/mathops.h:45"]
pub mod mathops_h {
    #[inline]
    #[c2rust::src_loc = "80:1"]
    pub unsafe extern "C" fn celt_maxabs16(
        mut x: *const opus_val16,
        mut len: libc::c_int,
    ) -> opus_val32 {
        let mut i: libc::c_int = 0;
        let mut maxval: opus_val16 = 0 as libc::c_int as opus_val16;
        let mut minval: opus_val16 = 0 as libc::c_int as opus_val16;
        i = 0 as libc::c_int;
        while i < len {
            maxval = if maxval > *x.offset(i as isize) {
                maxval
            } else {
                *x.offset(i as isize)
            };
            minval = if minval < *x.offset(i as isize) {
                minval
            } else {
                *x.offset(i as isize)
            };
            i += 1;
        }
        return if maxval > -minval { maxval } else { -minval };
    }
    use super::arch_h::{opus_val16, opus_val32};
}
pub use self::arch_h::{celt_ener, celt_fatal, celt_norm, celt_sig, opus_val16, opus_val32};
use self::bands_h::{
    compute_band_energies, haar1, hysteresis_decision, normalise_bands, quant_all_bands,
    spreading_decision,
};
pub use self::celt_h::{
    comb_filter, init_caps, resampling_factor, spread_icdf, tapset_icdf, tf_select_table,
    trim_icdf, AnalysisInfo, SILKInfo,
};
pub use self::entcode_h::{ec_ctx, ec_enc, ec_get_error, ec_tell, ec_tell_frac, ec_window};
use self::entenc_h::{
    ec_enc_bit_logp, ec_enc_bits, ec_enc_done, ec_enc_icdf, ec_enc_init, ec_enc_shrink, ec_enc_uint,
};
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::kiss_fft_h::{arch_fft_state, kiss_fft_state, kiss_twiddle_cpx};
use self::mathcalls_h::{exp, fabs, floor, log, sqrt};
pub use self::mathops_h::celt_maxabs16;
pub use self::mdct_h::{clt_mdct_forward_c, mdct_lookup};
pub use self::modes_h::{OpusCustomMode, PulseCache};
use self::opus_custom_h::opus_custom_mode_create;
pub use self::opus_types_h::{opus_int16, opus_int32, opus_uint32};
pub use self::pitch_h::{celt_inner_prod_c, pitch_downsample, pitch_search, remove_doubling};
use self::quant_bands_h::{
    amp2Log2, eMeans, quant_coarse_energy, quant_energy_finalise, quant_fine_energy,
};
use self::rate_h::clt_compute_allocation;
pub use self::stdarg_h::va_list;
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::uint32_t;
use self::stdlib_h::abs;
use self::string_h::{memcpy, memmove, memset};
pub use self::types_h::{__int16_t, __int32_t, __uint32_t};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "58:8"]
pub struct OpusCustomEncoder {
    pub mode: *const OpusCustomMode,
    pub channels: libc::c_int,
    pub stream_channels: libc::c_int,
    pub force_intra: libc::c_int,
    pub clip: libc::c_int,
    pub disable_pf: libc::c_int,
    pub complexity: libc::c_int,
    pub upsample: libc::c_int,
    pub start: libc::c_int,
    pub end: libc::c_int,
    pub bitrate: opus_int32,
    pub vbr: libc::c_int,
    pub signalling: libc::c_int,
    pub constrained_vbr: libc::c_int,
    pub loss_rate: libc::c_int,
    pub lsb_depth: libc::c_int,
    pub lfe: libc::c_int,
    pub disable_inv: libc::c_int,
    pub arch: libc::c_int,
    pub rng: opus_uint32,
    pub spread_decision: libc::c_int,
    pub delayedIntra: opus_val32,
    pub tonal_average: libc::c_int,
    pub lastCodedBands: libc::c_int,
    pub hf_average: libc::c_int,
    pub tapset_decision: libc::c_int,
    pub prefilter_period: libc::c_int,
    pub prefilter_gain: opus_val16,
    pub prefilter_tapset: libc::c_int,
    pub consec_transient: libc::c_int,
    pub analysis: AnalysisInfo,
    pub silk_info: SILKInfo,
    pub preemph_memE: [opus_val32; 2],
    pub preemph_memD: [opus_val32; 2],
    pub vbr_reservoir: opus_int32,
    pub vbr_drift: opus_int32,
    pub vbr_offset: opus_int32,
    pub vbr_count: opus_int32,
    pub overlap_max: opus_val32,
    pub stereo_saving: opus_val16,
    pub intensity: libc::c_int,
    pub energy_mask: *mut opus_val16,
    pub spec_avg: opus_val16,
    pub in_mem: [celt_sig; 1],
}
#[no_mangle]
#[c2rust::src_loc = "130:1"]
pub unsafe extern "C" fn celt_encoder_get_size(mut channels: libc::c_int) -> libc::c_int {
    let mut mode: *mut OpusCustomMode = opus_custom_mode_create(
        48000 as libc::c_int,
        960 as libc::c_int,
        0 as *mut libc::c_int,
    );
    return opus_custom_encoder_get_size(mode, channels);
}
#[inline]
#[c2rust::src_loc = "136:1"]
unsafe extern "C" fn opus_custom_encoder_get_size(
    mut mode: *const OpusCustomMode,
    mut channels: libc::c_int,
) -> libc::c_int {
    let mut size: libc::c_int = (::core::mem::size_of::<OpusCustomEncoder>() as libc::c_ulong)
        .wrapping_add(
            ((channels * (*mode).overlap - 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<celt_sig>() as libc::c_ulong),
        )
        .wrapping_add(
            ((channels * 1024 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<celt_sig>() as libc::c_ulong),
        )
        .wrapping_add(
            ((4 as libc::c_int * channels * (*mode).nbEBands) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<opus_val16>() as libc::c_ulong),
        ) as libc::c_int;
    return size;
}
#[c2rust::src_loc = "166:1"]
unsafe extern "C" fn opus_custom_encoder_init_arch(
    mut st: *mut OpusCustomEncoder,
    mut mode: *const OpusCustomMode,
    mut channels: libc::c_int,
    mut arch: libc::c_int,
) -> libc::c_int {
    if channels < 0 as libc::c_int || channels > 2 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if st.is_null() || mode.is_null() {
        return -(7 as libc::c_int);
    }
    memset(
        st as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
        (opus_custom_encoder_get_size(mode, channels) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    (*st).mode = mode;
    (*st).channels = channels;
    (*st).stream_channels = (*st).channels;
    (*st).upsample = 1 as libc::c_int;
    (*st).start = 0 as libc::c_int;
    (*st).end = (*(*st).mode).effEBands;
    (*st).signalling = 1 as libc::c_int;
    (*st).arch = arch;
    (*st).constrained_vbr = 1 as libc::c_int;
    (*st).clip = 1 as libc::c_int;
    (*st).bitrate = -(1 as libc::c_int);
    (*st).vbr = 0 as libc::c_int;
    (*st).force_intra = 0 as libc::c_int;
    (*st).complexity = 5 as libc::c_int;
    (*st).lsb_depth = 24 as libc::c_int;
    opus_custom_encoder_ctl(st, 4028 as libc::c_int);
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "207:1"]
pub unsafe extern "C" fn celt_encoder_init(
    mut st: *mut OpusCustomEncoder,
    mut sampling_rate: opus_int32,
    mut channels: libc::c_int,
    mut arch: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = opus_custom_encoder_init_arch(
        st,
        opus_custom_mode_create(
            48000 as libc::c_int,
            960 as libc::c_int,
            0 as *mut libc::c_int,
        ),
        channels,
        arch,
    );
    if ret != 0 as libc::c_int {
        return ret;
    }
    (*st).upsample = resampling_factor(sampling_rate);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "227:1"]
unsafe extern "C" fn transient_analysis(
    mut in_0: *const opus_val32,
    mut len: libc::c_int,
    mut C: libc::c_int,
    mut tf_estimate: *mut opus_val16,
    mut tf_chan: *mut libc::c_int,
    mut allow_weak_transients: libc::c_int,
    mut weak_transient: *mut libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut mem0: opus_val32 = 0.;
    let mut mem1: opus_val32 = 0.;
    let mut is_transient: libc::c_int = 0 as libc::c_int;
    let mut mask_metric: opus_int32 = 0 as libc::c_int;
    let mut c: libc::c_int = 0;
    let mut tf_max: opus_val16 = 0.;
    let mut len2: libc::c_int = 0;
    let mut forward_decay: opus_val16 = 0.0625f32;
    static mut inv_table: [libc::c_uchar; 128] = [
        255 as libc::c_int as libc::c_uchar,
        255 as libc::c_int as libc::c_uchar,
        156 as libc::c_int as libc::c_uchar,
        110 as libc::c_int as libc::c_uchar,
        86 as libc::c_int as libc::c_uchar,
        70 as libc::c_int as libc::c_uchar,
        59 as libc::c_int as libc::c_uchar,
        51 as libc::c_int as libc::c_uchar,
        45 as libc::c_int as libc::c_uchar,
        40 as libc::c_int as libc::c_uchar,
        37 as libc::c_int as libc::c_uchar,
        33 as libc::c_int as libc::c_uchar,
        31 as libc::c_int as libc::c_uchar,
        28 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        25 as libc::c_int as libc::c_uchar,
        23 as libc::c_int as libc::c_uchar,
        22 as libc::c_int as libc::c_uchar,
        21 as libc::c_int as libc::c_uchar,
        20 as libc::c_int as libc::c_uchar,
        19 as libc::c_int as libc::c_uchar,
        18 as libc::c_int as libc::c_uchar,
        17 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
        15 as libc::c_int as libc::c_uchar,
        15 as libc::c_int as libc::c_uchar,
        14 as libc::c_int as libc::c_uchar,
        13 as libc::c_int as libc::c_uchar,
        13 as libc::c_int as libc::c_uchar,
        12 as libc::c_int as libc::c_uchar,
        12 as libc::c_int as libc::c_uchar,
        12 as libc::c_int as libc::c_uchar,
        12 as libc::c_int as libc::c_uchar,
        11 as libc::c_int as libc::c_uchar,
        11 as libc::c_int as libc::c_uchar,
        11 as libc::c_int as libc::c_uchar,
        10 as libc::c_int as libc::c_uchar,
        10 as libc::c_int as libc::c_uchar,
        10 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
    ];
    let vla = len as usize;
    let mut tmp: Vec<opus_val16> = ::std::vec::from_elem(0., vla);
    *weak_transient = 0 as libc::c_int;
    if allow_weak_transients != 0 {
        forward_decay = 0.03125f32;
    }
    len2 = len / 2 as libc::c_int;
    c = 0 as libc::c_int;
    while c < C {
        let mut mean: opus_val32 = 0.;
        let mut unmask: opus_int32 = 0 as libc::c_int;
        let mut norm: opus_val32 = 0.;
        let mut maxE: opus_val16 = 0.;
        mem0 = 0 as libc::c_int as opus_val32;
        mem1 = 0 as libc::c_int as opus_val32;
        i = 0 as libc::c_int;
        while i < len {
            let mut x: opus_val32 = 0.;
            let mut y: opus_val32 = 0.;
            x = *in_0.offset((i + c * len) as isize);
            y = mem0 + x;
            mem0 = mem1 + y - 2 as libc::c_int as libc::c_float * x;
            mem1 = x - 0.5f32 * y;
            *tmp.as_mut_ptr().offset(i as isize) = y;
            i += 1;
        }
        memset(
            tmp.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            (12 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<opus_val16>() as libc::c_ulong),
        );
        mean = 0 as libc::c_int as opus_val32;
        mem0 = 0 as libc::c_int as opus_val32;
        i = 0 as libc::c_int;
        while i < len2 {
            let mut x2: opus_val16 = *tmp.as_mut_ptr().offset((2 as libc::c_int * i) as isize)
                * *tmp.as_mut_ptr().offset((2 as libc::c_int * i) as isize)
                + *tmp
                    .as_mut_ptr()
                    .offset((2 as libc::c_int * i + 1 as libc::c_int) as isize)
                    * *tmp
                        .as_mut_ptr()
                        .offset((2 as libc::c_int * i + 1 as libc::c_int) as isize);
            mean += x2;
            *tmp.as_mut_ptr().offset(i as isize) = mem0 + forward_decay * (x2 - mem0);
            mem0 = *tmp.as_mut_ptr().offset(i as isize);
            i += 1;
        }
        mem0 = 0 as libc::c_int as opus_val32;
        maxE = 0 as libc::c_int as opus_val16;
        i = len2 - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            *tmp.as_mut_ptr().offset(i as isize) =
                mem0 + 0.125f32 * (*tmp.as_mut_ptr().offset(i as isize) - mem0);
            mem0 = *tmp.as_mut_ptr().offset(i as isize);
            maxE = if maxE > mem0 { maxE } else { mem0 };
            i -= 1;
        }
        mean = sqrt((mean * maxE) as libc::c_double * 0.5f64 * len2 as libc::c_double)
            as libc::c_float;
        norm = len2 as libc::c_float / (1e-15f32 + mean);
        unmask = 0 as libc::c_int;
        if *tmp.as_mut_ptr().offset(0 as libc::c_int as isize)
            != *tmp.as_mut_ptr().offset(0 as libc::c_int as isize)
        {
            celt_fatal(
                b"assertion failed: !celt_isnan(tmp[0])\0" as *const u8 as *const libc::c_char,
                b"celt/celt_encoder.c\0" as *const u8 as *const libc::c_char,
                369 as libc::c_int,
            );
        }
        if norm != norm {
            celt_fatal(
                b"assertion failed: !celt_isnan(norm)\0" as *const u8 as *const libc::c_char,
                b"celt/celt_encoder.c\0" as *const u8 as *const libc::c_char,
                370 as libc::c_int,
            );
        }
        i = 12 as libc::c_int;
        while i < len2 - 5 as libc::c_int {
            let mut id: libc::c_int = 0;
            id = (if 0 as libc::c_int as libc::c_double
                > (if (127 as libc::c_int as libc::c_double)
                    < floor(
                        (64 as libc::c_int as libc::c_float
                            * norm
                            * (*tmp.as_mut_ptr().offset(i as isize) + 1e-15f32))
                            as libc::c_double,
                    )
                {
                    127 as libc::c_int as libc::c_double
                } else {
                    floor(
                        (64 as libc::c_int as libc::c_float
                            * norm
                            * (*tmp.as_mut_ptr().offset(i as isize) + 1e-15f32))
                            as libc::c_double,
                    )
                }) {
                0 as libc::c_int as libc::c_double
            } else if (127 as libc::c_int as libc::c_double)
                < floor(
                    (64 as libc::c_int as libc::c_float
                        * norm
                        * (*tmp.as_mut_ptr().offset(i as isize) + 1e-15f32))
                        as libc::c_double,
                )
            {
                127 as libc::c_int as libc::c_double
            } else {
                floor(
                    (64 as libc::c_int as libc::c_float
                        * norm
                        * (*tmp.as_mut_ptr().offset(i as isize) + 1e-15f32))
                        as libc::c_double,
                )
            }) as libc::c_int;
            unmask += inv_table[id as usize] as libc::c_int;
            i += 4 as libc::c_int;
        }
        unmask = 64 as libc::c_int * unmask * 4 as libc::c_int
            / (6 as libc::c_int * (len2 - 17 as libc::c_int));
        if unmask > mask_metric {
            *tf_chan = c;
            mask_metric = unmask;
        }
        c += 1;
    }
    is_transient = (mask_metric > 200 as libc::c_int) as libc::c_int;
    if allow_weak_transients != 0 && is_transient != 0 && mask_metric < 600 as libc::c_int {
        is_transient = 0 as libc::c_int;
        *weak_transient = 1 as libc::c_int;
    }
    tf_max = if 0 as libc::c_int as libc::c_float
        > sqrt((27 as libc::c_int * mask_metric) as libc::c_double) as libc::c_float
            - 42 as libc::c_int as libc::c_float
    {
        0 as libc::c_int as libc::c_float
    } else {
        sqrt((27 as libc::c_int * mask_metric) as libc::c_double) as libc::c_float
            - 42 as libc::c_int as libc::c_float
    };
    *tf_estimate = sqrt(
        if 0 as libc::c_int as libc::c_double
            > (0.0069f64 as opus_val32
                * (if (163 as libc::c_int as libc::c_float) < tf_max {
                    163 as libc::c_int as libc::c_float
                } else {
                    tf_max
                })) as libc::c_double
                - 0.139f64
        {
            0 as libc::c_int as libc::c_double
        } else {
            (0.0069f64 as opus_val32
                * (if (163 as libc::c_int as libc::c_float) < tf_max {
                    163 as libc::c_int as libc::c_float
                } else {
                    tf_max
                })) as libc::c_double
                - 0.139f64
        },
    ) as libc::c_float;
    return is_transient;
}
#[c2rust::src_loc = "412:1"]
unsafe extern "C" fn patch_transient_decision(
    mut newE: *mut opus_val16,
    mut oldE: *mut opus_val16,
    mut nbEBands: libc::c_int,
    mut start: libc::c_int,
    mut end: libc::c_int,
    mut C: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut mean_diff: opus_val32 = 0 as libc::c_int as opus_val32;
    let mut spread_old: [opus_val16; 26] = [0.; 26];
    if C == 1 as libc::c_int {
        spread_old[start as usize] = *oldE.offset(start as isize);
        i = start + 1 as libc::c_int;
        while i < end {
            spread_old[i as usize] = if spread_old[(i - 1 as libc::c_int) as usize] - 1.0f32
                > *oldE.offset(i as isize)
            {
                spread_old[(i - 1 as libc::c_int) as usize] - 1.0f32
            } else {
                *oldE.offset(i as isize)
            };
            i += 1;
        }
    } else {
        spread_old[start as usize] =
            if *oldE.offset(start as isize) > *oldE.offset((start + nbEBands) as isize) {
                *oldE.offset(start as isize)
            } else {
                *oldE.offset((start + nbEBands) as isize)
            };
        i = start + 1 as libc::c_int;
        while i < end {
            spread_old[i as usize] = if spread_old[(i - 1 as libc::c_int) as usize] - 1.0f32
                > (if *oldE.offset(i as isize) > *oldE.offset((i + nbEBands) as isize) {
                    *oldE.offset(i as isize)
                } else {
                    *oldE.offset((i + nbEBands) as isize)
                }) {
                spread_old[(i - 1 as libc::c_int) as usize] - 1.0f32
            } else if *oldE.offset(i as isize) > *oldE.offset((i + nbEBands) as isize) {
                *oldE.offset(i as isize)
            } else {
                *oldE.offset((i + nbEBands) as isize)
            };
            i += 1;
        }
    }
    i = end - 2 as libc::c_int;
    while i >= start {
        spread_old[i as usize] =
            if spread_old[i as usize] > spread_old[(i + 1 as libc::c_int) as usize] - 1.0f32 {
                spread_old[i as usize]
            } else {
                spread_old[(i + 1 as libc::c_int) as usize] - 1.0f32
            };
        i -= 1;
    }
    c = 0 as libc::c_int;
    loop {
        i = if 2 as libc::c_int > start {
            2 as libc::c_int
        } else {
            start
        };
        while i < end - 1 as libc::c_int {
            let mut x1: opus_val16 = 0.;
            let mut x2: opus_val16 = 0.;
            x1 = if 0 as libc::c_int as libc::c_float > *newE.offset((i + c * nbEBands) as isize) {
                0 as libc::c_int as libc::c_float
            } else {
                *newE.offset((i + c * nbEBands) as isize)
            };
            x2 = if 0 as libc::c_int as libc::c_float > spread_old[i as usize] {
                0 as libc::c_int as libc::c_float
            } else {
                spread_old[i as usize]
            };
            mean_diff = mean_diff
                + (if 0 as libc::c_int as libc::c_float > x1 - x2 {
                    0 as libc::c_int as libc::c_float
                } else {
                    x1 - x2
                });
            i += 1;
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
    mean_diff = mean_diff
        / (C * (end
            - 1 as libc::c_int
            - (if 2 as libc::c_int > start {
                2 as libc::c_int
            } else {
                start
            }))) as opus_val32;
    return (mean_diff > 1.0f32) as libc::c_int;
}
#[c2rust::src_loc = "450:1"]
unsafe extern "C" fn compute_mdcts(
    mut mode: *const OpusCustomMode,
    mut shortBlocks: libc::c_int,
    mut in_0: *mut celt_sig,
    mut out: *mut celt_sig,
    mut C: libc::c_int,
    mut CC: libc::c_int,
    mut LM: libc::c_int,
    mut upsample: libc::c_int,
    mut arch: libc::c_int,
) {
    let overlap: libc::c_int = (*mode).overlap;
    let mut N: libc::c_int = 0;
    let mut B: libc::c_int = 0;
    let mut shift: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    if shortBlocks != 0 {
        B = shortBlocks;
        N = (*mode).shortMdctSize;
        shift = (*mode).maxLM;
    } else {
        B = 1 as libc::c_int;
        N = (*mode).shortMdctSize << LM;
        shift = (*mode).maxLM - LM;
    }
    c = 0 as libc::c_int;
    loop {
        b = 0 as libc::c_int;
        while b < B {
            clt_mdct_forward_c(
                &(*mode).mdct,
                in_0.offset((c * (B * N + overlap)) as isize)
                    .offset((b * N) as isize),
                &mut *out.offset((b + c * N * B) as isize),
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
    if CC == 2 as libc::c_int && C == 1 as libc::c_int {
        i = 0 as libc::c_int;
        while i < B * N {
            *out.offset(i as isize) =
                0.5f32 * *out.offset(i as isize) + 0.5f32 * *out.offset((B * N + i) as isize);
            i += 1;
        }
    }
    if upsample != 1 as libc::c_int {
        c = 0 as libc::c_int;
        loop {
            let mut bound: libc::c_int = B * N / upsample;
            i = 0 as libc::c_int;
            while i < bound {
                let ref mut fresh0 = *out.offset((c * B * N + i) as isize);
                *fresh0 *= upsample as libc::c_float;
                i += 1;
            }
            memset(
                &mut *out.offset((c * B * N + bound) as isize) as *mut celt_sig
                    as *mut libc::c_void,
                0 as libc::c_int,
                ((B * N - bound) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<celt_sig>() as libc::c_ulong),
            );
            c += 1;
            if !(c < C) {
                break;
            }
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "496:1"]
pub unsafe extern "C" fn celt_preemphasis(
    mut pcmp: *const opus_val16,
    mut inp: *mut celt_sig,
    mut N: libc::c_int,
    mut CC: libc::c_int,
    mut upsample: libc::c_int,
    mut coef: *const opus_val16,
    mut mem: *mut celt_sig,
    mut clip: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut coef0: opus_val16 = 0.;
    let mut m: celt_sig = 0.;
    let mut Nu: libc::c_int = 0;
    coef0 = *coef.offset(0 as libc::c_int as isize);
    m = *mem;
    if *coef.offset(1 as libc::c_int as isize) == 0 as libc::c_int as libc::c_float
        && upsample == 1 as libc::c_int
        && clip == 0
    {
        i = 0 as libc::c_int;
        while i < N {
            let mut x: opus_val16 = 0.;
            x = *pcmp.offset((CC * i) as isize) * 32768.0f32;
            *inp.offset(i as isize) = x - m;
            m = coef0 * x;
            i += 1;
        }
        *mem = m;
        return;
    }
    Nu = N / upsample;
    if upsample != 1 as libc::c_int {
        memset(
            inp as *mut libc::c_void,
            0 as libc::c_int,
            (N as libc::c_ulong).wrapping_mul(::core::mem::size_of::<celt_sig>() as libc::c_ulong),
        );
    }
    i = 0 as libc::c_int;
    while i < Nu {
        *inp.offset((i * upsample) as isize) = *pcmp.offset((CC * i) as isize) * 32768.0f32;
        i += 1;
    }
    if clip != 0 {
        i = 0 as libc::c_int;
        while i < Nu {
            *inp.offset((i * upsample) as isize) = if -65536.0f32
                > (if 65536.0f32 < *inp.offset((i * upsample) as isize) {
                    65536.0f32
                } else {
                    *inp.offset((i * upsample) as isize)
                }) {
                -65536.0f32
            } else if 65536.0f32 < *inp.offset((i * upsample) as isize) {
                65536.0f32
            } else {
                *inp.offset((i * upsample) as isize)
            };
            i += 1;
        }
    }
    i = 0 as libc::c_int;
    while i < N {
        let mut x_0: opus_val16 = 0.;
        x_0 = *inp.offset(i as isize);
        *inp.offset(i as isize) = x_0 - m;
        m = coef0 * x_0;
        i += 1;
    }
    *mem = m;
}
#[c2rust::src_loc = "571:1"]
unsafe extern "C" fn l1_metric(
    mut tmp: *const celt_norm,
    mut N: libc::c_int,
    mut LM: libc::c_int,
    mut bias: opus_val16,
) -> opus_val32 {
    let mut i: libc::c_int = 0;
    let mut L1: opus_val32 = 0.;
    L1 = 0 as libc::c_int as opus_val32;
    i = 0 as libc::c_int;
    while i < N {
        L1 += fabs(*tmp.offset(i as isize) as libc::c_double) as libc::c_float;
        i += 1;
    }
    L1 = L1 + LM as libc::c_float * bias * L1;
    return L1;
}
#[c2rust::src_loc = "584:1"]
unsafe extern "C" fn tf_analysis(
    mut m: *const OpusCustomMode,
    mut len: libc::c_int,
    mut isTransient: libc::c_int,
    mut tf_res: *mut libc::c_int,
    mut lambda: libc::c_int,
    mut X: *mut celt_norm,
    mut N0: libc::c_int,
    mut LM: libc::c_int,
    mut tf_estimate: opus_val16,
    mut tf_chan: libc::c_int,
    mut importance: *mut libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut cost0: libc::c_int = 0;
    let mut cost1: libc::c_int = 0;
    let mut sel: libc::c_int = 0;
    let mut selcost: [libc::c_int; 2] = [0; 2];
    let mut tf_select: libc::c_int = 0 as libc::c_int;
    let mut bias: opus_val16 = 0.;
    bias = 0.04f32
        * (if -0.25f32 > 0.5f32 - tf_estimate {
            -0.25f32
        } else {
            0.5f32 - tf_estimate
        });
    let vla = len as usize;
    let mut metric: Vec<libc::c_int> = ::std::vec::from_elem(0, vla);
    let vla_0 = ((*((*m).eBands).offset(len as isize) as libc::c_int
        - *((*m).eBands).offset((len - 1 as libc::c_int) as isize) as libc::c_int)
        << LM) as usize;
    let mut tmp: Vec<celt_norm> = ::std::vec::from_elem(0., vla_0);
    let vla_1 = ((*((*m).eBands).offset(len as isize) as libc::c_int
        - *((*m).eBands).offset((len - 1 as libc::c_int) as isize) as libc::c_int)
        << LM) as usize;
    let mut tmp_1: Vec<celt_norm> = ::std::vec::from_elem(0., vla_1);
    let vla_2 = len as usize;
    let mut path0: Vec<libc::c_int> = ::std::vec::from_elem(0, vla_2);
    let vla_3 = len as usize;
    let mut path1: Vec<libc::c_int> = ::std::vec::from_elem(0, vla_3);
    i = 0 as libc::c_int;
    while i < len {
        let mut k: libc::c_int = 0;
        let mut N: libc::c_int = 0;
        let mut narrow: libc::c_int = 0;
        let mut L1: opus_val32 = 0.;
        let mut best_L1: opus_val32 = 0.;
        let mut best_level: libc::c_int = 0 as libc::c_int;
        N = (*((*m).eBands).offset((i + 1 as libc::c_int) as isize) as libc::c_int
            - *((*m).eBands).offset(i as isize) as libc::c_int)
            << LM;
        narrow = (*((*m).eBands).offset((i + 1 as libc::c_int) as isize) as libc::c_int
            - *((*m).eBands).offset(i as isize) as libc::c_int
            == 1 as libc::c_int) as libc::c_int;
        memcpy(
            tmp.as_mut_ptr() as *mut libc::c_void,
            &mut *X.offset(
                (tf_chan * N0 + ((*((*m).eBands).offset(i as isize) as libc::c_int) << LM))
                    as isize,
            ) as *mut celt_norm as *const libc::c_void,
            (N as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<celt_norm>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * tmp.as_mut_ptr().offset_from(&mut *X.offset(
                            (tf_chan * N0
                                + ((*((*m).eBands).offset(i as isize) as libc::c_int) << LM))
                                as isize,
                        )) as libc::c_long) as libc::c_ulong,
                ),
        );
        L1 = l1_metric(
            tmp.as_mut_ptr(),
            N,
            if isTransient != 0 {
                LM
            } else {
                0 as libc::c_int
            },
            bias,
        );
        best_L1 = L1;
        if isTransient != 0 && narrow == 0 {
            memcpy(
                tmp_1.as_mut_ptr() as *mut libc::c_void,
                tmp.as_mut_ptr() as *const libc::c_void,
                (N as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<celt_norm>() as libc::c_ulong)
                    .wrapping_add(
                        (0 as libc::c_int as libc::c_long
                            * tmp_1.as_mut_ptr().offset_from(tmp.as_mut_ptr()) as libc::c_long)
                            as libc::c_ulong,
                    ),
            );
            haar1(tmp_1.as_mut_ptr(), N >> LM, (1 as libc::c_int) << LM);
            L1 = l1_metric(tmp_1.as_mut_ptr(), N, LM + 1 as libc::c_int, bias);
            if L1 < best_L1 {
                best_L1 = L1;
                best_level = -(1 as libc::c_int);
            }
        }
        k = 0 as libc::c_int;
        while k < LM + !(isTransient != 0 || narrow != 0) as libc::c_int {
            let mut B: libc::c_int = 0;
            if isTransient != 0 {
                B = LM - k - 1 as libc::c_int;
            } else {
                B = k + 1 as libc::c_int;
            }
            haar1(tmp.as_mut_ptr(), N >> k, (1 as libc::c_int) << k);
            L1 = l1_metric(tmp.as_mut_ptr(), N, B, bias);
            if L1 < best_L1 {
                best_L1 = L1;
                best_level = k + 1 as libc::c_int;
            }
            k += 1;
        }
        if isTransient != 0 {
            *metric.as_mut_ptr().offset(i as isize) = 2 as libc::c_int * best_level;
        } else {
            *metric.as_mut_ptr().offset(i as isize) = -(2 as libc::c_int) * best_level;
        }
        if narrow != 0
            && (*metric.as_mut_ptr().offset(i as isize) == 0 as libc::c_int
                || *metric.as_mut_ptr().offset(i as isize) == -(2 as libc::c_int) * LM)
        {
            *metric.as_mut_ptr().offset(i as isize) -= 1 as libc::c_int;
        }
        i += 1;
    }
    tf_select = 0 as libc::c_int;
    sel = 0 as libc::c_int;
    while sel < 2 as libc::c_int {
        cost0 = *importance.offset(0 as libc::c_int as isize)
            * abs(*metric.as_mut_ptr().offset(0 as libc::c_int as isize)
                - 2 as libc::c_int
                    * tf_select_table[LM as usize][(4 as libc::c_int * isTransient
                        + 2 as libc::c_int * sel
                        + 0 as libc::c_int)
                        as usize] as libc::c_int);
        cost1 = *importance.offset(0 as libc::c_int as isize)
            * abs(*metric.as_mut_ptr().offset(0 as libc::c_int as isize)
                - 2 as libc::c_int
                    * tf_select_table[LM as usize][(4 as libc::c_int * isTransient
                        + 2 as libc::c_int * sel
                        + 1 as libc::c_int)
                        as usize] as libc::c_int)
            + (if isTransient != 0 {
                0 as libc::c_int
            } else {
                lambda
            });
        i = 1 as libc::c_int;
        while i < len {
            let mut curr0: libc::c_int = 0;
            let mut curr1: libc::c_int = 0;
            curr0 = if cost0 < cost1 + lambda {
                cost0
            } else {
                cost1 + lambda
            };
            curr1 = if cost0 + lambda < cost1 {
                cost0 + lambda
            } else {
                cost1
            };
            cost0 = curr0
                + *importance.offset(i as isize)
                    * abs(*metric.as_mut_ptr().offset(i as isize)
                        - 2 as libc::c_int
                            * tf_select_table[LM as usize][(4 as libc::c_int * isTransient
                                + 2 as libc::c_int * sel
                                + 0 as libc::c_int)
                                as usize] as libc::c_int);
            cost1 = curr1
                + *importance.offset(i as isize)
                    * abs(*metric.as_mut_ptr().offset(i as isize)
                        - 2 as libc::c_int
                            * tf_select_table[LM as usize][(4 as libc::c_int * isTransient
                                + 2 as libc::c_int * sel
                                + 1 as libc::c_int)
                                as usize] as libc::c_int);
            i += 1;
        }
        cost0 = if cost0 < cost1 { cost0 } else { cost1 };
        selcost[sel as usize] = cost0;
        sel += 1;
    }
    if selcost[1 as libc::c_int as usize] < selcost[0 as libc::c_int as usize] && isTransient != 0 {
        tf_select = 1 as libc::c_int;
    }
    cost0 = *importance.offset(0 as libc::c_int as isize)
        * abs(*metric.as_mut_ptr().offset(0 as libc::c_int as isize)
            - 2 as libc::c_int
                * tf_select_table[LM as usize][(4 as libc::c_int * isTransient
                    + 2 as libc::c_int * tf_select
                    + 0 as libc::c_int) as usize] as libc::c_int);
    cost1 = *importance.offset(0 as libc::c_int as isize)
        * abs(*metric.as_mut_ptr().offset(0 as libc::c_int as isize)
            - 2 as libc::c_int
                * tf_select_table[LM as usize][(4 as libc::c_int * isTransient
                    + 2 as libc::c_int * tf_select
                    + 1 as libc::c_int) as usize] as libc::c_int)
        + (if isTransient != 0 {
            0 as libc::c_int
        } else {
            lambda
        });
    i = 1 as libc::c_int;
    while i < len {
        let mut curr0_0: libc::c_int = 0;
        let mut curr1_0: libc::c_int = 0;
        let mut from0: libc::c_int = 0;
        let mut from1: libc::c_int = 0;
        from0 = cost0;
        from1 = cost1 + lambda;
        if from0 < from1 {
            curr0_0 = from0;
            *path0.as_mut_ptr().offset(i as isize) = 0 as libc::c_int;
        } else {
            curr0_0 = from1;
            *path0.as_mut_ptr().offset(i as isize) = 1 as libc::c_int;
        }
        from0 = cost0 + lambda;
        from1 = cost1;
        if from0 < from1 {
            curr1_0 = from0;
            *path1.as_mut_ptr().offset(i as isize) = 0 as libc::c_int;
        } else {
            curr1_0 = from1;
            *path1.as_mut_ptr().offset(i as isize) = 1 as libc::c_int;
        }
        cost0 = curr0_0
            + *importance.offset(i as isize)
                * abs(*metric.as_mut_ptr().offset(i as isize)
                    - 2 as libc::c_int
                        * tf_select_table[LM as usize][(4 as libc::c_int * isTransient
                            + 2 as libc::c_int * tf_select
                            + 0 as libc::c_int)
                            as usize] as libc::c_int);
        cost1 = curr1_0
            + *importance.offset(i as isize)
                * abs(*metric.as_mut_ptr().offset(i as isize)
                    - 2 as libc::c_int
                        * tf_select_table[LM as usize][(4 as libc::c_int * isTransient
                            + 2 as libc::c_int * tf_select
                            + 1 as libc::c_int)
                            as usize] as libc::c_int);
        i += 1;
    }
    *tf_res.offset((len - 1 as libc::c_int) as isize) = if cost0 < cost1 {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
    i = len - 2 as libc::c_int;
    while i >= 0 as libc::c_int {
        if *tf_res.offset((i + 1 as libc::c_int) as isize) == 1 as libc::c_int {
            *tf_res.offset(i as isize) =
                *path1.as_mut_ptr().offset((i + 1 as libc::c_int) as isize);
        } else {
            *tf_res.offset(i as isize) =
                *path0.as_mut_ptr().offset((i + 1 as libc::c_int) as isize);
        }
        i -= 1;
    }
    return tf_select;
}
#[c2rust::src_loc = "745:1"]
unsafe extern "C" fn tf_encode(
    mut start: libc::c_int,
    mut end: libc::c_int,
    mut isTransient: libc::c_int,
    mut tf_res: *mut libc::c_int,
    mut LM: libc::c_int,
    mut tf_select: libc::c_int,
    mut enc: *mut ec_enc,
) {
    let mut curr: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut tf_select_rsv: libc::c_int = 0;
    let mut tf_changed: libc::c_int = 0;
    let mut logp: libc::c_int = 0;
    let mut budget: opus_uint32 = 0;
    let mut tell: opus_uint32 = 0;
    budget = ((*enc).storage).wrapping_mul(8 as libc::c_int as libc::c_uint);
    tell = ec_tell(enc) as opus_uint32;
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
    tf_changed = 0 as libc::c_int;
    curr = tf_changed;
    i = start;
    while i < end {
        if tell.wrapping_add(logp as libc::c_uint) <= budget {
            ec_enc_bit_logp(enc, *tf_res.offset(i as isize) ^ curr, logp as libc::c_uint);
            tell = ec_tell(enc) as opus_uint32;
            curr = *tf_res.offset(i as isize);
            tf_changed |= curr;
        } else {
            *tf_res.offset(i as isize) = curr;
        }
        logp = if isTransient != 0 {
            4 as libc::c_int
        } else {
            5 as libc::c_int
        };
        i += 1;
    }
    if tf_select_rsv != 0
        && tf_select_table[LM as usize]
            [(4 as libc::c_int * isTransient + 0 as libc::c_int + tf_changed) as usize]
            as libc::c_int
            != tf_select_table[LM as usize]
                [(4 as libc::c_int * isTransient + 2 as libc::c_int + tf_changed) as usize]
                as libc::c_int
    {
        ec_enc_bit_logp(enc, tf_select, 1 as libc::c_int as libc::c_uint);
    } else {
        tf_select = 0 as libc::c_int;
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
#[c2rust::src_loc = "786:1"]
unsafe extern "C" fn alloc_trim_analysis(
    mut m: *const OpusCustomMode,
    mut X: *const celt_norm,
    mut bandLogE: *const opus_val16,
    mut end: libc::c_int,
    mut LM: libc::c_int,
    mut C: libc::c_int,
    mut N0: libc::c_int,
    mut analysis: *mut AnalysisInfo,
    mut stereo_saving: *mut opus_val16,
    mut tf_estimate: opus_val16,
    mut intensity: libc::c_int,
    mut surround_trim: opus_val16,
    mut equiv_rate: opus_int32,
    mut arch: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut diff: opus_val32 = 0 as libc::c_int as opus_val32;
    let mut c: libc::c_int = 0;
    let mut trim_index: libc::c_int = 0;
    let mut trim: opus_val16 = 5.0f32;
    let mut logXC: opus_val16 = 0.;
    let mut logXC2: opus_val16 = 0.;
    if equiv_rate < 64000 as libc::c_int {
        trim = 4.0f32;
    } else if equiv_rate < 80000 as libc::c_int {
        let mut frac: opus_int32 = equiv_rate - 64000 as libc::c_int >> 10 as libc::c_int;
        trim = 4.0f32 + 1.0f32 / 16.0f32 * frac as libc::c_float;
    }
    if C == 2 as libc::c_int {
        let mut sum: opus_val16 = 0 as libc::c_int as opus_val16;
        let mut minXC: opus_val16 = 0.;
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            let mut partial: opus_val32 = 0.;
            partial = celt_inner_prod_c(
                &*X.offset(((*((*m).eBands).offset(i as isize) as libc::c_int) << LM) as isize),
                &*X.offset(
                    (N0 + ((*((*m).eBands).offset(i as isize) as libc::c_int) << LM)) as isize,
                ),
                (*((*m).eBands).offset((i + 1 as libc::c_int) as isize) as libc::c_int
                    - *((*m).eBands).offset(i as isize) as libc::c_int)
                    << LM,
            );
            sum = sum + partial;
            i += 1;
        }
        sum = 1.0f32 / 8 as libc::c_int as libc::c_float * sum;
        sum = if 1.0f32 < fabs(sum as libc::c_double) as libc::c_float {
            1.0f32
        } else {
            fabs(sum as libc::c_double) as libc::c_float
        };
        minXC = sum;
        i = 8 as libc::c_int;
        while i < intensity {
            let mut partial_0: opus_val32 = 0.;
            partial_0 = celt_inner_prod_c(
                &*X.offset(((*((*m).eBands).offset(i as isize) as libc::c_int) << LM) as isize),
                &*X.offset(
                    (N0 + ((*((*m).eBands).offset(i as isize) as libc::c_int) << LM)) as isize,
                ),
                (*((*m).eBands).offset((i + 1 as libc::c_int) as isize) as libc::c_int
                    - *((*m).eBands).offset(i as isize) as libc::c_int)
                    << LM,
            );
            minXC = if minXC < fabs(partial_0 as libc::c_double) as libc::c_float {
                minXC
            } else {
                fabs(partial_0 as libc::c_double) as libc::c_float
            };
            i += 1;
        }
        minXC = if 1.0f32 < fabs(minXC as libc::c_double) as libc::c_float {
            1.0f32
        } else {
            fabs(minXC as libc::c_double) as libc::c_float
        };
        logXC = (1.442695040888963387f64 * log((1.001f32 - sum * sum) as libc::c_double))
            as libc::c_float;
        logXC2 = if 0.5f32 * logXC
            > (1.442695040888963387f64 * log((1.001f32 - minXC * minXC) as libc::c_double))
                as libc::c_float
        {
            0.5f32 * logXC
        } else {
            (1.442695040888963387f64 * log((1.001f32 - minXC * minXC) as libc::c_double))
                as libc::c_float
        };
        trim += if -4.0f32 > 0.75f32 * logXC {
            -4.0f32
        } else {
            0.75f32 * logXC
        };
        *stereo_saving = if *stereo_saving + 0.25f32 < -(0.5f32 * logXC2) {
            *stereo_saving + 0.25f32
        } else {
            -(0.5f32 * logXC2)
        };
    }
    c = 0 as libc::c_int;
    loop {
        i = 0 as libc::c_int;
        while i < end - 1 as libc::c_int {
            diff += *bandLogE.offset((i + c * (*m).nbEBands) as isize)
                * (2 as libc::c_int + 2 as libc::c_int * i - end) as libc::c_float;
            i += 1;
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
    diff /= (C * (end - 1 as libc::c_int)) as libc::c_float;
    trim -= if -2.0f32
        > (if 2.0f32 < (diff + 1.0f32) / 6 as libc::c_int as libc::c_float {
            2.0f32
        } else {
            (diff + 1.0f32) / 6 as libc::c_int as libc::c_float
        }) {
        -2.0f32
    } else if 2.0f32 < (diff + 1.0f32) / 6 as libc::c_int as libc::c_float {
        2.0f32
    } else {
        (diff + 1.0f32) / 6 as libc::c_int as libc::c_float
    };
    trim -= surround_trim;
    trim -= 2 as libc::c_int as libc::c_float * tf_estimate;
    if (*analysis).valid != 0 {
        trim -= if -2.0f32
            > (if 2.0f32 < 2.0f32 * ((*analysis).tonality_slope + 0.05f32) {
                2.0f32
            } else {
                2.0f32 * ((*analysis).tonality_slope + 0.05f32)
            }) {
            -2.0f32
        } else if 2.0f32 < 2.0f32 * ((*analysis).tonality_slope + 0.05f32) {
            2.0f32
        } else {
            2.0f32 * ((*analysis).tonality_slope + 0.05f32)
        };
    }
    trim_index = floor((0.5f32 + trim) as libc::c_double) as libc::c_int;
    trim_index = if 0 as libc::c_int
        > (if (10 as libc::c_int) < trim_index {
            10 as libc::c_int
        } else {
            trim_index
        }) {
        0 as libc::c_int
    } else if (10 as libc::c_int) < trim_index {
        10 as libc::c_int
    } else {
        trim_index
    };
    return trim_index;
}
#[c2rust::src_loc = "878:1"]
unsafe extern "C" fn stereo_analysis(
    mut m: *const OpusCustomMode,
    mut X: *const celt_norm,
    mut LM: libc::c_int,
    mut N0: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut thetas: libc::c_int = 0;
    let mut sumLR: opus_val32 = 1e-15f32;
    let mut sumMS: opus_val32 = 1e-15f32;
    i = 0 as libc::c_int;
    while i < 13 as libc::c_int {
        let mut j: libc::c_int = 0;
        j = (*((*m).eBands).offset(i as isize) as libc::c_int) << LM;
        while j < (*((*m).eBands).offset((i + 1 as libc::c_int) as isize) as libc::c_int) << LM {
            let mut L: opus_val32 = 0.;
            let mut R: opus_val32 = 0.;
            let mut M: opus_val32 = 0.;
            let mut S: opus_val32 = 0.;
            L = *X.offset(j as isize);
            R = *X.offset((N0 + j) as isize);
            M = L + R;
            S = L - R;
            sumLR = sumLR
                + (fabs(L as libc::c_double) as libc::c_float
                    + fabs(R as libc::c_double) as libc::c_float);
            sumMS = sumMS
                + (fabs(M as libc::c_double) as libc::c_float
                    + fabs(S as libc::c_double) as libc::c_float);
            j += 1;
        }
        i += 1;
    }
    sumMS = 0.707107f32 * sumMS;
    thetas = 13 as libc::c_int;
    if LM <= 1 as libc::c_int {
        thetas -= 8 as libc::c_int;
    }
    return ((((*((*m).eBands).offset(13 as libc::c_int as isize) as libc::c_int)
        << LM + 1 as libc::c_int)
        + thetas) as libc::c_float
        * sumMS
        > ((*((*m).eBands).offset(13 as libc::c_int as isize) as libc::c_int)
            << LM + 1 as libc::c_int) as libc::c_float
            * sumLR) as libc::c_int;
}
#[c2rust::src_loc = "911:1"]
unsafe extern "C" fn median_of_5(mut x: *const opus_val16) -> opus_val16 {
    let mut t0: opus_val16 = 0.;
    let mut t1: opus_val16 = 0.;
    let mut t2: opus_val16 = 0.;
    let mut t3: opus_val16 = 0.;
    let mut t4: opus_val16 = 0.;
    t2 = *x.offset(2 as libc::c_int as isize);
    if *x.offset(0 as libc::c_int as isize) > *x.offset(1 as libc::c_int as isize) {
        t0 = *x.offset(1 as libc::c_int as isize);
        t1 = *x.offset(0 as libc::c_int as isize);
    } else {
        t0 = *x.offset(0 as libc::c_int as isize);
        t1 = *x.offset(1 as libc::c_int as isize);
    }
    if *x.offset(3 as libc::c_int as isize) > *x.offset(4 as libc::c_int as isize) {
        t3 = *x.offset(4 as libc::c_int as isize);
        t4 = *x.offset(3 as libc::c_int as isize);
    } else {
        t3 = *x.offset(3 as libc::c_int as isize);
        t4 = *x.offset(4 as libc::c_int as isize);
    }
    if t0 > t3 {
        let mut tmp: opus_val16 = t0;
        t0 = t3;
        t3 = tmp;
        let mut tmp_0: opus_val16 = t1;
        t1 = t4;
        t4 = tmp_0;
    }
    if t2 > t1 {
        if t1 < t3 {
            return if t2 < t3 { t2 } else { t3 };
        } else {
            return if t4 < t1 { t4 } else { t1 };
        }
    } else if t2 < t3 {
        return if t1 < t3 { t1 } else { t3 };
    } else {
        return if t2 < t4 { t2 } else { t4 };
    };
}
#[c2rust::src_loc = "950:1"]
unsafe extern "C" fn median_of_3(mut x: *const opus_val16) -> opus_val16 {
    let mut t0: opus_val16 = 0.;
    let mut t1: opus_val16 = 0.;
    let mut t2: opus_val16 = 0.;
    if *x.offset(0 as libc::c_int as isize) > *x.offset(1 as libc::c_int as isize) {
        t0 = *x.offset(1 as libc::c_int as isize);
        t1 = *x.offset(0 as libc::c_int as isize);
    } else {
        t0 = *x.offset(0 as libc::c_int as isize);
        t1 = *x.offset(1 as libc::c_int as isize);
    }
    t2 = *x.offset(2 as libc::c_int as isize);
    if t1 < t2 {
        return t1;
    } else if t0 < t2 {
        return t2;
    } else {
        return t0;
    };
}
#[c2rust::src_loc = "970:1"]
unsafe extern "C" fn dynalloc_analysis(
    mut bandLogE: *const opus_val16,
    mut bandLogE2: *const opus_val16,
    mut nbEBands: libc::c_int,
    mut start: libc::c_int,
    mut end: libc::c_int,
    mut C: libc::c_int,
    mut offsets: *mut libc::c_int,
    mut lsb_depth: libc::c_int,
    mut logN: *const opus_int16,
    mut isTransient: libc::c_int,
    mut vbr: libc::c_int,
    mut constrained_vbr: libc::c_int,
    mut eBands: *const opus_int16,
    mut LM: libc::c_int,
    mut effectiveBytes: libc::c_int,
    mut tot_boost_: *mut opus_int32,
    mut lfe: libc::c_int,
    mut surround_dynalloc: *mut opus_val16,
    mut analysis: *mut AnalysisInfo,
    mut importance: *mut libc::c_int,
    mut spread_weight: *mut libc::c_int,
) -> opus_val16 {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut tot_boost: opus_int32 = 0 as libc::c_int;
    let mut maxDepth: opus_val16 = 0.;
    let vla = (C * nbEBands) as usize;
    let mut follower: Vec<opus_val16> = ::std::vec::from_elem(0., vla);
    let vla_0 = (C * nbEBands) as usize;
    let mut noise_floor: Vec<opus_val16> = ::std::vec::from_elem(0., vla_0);
    memset(
        offsets as *mut libc::c_void,
        0 as libc::c_int,
        (nbEBands as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    maxDepth = -31.9f32;
    i = 0 as libc::c_int;
    while i < end {
        *noise_floor.as_mut_ptr().offset(i as isize) = 0.0625f32
            * *logN.offset(i as isize) as opus_val32
            + 0.5f32
            + (9 as libc::c_int - lsb_depth) as libc::c_float
            - eMeans[i as usize]
            + 0.0062f64 as opus_val32
                * ((i + 5 as libc::c_int) * (i + 5 as libc::c_int)) as opus_val32;
        i += 1;
    }
    c = 0 as libc::c_int;
    loop {
        i = 0 as libc::c_int;
        while i < end {
            maxDepth = if maxDepth
                > *bandLogE.offset((c * nbEBands + i) as isize)
                    - *noise_floor.as_mut_ptr().offset(i as isize)
            {
                maxDepth
            } else {
                *bandLogE.offset((c * nbEBands + i) as isize)
                    - *noise_floor.as_mut_ptr().offset(i as isize)
            };
            i += 1;
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
    let vla_1 = nbEBands as usize;
    let mut mask: Vec<opus_val16> = ::std::vec::from_elem(0., vla_1);
    let vla_2 = nbEBands as usize;
    let mut sig: Vec<opus_val16> = ::std::vec::from_elem(0., vla_2);
    i = 0 as libc::c_int;
    while i < end {
        *mask.as_mut_ptr().offset(i as isize) =
            *bandLogE.offset(i as isize) - *noise_floor.as_mut_ptr().offset(i as isize);
        i += 1;
    }
    if C == 2 as libc::c_int {
        i = 0 as libc::c_int;
        while i < end {
            *mask.as_mut_ptr().offset(i as isize) = if *mask.as_mut_ptr().offset(i as isize)
                > *bandLogE.offset((nbEBands + i) as isize)
                    - *noise_floor.as_mut_ptr().offset(i as isize)
            {
                *mask.as_mut_ptr().offset(i as isize)
            } else {
                *bandLogE.offset((nbEBands + i) as isize)
                    - *noise_floor.as_mut_ptr().offset(i as isize)
            };
            i += 1;
        }
    }
    memcpy(
        sig.as_mut_ptr() as *mut libc::c_void,
        mask.as_mut_ptr() as *const libc::c_void,
        (end as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<opus_val16>() as libc::c_ulong)
            .wrapping_add(
                (0 as libc::c_int as libc::c_long
                    * sig.as_mut_ptr().offset_from(mask.as_mut_ptr()) as libc::c_long)
                    as libc::c_ulong,
            ),
    );
    i = 1 as libc::c_int;
    while i < end {
        *mask.as_mut_ptr().offset(i as isize) = if *mask.as_mut_ptr().offset(i as isize)
            > *mask.as_mut_ptr().offset((i - 1 as libc::c_int) as isize) - 2.0f32
        {
            *mask.as_mut_ptr().offset(i as isize)
        } else {
            *mask.as_mut_ptr().offset((i - 1 as libc::c_int) as isize) - 2.0f32
        };
        i += 1;
    }
    i = end - 2 as libc::c_int;
    while i >= 0 as libc::c_int {
        *mask.as_mut_ptr().offset(i as isize) = if *mask.as_mut_ptr().offset(i as isize)
            > *mask.as_mut_ptr().offset((i + 1 as libc::c_int) as isize) - 3.0f32
        {
            *mask.as_mut_ptr().offset(i as isize)
        } else {
            *mask.as_mut_ptr().offset((i + 1 as libc::c_int) as isize) - 3.0f32
        };
        i -= 1;
    }
    i = 0 as libc::c_int;
    while i < end {
        let mut smr: opus_val16 = *sig.as_mut_ptr().offset(i as isize)
            - (if (if 0 as libc::c_int as libc::c_float > maxDepth - 12.0f32 {
                0 as libc::c_int as libc::c_float
            } else {
                maxDepth - 12.0f32
            }) > *mask.as_mut_ptr().offset(i as isize)
            {
                (if 0 as libc::c_int as libc::c_float > maxDepth - 12.0f32 {
                    0 as libc::c_int as libc::c_float
                } else {
                    maxDepth - 12.0f32
                })
            } else {
                *mask.as_mut_ptr().offset(i as isize)
            });
        let mut shift: libc::c_int = if (5 as libc::c_int)
            < (if 0 as libc::c_int > -(floor((0.5f32 + smr) as libc::c_double) as libc::c_int) {
                0 as libc::c_int
            } else {
                -(floor((0.5f32 + smr) as libc::c_double) as libc::c_int)
            }) {
            5 as libc::c_int
        } else if 0 as libc::c_int > -(floor((0.5f32 + smr) as libc::c_double) as libc::c_int) {
            0 as libc::c_int
        } else {
            -(floor((0.5f32 + smr) as libc::c_double) as libc::c_int)
        };
        *spread_weight.offset(i as isize) = 32 as libc::c_int >> shift;
        i += 1;
    }
    if effectiveBytes > 50 as libc::c_int && LM >= 1 as libc::c_int && lfe == 0 {
        let mut last: libc::c_int = 0 as libc::c_int;
        c = 0 as libc::c_int;
        loop {
            let mut offset: opus_val16 = 0.;
            let mut tmp: opus_val16 = 0.;
            let mut f: *mut opus_val16 = 0 as *mut opus_val16;
            f = &mut *follower.as_mut_ptr().offset((c * nbEBands) as isize) as *mut opus_val16;
            *f.offset(0 as libc::c_int as isize) = *bandLogE2.offset((c * nbEBands) as isize);
            i = 1 as libc::c_int;
            while i < end {
                if *bandLogE2.offset((c * nbEBands + i) as isize)
                    > *bandLogE2.offset((c * nbEBands + i - 1 as libc::c_int) as isize) + 0.5f32
                {
                    last = i;
                }
                *f.offset(i as isize) = if *f.offset((i - 1 as libc::c_int) as isize) + 1.5f32
                    < *bandLogE2.offset((c * nbEBands + i) as isize)
                {
                    *f.offset((i - 1 as libc::c_int) as isize) + 1.5f32
                } else {
                    *bandLogE2.offset((c * nbEBands + i) as isize)
                };
                i += 1;
            }
            i = last - 1 as libc::c_int;
            while i >= 0 as libc::c_int {
                *f.offset(i as isize) = if *f.offset(i as isize)
                    < (if *f.offset((i + 1 as libc::c_int) as isize) + 2.0f32
                        < *bandLogE2.offset((c * nbEBands + i) as isize)
                    {
                        *f.offset((i + 1 as libc::c_int) as isize) + 2.0f32
                    } else {
                        *bandLogE2.offset((c * nbEBands + i) as isize)
                    }) {
                    *f.offset(i as isize)
                } else if *f.offset((i + 1 as libc::c_int) as isize) + 2.0f32
                    < *bandLogE2.offset((c * nbEBands + i) as isize)
                {
                    *f.offset((i + 1 as libc::c_int) as isize) + 2.0f32
                } else {
                    *bandLogE2.offset((c * nbEBands + i) as isize)
                };
                i -= 1;
            }
            offset = 1.0f32;
            i = 2 as libc::c_int;
            while i < end - 2 as libc::c_int {
                *f.offset(i as isize) = if *f.offset(i as isize)
                    > median_of_5(
                        &*bandLogE2.offset((c * nbEBands + i - 2 as libc::c_int) as isize),
                    ) - offset
                {
                    *f.offset(i as isize)
                } else {
                    median_of_5(&*bandLogE2.offset((c * nbEBands + i - 2 as libc::c_int) as isize))
                        - offset
                };
                i += 1;
            }
            tmp = median_of_3(&*bandLogE2.offset((c * nbEBands) as isize)) - offset;
            *f.offset(0 as libc::c_int as isize) = if *f.offset(0 as libc::c_int as isize) > tmp {
                *f.offset(0 as libc::c_int as isize)
            } else {
                tmp
            };
            *f.offset(1 as libc::c_int as isize) = if *f.offset(1 as libc::c_int as isize) > tmp {
                *f.offset(1 as libc::c_int as isize)
            } else {
                tmp
            };
            tmp = median_of_3(&*bandLogE2.offset((c * nbEBands + end - 3 as libc::c_int) as isize))
                - offset;
            *f.offset((end - 2 as libc::c_int) as isize) =
                if *f.offset((end - 2 as libc::c_int) as isize) > tmp {
                    *f.offset((end - 2 as libc::c_int) as isize)
                } else {
                    tmp
                };
            *f.offset((end - 1 as libc::c_int) as isize) =
                if *f.offset((end - 1 as libc::c_int) as isize) > tmp {
                    *f.offset((end - 1 as libc::c_int) as isize)
                } else {
                    tmp
                };
            i = 0 as libc::c_int;
            while i < end {
                *f.offset(i as isize) =
                    if *f.offset(i as isize) > *noise_floor.as_mut_ptr().offset(i as isize) {
                        *f.offset(i as isize)
                    } else {
                        *noise_floor.as_mut_ptr().offset(i as isize)
                    };
                i += 1;
            }
            c += 1;
            if !(c < C) {
                break;
            }
        }
        if C == 2 as libc::c_int {
            i = start;
            while i < end {
                *follower.as_mut_ptr().offset((nbEBands + i) as isize) =
                    if *follower.as_mut_ptr().offset((nbEBands + i) as isize)
                        > *follower.as_mut_ptr().offset(i as isize) - 4.0f32
                    {
                        *follower.as_mut_ptr().offset((nbEBands + i) as isize)
                    } else {
                        *follower.as_mut_ptr().offset(i as isize) - 4.0f32
                    };
                *follower.as_mut_ptr().offset(i as isize) =
                    if *follower.as_mut_ptr().offset(i as isize)
                        > *follower.as_mut_ptr().offset((nbEBands + i) as isize) - 4.0f32
                    {
                        *follower.as_mut_ptr().offset(i as isize)
                    } else {
                        *follower.as_mut_ptr().offset((nbEBands + i) as isize) - 4.0f32
                    };
                *follower.as_mut_ptr().offset(i as isize) = 0.5f32
                    * ((if 0 as libc::c_int as libc::c_float
                        > *bandLogE.offset(i as isize) - *follower.as_mut_ptr().offset(i as isize)
                    {
                        0 as libc::c_int as libc::c_float
                    } else {
                        *bandLogE.offset(i as isize) - *follower.as_mut_ptr().offset(i as isize)
                    }) + (if 0 as libc::c_int as libc::c_float
                        > *bandLogE.offset((nbEBands + i) as isize)
                            - *follower.as_mut_ptr().offset((nbEBands + i) as isize)
                    {
                        0 as libc::c_int as libc::c_float
                    } else {
                        *bandLogE.offset((nbEBands + i) as isize)
                            - *follower.as_mut_ptr().offset((nbEBands + i) as isize)
                    }));
                i += 1;
            }
        } else {
            i = start;
            while i < end {
                *follower.as_mut_ptr().offset(i as isize) = if 0 as libc::c_int as libc::c_float
                    > *bandLogE.offset(i as isize) - *follower.as_mut_ptr().offset(i as isize)
                {
                    0 as libc::c_int as libc::c_float
                } else {
                    *bandLogE.offset(i as isize) - *follower.as_mut_ptr().offset(i as isize)
                };
                i += 1;
            }
        }
        i = start;
        while i < end {
            *follower.as_mut_ptr().offset(i as isize) = if *follower.as_mut_ptr().offset(i as isize)
                > *surround_dynalloc.offset(i as isize)
            {
                *follower.as_mut_ptr().offset(i as isize)
            } else {
                *surround_dynalloc.offset(i as isize)
            };
            i += 1;
        }
        i = start;
        while i < end {
            *importance.offset(i as isize) = floor(
                (0.5f32
                    + 13 as libc::c_int as libc::c_float
                        * exp(0.6931471805599453094f64
                            * (if *follower.as_mut_ptr().offset(i as isize) < 4.0f32 {
                                *follower.as_mut_ptr().offset(i as isize)
                            } else {
                                4.0f32
                            }) as libc::c_double) as libc::c_float)
                    as libc::c_double,
            ) as libc::c_int;
            i += 1;
        }
        if (vbr == 0 || constrained_vbr != 0) && isTransient == 0 {
            i = start;
            while i < end {
                *follower.as_mut_ptr().offset(i as isize) =
                    0.5f32 * *follower.as_mut_ptr().offset(i as isize);
                i += 1;
            }
        }
        i = start;
        while i < end {
            if i < 8 as libc::c_int {
                let ref mut fresh1 = *follower.as_mut_ptr().offset(i as isize);
                *fresh1 *= 2 as libc::c_int as libc::c_float;
            }
            if i >= 12 as libc::c_int {
                *follower.as_mut_ptr().offset(i as isize) =
                    0.5f32 * *follower.as_mut_ptr().offset(i as isize);
            }
            i += 1;
        }
        if (*analysis).valid != 0 {
            i = start;
            while i
                < (if (19 as libc::c_int) < end {
                    19 as libc::c_int
                } else {
                    end
                })
            {
                *follower.as_mut_ptr().offset(i as isize) =
                    *follower.as_mut_ptr().offset(i as isize)
                        + 1.0f32 / 64.0f32
                            * (*analysis).leak_boost[i as usize] as libc::c_int as libc::c_float;
                i += 1;
            }
        }
        i = start;
        while i < end {
            let mut width: libc::c_int = 0;
            let mut boost: libc::c_int = 0;
            let mut boost_bits: libc::c_int = 0;
            *follower.as_mut_ptr().offset(i as isize) =
                if *follower.as_mut_ptr().offset(i as isize) < 4 as libc::c_int as libc::c_float {
                    *follower.as_mut_ptr().offset(i as isize)
                } else {
                    4 as libc::c_int as libc::c_float
                };
            width = C
                * (*eBands.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                    - *eBands.offset(i as isize) as libc::c_int)
                << LM;
            if width < 6 as libc::c_int {
                boost = *follower.as_mut_ptr().offset(i as isize) as libc::c_int;
                boost_bits = boost * width << 3 as libc::c_int;
            } else if width > 48 as libc::c_int {
                boost = (*follower.as_mut_ptr().offset(i as isize)
                    * 8 as libc::c_int as libc::c_float) as libc::c_int;
                boost_bits = (boost * width << 3 as libc::c_int) / 8 as libc::c_int;
            } else {
                boost = (*follower.as_mut_ptr().offset(i as isize) * width as libc::c_float
                    / 6 as libc::c_int as libc::c_float) as libc::c_int;
                boost_bits = (boost * 6 as libc::c_int) << 3 as libc::c_int;
            }
            if (vbr == 0 || constrained_vbr != 0 && isTransient == 0)
                && tot_boost + boost_bits >> 3 as libc::c_int >> 3 as libc::c_int
                    > 2 as libc::c_int * effectiveBytes / 3 as libc::c_int
            {
                let mut cap: opus_int32 = ((2 as libc::c_int * effectiveBytes / 3 as libc::c_int)
                    << 3 as libc::c_int)
                    << 3 as libc::c_int;
                *offsets.offset(i as isize) = cap - tot_boost;
                tot_boost = cap;
                break;
            } else {
                *offsets.offset(i as isize) = boost;
                tot_boost += boost_bits;
                i += 1;
            }
        }
    } else {
        i = start;
        while i < end {
            *importance.offset(i as isize) = 13 as libc::c_int;
            i += 1;
        }
    }
    *tot_boost_ = tot_boost;
    return maxDepth;
}
#[c2rust::src_loc = "1165:1"]
unsafe extern "C" fn run_prefilter(
    mut st: *mut OpusCustomEncoder,
    mut in_0: *mut celt_sig,
    mut prefilter_mem: *mut celt_sig,
    mut CC: libc::c_int,
    mut N: libc::c_int,
    mut prefilter_tapset: libc::c_int,
    mut pitch: *mut libc::c_int,
    mut gain: *mut opus_val16,
    mut qgain: *mut libc::c_int,
    mut enabled: libc::c_int,
    mut nbAvailableBytes: libc::c_int,
    mut analysis: *mut AnalysisInfo,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut pre: [*mut celt_sig; 2] = [0 as *mut celt_sig; 2];
    let mut mode: *const OpusCustomMode = 0 as *const OpusCustomMode;
    let mut pitch_index: libc::c_int = 0;
    let mut gain1: opus_val16 = 0.;
    let mut pf_threshold: opus_val16 = 0.;
    let mut pf_on: libc::c_int = 0;
    let mut qg: libc::c_int = 0;
    let mut overlap: libc::c_int = 0;
    mode = (*st).mode;
    overlap = (*mode).overlap;
    let vla = (CC * (N + 1024 as libc::c_int)) as usize;
    let mut _pre: Vec<celt_sig> = ::std::vec::from_elem(0., vla);
    pre[0 as libc::c_int as usize] = _pre.as_mut_ptr();
    pre[1 as libc::c_int as usize] = _pre.as_mut_ptr().offset((N + 1024 as libc::c_int) as isize);
    c = 0 as libc::c_int;
    loop {
        memcpy(
            pre[c as usize] as *mut libc::c_void,
            prefilter_mem.offset((c * 1024 as libc::c_int) as isize) as *const libc::c_void,
            (1024 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<celt_sig>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * (pre[c as usize])
                            .offset_from(prefilter_mem.offset((c * 1024 as libc::c_int) as isize))
                            as libc::c_long) as libc::c_ulong,
                ),
        );
        memcpy(
            (pre[c as usize]).offset(1024 as libc::c_int as isize) as *mut libc::c_void,
            in_0.offset((c * (N + overlap)) as isize)
                .offset(overlap as isize) as *const libc::c_void,
            (N as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<celt_sig>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * (pre[c as usize])
                            .offset(1024 as libc::c_int as isize)
                            .offset_from(
                                in_0.offset((c * (N + overlap)) as isize)
                                    .offset(overlap as isize),
                            ) as libc::c_long) as libc::c_ulong,
                ),
        );
        c += 1;
        if !(c < CC) {
            break;
        }
    }
    if enabled != 0 {
        let vla_0 = (1024 as libc::c_int + N >> 1 as libc::c_int) as usize;
        let mut pitch_buf: Vec<opus_val16> = ::std::vec::from_elem(0., vla_0);
        pitch_downsample(
            pre.as_mut_ptr() as *mut *mut celt_sig,
            pitch_buf.as_mut_ptr(),
            1024 as libc::c_int + N,
            CC,
            (*st).arch,
        );
        pitch_search(
            pitch_buf
                .as_mut_ptr()
                .offset((1024 as libc::c_int >> 1 as libc::c_int) as isize),
            pitch_buf.as_mut_ptr(),
            N,
            1024 as libc::c_int - 3 as libc::c_int * 15 as libc::c_int,
            &mut pitch_index,
            (*st).arch,
        );
        pitch_index = 1024 as libc::c_int - pitch_index;
        gain1 = remove_doubling(
            pitch_buf.as_mut_ptr(),
            1024 as libc::c_int,
            15 as libc::c_int,
            N,
            &mut pitch_index,
            (*st).prefilter_period,
            (*st).prefilter_gain,
            (*st).arch,
        );
        if pitch_index > 1024 as libc::c_int - 2 as libc::c_int {
            pitch_index = 1024 as libc::c_int - 2 as libc::c_int;
        }
        gain1 = 0.7f32 * gain1;
        if (*st).loss_rate > 2 as libc::c_int {
            gain1 = 0.5f32 * gain1;
        }
        if (*st).loss_rate > 4 as libc::c_int {
            gain1 = 0.5f32 * gain1;
        }
        if (*st).loss_rate > 8 as libc::c_int {
            gain1 = 0 as libc::c_int as opus_val16;
        }
    } else {
        gain1 = 0 as libc::c_int as opus_val16;
        pitch_index = 15 as libc::c_int;
    }
    if (*analysis).valid != 0 {
        gain1 = gain1 * (*analysis).max_pitch_ratio;
    }
    pf_threshold = 0.2f32;
    if abs(pitch_index - (*st).prefilter_period) * 10 as libc::c_int > pitch_index {
        pf_threshold += 0.2f32;
    }
    if nbAvailableBytes < 25 as libc::c_int {
        pf_threshold += 0.1f32;
    }
    if nbAvailableBytes < 35 as libc::c_int {
        pf_threshold += 0.1f32;
    }
    if (*st).prefilter_gain > 0.4f32 {
        pf_threshold -= 0.1f32;
    }
    if (*st).prefilter_gain > 0.55f32 {
        pf_threshold -= 0.1f32;
    }
    pf_threshold = if pf_threshold > 0.2f32 {
        pf_threshold
    } else {
        0.2f32
    };
    if gain1 < pf_threshold {
        gain1 = 0 as libc::c_int as opus_val16;
        pf_on = 0 as libc::c_int;
        qg = 0 as libc::c_int;
    } else {
        if (fabs((gain1 - (*st).prefilter_gain) as libc::c_double) as libc::c_float) < 0.1f32 {
            gain1 = (*st).prefilter_gain;
        }
        qg = floor(
            (0.5f32
                + gain1 * 32 as libc::c_int as libc::c_float / 3 as libc::c_int as libc::c_float)
                as libc::c_double,
        ) as libc::c_int
            - 1 as libc::c_int;
        qg = if 0 as libc::c_int
            > (if (7 as libc::c_int) < qg {
                7 as libc::c_int
            } else {
                qg
            }) {
            0 as libc::c_int
        } else if (7 as libc::c_int) < qg {
            7 as libc::c_int
        } else {
            qg
        };
        gain1 = 0.09375f32 * (qg + 1 as libc::c_int) as libc::c_float;
        pf_on = 1 as libc::c_int;
    }
    c = 0 as libc::c_int;
    loop {
        let mut offset: libc::c_int = (*mode).shortMdctSize - overlap;
        (*st).prefilter_period = if (*st).prefilter_period > 15 as libc::c_int {
            (*st).prefilter_period
        } else {
            15 as libc::c_int
        };
        memcpy(
            in_0.offset((c * (N + overlap)) as isize) as *mut libc::c_void,
            ((*st).in_mem).as_mut_ptr().offset((c * overlap) as isize) as *const libc::c_void,
            (overlap as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<celt_sig>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * in_0
                            .offset((c * (N + overlap)) as isize)
                            .offset_from(((*st).in_mem).as_mut_ptr().offset((c * overlap) as isize))
                            as libc::c_long) as libc::c_ulong,
                ),
        );
        if offset != 0 {
            comb_filter(
                in_0.offset((c * (N + overlap)) as isize)
                    .offset(overlap as isize),
                (pre[c as usize]).offset(1024 as libc::c_int as isize),
                (*st).prefilter_period,
                (*st).prefilter_period,
                offset,
                -(*st).prefilter_gain,
                -(*st).prefilter_gain,
                (*st).prefilter_tapset,
                (*st).prefilter_tapset,
                0 as *const opus_val16,
                0 as libc::c_int,
                (*st).arch,
            );
        }
        comb_filter(
            in_0.offset((c * (N + overlap)) as isize)
                .offset(overlap as isize)
                .offset(offset as isize),
            (pre[c as usize])
                .offset(1024 as libc::c_int as isize)
                .offset(offset as isize),
            (*st).prefilter_period,
            pitch_index,
            N - offset,
            -(*st).prefilter_gain,
            -gain1,
            (*st).prefilter_tapset,
            prefilter_tapset,
            (*mode).window,
            overlap,
            (*st).arch,
        );
        memcpy(
            ((*st).in_mem).as_mut_ptr().offset((c * overlap) as isize) as *mut libc::c_void,
            in_0.offset((c * (N + overlap)) as isize).offset(N as isize) as *const libc::c_void,
            (overlap as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<celt_sig>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * ((*st).in_mem)
                            .as_mut_ptr()
                            .offset((c * overlap) as isize)
                            .offset_from(
                                in_0.offset((c * (N + overlap)) as isize).offset(N as isize),
                            ) as libc::c_long) as libc::c_ulong,
                ),
        );
        if N > 1024 as libc::c_int {
            memcpy(
                prefilter_mem.offset((c * 1024 as libc::c_int) as isize) as *mut libc::c_void,
                (pre[c as usize]).offset(N as isize) as *const libc::c_void,
                (1024 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<celt_sig>() as libc::c_ulong)
                    .wrapping_add(
                        (0 as libc::c_int as libc::c_long
                            * prefilter_mem
                                .offset((c * 1024 as libc::c_int) as isize)
                                .offset_from((pre[c as usize]).offset(N as isize))
                                as libc::c_long) as libc::c_ulong,
                    ),
            );
        } else {
            memmove(
                prefilter_mem.offset((c * 1024 as libc::c_int) as isize) as *mut libc::c_void,
                prefilter_mem
                    .offset((c * 1024 as libc::c_int) as isize)
                    .offset(N as isize) as *const libc::c_void,
                ((1024 as libc::c_int - N) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<celt_sig>() as libc::c_ulong)
                    .wrapping_add(
                        (0 as libc::c_int as libc::c_long
                            * prefilter_mem
                                .offset((c * 1024 as libc::c_int) as isize)
                                .offset_from(
                                    prefilter_mem
                                        .offset((c * 1024 as libc::c_int) as isize)
                                        .offset(N as isize),
                                ) as libc::c_long) as libc::c_ulong,
                    ),
            );
            memcpy(
                prefilter_mem
                    .offset((c * 1024 as libc::c_int) as isize)
                    .offset(1024 as libc::c_int as isize)
                    .offset(-(N as isize)) as *mut libc::c_void,
                (pre[c as usize]).offset(1024 as libc::c_int as isize) as *const libc::c_void,
                (N as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<celt_sig>() as libc::c_ulong)
                    .wrapping_add(
                        (0 as libc::c_int as libc::c_long
                            * prefilter_mem
                                .offset((c * 1024 as libc::c_int) as isize)
                                .offset(1024 as libc::c_int as isize)
                                .offset(-(N as isize))
                                .offset_from((pre[c as usize]).offset(1024 as libc::c_int as isize))
                                as libc::c_long) as libc::c_ulong,
                    ),
            );
        }
        c += 1;
        if !(c < CC) {
            break;
        }
    }
    *gain = gain1;
    *pitch = pitch_index;
    *qgain = qg;
    return pf_on;
}
#[c2rust::src_loc = "1297:1"]
unsafe extern "C" fn compute_vbr(
    mut mode: *const OpusCustomMode,
    mut analysis: *mut AnalysisInfo,
    mut base_target: opus_int32,
    mut LM: libc::c_int,
    mut bitrate: opus_int32,
    mut lastCodedBands: libc::c_int,
    mut C: libc::c_int,
    mut intensity: libc::c_int,
    mut constrained_vbr: libc::c_int,
    mut stereo_saving: opus_val16,
    mut tot_boost: libc::c_int,
    mut tf_estimate: opus_val16,
    mut pitch_change: libc::c_int,
    mut maxDepth: opus_val16,
    mut lfe: libc::c_int,
    mut has_surround_mask: libc::c_int,
    mut surround_masking: opus_val16,
    mut temporal_vbr: opus_val16,
) -> libc::c_int {
    let mut target: opus_int32 = 0;
    let mut coded_bins: libc::c_int = 0;
    let mut coded_bands: libc::c_int = 0;
    let mut tf_calibration: opus_val16 = 0.;
    let mut nbEBands: libc::c_int = 0;
    let mut eBands: *const opus_int16 = 0 as *const opus_int16;
    nbEBands = (*mode).nbEBands;
    eBands = (*mode).eBands;
    coded_bands = if lastCodedBands != 0 {
        lastCodedBands
    } else {
        nbEBands
    };
    coded_bins = (*eBands.offset(coded_bands as isize) as libc::c_int) << LM;
    if C == 2 as libc::c_int {
        coded_bins += (*eBands.offset(
            (if intensity < coded_bands {
                intensity
            } else {
                coded_bands
            }) as isize,
        ) as libc::c_int)
            << LM;
    }
    target = base_target;
    if (*analysis).valid != 0 && ((*analysis).activity as libc::c_double) < 0.4f64 {
        target -= ((coded_bins << 3 as libc::c_int) as libc::c_float
            * (0.4f32 - (*analysis).activity)) as opus_int32;
    }
    if C == 2 as libc::c_int {
        let mut coded_stereo_bands: libc::c_int = 0;
        let mut coded_stereo_dof: libc::c_int = 0;
        let mut max_frac: opus_val16 = 0.;
        coded_stereo_bands = if intensity < coded_bands {
            intensity
        } else {
            coded_bands
        };
        coded_stereo_dof = ((*eBands.offset(coded_stereo_bands as isize) as libc::c_int) << LM)
            - coded_stereo_bands;
        max_frac = 0.8f32 * coded_stereo_dof as opus_val32 / coded_bins as opus_val16;
        stereo_saving = if stereo_saving < 1.0f32 {
            stereo_saving
        } else {
            1.0f32
        };
        target -= (if (max_frac * target as libc::c_float)
            < (stereo_saving - 0.1f32) * (coded_stereo_dof << 3 as libc::c_int) as opus_val32
        {
            max_frac * target as libc::c_float
        } else {
            (stereo_saving - 0.1f32) * (coded_stereo_dof << 3 as libc::c_int) as opus_val32
        }) as opus_int32;
    }
    target += tot_boost - ((19 as libc::c_int) << LM);
    tf_calibration = 0.044f32;
    target += ((tf_estimate - tf_calibration) * target as libc::c_float) as opus_int32;
    if (*analysis).valid != 0 && lfe == 0 {
        let mut tonal_target: opus_int32 = 0;
        let mut tonal: libc::c_float = 0.;
        tonal = (if 0.0f32 > (*analysis).tonality - 0.15f32 {
            0.0f32
        } else {
            (*analysis).tonality - 0.15f32
        }) - 0.12f32;
        tonal_target = target
            + ((coded_bins << 3 as libc::c_int) as libc::c_float * 1.2f32 * tonal) as opus_int32;
        if pitch_change != 0 {
            tonal_target +=
                ((coded_bins << 3 as libc::c_int) as libc::c_float * 0.8f32) as opus_int32;
        }
        target = tonal_target;
    }
    if has_surround_mask != 0 && lfe == 0 {
        let mut surround_target: opus_int32 = target
            + (surround_masking * (coded_bins << 3 as libc::c_int) as opus_val32) as opus_int32;
        target = if target / 4 as libc::c_int > surround_target {
            target / 4 as libc::c_int
        } else {
            surround_target
        };
    }
    let mut floor_depth: opus_int32 = 0;
    let mut bins: libc::c_int = 0;
    bins = (*eBands.offset((nbEBands - 2 as libc::c_int) as isize) as libc::c_int) << LM;
    floor_depth = ((C * bins << 3 as libc::c_int) as opus_val32 * maxDepth) as opus_int32;
    floor_depth = if floor_depth > target >> 2 as libc::c_int {
        floor_depth
    } else {
        target >> 2 as libc::c_int
    };
    target = if target < floor_depth {
        target
    } else {
        floor_depth
    };
    if (has_surround_mask == 0 || lfe != 0) && constrained_vbr != 0 {
        target = base_target + (0.67f32 * (target - base_target) as libc::c_float) as opus_int32;
    }
    if has_surround_mask == 0 && tf_estimate < 0.2f32 {
        let mut amount: opus_val16 = 0.;
        let mut tvbr_factor: opus_val16 = 0.;
        amount = 0.0000031f32
            * (if 0 as libc::c_int
                > (if (32000 as libc::c_int) < 96000 as libc::c_int - bitrate {
                    32000 as libc::c_int
                } else {
                    96000 as libc::c_int - bitrate
                })
            {
                0 as libc::c_int
            } else {
                (if (32000 as libc::c_int) < 96000 as libc::c_int - bitrate {
                    32000 as libc::c_int
                } else {
                    96000 as libc::c_int - bitrate
                })
            }) as libc::c_float;
        tvbr_factor = temporal_vbr * amount;
        target += (tvbr_factor * target as libc::c_float) as opus_int32;
    }
    target = if 2 as libc::c_int * base_target < target {
        2 as libc::c_int * base_target
    } else {
        target
    };
    return target;
}
#[no_mangle]
#[c2rust::src_loc = "1408:1"]
pub unsafe extern "C" fn celt_encode_with_ec(
    mut st: *mut OpusCustomEncoder,
    mut pcm: *const opus_val16,
    mut frame_size: libc::c_int,
    mut compressed: *mut libc::c_uchar,
    mut nbCompressedBytes: libc::c_int,
    mut enc: *mut ec_enc,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut N: libc::c_int = 0;
    let mut bits: opus_int32 = 0;
    let mut _enc: ec_enc = ec_enc {
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
    let mut prefilter_mem: *mut celt_sig = 0 as *mut celt_sig;
    let mut oldBandE: *mut opus_val16 = 0 as *mut opus_val16;
    let mut oldLogE: *mut opus_val16 = 0 as *mut opus_val16;
    let mut oldLogE2: *mut opus_val16 = 0 as *mut opus_val16;
    let mut energyError: *mut opus_val16 = 0 as *mut opus_val16;
    let mut shortBlocks: libc::c_int = 0 as libc::c_int;
    let mut isTransient: libc::c_int = 0 as libc::c_int;
    let CC: libc::c_int = (*st).channels;
    let C: libc::c_int = (*st).stream_channels;
    let mut LM: libc::c_int = 0;
    let mut M: libc::c_int = 0;
    let mut tf_select: libc::c_int = 0;
    let mut nbFilledBytes: libc::c_int = 0;
    let mut nbAvailableBytes: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut effEnd: libc::c_int = 0;
    let mut codedBands: libc::c_int = 0;
    let mut alloc_trim: libc::c_int = 0;
    let mut pitch_index: libc::c_int = 15 as libc::c_int;
    let mut gain1: opus_val16 = 0 as libc::c_int as opus_val16;
    let mut dual_stereo: libc::c_int = 0 as libc::c_int;
    let mut effectiveBytes: libc::c_int = 0;
    let mut dynalloc_logp: libc::c_int = 0;
    let mut vbr_rate: opus_int32 = 0;
    let mut total_bits: opus_int32 = 0;
    let mut total_boost: opus_int32 = 0;
    let mut balance: opus_int32 = 0;
    let mut tell: opus_int32 = 0;
    let mut tell0_frac: opus_int32 = 0;
    let mut prefilter_tapset: libc::c_int = 0 as libc::c_int;
    let mut pf_on: libc::c_int = 0;
    let mut anti_collapse_rsv: libc::c_int = 0;
    let mut anti_collapse_on: libc::c_int = 0 as libc::c_int;
    let mut silence: libc::c_int = 0 as libc::c_int;
    let mut tf_chan: libc::c_int = 0 as libc::c_int;
    let mut tf_estimate: opus_val16 = 0.;
    let mut pitch_change: libc::c_int = 0 as libc::c_int;
    let mut tot_boost: opus_int32 = 0;
    let mut sample_max: opus_val32 = 0.;
    let mut maxDepth: opus_val16 = 0.;
    let mut mode: *const OpusCustomMode = 0 as *const OpusCustomMode;
    let mut nbEBands: libc::c_int = 0;
    let mut overlap: libc::c_int = 0;
    let mut eBands: *const opus_int16 = 0 as *const opus_int16;
    let mut secondMdct: libc::c_int = 0;
    let mut signalBandwidth: libc::c_int = 0;
    let mut transient_got_disabled: libc::c_int = 0 as libc::c_int;
    let mut surround_masking: opus_val16 = 0 as libc::c_int as opus_val16;
    let mut temporal_vbr: opus_val16 = 0 as libc::c_int as opus_val16;
    let mut surround_trim: opus_val16 = 0 as libc::c_int as opus_val16;
    let mut equiv_rate: opus_int32 = 0;
    let mut hybrid: libc::c_int = 0;
    let mut weak_transient: libc::c_int = 0 as libc::c_int;
    let mut enable_tf_analysis: libc::c_int = 0;
    mode = (*st).mode;
    nbEBands = (*mode).nbEBands;
    overlap = (*mode).overlap;
    eBands = (*mode).eBands;
    start = (*st).start;
    end = (*st).end;
    hybrid = (start != 0 as libc::c_int) as libc::c_int;
    tf_estimate = 0 as libc::c_int as opus_val16;
    if nbCompressedBytes < 2 as libc::c_int || pcm.is_null() {
        return -(1 as libc::c_int);
    }
    frame_size *= (*st).upsample;
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
    N = M * (*mode).shortMdctSize;
    prefilter_mem = ((*st).in_mem).as_mut_ptr().offset((CC * overlap) as isize);
    oldBandE = ((*st).in_mem)
        .as_mut_ptr()
        .offset((CC * (overlap + 1024 as libc::c_int)) as isize) as *mut opus_val16;
    oldLogE = oldBandE.offset((CC * nbEBands) as isize);
    oldLogE2 = oldLogE.offset((CC * nbEBands) as isize);
    energyError = oldLogE2.offset((CC * nbEBands) as isize);
    if enc.is_null() {
        tell = 1 as libc::c_int;
        tell0_frac = tell;
        nbFilledBytes = 0 as libc::c_int;
    } else {
        tell0_frac = ec_tell_frac(enc) as opus_int32;
        tell = ec_tell(enc);
        nbFilledBytes = tell + 4 as libc::c_int >> 3 as libc::c_int;
    }
    if !((*st).signalling == 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: st->signalling==0\0" as *const u8 as *const libc::c_char,
            b"celt/celt_encoder.c\0" as *const u8 as *const libc::c_char,
            1547 as libc::c_int,
        );
    }
    nbCompressedBytes = if nbCompressedBytes < 1275 as libc::c_int {
        nbCompressedBytes
    } else {
        1275 as libc::c_int
    };
    nbAvailableBytes = nbCompressedBytes - nbFilledBytes;
    if (*st).vbr != 0 && (*st).bitrate != -(1 as libc::c_int) {
        let mut den: opus_int32 = (*mode).Fs >> 3 as libc::c_int;
        vbr_rate = ((*st).bitrate * frame_size + (den >> 1 as libc::c_int)) / den;
        effectiveBytes = vbr_rate >> 3 as libc::c_int + 3 as libc::c_int;
    } else {
        let mut tmp: opus_int32 = 0;
        vbr_rate = 0 as libc::c_int;
        tmp = (*st).bitrate * frame_size;
        if tell > 1 as libc::c_int {
            tmp += tell;
        }
        if (*st).bitrate != -(1 as libc::c_int) {
            nbCompressedBytes = if 2 as libc::c_int
                > (if nbCompressedBytes
                    < (tmp + 4 as libc::c_int * (*mode).Fs) / (8 as libc::c_int * (*mode).Fs)
                        - ((*st).signalling != 0) as libc::c_int
                {
                    nbCompressedBytes
                } else {
                    (tmp + 4 as libc::c_int * (*mode).Fs) / (8 as libc::c_int * (*mode).Fs)
                        - ((*st).signalling != 0) as libc::c_int
                }) {
                2 as libc::c_int
            } else if nbCompressedBytes
                < (tmp + 4 as libc::c_int * (*mode).Fs) / (8 as libc::c_int * (*mode).Fs)
                    - ((*st).signalling != 0) as libc::c_int
            {
                nbCompressedBytes
            } else {
                (tmp + 4 as libc::c_int * (*mode).Fs) / (8 as libc::c_int * (*mode).Fs)
                    - ((*st).signalling != 0) as libc::c_int
            };
        }
        effectiveBytes = nbCompressedBytes - nbFilledBytes;
    }
    equiv_rate = (nbCompressedBytes * 8 as libc::c_int * 50 as libc::c_int
        >> 3 as libc::c_int - LM)
        - (40 as libc::c_int * C + 20 as libc::c_int)
            * ((400 as libc::c_int >> LM) - 50 as libc::c_int);
    if (*st).bitrate != -(1 as libc::c_int) {
        equiv_rate = if equiv_rate
            < (*st).bitrate
                - (40 as libc::c_int * C + 20 as libc::c_int)
                    * ((400 as libc::c_int >> LM) - 50 as libc::c_int)
        {
            equiv_rate
        } else {
            (*st).bitrate
                - (40 as libc::c_int * C + 20 as libc::c_int)
                    * ((400 as libc::c_int >> LM) - 50 as libc::c_int)
        };
    }
    if enc.is_null() {
        ec_enc_init(&mut _enc, compressed, nbCompressedBytes as opus_uint32);
        enc = &mut _enc;
    }
    if vbr_rate > 0 as libc::c_int {
        if (*st).constrained_vbr != 0 {
            let mut vbr_bound: opus_int32 = 0;
            let mut max_allowed: opus_int32 = 0;
            vbr_bound = vbr_rate;
            max_allowed = if (if (if tell == 1 as libc::c_int {
                2 as libc::c_int
            } else {
                0 as libc::c_int
            }) > vbr_rate + vbr_bound - (*st).vbr_reservoir
                >> 3 as libc::c_int + 3 as libc::c_int
            {
                (if tell == 1 as libc::c_int {
                    2 as libc::c_int
                } else {
                    0 as libc::c_int
                })
            } else {
                vbr_rate + vbr_bound - (*st).vbr_reservoir >> 3 as libc::c_int + 3 as libc::c_int
            }) < nbAvailableBytes
            {
                if (if tell == 1 as libc::c_int {
                    2 as libc::c_int
                } else {
                    0 as libc::c_int
                }) > vbr_rate + vbr_bound - (*st).vbr_reservoir
                    >> 3 as libc::c_int + 3 as libc::c_int
                {
                    if tell == 1 as libc::c_int {
                        2 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }
                } else {
                    vbr_rate + vbr_bound - (*st).vbr_reservoir
                        >> 3 as libc::c_int + 3 as libc::c_int
                }
            } else {
                nbAvailableBytes
            };
            if max_allowed < nbAvailableBytes {
                nbCompressedBytes = nbFilledBytes + max_allowed;
                nbAvailableBytes = max_allowed;
                ec_enc_shrink(enc, nbCompressedBytes as opus_uint32);
            }
        }
    }
    total_bits = nbCompressedBytes * 8 as libc::c_int;
    effEnd = end;
    if effEnd > (*mode).effEBands {
        effEnd = (*mode).effEBands;
    }
    let vla = (CC * (N + overlap)) as usize;
    let mut in_0: Vec<celt_sig> = ::std::vec::from_elem(0., vla);
    sample_max = if (*st).overlap_max > celt_maxabs16(pcm, C * (N - overlap) / (*st).upsample) {
        (*st).overlap_max
    } else {
        celt_maxabs16(pcm, C * (N - overlap) / (*st).upsample)
    };
    (*st).overlap_max = celt_maxabs16(
        pcm.offset((C * (N - overlap) / (*st).upsample) as isize),
        C * overlap / (*st).upsample,
    );
    sample_max = if sample_max > (*st).overlap_max {
        sample_max
    } else {
        (*st).overlap_max
    };
    silence = (sample_max
        <= 1 as libc::c_int as opus_val16
            / ((1 as libc::c_int) << (*st).lsb_depth) as libc::c_float)
        as libc::c_int;
    if tell == 1 as libc::c_int {
        ec_enc_bit_logp(enc, silence, 15 as libc::c_int as libc::c_uint);
    } else {
        silence = 0 as libc::c_int;
    }
    if silence != 0 {
        if vbr_rate > 0 as libc::c_int {
            nbCompressedBytes = if nbCompressedBytes < nbFilledBytes + 2 as libc::c_int {
                nbCompressedBytes
            } else {
                nbFilledBytes + 2 as libc::c_int
            };
            effectiveBytes = nbCompressedBytes;
            total_bits = nbCompressedBytes * 8 as libc::c_int;
            nbAvailableBytes = 2 as libc::c_int;
            ec_enc_shrink(enc, nbCompressedBytes as opus_uint32);
        }
        tell = nbCompressedBytes * 8 as libc::c_int;
        (*enc).nbits_total += tell - ec_tell(enc);
    }
    c = 0 as libc::c_int;
    loop {
        let mut need_clip: libc::c_int = 0 as libc::c_int;
        need_clip = ((*st).clip != 0 && sample_max > 65536.0f32) as libc::c_int;
        celt_preemphasis(
            pcm.offset(c as isize),
            in_0.as_mut_ptr()
                .offset((c * (N + overlap)) as isize)
                .offset(overlap as isize),
            N,
            CC,
            (*st).upsample,
            ((*mode).preemph).as_ptr(),
            ((*st).preemph_memE).as_mut_ptr().offset(c as isize),
            need_clip,
        );
        c += 1;
        if !(c < CC) {
            break;
        }
    }
    let mut enabled: libc::c_int = 0;
    let mut qg: libc::c_int = 0;
    enabled = (((*st).lfe != 0 && nbAvailableBytes > 3 as libc::c_int
        || nbAvailableBytes > 12 as libc::c_int * C)
        && hybrid == 0
        && silence == 0
        && (*st).disable_pf == 0
        && (*st).complexity >= 5 as libc::c_int) as libc::c_int;
    prefilter_tapset = (*st).tapset_decision;
    pf_on = run_prefilter(
        st,
        in_0.as_mut_ptr(),
        prefilter_mem,
        CC,
        N,
        prefilter_tapset,
        &mut pitch_index,
        &mut gain1,
        &mut qg,
        enabled,
        nbAvailableBytes,
        &mut (*st).analysis,
    );
    if (gain1 > 0.4f32 || (*st).prefilter_gain > 0.4f32)
        && ((*st).analysis.valid == 0 || (*st).analysis.tonality as libc::c_double > 0.3f64)
        && (pitch_index as libc::c_double > 1.26f64 * (*st).prefilter_period as libc::c_double
            || (pitch_index as libc::c_double) < 0.79f64 * (*st).prefilter_period as libc::c_double)
    {
        pitch_change = 1 as libc::c_int;
    }
    if pf_on == 0 as libc::c_int {
        if hybrid == 0 && tell + 16 as libc::c_int <= total_bits {
            ec_enc_bit_logp(enc, 0 as libc::c_int, 1 as libc::c_int as libc::c_uint);
        }
    } else {
        let mut octave: libc::c_int = 0;
        ec_enc_bit_logp(enc, 1 as libc::c_int, 1 as libc::c_int as libc::c_uint);
        pitch_index += 1 as libc::c_int;
        octave = ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int
            * 8 as libc::c_int
            - (pitch_index as libc::c_uint).leading_zeros() as i32
            - 5 as libc::c_int;
        ec_enc_uint(enc, octave as opus_uint32, 6 as libc::c_int as opus_uint32);
        ec_enc_bits(
            enc,
            (pitch_index - ((16 as libc::c_int) << octave)) as opus_uint32,
            (4 as libc::c_int + octave) as libc::c_uint,
        );
        pitch_index -= 1 as libc::c_int;
        ec_enc_bits(enc, qg as opus_uint32, 3 as libc::c_int as libc::c_uint);
        ec_enc_icdf(
            enc,
            prefilter_tapset,
            tapset_icdf.as_ptr(),
            2 as libc::c_int as libc::c_uint,
        );
    }
    isTransient = 0 as libc::c_int;
    shortBlocks = 0 as libc::c_int;
    if (*st).complexity >= 1 as libc::c_int && (*st).lfe == 0 {
        let mut allow_weak_transients: libc::c_int = (hybrid != 0
            && effectiveBytes < 15 as libc::c_int
            && (*st).silk_info.signalType != 2 as libc::c_int)
            as libc::c_int;
        isTransient = transient_analysis(
            in_0.as_mut_ptr(),
            N + overlap,
            CC,
            &mut tf_estimate,
            &mut tf_chan,
            allow_weak_transients,
            &mut weak_transient,
        );
    }
    if LM > 0 as libc::c_int && ec_tell(enc) + 3 as libc::c_int <= total_bits {
        if isTransient != 0 {
            shortBlocks = M;
        }
    } else {
        isTransient = 0 as libc::c_int;
        transient_got_disabled = 1 as libc::c_int;
    }
    let vla_0 = (CC * N) as usize;
    let mut freq: Vec<celt_sig> = ::std::vec::from_elem(0., vla_0);
    let vla_1 = (nbEBands * CC) as usize;
    let mut bandE: Vec<celt_ener> = ::std::vec::from_elem(0., vla_1);
    let vla_2 = (nbEBands * CC) as usize;
    let mut bandLogE: Vec<opus_val16> = ::std::vec::from_elem(0., vla_2);
    secondMdct = (shortBlocks != 0 && (*st).complexity >= 8 as libc::c_int) as libc::c_int;
    let vla_3 = (C * nbEBands) as usize;
    let mut bandLogE2: Vec<opus_val16> = ::std::vec::from_elem(0., vla_3);
    if secondMdct != 0 {
        compute_mdcts(
            mode,
            0 as libc::c_int,
            in_0.as_mut_ptr(),
            freq.as_mut_ptr(),
            C,
            CC,
            LM,
            (*st).upsample,
            (*st).arch,
        );
        compute_band_energies(
            mode,
            freq.as_mut_ptr(),
            bandE.as_mut_ptr(),
            effEnd,
            C,
            LM,
            (*st).arch,
        );
        amp2Log2(
            mode,
            effEnd,
            end,
            bandE.as_mut_ptr(),
            bandLogE2.as_mut_ptr(),
            C,
        );
        i = 0 as libc::c_int;
        while i < C * nbEBands {
            let ref mut fresh2 = *bandLogE2.as_mut_ptr().offset(i as isize);
            *fresh2 += 0.5f32 * LM as libc::c_float;
            i += 1;
        }
    }
    compute_mdcts(
        mode,
        shortBlocks,
        in_0.as_mut_ptr(),
        freq.as_mut_ptr(),
        C,
        CC,
        LM,
        (*st).upsample,
        (*st).arch,
    );
    if !(!(*freq.as_mut_ptr().offset(0 as libc::c_int as isize)
        != *freq.as_mut_ptr().offset(0 as libc::c_int as isize))
        && (C == 1 as libc::c_int
            || !(*freq.as_mut_ptr().offset(N as isize) != *freq.as_mut_ptr().offset(N as isize))))
    {
        celt_fatal(
            b"assertion failed: !celt_isnan(freq[0]) && (C==1 || !celt_isnan(freq[N]))\0"
                as *const u8 as *const libc::c_char,
            b"celt/celt_encoder.c\0" as *const u8 as *const libc::c_char,
            1729 as libc::c_int,
        );
    }
    if CC == 2 as libc::c_int && C == 1 as libc::c_int {
        tf_chan = 0 as libc::c_int;
    }
    compute_band_energies(
        mode,
        freq.as_mut_ptr(),
        bandE.as_mut_ptr(),
        effEnd,
        C,
        LM,
        (*st).arch,
    );
    if (*st).lfe != 0 {
        i = 2 as libc::c_int;
        while i < end {
            *bandE.as_mut_ptr().offset(i as isize) = if *bandE.as_mut_ptr().offset(i as isize)
                < 1e-4f32 * *bandE.as_mut_ptr().offset(0 as libc::c_int as isize)
            {
                *bandE.as_mut_ptr().offset(i as isize)
            } else {
                1e-4f32 * *bandE.as_mut_ptr().offset(0 as libc::c_int as isize)
            };
            *bandE.as_mut_ptr().offset(i as isize) =
                if *bandE.as_mut_ptr().offset(i as isize) > 1e-15f32 {
                    *bandE.as_mut_ptr().offset(i as isize)
                } else {
                    1e-15f32
                };
            i += 1;
        }
    }
    amp2Log2(
        mode,
        effEnd,
        end,
        bandE.as_mut_ptr(),
        bandLogE.as_mut_ptr(),
        C,
    );
    let vla_4 = (C * nbEBands) as usize;
    let mut surround_dynalloc: Vec<opus_val16> = ::std::vec::from_elem(0., vla_4);
    memset(
        surround_dynalloc.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (end as libc::c_ulong).wrapping_mul(::core::mem::size_of::<opus_val16>() as libc::c_ulong),
    );
    if hybrid == 0 && !((*st).energy_mask).is_null() && (*st).lfe == 0 {
        let mut mask_end: libc::c_int = 0;
        let mut midband: libc::c_int = 0;
        let mut count_dynalloc: libc::c_int = 0;
        let mut mask_avg: opus_val32 = 0 as libc::c_int as opus_val32;
        let mut diff: opus_val32 = 0 as libc::c_int as opus_val32;
        let mut count: libc::c_int = 0 as libc::c_int;
        mask_end = if 2 as libc::c_int > (*st).lastCodedBands {
            2 as libc::c_int
        } else {
            (*st).lastCodedBands
        };
        c = 0 as libc::c_int;
        while c < C {
            i = 0 as libc::c_int;
            while i < mask_end {
                let mut mask: opus_val16 = 0.;
                mask = if (if *((*st).energy_mask).offset((nbEBands * c + i) as isize) < 0.25f32 {
                    *((*st).energy_mask).offset((nbEBands * c + i) as isize)
                } else {
                    0.25f32
                }) > -2.0f32
                {
                    if *((*st).energy_mask).offset((nbEBands * c + i) as isize) < 0.25f32 {
                        *((*st).energy_mask).offset((nbEBands * c + i) as isize)
                    } else {
                        0.25f32
                    }
                } else {
                    -2.0f32
                };
                if mask > 0 as libc::c_int as libc::c_float {
                    mask = 0.5f32 * mask;
                }
                mask_avg += mask
                    * (*eBands.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                        - *eBands.offset(i as isize) as libc::c_int)
                        as opus_val32;
                count += *eBands.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                    - *eBands.offset(i as isize) as libc::c_int;
                diff += mask * (1 as libc::c_int + 2 as libc::c_int * i - mask_end) as opus_val32;
                i += 1;
            }
            c += 1;
        }
        if !(count > 0 as libc::c_int) {
            celt_fatal(
                b"assertion failed: count>0\0" as *const u8 as *const libc::c_char,
                b"celt/celt_encoder.c\0" as *const u8 as *const libc::c_char,
                1770 as libc::c_int,
            );
        }
        mask_avg = mask_avg / count as opus_val16;
        mask_avg += 0.2f32;
        diff = diff * 6 as libc::c_int as libc::c_float
            / (C * (mask_end - 1 as libc::c_int) * (mask_end + 1 as libc::c_int) * mask_end)
                as libc::c_float;
        diff = 0.5f32 * diff;
        diff = if (if diff < 0.031f32 { diff } else { 0.031f32 }) > -0.031f32 {
            if diff < 0.031f32 {
                diff
            } else {
                0.031f32
            }
        } else {
            -0.031f32
        };
        midband = 0 as libc::c_int;
        while (*eBands.offset((midband + 1 as libc::c_int) as isize) as libc::c_int)
            < *eBands.offset(mask_end as isize) as libc::c_int / 2 as libc::c_int
        {
            midband += 1;
        }
        count_dynalloc = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < mask_end {
            let mut lin: opus_val32 = 0.;
            let mut unmask: opus_val16 = 0.;
            lin = mask_avg + diff * (i - midband) as libc::c_float;
            if C == 2 as libc::c_int {
                unmask = if *((*st).energy_mask).offset(i as isize)
                    > *((*st).energy_mask).offset((nbEBands + i) as isize)
                {
                    *((*st).energy_mask).offset(i as isize)
                } else {
                    *((*st).energy_mask).offset((nbEBands + i) as isize)
                };
            } else {
                unmask = *((*st).energy_mask).offset(i as isize);
            }
            unmask = if unmask < 0.0f32 { unmask } else { 0.0f32 };
            unmask -= lin;
            if unmask > 0.25f32 {
                *surround_dynalloc.as_mut_ptr().offset(i as isize) = unmask - 0.25f32;
                count_dynalloc += 1;
            }
            i += 1;
        }
        if count_dynalloc >= 3 as libc::c_int {
            mask_avg += 0.25f32;
            if mask_avg > 0 as libc::c_int as libc::c_float {
                mask_avg = 0 as libc::c_int as opus_val32;
                diff = 0 as libc::c_int as opus_val32;
                memset(
                    surround_dynalloc.as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    (mask_end as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<opus_val16>() as libc::c_ulong),
                );
            } else {
                i = 0 as libc::c_int;
                while i < mask_end {
                    *surround_dynalloc.as_mut_ptr().offset(i as isize) = if 0 as libc::c_int
                        as libc::c_float
                        > *surround_dynalloc.as_mut_ptr().offset(i as isize) - 0.25f32
                    {
                        0 as libc::c_int as libc::c_float
                    } else {
                        *surround_dynalloc.as_mut_ptr().offset(i as isize) - 0.25f32
                    };
                    i += 1;
                }
            }
        }
        mask_avg += 0.2f32;
        surround_trim = 64 as libc::c_int as libc::c_float * diff;
        surround_masking = mask_avg;
    }
    if (*st).lfe == 0 {
        let mut follow: opus_val16 = -10.0f32;
        let mut frame_avg: opus_val32 = 0 as libc::c_int as opus_val32;
        let mut offset: opus_val16 = if shortBlocks != 0 {
            0.5f32 * LM as libc::c_float
        } else {
            0 as libc::c_int as libc::c_float
        };
        i = start;
        while i < end {
            follow = if follow - 1.0f32 > *bandLogE.as_mut_ptr().offset(i as isize) - offset {
                follow - 1.0f32
            } else {
                *bandLogE.as_mut_ptr().offset(i as isize) - offset
            };
            if C == 2 as libc::c_int {
                follow = if follow > *bandLogE.as_mut_ptr().offset((i + nbEBands) as isize) - offset
                {
                    follow
                } else {
                    *bandLogE.as_mut_ptr().offset((i + nbEBands) as isize) - offset
                };
            }
            frame_avg += follow;
            i += 1;
        }
        frame_avg /= (end - start) as libc::c_float;
        temporal_vbr = frame_avg - (*st).spec_avg;
        temporal_vbr = if 3.0f32
            < (if -1.5f32 > temporal_vbr {
                -1.5f32
            } else {
                temporal_vbr
            }) {
            3.0f32
        } else if -1.5f32 > temporal_vbr {
            -1.5f32
        } else {
            temporal_vbr
        };
        (*st).spec_avg += 0.02f32 * temporal_vbr;
    }
    if secondMdct == 0 {
        memcpy(
            bandLogE2.as_mut_ptr() as *mut libc::c_void,
            bandLogE.as_mut_ptr() as *const libc::c_void,
            ((C * nbEBands) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<opus_val16>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * bandLogE2.as_mut_ptr().offset_from(bandLogE.as_mut_ptr()) as libc::c_long)
                        as libc::c_ulong,
                ),
        );
    }
    if LM > 0 as libc::c_int
        && ec_tell(enc) + 3 as libc::c_int <= total_bits
        && isTransient == 0
        && (*st).complexity >= 5 as libc::c_int
        && (*st).lfe == 0
        && hybrid == 0
    {
        if patch_transient_decision(bandLogE.as_mut_ptr(), oldBandE, nbEBands, start, end, C) != 0 {
            isTransient = 1 as libc::c_int;
            shortBlocks = M;
            compute_mdcts(
                mode,
                shortBlocks,
                in_0.as_mut_ptr(),
                freq.as_mut_ptr(),
                C,
                CC,
                LM,
                (*st).upsample,
                (*st).arch,
            );
            compute_band_energies(
                mode,
                freq.as_mut_ptr(),
                bandE.as_mut_ptr(),
                effEnd,
                C,
                LM,
                (*st).arch,
            );
            amp2Log2(
                mode,
                effEnd,
                end,
                bandE.as_mut_ptr(),
                bandLogE.as_mut_ptr(),
                C,
            );
            i = 0 as libc::c_int;
            while i < C * nbEBands {
                let ref mut fresh3 = *bandLogE2.as_mut_ptr().offset(i as isize);
                *fresh3 += 0.5f32 * LM as libc::c_float;
                i += 1;
            }
            tf_estimate = 0.2f32;
        }
    }
    if LM > 0 as libc::c_int && ec_tell(enc) + 3 as libc::c_int <= total_bits {
        ec_enc_bit_logp(enc, isTransient, 3 as libc::c_int as libc::c_uint);
    }
    let vla_5 = (C * N) as usize;
    let mut X: Vec<celt_norm> = ::std::vec::from_elem(0., vla_5);
    normalise_bands(
        mode,
        freq.as_mut_ptr(),
        X.as_mut_ptr(),
        bandE.as_mut_ptr(),
        effEnd,
        C,
        M,
    );
    enable_tf_analysis = (effectiveBytes >= 15 as libc::c_int * C
        && hybrid == 0
        && (*st).complexity >= 2 as libc::c_int
        && (*st).lfe == 0) as libc::c_int;
    let vla_6 = nbEBands as usize;
    let mut offsets: Vec<libc::c_int> = ::std::vec::from_elem(0, vla_6);
    let vla_7 = nbEBands as usize;
    let mut importance: Vec<libc::c_int> = ::std::vec::from_elem(0, vla_7);
    let vla_8 = nbEBands as usize;
    let mut spread_weight: Vec<libc::c_int> = ::std::vec::from_elem(0, vla_8);
    maxDepth = dynalloc_analysis(
        bandLogE.as_mut_ptr(),
        bandLogE2.as_mut_ptr(),
        nbEBands,
        start,
        end,
        C,
        offsets.as_mut_ptr(),
        (*st).lsb_depth,
        (*mode).logN,
        isTransient,
        (*st).vbr,
        (*st).constrained_vbr,
        eBands,
        LM,
        effectiveBytes,
        &mut tot_boost,
        (*st).lfe,
        surround_dynalloc.as_mut_ptr(),
        &mut (*st).analysis,
        importance.as_mut_ptr(),
        spread_weight.as_mut_ptr(),
    );
    let vla_9 = nbEBands as usize;
    let mut tf_res: Vec<libc::c_int> = ::std::vec::from_elem(0, vla_9);
    if enable_tf_analysis != 0 {
        let mut lambda: libc::c_int = 0;
        lambda = if 80 as libc::c_int > 20480 as libc::c_int / effectiveBytes + 2 as libc::c_int {
            80 as libc::c_int
        } else {
            20480 as libc::c_int / effectiveBytes + 2 as libc::c_int
        };
        tf_select = tf_analysis(
            mode,
            effEnd,
            isTransient,
            tf_res.as_mut_ptr(),
            lambda,
            X.as_mut_ptr(),
            N,
            LM,
            tf_estimate,
            tf_chan,
            importance.as_mut_ptr(),
        );
        i = effEnd;
        while i < end {
            *tf_res.as_mut_ptr().offset(i as isize) = *tf_res
                .as_mut_ptr()
                .offset((effEnd - 1 as libc::c_int) as isize);
            i += 1;
        }
    } else if hybrid != 0 && weak_transient != 0 {
        i = 0 as libc::c_int;
        while i < end {
            *tf_res.as_mut_ptr().offset(i as isize) = 1 as libc::c_int;
            i += 1;
        }
        tf_select = 0 as libc::c_int;
    } else if hybrid != 0
        && effectiveBytes < 15 as libc::c_int
        && (*st).silk_info.signalType != 2 as libc::c_int
    {
        i = 0 as libc::c_int;
        while i < end {
            *tf_res.as_mut_ptr().offset(i as isize) = 0 as libc::c_int;
            i += 1;
        }
        tf_select = isTransient;
    } else {
        i = 0 as libc::c_int;
        while i < end {
            *tf_res.as_mut_ptr().offset(i as isize) = isTransient;
            i += 1;
        }
        tf_select = 0 as libc::c_int;
    }
    let vla_10 = (C * nbEBands) as usize;
    let mut error: Vec<opus_val16> = ::std::vec::from_elem(0., vla_10);
    c = 0 as libc::c_int;
    loop {
        i = start;
        while i < end {
            if (fabs(
                (*bandLogE.as_mut_ptr().offset((i + c * nbEBands) as isize)
                    - *oldBandE.offset((i + c * nbEBands) as isize))
                    as libc::c_double,
            ) as libc::c_float)
                < 2.0f32
            {
                let ref mut fresh4 = *bandLogE.as_mut_ptr().offset((i + c * nbEBands) as isize);
                *fresh4 -= *energyError.offset((i + c * nbEBands) as isize) * 0.25f32;
            }
            i += 1;
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
    quant_coarse_energy(
        mode,
        start,
        end,
        effEnd,
        bandLogE.as_mut_ptr(),
        oldBandE,
        total_bits as opus_uint32,
        error.as_mut_ptr(),
        enc,
        C,
        LM,
        nbAvailableBytes,
        (*st).force_intra,
        &mut (*st).delayedIntra,
        ((*st).complexity >= 4 as libc::c_int) as libc::c_int,
        (*st).loss_rate,
        (*st).lfe,
    );
    tf_encode(
        start,
        end,
        isTransient,
        tf_res.as_mut_ptr(),
        LM,
        tf_select,
        enc,
    );
    if ec_tell(enc) + 4 as libc::c_int <= total_bits {
        if (*st).lfe != 0 {
            (*st).tapset_decision = 0 as libc::c_int;
            (*st).spread_decision = 2 as libc::c_int;
        } else if hybrid != 0 {
            if (*st).complexity == 0 as libc::c_int {
                (*st).spread_decision = 0 as libc::c_int;
            } else if isTransient != 0 {
                (*st).spread_decision = 2 as libc::c_int;
            } else {
                (*st).spread_decision = 3 as libc::c_int;
            }
        } else if shortBlocks != 0
            || (*st).complexity < 3 as libc::c_int
            || nbAvailableBytes < 10 as libc::c_int * C
        {
            if (*st).complexity == 0 as libc::c_int {
                (*st).spread_decision = 0 as libc::c_int;
            } else {
                (*st).spread_decision = 2 as libc::c_int;
            }
        } else {
            (*st).spread_decision = spreading_decision(
                mode,
                X.as_mut_ptr(),
                &mut (*st).tonal_average,
                (*st).spread_decision,
                &mut (*st).hf_average,
                &mut (*st).tapset_decision,
                (pf_on != 0 && shortBlocks == 0) as libc::c_int,
                effEnd,
                C,
                M,
                spread_weight.as_mut_ptr(),
            );
        }
        ec_enc_icdf(
            enc,
            (*st).spread_decision,
            spread_icdf.as_ptr(),
            5 as libc::c_int as libc::c_uint,
        );
    }
    if (*st).lfe != 0 {
        *offsets.as_mut_ptr().offset(0 as libc::c_int as isize) =
            if (8 as libc::c_int) < effectiveBytes / 3 as libc::c_int {
                8 as libc::c_int
            } else {
                effectiveBytes / 3 as libc::c_int
            };
    }
    let vla_11 = nbEBands as usize;
    let mut cap: Vec<libc::c_int> = ::std::vec::from_elem(0, vla_11);
    init_caps(mode, cap.as_mut_ptr(), LM, C);
    dynalloc_logp = 6 as libc::c_int;
    total_bits <<= 3 as libc::c_int;
    total_boost = 0 as libc::c_int;
    tell = ec_tell_frac(enc) as opus_int32;
    i = start;
    while i < end {
        let mut width: libc::c_int = 0;
        let mut quanta: libc::c_int = 0;
        let mut dynalloc_loop_logp: libc::c_int = 0;
        let mut boost: libc::c_int = 0;
        let mut j: libc::c_int = 0;
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
        j = 0 as libc::c_int;
        while tell + (dynalloc_loop_logp << 3 as libc::c_int) < total_bits - total_boost
            && boost < *cap.as_mut_ptr().offset(i as isize)
        {
            let mut flag: libc::c_int = 0;
            flag = (j < *offsets.as_mut_ptr().offset(i as isize)) as libc::c_int;
            ec_enc_bit_logp(enc, flag, dynalloc_loop_logp as libc::c_uint);
            tell = ec_tell_frac(enc) as opus_int32;
            if flag == 0 {
                break;
            }
            boost += quanta;
            total_boost += quanta;
            dynalloc_loop_logp = 1 as libc::c_int;
            j += 1;
        }
        if j != 0 {
            dynalloc_logp = if 2 as libc::c_int > dynalloc_logp - 1 as libc::c_int {
                2 as libc::c_int
            } else {
                dynalloc_logp - 1 as libc::c_int
            };
        }
        *offsets.as_mut_ptr().offset(i as isize) = boost;
        i += 1;
    }
    if C == 2 as libc::c_int {
        static mut intensity_thresholds: [opus_val16; 21] = [
            1 as libc::c_int as opus_val16,
            2 as libc::c_int as opus_val16,
            3 as libc::c_int as opus_val16,
            4 as libc::c_int as opus_val16,
            5 as libc::c_int as opus_val16,
            6 as libc::c_int as opus_val16,
            7 as libc::c_int as opus_val16,
            8 as libc::c_int as opus_val16,
            16 as libc::c_int as opus_val16,
            24 as libc::c_int as opus_val16,
            36 as libc::c_int as opus_val16,
            44 as libc::c_int as opus_val16,
            50 as libc::c_int as opus_val16,
            56 as libc::c_int as opus_val16,
            62 as libc::c_int as opus_val16,
            67 as libc::c_int as opus_val16,
            72 as libc::c_int as opus_val16,
            79 as libc::c_int as opus_val16,
            88 as libc::c_int as opus_val16,
            106 as libc::c_int as opus_val16,
            134 as libc::c_int as opus_val16,
        ];
        static mut intensity_histeresis: [opus_val16; 21] = [
            1 as libc::c_int as opus_val16,
            1 as libc::c_int as opus_val16,
            1 as libc::c_int as opus_val16,
            1 as libc::c_int as opus_val16,
            1 as libc::c_int as opus_val16,
            1 as libc::c_int as opus_val16,
            1 as libc::c_int as opus_val16,
            2 as libc::c_int as opus_val16,
            2 as libc::c_int as opus_val16,
            2 as libc::c_int as opus_val16,
            2 as libc::c_int as opus_val16,
            2 as libc::c_int as opus_val16,
            2 as libc::c_int as opus_val16,
            2 as libc::c_int as opus_val16,
            3 as libc::c_int as opus_val16,
            3 as libc::c_int as opus_val16,
            4 as libc::c_int as opus_val16,
            5 as libc::c_int as opus_val16,
            6 as libc::c_int as opus_val16,
            8 as libc::c_int as opus_val16,
            8 as libc::c_int as opus_val16,
        ];
        if LM != 0 as libc::c_int {
            dual_stereo = stereo_analysis(mode, X.as_mut_ptr(), LM, N);
        }
        (*st).intensity = hysteresis_decision(
            (equiv_rate / 1000 as libc::c_int) as opus_val16,
            intensity_thresholds.as_ptr(),
            intensity_histeresis.as_ptr(),
            21 as libc::c_int,
            (*st).intensity,
        );
        (*st).intensity = if end
            < (if start > (*st).intensity {
                start
            } else {
                (*st).intensity
            }) {
            end
        } else if start > (*st).intensity {
            start
        } else {
            (*st).intensity
        };
    }
    alloc_trim = 5 as libc::c_int;
    if tell + ((6 as libc::c_int) << 3 as libc::c_int) <= total_bits - total_boost {
        if start > 0 as libc::c_int || (*st).lfe != 0 {
            (*st).stereo_saving = 0 as libc::c_int as opus_val16;
            alloc_trim = 5 as libc::c_int;
        } else {
            alloc_trim = alloc_trim_analysis(
                mode,
                X.as_mut_ptr(),
                bandLogE.as_mut_ptr(),
                end,
                LM,
                C,
                N,
                &mut (*st).analysis,
                &mut (*st).stereo_saving,
                tf_estimate,
                (*st).intensity,
                surround_trim,
                equiv_rate,
                (*st).arch,
            );
        }
        ec_enc_icdf(
            enc,
            alloc_trim,
            trim_icdf.as_ptr(),
            7 as libc::c_int as libc::c_uint,
        );
        tell = ec_tell_frac(enc) as opus_int32;
    }
    if vbr_rate > 0 as libc::c_int {
        let mut alpha: opus_val16 = 0.;
        let mut delta: opus_int32 = 0;
        let mut target: opus_int32 = 0;
        let mut base_target: opus_int32 = 0;
        let mut min_allowed: opus_int32 = 0;
        let mut lm_diff: libc::c_int = (*mode).maxLM - LM;
        nbCompressedBytes = if nbCompressedBytes < 1275 as libc::c_int >> 3 as libc::c_int - LM {
            nbCompressedBytes
        } else {
            1275 as libc::c_int >> 3 as libc::c_int - LM
        };
        if hybrid == 0 {
            base_target =
                vbr_rate - ((40 as libc::c_int * C + 20 as libc::c_int) << 3 as libc::c_int);
        } else {
            base_target = if 0 as libc::c_int
                > vbr_rate - ((9 as libc::c_int * C + 4 as libc::c_int) << 3 as libc::c_int)
            {
                0 as libc::c_int
            } else {
                vbr_rate - ((9 as libc::c_int * C + 4 as libc::c_int) << 3 as libc::c_int)
            };
        }
        if (*st).constrained_vbr != 0 {
            base_target += (*st).vbr_offset >> lm_diff;
        }
        if hybrid == 0 {
            target = compute_vbr(
                mode,
                &mut (*st).analysis,
                base_target,
                LM,
                equiv_rate,
                (*st).lastCodedBands,
                C,
                (*st).intensity,
                (*st).constrained_vbr,
                (*st).stereo_saving,
                tot_boost,
                tf_estimate,
                pitch_change,
                maxDepth,
                (*st).lfe,
                ((*st).energy_mask != 0 as *mut libc::c_void as *mut opus_val16) as libc::c_int,
                surround_masking,
                temporal_vbr,
            );
        } else {
            target = base_target;
            if (*st).silk_info.offset < 100 as libc::c_int {
                target += (12 as libc::c_int) << 3 as libc::c_int >> 3 as libc::c_int - LM;
            }
            if (*st).silk_info.offset > 100 as libc::c_int {
                target -= (18 as libc::c_int) << 3 as libc::c_int >> 3 as libc::c_int - LM;
            }
            target += ((tf_estimate - 0.25f32)
                * ((50 as libc::c_int) << 3 as libc::c_int) as libc::c_float)
                as opus_int32;
            if tf_estimate > 0.7f32 {
                target = if target > (50 as libc::c_int) << 3 as libc::c_int {
                    target
                } else {
                    (50 as libc::c_int) << 3 as libc::c_int
                };
            }
        }
        target = target + tell;
        min_allowed =
            (tell + total_boost + ((1 as libc::c_int) << 3 as libc::c_int + 3 as libc::c_int)
                - 1 as libc::c_int
                >> 3 as libc::c_int + 3 as libc::c_int)
                + 2 as libc::c_int;
        if hybrid != 0 {
            min_allowed = if min_allowed
                > tell0_frac
                    + ((37 as libc::c_int) << 3 as libc::c_int)
                    + total_boost
                    + ((1 as libc::c_int) << 3 as libc::c_int + 3 as libc::c_int)
                    - 1 as libc::c_int
                    >> 3 as libc::c_int + 3 as libc::c_int
            {
                min_allowed
            } else {
                tell0_frac
                    + ((37 as libc::c_int) << 3 as libc::c_int)
                    + total_boost
                    + ((1 as libc::c_int) << 3 as libc::c_int + 3 as libc::c_int)
                    - 1 as libc::c_int
                    >> 3 as libc::c_int + 3 as libc::c_int
            };
        }
        nbAvailableBytes = target + ((1 as libc::c_int) << 3 as libc::c_int + 2 as libc::c_int)
            >> 3 as libc::c_int + 3 as libc::c_int;
        nbAvailableBytes = if min_allowed > nbAvailableBytes {
            min_allowed
        } else {
            nbAvailableBytes
        };
        nbAvailableBytes = if nbCompressedBytes < nbAvailableBytes {
            nbCompressedBytes
        } else {
            nbAvailableBytes
        };
        delta = target - vbr_rate;
        target = nbAvailableBytes << 3 as libc::c_int + 3 as libc::c_int;
        if silence != 0 {
            nbAvailableBytes = 2 as libc::c_int;
            target = (2 as libc::c_int * 8 as libc::c_int) << 3 as libc::c_int;
            delta = 0 as libc::c_int;
        }
        if (*st).vbr_count < 970 as libc::c_int {
            (*st).vbr_count += 1;
            alpha = 1.0f32 / ((*st).vbr_count + 20 as libc::c_int) as libc::c_float;
        } else {
            alpha = 0.001f32;
        }
        if (*st).constrained_vbr != 0 {
            (*st).vbr_reservoir += target - vbr_rate;
        }
        if (*st).constrained_vbr != 0 {
            (*st).vbr_drift += (alpha
                * (delta * ((1 as libc::c_int) << lm_diff) - (*st).vbr_offset - (*st).vbr_drift)
                    as libc::c_float) as opus_int32;
            (*st).vbr_offset = -(*st).vbr_drift;
        }
        if (*st).constrained_vbr != 0 && (*st).vbr_reservoir < 0 as libc::c_int {
            let mut adjust: libc::c_int =
                -(*st).vbr_reservoir / ((8 as libc::c_int) << 3 as libc::c_int);
            nbAvailableBytes += if silence != 0 {
                0 as libc::c_int
            } else {
                adjust
            };
            (*st).vbr_reservoir = 0 as libc::c_int;
        }
        nbCompressedBytes = if nbCompressedBytes < nbAvailableBytes {
            nbCompressedBytes
        } else {
            nbAvailableBytes
        };
        ec_enc_shrink(enc, nbCompressedBytes as opus_uint32);
    }
    let vla_12 = nbEBands as usize;
    let mut fine_quant: Vec<libc::c_int> = ::std::vec::from_elem(0, vla_12);
    let vla_13 = nbEBands as usize;
    let mut pulses: Vec<libc::c_int> = ::std::vec::from_elem(0, vla_13);
    let vla_14 = nbEBands as usize;
    let mut fine_priority: Vec<libc::c_int> = ::std::vec::from_elem(0, vla_14);
    bits = (((nbCompressedBytes * 8 as libc::c_int) << 3 as libc::c_int) as libc::c_uint)
        .wrapping_sub(ec_tell_frac(enc))
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
    signalBandwidth = end - 1 as libc::c_int;
    if (*st).analysis.valid != 0 {
        let mut min_bandwidth: libc::c_int = 0;
        if equiv_rate < 32000 as libc::c_int * C {
            min_bandwidth = 13 as libc::c_int;
        } else if equiv_rate < 48000 as libc::c_int * C {
            min_bandwidth = 16 as libc::c_int;
        } else if equiv_rate < 60000 as libc::c_int * C {
            min_bandwidth = 18 as libc::c_int;
        } else if equiv_rate < 80000 as libc::c_int * C {
            min_bandwidth = 19 as libc::c_int;
        } else {
            min_bandwidth = 20 as libc::c_int;
        }
        signalBandwidth = if (*st).analysis.bandwidth > min_bandwidth {
            (*st).analysis.bandwidth
        } else {
            min_bandwidth
        };
    }
    if (*st).lfe != 0 {
        signalBandwidth = 1 as libc::c_int;
    }
    codedBands = clt_compute_allocation(
        mode,
        start,
        end,
        offsets.as_mut_ptr(),
        cap.as_mut_ptr(),
        alloc_trim,
        &mut (*st).intensity,
        &mut dual_stereo,
        bits,
        &mut balance,
        pulses.as_mut_ptr(),
        fine_quant.as_mut_ptr(),
        fine_priority.as_mut_ptr(),
        C,
        LM,
        enc,
        1 as libc::c_int,
        (*st).lastCodedBands,
        signalBandwidth,
    );
    if (*st).lastCodedBands != 0 {
        (*st).lastCodedBands = if ((*st).lastCodedBands + 1 as libc::c_int)
            < (if (*st).lastCodedBands - 1 as libc::c_int > codedBands {
                (*st).lastCodedBands - 1 as libc::c_int
            } else {
                codedBands
            }) {
            (*st).lastCodedBands + 1 as libc::c_int
        } else if (*st).lastCodedBands - 1 as libc::c_int > codedBands {
            (*st).lastCodedBands - 1 as libc::c_int
        } else {
            codedBands
        };
    } else {
        (*st).lastCodedBands = codedBands;
    }
    quant_fine_energy(
        mode,
        start,
        end,
        oldBandE,
        error.as_mut_ptr(),
        fine_quant.as_mut_ptr(),
        enc,
        C,
    );
    let vla_15 = (C * nbEBands) as usize;
    let mut collapse_masks: Vec<libc::c_uchar> = ::std::vec::from_elem(0, vla_15);
    quant_all_bands(
        1 as libc::c_int,
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
        bandE.as_mut_ptr(),
        pulses.as_mut_ptr(),
        shortBlocks,
        (*st).spread_decision,
        dual_stereo,
        (*st).intensity,
        tf_res.as_mut_ptr(),
        nbCompressedBytes * ((8 as libc::c_int) << 3 as libc::c_int) - anti_collapse_rsv,
        balance,
        enc,
        LM,
        codedBands,
        &mut (*st).rng,
        (*st).complexity,
        (*st).arch,
        (*st).disable_inv,
    );
    if anti_collapse_rsv > 0 as libc::c_int {
        anti_collapse_on = ((*st).consec_transient < 2 as libc::c_int) as libc::c_int;
        ec_enc_bits(
            enc,
            anti_collapse_on as opus_uint32,
            1 as libc::c_int as libc::c_uint,
        );
    }
    quant_energy_finalise(
        mode,
        start,
        end,
        oldBandE,
        error.as_mut_ptr(),
        fine_quant.as_mut_ptr(),
        fine_priority.as_mut_ptr(),
        nbCompressedBytes * 8 as libc::c_int - ec_tell(enc),
        enc,
        C,
    );
    memset(
        energyError as *mut libc::c_void,
        0 as libc::c_int,
        ((nbEBands * CC) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<opus_val16>() as libc::c_ulong),
    );
    c = 0 as libc::c_int;
    loop {
        i = start;
        while i < end {
            *energyError.offset((i + c * nbEBands) as isize) = if -0.5f32
                > (if 0.5f32 < *error.as_mut_ptr().offset((i + c * nbEBands) as isize) {
                    0.5f32
                } else {
                    *error.as_mut_ptr().offset((i + c * nbEBands) as isize)
                }) {
                -0.5f32
            } else if 0.5f32 < *error.as_mut_ptr().offset((i + c * nbEBands) as isize) {
                0.5f32
            } else {
                *error.as_mut_ptr().offset((i + c * nbEBands) as isize)
            };
            i += 1;
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
    if silence != 0 {
        i = 0 as libc::c_int;
        while i < C * nbEBands {
            *oldBandE.offset(i as isize) = -28.0f32;
            i += 1;
        }
    }
    (*st).prefilter_period = pitch_index;
    (*st).prefilter_gain = gain1;
    (*st).prefilter_tapset = prefilter_tapset;
    if CC == 2 as libc::c_int && C == 1 as libc::c_int {
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
        memcpy(
            oldLogE2 as *mut libc::c_void,
            oldLogE as *const libc::c_void,
            ((CC * nbEBands) as libc::c_ulong)
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
            ((CC * nbEBands) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<opus_val16>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * oldLogE.offset_from(oldBandE) as libc::c_long)
                        as libc::c_ulong,
                ),
        );
    } else {
        i = 0 as libc::c_int;
        while i < CC * nbEBands {
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
            let ref mut fresh5 = *oldLogE2.offset((c * nbEBands + i) as isize);
            *fresh5 = -28.0f32;
            *oldLogE.offset((c * nbEBands + i) as isize) = *fresh5;
            i += 1;
        }
        i = end;
        while i < nbEBands {
            *oldBandE.offset((c * nbEBands + i) as isize) = 0 as libc::c_int as opus_val16;
            let ref mut fresh6 = *oldLogE2.offset((c * nbEBands + i) as isize);
            *fresh6 = -28.0f32;
            *oldLogE.offset((c * nbEBands + i) as isize) = *fresh6;
            i += 1;
        }
        c += 1;
        if !(c < CC) {
            break;
        }
    }
    if isTransient != 0 || transient_got_disabled != 0 {
        (*st).consec_transient += 1;
    } else {
        (*st).consec_transient = 0 as libc::c_int;
    }
    (*st).rng = (*enc).rng;
    ec_enc_done(enc);
    if ec_get_error(enc) != 0 {
        return -(3 as libc::c_int);
    } else {
        return nbCompressedBytes;
    };
}
#[no_mangle]
#[c2rust::src_loc = "2409:1"]
pub unsafe extern "C" fn opus_custom_encoder_ctl(
    mut st: *mut OpusCustomEncoder,
    mut request: libc::c_int,
    mut args: ...
) -> libc::c_int {
    let mut current_block: u64;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    match request {
        4010 => {
            let mut value: libc::c_int = ap.arg::<opus_int32>();
            if value < 0 as libc::c_int || value > 10 as libc::c_int {
                current_block = 2472048668343472511;
            } else {
                (*st).complexity = value;
                current_block = 10007731352114176167;
            }
        }
        10010 => {
            let mut value_0: opus_int32 = ap.arg::<opus_int32>();
            if value_0 < 0 as libc::c_int || value_0 >= (*(*st).mode).nbEBands {
                current_block = 2472048668343472511;
            } else {
                (*st).start = value_0;
                current_block = 10007731352114176167;
            }
        }
        10012 => {
            let mut value_1: opus_int32 = ap.arg::<opus_int32>();
            if value_1 < 1 as libc::c_int || value_1 > (*(*st).mode).nbEBands {
                current_block = 2472048668343472511;
            } else {
                (*st).end = value_1;
                current_block = 10007731352114176167;
            }
        }
        10002 => {
            let mut value_2: libc::c_int = ap.arg::<opus_int32>();
            if value_2 < 0 as libc::c_int || value_2 > 2 as libc::c_int {
                current_block = 2472048668343472511;
            } else {
                (*st).disable_pf = (value_2 <= 1 as libc::c_int) as libc::c_int;
                (*st).force_intra = (value_2 == 0 as libc::c_int) as libc::c_int;
                current_block = 10007731352114176167;
            }
        }
        4014 => {
            let mut value_3: libc::c_int = ap.arg::<opus_int32>();
            if value_3 < 0 as libc::c_int || value_3 > 100 as libc::c_int {
                current_block = 2472048668343472511;
            } else {
                (*st).loss_rate = value_3;
                current_block = 10007731352114176167;
            }
        }
        4020 => {
            let mut value_4: opus_int32 = ap.arg::<opus_int32>();
            (*st).constrained_vbr = value_4;
            current_block = 10007731352114176167;
        }
        4006 => {
            let mut value_5: opus_int32 = ap.arg::<opus_int32>();
            (*st).vbr = value_5;
            current_block = 10007731352114176167;
        }
        4002 => {
            let mut value_6: opus_int32 = ap.arg::<opus_int32>();
            if value_6 <= 500 as libc::c_int && value_6 != -(1 as libc::c_int) {
                current_block = 2472048668343472511;
            } else {
                value_6 = if value_6 < 260000 as libc::c_int * (*st).channels {
                    value_6
                } else {
                    260000 as libc::c_int * (*st).channels
                };
                (*st).bitrate = value_6;
                current_block = 10007731352114176167;
            }
        }
        10008 => {
            let mut value_7: opus_int32 = ap.arg::<opus_int32>();
            if value_7 < 1 as libc::c_int || value_7 > 2 as libc::c_int {
                current_block = 2472048668343472511;
            } else {
                (*st).stream_channels = value_7;
                current_block = 10007731352114176167;
            }
        }
        4036 => {
            let mut value_8: opus_int32 = ap.arg::<opus_int32>();
            if value_8 < 8 as libc::c_int || value_8 > 24 as libc::c_int {
                current_block = 2472048668343472511;
            } else {
                (*st).lsb_depth = value_8;
                current_block = 10007731352114176167;
            }
        }
        4037 => {
            let mut value_9: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            *value_9 = (*st).lsb_depth;
            current_block = 10007731352114176167;
        }
        4046 => {
            let mut value_10: opus_int32 = ap.arg::<opus_int32>();
            if value_10 < 0 as libc::c_int || value_10 > 1 as libc::c_int {
                current_block = 2472048668343472511;
            } else {
                (*st).disable_inv = value_10;
                current_block = 10007731352114176167;
            }
        }
        4047 => {
            let mut value_11: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_11.is_null() {
                current_block = 2472048668343472511;
            } else {
                *value_11 = (*st).disable_inv;
                current_block = 10007731352114176167;
            }
        }
        4028 => {
            let mut i: libc::c_int = 0;
            let mut oldBandE: *mut opus_val16 = 0 as *mut opus_val16;
            let mut oldLogE: *mut opus_val16 = 0 as *mut opus_val16;
            let mut oldLogE2: *mut opus_val16 = 0 as *mut opus_val16;
            oldBandE = ((*st).in_mem)
                .as_mut_ptr()
                .offset(((*st).channels * ((*(*st).mode).overlap + 1024 as libc::c_int)) as isize)
                as *mut opus_val16;
            oldLogE = oldBandE.offset(((*st).channels * (*(*st).mode).nbEBands) as isize);
            oldLogE2 = oldLogE.offset(((*st).channels * (*(*st).mode).nbEBands) as isize);
            memset(
                &mut (*st).rng as *mut opus_uint32 as *mut libc::c_char as *mut libc::c_void,
                0 as libc::c_int,
                ((opus_custom_encoder_get_size((*st).mode, (*st).channels) as libc::c_long
                    - (&mut (*st).rng as *mut opus_uint32 as *mut libc::c_char)
                        .offset_from(st as *mut libc::c_char) as libc::c_long)
                    as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
            i = 0 as libc::c_int;
            while i < (*st).channels * (*(*st).mode).nbEBands {
                let ref mut fresh7 = *oldLogE2.offset(i as isize);
                *fresh7 = -28.0f32;
                *oldLogE.offset(i as isize) = *fresh7;
                i += 1;
            }
            (*st).vbr_offset = 0 as libc::c_int;
            (*st).delayedIntra = 1 as libc::c_int as opus_val32;
            (*st).spread_decision = 2 as libc::c_int;
            (*st).tonal_average = 256 as libc::c_int;
            (*st).hf_average = 0 as libc::c_int;
            (*st).tapset_decision = 0 as libc::c_int;
            current_block = 10007731352114176167;
        }
        10016 => {
            let mut value_12: opus_int32 = ap.arg::<opus_int32>();
            (*st).signalling = value_12;
            current_block = 10007731352114176167;
        }
        10022 => {
            let mut info: *mut AnalysisInfo = ap.arg::<*mut AnalysisInfo>();
            if !info.is_null() {
                memcpy(
                    &mut (*st).analysis as *mut AnalysisInfo as *mut libc::c_void,
                    info as *const libc::c_void,
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<AnalysisInfo>() as libc::c_ulong)
                        .wrapping_add(
                            (0 as libc::c_int as libc::c_long
                                * (&mut (*st).analysis as *mut AnalysisInfo).offset_from(info)
                                    as libc::c_long) as libc::c_ulong,
                        ),
                );
            }
            current_block = 10007731352114176167;
        }
        10028 => {
            let mut info_0: *mut SILKInfo = ap.arg::<*mut SILKInfo>();
            if !info_0.is_null() {
                memcpy(
                    &mut (*st).silk_info as *mut SILKInfo as *mut libc::c_void,
                    info_0 as *const libc::c_void,
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<SILKInfo>() as libc::c_ulong)
                        .wrapping_add(
                            (0 as libc::c_int as libc::c_long
                                * (&mut (*st).silk_info as *mut SILKInfo).offset_from(info_0)
                                    as libc::c_long) as libc::c_ulong,
                        ),
                );
            }
            current_block = 10007731352114176167;
        }
        10015 => {
            let mut value_13: *mut *const OpusCustomMode = ap.arg::<*mut *const OpusCustomMode>();
            if value_13.is_null() {
                current_block = 2472048668343472511;
            } else {
                *value_13 = (*st).mode;
                current_block = 10007731352114176167;
            }
        }
        4031 => {
            let mut value_14: *mut opus_uint32 = ap.arg::<*mut opus_uint32>();
            if value_14.is_null() {
                current_block = 2472048668343472511;
            } else {
                *value_14 = (*st).rng;
                current_block = 10007731352114176167;
            }
        }
        10024 => {
            let mut value_15: opus_int32 = ap.arg::<opus_int32>();
            (*st).lfe = value_15;
            current_block = 10007731352114176167;
        }
        10026 => {
            let mut value_16: *mut opus_val16 = ap.arg::<*mut opus_val16>();
            (*st).energy_mask = value_16;
            current_block = 10007731352114176167;
        }
        _ => return -(5 as libc::c_int),
    }
    match current_block {
        10007731352114176167 => return 0 as libc::c_int,
        _ => return -(1 as libc::c_int),
    };
}
