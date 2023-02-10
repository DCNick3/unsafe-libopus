use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:34"]
pub mod types_h {
    #[c2rust::src_loc = "37:1"]
    pub type __int8_t = libc::c_schar;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:36"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "24:1"]
    pub type int8_t = __int8_t;
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int8_t, __int16_t, __int32_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:36"]
pub mod opus_types_h {
    #[c2rust::src_loc = "51:4"]
    pub type opus_int8 = int8_t;
    #[c2rust::src_loc = "53:4"]
    pub type opus_int16 = int16_t;
    #[c2rust::src_loc = "55:4"]
    pub type opus_int32 = int32_t;
    use super::stdint_intn_h::{int8_t, int16_t, int32_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:36"]
pub mod arch_h {
    #[c2rust::src_loc = "179:1"]
    pub type opus_val16 = libc::c_float;
    #[c2rust::src_loc = "180:1"]
    pub type opus_val32 = libc::c_float;
    #[c2rust::src_loc = "181:1"]
    pub type opus_val64 = libc::c_float;
    extern "C" {
        #[c2rust::src_loc = "63:1"]
        pub fn celt_fatal(
            str: *const libc::c_char,
            file: *const libc::c_char,
            line: libc::c_int,
        ) -> !;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/kiss_fft.h:37"]
pub mod kiss_fft_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "65:9"]
    pub struct kiss_fft_cpx {
        pub r: libc::c_float,
        pub i: libc::c_float,
    }
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
    extern "C" {
        #[c2rust::src_loc = "142:1"]
        pub fn opus_fft_c(
            cfg: *const kiss_fft_state,
            fin: *const kiss_fft_cpx,
            fout: *mut kiss_fft_cpx,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/modes.h:39"]
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
    use super::opus_types_h::{opus_int32, opus_int16};
    use super::arch_h::opus_val16;
    use super::mdct_h::mdct_lookup;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/mdct.h:39"]
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
    use super::kiss_fft_h::kiss_fft_state;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/celt.h:38"]
pub mod celt_h {
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
    #[c2rust::src_loc = "53:9"]
    pub const LEAK_BANDS: libc::c_int = 19 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/src/opus_private.h:42"]
pub mod opus_private_h {
    #[c2rust::src_loc = "135:1"]
    pub type downmix_func = Option::<
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
    use super::arch_h::{opus_val32, opus_val16};
    extern "C" {
        #[c2rust::src_loc = "138:1"]
        pub fn is_digital_silence(
            pcm: *const opus_val16,
            frame_size: libc::c_int,
            channels: libc::c_int,
            lsb_depth: libc::c_int,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/src/mlp.h:42"]
pub mod mlp_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:9"]
    pub struct DenseLayer {
        pub bias: *const opus_int8,
        pub input_weights: *const opus_int8,
        pub nb_inputs: libc::c_int,
        pub nb_neurons: libc::c_int,
        pub sigmoid: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "44:9"]
    pub struct GRULayer {
        pub bias: *const opus_int8,
        pub input_weights: *const opus_int8,
        pub recurrent_weights: *const opus_int8,
        pub nb_inputs: libc::c_int,
        pub nb_neurons: libc::c_int,
    }
    use super::opus_types_h::opus_int8;
    extern "C" {
        #[c2rust::src_loc = "52:25"]
        pub static layer0: DenseLayer;
        #[c2rust::src_loc = "53:23"]
        pub static layer1: GRULayer;
        #[c2rust::src_loc = "54:25"]
        pub static layer2: DenseLayer;
        #[c2rust::src_loc = "56:1"]
        pub fn compute_dense(
            layer: *const DenseLayer,
            output: *mut libc::c_float,
            input: *const libc::c_float,
        );
        #[c2rust::src_loc = "58:1"]
        pub fn compute_gru(
            gru: *const GRULayer,
            state: *mut libc::c_float,
            input: *const libc::c_float,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/src/analysis.h:42"]
pub mod analysis_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "47:9"]
    pub struct TonalityAnalysisState {
        pub arch: libc::c_int,
        pub application: libc::c_int,
        pub Fs: opus_int32,
        pub angle: [libc::c_float; 240],
        pub d_angle: [libc::c_float; 240],
        pub d2_angle: [libc::c_float; 240],
        pub inmem: [opus_val32; 720],
        pub mem_fill: libc::c_int,
        pub prev_band_tonality: [libc::c_float; 18],
        pub prev_tonality: libc::c_float,
        pub prev_bandwidth: libc::c_int,
        pub E: [[libc::c_float; 18]; 8],
        pub logE: [[libc::c_float; 18]; 8],
        pub lowE: [libc::c_float; 18],
        pub highE: [libc::c_float; 18],
        pub meanE: [libc::c_float; 19],
        pub mem: [libc::c_float; 32],
        pub cmean: [libc::c_float; 8],
        pub std: [libc::c_float; 9],
        pub Etracker: libc::c_float,
        pub lowECount: libc::c_float,
        pub E_count: libc::c_int,
        pub count: libc::c_int,
        pub analysis_offset: libc::c_int,
        pub write_pos: libc::c_int,
        pub read_pos: libc::c_int,
        pub read_subframe: libc::c_int,
        pub hp_ener_accum: libc::c_float,
        pub initialized: libc::c_int,
        pub rnn_state: [libc::c_float; 32],
        pub downmix_state: [opus_val32; 3],
        pub info: [AnalysisInfo; 100],
    }
    #[c2rust::src_loc = "37:9"]
    pub const ANALYSIS_BUF_SIZE: libc::c_int = 720 as libc::c_int;
    #[c2rust::src_loc = "42:9"]
    pub const DETECT_SIZE: libc::c_int = 100 as libc::c_int;
    #[c2rust::src_loc = "35:9"]
    pub const NB_FRAMES: libc::c_int = 8 as libc::c_int;
    #[c2rust::src_loc = "36:9"]
    pub const NB_TBANDS: libc::c_int = 18 as libc::c_int;
    use super::opus_types_h::opus_int32;
    use super::arch_h::opus_val32;
    use super::celt_h::AnalysisInfo;
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/xmmintrin.h:45"]
pub mod xmmintrin_h {
    #[cfg(target_arch = "x86")]
    pub use core::arch::x86::{__m128, _mm_set_ss, _mm_cvt_ss2si, _mm_cvtss_si32};
    #[cfg(target_arch = "x86_64")]
    pub use core::arch::x86_64::{__m128, _mm_set_ss, _mm_cvt_ss2si, _mm_cvtss_si32};
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:34"]
pub mod stddef_h {
    #[c2rust::src_loc = "89:11"]
    pub const NULL: libc::c_int = 0 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/math.h:36"]
pub mod math_h {
    #[c2rust::src_loc = "1151:10"]
    pub const M_PI: libc::c_double = 3.14159265358979323846f64;
}
#[c2rust::header_src = "/usr/include/bits/mathcalls.h:36"]
pub mod mathcalls_h {
    extern "C" {
        #[c2rust::src_loc = "104:17"]
        pub fn log(_: libc::c_double) -> libc::c_double;
        #[c2rust::src_loc = "107:17"]
        pub fn log10(_: libc::c_double) -> libc::c_double;
        #[c2rust::src_loc = "143:13"]
        pub fn sqrt(_: libc::c_double) -> libc::c_double;
        #[c2rust::src_loc = "162:14"]
        pub fn fabs(_: libc::c_double) -> libc::c_double;
        #[c2rust::src_loc = "165:14"]
        pub fn floor(_: libc::c_double) -> libc::c_double;
    }
}
#[c2rust::header_src = "/usr/include/string.h:36"]
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
        pub fn memset(
            _: *mut libc::c_void,
            _: libc::c_int,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/mathops.h:36"]
pub mod mathops_h {
    #[inline]
    #[c2rust::src_loc = "54:1"]
    pub unsafe extern "C" fn fast_atan2f(
        y: libc::c_float,
        x: libc::c_float,
    ) -> libc::c_float {
        let mut x2: libc::c_float = 0.;
        let mut y2: libc::c_float = 0.;
        x2 = x * x;
        y2 = y * y;
        if x2 + y2 < 1e-18f32 {
            return 0 as libc::c_int as libc::c_float;
        }
        if x2 < y2 {
            let den: libc::c_float = (y2 + cB * x2) * (y2 + cC * x2);
            return -x * y * (y2 + cA * x2) / den
                + (if y < 0 as libc::c_int as libc::c_float { -cE } else { cE });
        } else {
            let den_0: libc::c_float = (x2 + cB * y2) * (x2 + cC * y2);
            return x * y * (x2 + cA * y2) / den_0
                + (if y < 0 as libc::c_int as libc::c_float { -cE } else { cE })
                - (if x * y < 0 as libc::c_int as libc::c_float { -cE } else { cE });
        };
    }
    #[c2rust::src_loc = "41:9"]
    pub const PI: libc::c_float = 3.141592653f32;
    #[c2rust::src_loc = "53:9"]
    pub const cE: libc::c_float = PI / 2 as libc::c_int as libc::c_float;
    #[c2rust::src_loc = "52:9"]
    pub const cC: libc::c_float = 0.08595542f32;
    #[c2rust::src_loc = "51:9"]
    pub const cB: libc::c_float = 0.67848403f32;
    #[c2rust::src_loc = "50:9"]
    pub const cA: libc::c_float = 0.43157974f32;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/cpu_support.h:37"]
pub mod cpu_support_h {
    #[inline]
    #[c2rust::src_loc = "65:1"]
    pub unsafe extern "C" fn opus_select_arch() -> libc::c_int {
        return 0 as libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/float_cast.h:45"]
pub mod float_cast_h {
    #[inline]
    #[c2rust::src_loc = "68:1"]
    pub unsafe extern "C" fn float2int(x: libc::c_float) -> opus_int32 {
        return _mm_cvt_ss2si(_mm_set_ss(x));
    }
    use super::opus_types_h::opus_int32;
    use super::xmmintrin_h::{_mm_cvt_ss2si, _mm_set_ss};
}
pub use self::types_h::{__int8_t, __int16_t, __int32_t};
pub use self::stdint_intn_h::{int8_t, int16_t, int32_t};
pub use self::opus_types_h::{opus_int8, opus_int16, opus_int32};
pub use self::arch_h::{opus_val16, opus_val32, opus_val64, celt_fatal};
pub use self::kiss_fft_h::{
    kiss_fft_cpx, kiss_twiddle_cpx, arch_fft_state, kiss_fft_state, opus_fft_c,
};
pub use self::modes_h::{OpusCustomMode, PulseCache};
pub use self::mdct_h::mdct_lookup;
pub use self::celt_h::{AnalysisInfo, LEAK_BANDS};
pub use self::opus_private_h::{downmix_func, is_digital_silence};
pub use self::mlp_h::{
    DenseLayer, GRULayer, layer0, layer1, layer2, compute_dense, compute_gru,
};
pub use self::analysis_h::{
    TonalityAnalysisState, ANALYSIS_BUF_SIZE, DETECT_SIZE, NB_FRAMES, NB_TBANDS,
};
pub use self::stddef_h::NULL;
pub use self::math_h::M_PI;
use self::mathcalls_h::{log, log10, sqrt, fabs, floor};
use self::string_h::{memcpy, memmove, memset};
pub use self::mathops_h::{fast_atan2f, PI, cE, cC, cB, cA};
pub use self::cpu_support_h::opus_select_arch;
pub use self::float_cast_h::float2int;
#[c2rust::src_loc = "55:20"]
static mut dct_table: [libc::c_float; 128] = [
    0.250000f32,
    0.250000f32,
    0.250000f32,
    0.250000f32,
    0.250000f32,
    0.250000f32,
    0.250000f32,
    0.250000f32,
    0.250000f32,
    0.250000f32,
    0.250000f32,
    0.250000f32,
    0.250000f32,
    0.250000f32,
    0.250000f32,
    0.250000f32,
    0.351851f32,
    0.338330f32,
    0.311806f32,
    0.273300f32,
    0.224292f32,
    0.166664f32,
    0.102631f32,
    0.034654f32,
    -0.034654f32,
    -0.102631f32,
    -0.166664f32,
    -0.224292f32,
    -0.273300f32,
    -0.311806f32,
    -0.338330f32,
    -0.351851f32,
    0.346760f32,
    0.293969f32,
    0.196424f32,
    0.068975f32,
    -0.068975f32,
    -0.196424f32,
    -0.293969f32,
    -0.346760f32,
    -0.346760f32,
    -0.293969f32,
    -0.196424f32,
    -0.068975f32,
    0.068975f32,
    0.196424f32,
    0.293969f32,
    0.346760f32,
    0.338330f32,
    0.224292f32,
    0.034654f32,
    -0.166664f32,
    -0.311806f32,
    -0.351851f32,
    -0.273300f32,
    -0.102631f32,
    0.102631f32,
    0.273300f32,
    0.351851f32,
    0.311806f32,
    0.166664f32,
    -0.034654f32,
    -0.224292f32,
    -0.338330f32,
    0.326641f32,
    0.135299f32,
    -0.135299f32,
    -0.326641f32,
    -0.326641f32,
    -0.135299f32,
    0.135299f32,
    0.326641f32,
    0.326641f32,
    0.135299f32,
    -0.135299f32,
    -0.326641f32,
    -0.326641f32,
    -0.135299f32,
    0.135299f32,
    0.326641f32,
    0.311806f32,
    0.034654f32,
    -0.273300f32,
    -0.338330f32,
    -0.102631f32,
    0.224292f32,
    0.351851f32,
    0.166664f32,
    -0.166664f32,
    -0.351851f32,
    -0.224292f32,
    0.102631f32,
    0.338330f32,
    0.273300f32,
    -0.034654f32,
    -0.311806f32,
    0.293969f32,
    -0.068975f32,
    -0.346760f32,
    -0.196424f32,
    0.196424f32,
    0.346760f32,
    0.068975f32,
    -0.293969f32,
    -0.293969f32,
    0.068975f32,
    0.346760f32,
    0.196424f32,
    -0.196424f32,
    -0.346760f32,
    -0.068975f32,
    0.293969f32,
    0.273300f32,
    -0.166664f32,
    -0.338330f32,
    0.034654f32,
    0.351851f32,
    0.102631f32,
    -0.311806f32,
    -0.224292f32,
    0.224292f32,
    0.311806f32,
    -0.102631f32,
    -0.351851f32,
    -0.034654f32,
    0.338330f32,
    0.166664f32,
    -0.273300f32,
];
#[c2rust::src_loc = "74:20"]
static mut analysis_window: [libc::c_float; 240] = [
    0.000043f32,
    0.000171f32,
    0.000385f32,
    0.000685f32,
    0.001071f32,
    0.001541f32,
    0.002098f32,
    0.002739f32,
    0.003466f32,
    0.004278f32,
    0.005174f32,
    0.006156f32,
    0.007222f32,
    0.008373f32,
    0.009607f32,
    0.010926f32,
    0.012329f32,
    0.013815f32,
    0.015385f32,
    0.017037f32,
    0.018772f32,
    0.020590f32,
    0.022490f32,
    0.024472f32,
    0.026535f32,
    0.028679f32,
    0.030904f32,
    0.033210f32,
    0.035595f32,
    0.038060f32,
    0.040604f32,
    0.043227f32,
    0.045928f32,
    0.048707f32,
    0.051564f32,
    0.054497f32,
    0.057506f32,
    0.060591f32,
    0.063752f32,
    0.066987f32,
    0.070297f32,
    0.073680f32,
    0.077136f32,
    0.080665f32,
    0.084265f32,
    0.087937f32,
    0.091679f32,
    0.095492f32,
    0.099373f32,
    0.103323f32,
    0.107342f32,
    0.111427f32,
    0.115579f32,
    0.119797f32,
    0.124080f32,
    0.128428f32,
    0.132839f32,
    0.137313f32,
    0.141849f32,
    0.146447f32,
    0.151105f32,
    0.155823f32,
    0.160600f32,
    0.165435f32,
    0.170327f32,
    0.175276f32,
    0.180280f32,
    0.185340f32,
    0.190453f32,
    0.195619f32,
    0.200838f32,
    0.206107f32,
    0.211427f32,
    0.216797f32,
    0.222215f32,
    0.227680f32,
    0.233193f32,
    0.238751f32,
    0.244353f32,
    0.250000f32,
    0.255689f32,
    0.261421f32,
    0.267193f32,
    0.273005f32,
    0.278856f32,
    0.284744f32,
    0.290670f32,
    0.296632f32,
    0.302628f32,
    0.308658f32,
    0.314721f32,
    0.320816f32,
    0.326941f32,
    0.333097f32,
    0.339280f32,
    0.345492f32,
    0.351729f32,
    0.357992f32,
    0.364280f32,
    0.370590f32,
    0.376923f32,
    0.383277f32,
    0.389651f32,
    0.396044f32,
    0.402455f32,
    0.408882f32,
    0.415325f32,
    0.421783f32,
    0.428254f32,
    0.434737f32,
    0.441231f32,
    0.447736f32,
    0.454249f32,
    0.460770f32,
    0.467298f32,
    0.473832f32,
    0.480370f32,
    0.486912f32,
    0.493455f32,
    0.500000f32,
    0.506545f32,
    0.513088f32,
    0.519630f32,
    0.526168f32,
    0.532702f32,
    0.539230f32,
    0.545751f32,
    0.552264f32,
    0.558769f32,
    0.565263f32,
    0.571746f32,
    0.578217f32,
    0.584675f32,
    0.591118f32,
    0.597545f32,
    0.603956f32,
    0.610349f32,
    0.616723f32,
    0.623077f32,
    0.629410f32,
    0.635720f32,
    0.642008f32,
    0.648271f32,
    0.654508f32,
    0.660720f32,
    0.666903f32,
    0.673059f32,
    0.679184f32,
    0.685279f32,
    0.691342f32,
    0.697372f32,
    0.703368f32,
    0.709330f32,
    0.715256f32,
    0.721144f32,
    0.726995f32,
    0.732807f32,
    0.738579f32,
    0.744311f32,
    0.750000f32,
    0.755647f32,
    0.761249f32,
    0.766807f32,
    0.772320f32,
    0.777785f32,
    0.783203f32,
    0.788573f32,
    0.793893f32,
    0.799162f32,
    0.804381f32,
    0.809547f32,
    0.814660f32,
    0.819720f32,
    0.824724f32,
    0.829673f32,
    0.834565f32,
    0.839400f32,
    0.844177f32,
    0.848895f32,
    0.853553f32,
    0.858151f32,
    0.862687f32,
    0.867161f32,
    0.871572f32,
    0.875920f32,
    0.880203f32,
    0.884421f32,
    0.888573f32,
    0.892658f32,
    0.896677f32,
    0.900627f32,
    0.904508f32,
    0.908321f32,
    0.912063f32,
    0.915735f32,
    0.919335f32,
    0.922864f32,
    0.926320f32,
    0.929703f32,
    0.933013f32,
    0.936248f32,
    0.939409f32,
    0.942494f32,
    0.945503f32,
    0.948436f32,
    0.951293f32,
    0.954072f32,
    0.956773f32,
    0.959396f32,
    0.961940f32,
    0.964405f32,
    0.966790f32,
    0.969096f32,
    0.971321f32,
    0.973465f32,
    0.975528f32,
    0.977510f32,
    0.979410f32,
    0.981228f32,
    0.982963f32,
    0.984615f32,
    0.986185f32,
    0.987671f32,
    0.989074f32,
    0.990393f32,
    0.991627f32,
    0.992778f32,
    0.993844f32,
    0.994826f32,
    0.995722f32,
    0.996534f32,
    0.997261f32,
    0.997902f32,
    0.998459f32,
    0.998929f32,
    0.999315f32,
    0.999615f32,
    0.999829f32,
    0.999957f32,
    1.000000f32,
];
#[c2rust::src_loc = "107:18"]
static mut tbands: [libc::c_int; 19] = [
    4 as libc::c_int,
    8 as libc::c_int,
    12 as libc::c_int,
    16 as libc::c_int,
    20 as libc::c_int,
    24 as libc::c_int,
    28 as libc::c_int,
    32 as libc::c_int,
    40 as libc::c_int,
    48 as libc::c_int,
    56 as libc::c_int,
    64 as libc::c_int,
    80 as libc::c_int,
    96 as libc::c_int,
    112 as libc::c_int,
    136 as libc::c_int,
    160 as libc::c_int,
    192 as libc::c_int,
    240 as libc::c_int,
];
#[c2rust::src_loc = "111:9"]
pub const NB_TONAL_SKIP_BANDS: libc::c_int = 9 as libc::c_int;
#[c2rust::src_loc = "113:1"]
unsafe extern "C" fn silk_resampler_down2_hp(
    S: *mut opus_val32,
    out: *mut opus_val32,
    in_0: *const opus_val32,
    inLen: libc::c_int,
) -> opus_val32 {
    let mut k: libc::c_int = 0;
    let len2: libc::c_int = inLen / 2 as libc::c_int;
    let mut in32: opus_val32 = 0.;
    let mut out32: opus_val32 = 0.;
    let mut out32_hp: opus_val32 = 0.;
    let mut Y: opus_val32 = 0.;
    let mut X: opus_val32 = 0.;
    let mut hp_ener: opus_val64 = 0 as libc::c_int as opus_val64;
    k = 0 as libc::c_int;
    while k < len2 {
        in32 = *in_0.offset((2 as libc::c_int * k) as isize);
        Y = in32 - *S.offset(0 as libc::c_int as isize);
        X = 0.6074371f32 * Y;
        out32 = *S.offset(0 as libc::c_int as isize) + X;
        *S.offset(0 as libc::c_int as isize) = in32 + X;
        out32_hp = out32;
        in32 = *in_0.offset((2 as libc::c_int * k + 1 as libc::c_int) as isize);
        Y = in32 - *S.offset(1 as libc::c_int as isize);
        X = 0.15063f32 * Y;
        out32 = out32 + *S.offset(1 as libc::c_int as isize);
        out32 = out32 + X;
        *S.offset(1 as libc::c_int as isize) = in32 + X;
        Y = -in32 - *S.offset(2 as libc::c_int as isize);
        X = 0.15063f32 * Y;
        out32_hp = out32_hp + *S.offset(2 as libc::c_int as isize);
        out32_hp = out32_hp + X;
        *S.offset(2 as libc::c_int as isize) = -in32 + X;
        hp_ener += out32_hp * out32_hp;
        *out.offset(k as isize) = 0.5f32 * out32;
        k += 1;
    }
    return hp_ener;
}
#[c2rust::src_loc = "161:1"]
unsafe extern "C" fn downmix_and_resample(
    downmix: downmix_func,
    mut _x: *const libc::c_void,
    y: *mut opus_val32,
    S: *mut opus_val32,
    mut subframe: libc::c_int,
    mut offset: libc::c_int,
    c1: libc::c_int,
    c2: libc::c_int,
    C: libc::c_int,
    Fs: libc::c_int,
) -> opus_val32 {
    let mut scale: opus_val32 = 0.;
    let mut j: libc::c_int = 0;
    let mut ret: opus_val32 = 0 as libc::c_int as opus_val32;
    if subframe == 0 as libc::c_int {
        return 0 as libc::c_int as opus_val32;
    }
    if Fs == 48000 as libc::c_int {
        subframe *= 2 as libc::c_int;
        offset *= 2 as libc::c_int;
    } else if Fs == 16000 as libc::c_int {
        subframe = subframe * 2 as libc::c_int / 3 as libc::c_int;
        offset = offset * 2 as libc::c_int / 3 as libc::c_int;
    }
    let vla = subframe as usize;
    let mut tmp: Vec::<opus_val32> = ::std::vec::from_elem(0., vla);
    downmix
        .expect(
            "non-null function pointer",
        )(_x, tmp.as_mut_ptr(), subframe, offset, c1, c2, C);
    scale = 1.0f32 / 32768 as libc::c_int as libc::c_float;
    if c2 == -(2 as libc::c_int) {
        scale /= C as libc::c_float;
    } else if c2 > -(1 as libc::c_int) {
        scale /= 2 as libc::c_int as libc::c_float;
    }
    j = 0 as libc::c_int;
    while j < subframe {
        let ref mut fresh0 = *tmp.as_mut_ptr().offset(j as isize);
        *fresh0 *= scale;
        j += 1;
    }
    if Fs == 48000 as libc::c_int {
        ret = silk_resampler_down2_hp(S, y, tmp.as_mut_ptr(), subframe);
    } else if Fs == 24000 as libc::c_int {
        memcpy(
            y as *mut libc::c_void,
            tmp.as_mut_ptr() as *const libc::c_void,
            (subframe as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<opus_val32>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * y.offset_from(tmp.as_mut_ptr()) as libc::c_long)
                        as libc::c_ulong,
                ),
        );
    } else if Fs == 16000 as libc::c_int {
        let vla_0 = (3 as libc::c_int * subframe) as usize;
        let mut tmp3x: Vec::<opus_val32> = ::std::vec::from_elem(0., vla_0);
        j = 0 as libc::c_int;
        while j < subframe {
            *tmp3x
                .as_mut_ptr()
                .offset(
                    (3 as libc::c_int * j) as isize,
                ) = *tmp.as_mut_ptr().offset(j as isize);
            *tmp3x
                .as_mut_ptr()
                .offset(
                    (3 as libc::c_int * j + 1 as libc::c_int) as isize,
                ) = *tmp.as_mut_ptr().offset(j as isize);
            *tmp3x
                .as_mut_ptr()
                .offset(
                    (3 as libc::c_int * j + 2 as libc::c_int) as isize,
                ) = *tmp.as_mut_ptr().offset(j as isize);
            j += 1;
        }
        silk_resampler_down2_hp(S, y, tmp3x.as_mut_ptr(), 3 as libc::c_int * subframe);
    }
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "215:1"]
pub unsafe extern "C" fn tonality_analysis_init(
    mut tonal: *mut TonalityAnalysisState,
    Fs: opus_int32,
) {
    (*tonal).arch = opus_select_arch();
    (*tonal).Fs = Fs;
    tonality_analysis_reset(tonal);
}
#[no_mangle]
#[c2rust::src_loc = "224:1"]
pub unsafe extern "C" fn tonality_analysis_reset(tonal: *mut TonalityAnalysisState) {
    let start: *mut libc::c_char = &mut (*tonal).angle as *mut [libc::c_float; 240]
        as *mut libc::c_char;
    memset(
        start as *mut libc::c_void,
        0 as libc::c_int,
        (::core::mem::size_of::<TonalityAnalysisState>() as libc::c_ulong)
            .wrapping_sub(
                start.offset_from(tonal as *mut libc::c_char) as libc::c_long
                    as libc::c_ulong,
            )
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
}
#[no_mangle]
#[c2rust::src_loc = "231:1"]
pub unsafe extern "C" fn tonality_get_info(
    mut tonal: *mut TonalityAnalysisState,
    mut info_out: *mut AnalysisInfo,
    len: libc::c_int,
) {
    let mut pos: libc::c_int = 0;
    let mut curr_lookahead: libc::c_int = 0;
    let mut tonality_max: libc::c_float = 0.;
    let mut tonality_avg: libc::c_float = 0.;
    let mut tonality_count: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut pos0: libc::c_int = 0;
    let mut prob_avg: libc::c_float = 0.;
    let mut prob_count: libc::c_float = 0.;
    let mut prob_min: libc::c_float = 0.;
    let mut prob_max: libc::c_float = 0.;
    let mut vad_prob: libc::c_float = 0.;
    let mut mpos: libc::c_int = 0;
    let mut vpos: libc::c_int = 0;
    let mut bandwidth_span: libc::c_int = 0;
    pos = (*tonal).read_pos;
    curr_lookahead = (*tonal).write_pos - (*tonal).read_pos;
    if curr_lookahead < 0 as libc::c_int {
        curr_lookahead += DETECT_SIZE;
    }
    (*tonal).read_subframe += len / ((*tonal).Fs / 400 as libc::c_int);
    while (*tonal).read_subframe >= 8 as libc::c_int {
        (*tonal).read_subframe -= 8 as libc::c_int;
        (*tonal).read_pos += 1;
    }
    if (*tonal).read_pos >= DETECT_SIZE {
        (*tonal).read_pos -= DETECT_SIZE;
    }
    if len > (*tonal).Fs / 50 as libc::c_int && pos != (*tonal).write_pos {
        pos += 1;
        if pos == DETECT_SIZE {
            pos = 0 as libc::c_int;
        }
    }
    if pos == (*tonal).write_pos {
        pos -= 1;
    }
    if pos < 0 as libc::c_int {
        pos = DETECT_SIZE - 1 as libc::c_int;
    }
    pos0 = pos;
    memcpy(
        info_out as *mut libc::c_void,
        &mut *((*tonal).info).as_mut_ptr().offset(pos as isize) as *mut AnalysisInfo
            as *const libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<AnalysisInfo>() as libc::c_ulong)
            .wrapping_add(
                (0 as libc::c_int as libc::c_long
                    * info_out
                        .offset_from(
                            &mut *((*tonal).info).as_mut_ptr().offset(pos as isize),
                        ) as libc::c_long) as libc::c_ulong,
            ),
    );
    if (*info_out).valid == 0 {
        return;
    }
    tonality_avg = (*info_out).tonality;
    tonality_max = tonality_avg;
    tonality_count = 1 as libc::c_int;
    bandwidth_span = 6 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        pos += 1;
        if pos == DETECT_SIZE {
            pos = 0 as libc::c_int;
        }
        if pos == (*tonal).write_pos {
            break;
        }
        tonality_max = if tonality_max > (*tonal).info[pos as usize].tonality {
            tonality_max
        } else {
            (*tonal).info[pos as usize].tonality
        };
        tonality_avg += (*tonal).info[pos as usize].tonality;
        tonality_count += 1;
        (*info_out)
            .bandwidth = if (*info_out).bandwidth > (*tonal).info[pos as usize].bandwidth
        {
            (*info_out).bandwidth
        } else {
            (*tonal).info[pos as usize].bandwidth
        };
        bandwidth_span -= 1;
        i += 1;
    }
    pos = pos0;
    i = 0 as libc::c_int;
    while i < bandwidth_span {
        pos -= 1;
        if pos < 0 as libc::c_int {
            pos = DETECT_SIZE - 1 as libc::c_int;
        }
        if pos == (*tonal).write_pos {
            break;
        }
        (*info_out)
            .bandwidth = if (*info_out).bandwidth > (*tonal).info[pos as usize].bandwidth
        {
            (*info_out).bandwidth
        } else {
            (*tonal).info[pos as usize].bandwidth
        };
        i += 1;
    }
    (*info_out)
        .tonality = if tonality_avg / tonality_count as libc::c_float
        > tonality_max - 0.2f32
    {
        tonality_avg / tonality_count as libc::c_float
    } else {
        tonality_max - 0.2f32
    };
    vpos = pos0;
    mpos = vpos;
    if curr_lookahead > 15 as libc::c_int {
        mpos += 5 as libc::c_int;
        if mpos >= DETECT_SIZE {
            mpos -= DETECT_SIZE;
        }
        vpos += 1 as libc::c_int;
        if vpos >= DETECT_SIZE {
            vpos -= DETECT_SIZE;
        }
    }
    prob_min = 1.0f32;
    prob_max = 0.0f32;
    vad_prob = (*tonal).info[vpos as usize].activity_probability;
    prob_count = if 0.1f32 > vad_prob { 0.1f32 } else { vad_prob };
    prob_avg = (if 0.1f32 > vad_prob { 0.1f32 } else { vad_prob })
        * (*tonal).info[mpos as usize].music_prob;
    loop {
        let mut pos_vad: libc::c_float = 0.;
        mpos += 1;
        if mpos == DETECT_SIZE {
            mpos = 0 as libc::c_int;
        }
        if mpos == (*tonal).write_pos {
            break;
        }
        vpos += 1;
        if vpos == DETECT_SIZE {
            vpos = 0 as libc::c_int;
        }
        if vpos == (*tonal).write_pos {
            break;
        }
        pos_vad = (*tonal).info[vpos as usize].activity_probability;
        prob_min = if (prob_avg
            - 10 as libc::c_int as libc::c_float * (vad_prob - pos_vad)) / prob_count
            < prob_min
        {
            (prob_avg - 10 as libc::c_int as libc::c_float * (vad_prob - pos_vad))
                / prob_count
        } else {
            prob_min
        };
        prob_max = if (prob_avg
            + 10 as libc::c_int as libc::c_float * (vad_prob - pos_vad)) / prob_count
            > prob_max
        {
            (prob_avg + 10 as libc::c_int as libc::c_float * (vad_prob - pos_vad))
                / prob_count
        } else {
            prob_max
        };
        prob_count += if 0.1f32 > pos_vad { 0.1f32 } else { pos_vad };
        prob_avg
            += (if 0.1f32 > pos_vad { 0.1f32 } else { pos_vad })
                * (*tonal).info[mpos as usize].music_prob;
    }
    (*info_out).music_prob = prob_avg / prob_count;
    prob_min = if prob_avg / prob_count < prob_min {
        prob_avg / prob_count
    } else {
        prob_min
    };
    prob_max = if prob_avg / prob_count > prob_max {
        prob_avg / prob_count
    } else {
        prob_max
    };
    prob_min = if prob_min > 0.0f32 { prob_min } else { 0.0f32 };
    prob_max = if prob_max < 1.0f32 { prob_max } else { 1.0f32 };
    if curr_lookahead < 10 as libc::c_int {
        let mut pmin: libc::c_float = 0.;
        let mut pmax: libc::c_float = 0.;
        pmin = prob_min;
        pmax = prob_max;
        pos = pos0;
        i = 0 as libc::c_int;
        while i
            < (if ((*tonal).count - 1 as libc::c_int) < 15 as libc::c_int {
                (*tonal).count - 1 as libc::c_int
            } else {
                15 as libc::c_int
            })
        {
            pos -= 1;
            if pos < 0 as libc::c_int {
                pos = DETECT_SIZE - 1 as libc::c_int;
            }
            pmin = if pmin < (*tonal).info[pos as usize].music_prob {
                pmin
            } else {
                (*tonal).info[pos as usize].music_prob
            };
            pmax = if pmax > (*tonal).info[pos as usize].music_prob {
                pmax
            } else {
                (*tonal).info[pos as usize].music_prob
            };
            i += 1;
        }
        pmin = if 0.0f32 > pmin - 0.1f32 * vad_prob {
            0.0f32
        } else {
            pmin - 0.1f32 * vad_prob
        };
        pmax = if 1.0f32 < pmax + 0.1f32 * vad_prob {
            1.0f32
        } else {
            pmax + 0.1f32 * vad_prob
        };
        prob_min
            += (1.0f32 - 0.1f32 * curr_lookahead as libc::c_float) * (pmin - prob_min);
        prob_max
            += (1.0f32 - 0.1f32 * curr_lookahead as libc::c_float) * (pmax - prob_max);
    }
    (*info_out).music_prob_min = prob_min;
    (*info_out).music_prob_max = prob_max;
}
#[c2rust::src_loc = "409:20"]
static mut std_feature_bias: [libc::c_float; 9] = [
    5.684947f32,
    3.475288f32,
    1.770634f32,
    1.599784f32,
    3.773215f32,
    2.163313f32,
    1.260756f32,
    1.116868f32,
    1.918795f32,
];
#[c2rust::src_loc = "414:9"]
pub const LEAKAGE_OFFSET: libc::c_float = 2.5f32;
#[c2rust::src_loc = "415:9"]
pub const LEAKAGE_SLOPE: libc::c_float = 2.0f32;
#[c2rust::src_loc = "444:1"]
unsafe extern "C" fn tonality_analysis(
    mut tonal: *mut TonalityAnalysisState,
    celt_mode: *const OpusCustomMode,
    x: *const libc::c_void,
    mut len: libc::c_int,
    mut offset: libc::c_int,
    c1: libc::c_int,
    c2: libc::c_int,
    C: libc::c_int,
    lsb_depth: libc::c_int,
    downmix: downmix_func,
) {
    let mut i: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut kfft: *const kiss_fft_state = 0 as *const kiss_fft_state;
    let N: libc::c_int = 480 as libc::c_int;
    let N2: libc::c_int = 240 as libc::c_int;
    let A: *mut libc::c_float = ((*tonal).angle).as_mut_ptr();
    let dA: *mut libc::c_float = ((*tonal).d_angle).as_mut_ptr();
    let d2A: *mut libc::c_float = ((*tonal).d2_angle).as_mut_ptr();
    let mut band_tonality: [libc::c_float; 18] = [0.; 18];
    let mut logE: [libc::c_float; 18] = [0.; 18];
    let mut BFCC: [libc::c_float; 8] = [0.; 8];
    let mut features: [libc::c_float; 25] = [0.; 25];
    let mut frame_tonality: libc::c_float = 0.;
    let mut max_frame_tonality: libc::c_float = 0.;
    let mut frame_noisiness: libc::c_float = 0.;
    let pi4: libc::c_float = (M_PI * M_PI * M_PI * M_PI) as libc::c_float;
    let mut slope: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut frame_stationarity: libc::c_float = 0.;
    let mut relativeE: libc::c_float = 0.;
    let mut frame_probs: [libc::c_float; 2] = [0.; 2];
    let mut alpha: libc::c_float = 0.;
    let mut alphaE: libc::c_float = 0.;
    let mut alphaE2: libc::c_float = 0.;
    let mut frame_loudness: libc::c_float = 0.;
    let mut bandwidth_mask: libc::c_float = 0.;
    let mut is_masked: [libc::c_int; 19] = [0; 19];
    let mut bandwidth: libc::c_int = 0 as libc::c_int;
    let mut maxE: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut noise_floor: libc::c_float = 0.;
    let mut remaining: libc::c_int = 0;
    let mut info: *mut AnalysisInfo = 0 as *mut AnalysisInfo;
    let mut hp_ener: libc::c_float = 0.;
    let mut tonality2: [libc::c_float; 240] = [0.; 240];
    let mut midE: [libc::c_float; 8] = [0.; 8];
    let mut spec_variability: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut band_log2: [libc::c_float; 19] = [0.; 19];
    let mut leakage_from: [libc::c_float; 19] = [0.; 19];
    let mut leakage_to: [libc::c_float; 19] = [0.; 19];
    let mut layer_out: [libc::c_float; 32] = [0.; 32];
    let mut below_max_pitch: libc::c_float = 0.;
    let mut above_max_pitch: libc::c_float = 0.;
    let mut is_silence: libc::c_int = 0;
    if (*tonal).initialized == 0 {
        (*tonal).mem_fill = 240 as libc::c_int;
        (*tonal).initialized = 1 as libc::c_int;
    }
    alpha = 1.0f32
        / (if (10 as libc::c_int) < 1 as libc::c_int + (*tonal).count {
            10 as libc::c_int
        } else {
            1 as libc::c_int + (*tonal).count
        }) as libc::c_float;
    alphaE = 1.0f32
        / (if (25 as libc::c_int) < 1 as libc::c_int + (*tonal).count {
            25 as libc::c_int
        } else {
            1 as libc::c_int + (*tonal).count
        }) as libc::c_float;
    alphaE2 = 1.0f32
        / (if (100 as libc::c_int) < 1 as libc::c_int + (*tonal).count {
            100 as libc::c_int
        } else {
            1 as libc::c_int + (*tonal).count
        }) as libc::c_float;
    if (*tonal).count <= 1 as libc::c_int {
        alphaE2 = 1 as libc::c_int as libc::c_float;
    }
    if (*tonal).Fs == 48000 as libc::c_int {
        len /= 2 as libc::c_int;
        offset /= 2 as libc::c_int;
    } else if (*tonal).Fs == 16000 as libc::c_int {
        len = 3 as libc::c_int * len / 2 as libc::c_int;
        offset = 3 as libc::c_int * offset / 2 as libc::c_int;
    }
    kfft = (*celt_mode).mdct.kfft[0 as libc::c_int as usize];
    (*tonal).hp_ener_accum
        += downmix_and_resample(
            downmix,
            x,
            &mut *((*tonal).inmem).as_mut_ptr().offset((*tonal).mem_fill as isize),
            ((*tonal).downmix_state).as_mut_ptr(),
            if len < 720 as libc::c_int - (*tonal).mem_fill {
                len
            } else {
                720 as libc::c_int - (*tonal).mem_fill
            },
            offset,
            c1,
            c2,
            C,
            (*tonal).Fs,
        );
    if (*tonal).mem_fill + len < ANALYSIS_BUF_SIZE {
        (*tonal).mem_fill += len;
        return;
    }
    hp_ener = (*tonal).hp_ener_accum;
    let fresh1 = (*tonal).write_pos;
    (*tonal).write_pos = (*tonal).write_pos + 1;
    info = &mut *((*tonal).info).as_mut_ptr().offset(fresh1 as isize)
        as *mut AnalysisInfo;
    if (*tonal).write_pos >= DETECT_SIZE {
        (*tonal).write_pos -= DETECT_SIZE;
    }
    is_silence = is_digital_silence(
        ((*tonal).inmem).as_mut_ptr(),
        720 as libc::c_int,
        1 as libc::c_int,
        lsb_depth,
    );
    let mut in_0: [kiss_fft_cpx; 480] = [kiss_fft_cpx { r: 0., i: 0. }; 480];
    let mut out: [kiss_fft_cpx; 480] = [kiss_fft_cpx { r: 0., i: 0. }; 480];
    let mut tonality: [libc::c_float; 240] = [0.; 240];
    let mut noisiness: [libc::c_float; 240] = [0.; 240];
    i = 0 as libc::c_int;
    while i < N2 {
        let w: libc::c_float = analysis_window[i as usize];
        in_0[i as usize].r = w * (*tonal).inmem[i as usize];
        in_0[i as usize].i = w * (*tonal).inmem[(N2 + i) as usize];
        in_0[(N - i - 1 as libc::c_int) as usize]
            .r = w * (*tonal).inmem[(N - i - 1 as libc::c_int) as usize];
        in_0[(N - i - 1 as libc::c_int) as usize]
            .i = w * (*tonal).inmem[(N + N2 - i - 1 as libc::c_int) as usize];
        i += 1;
    }
    memmove(
        ((*tonal).inmem).as_mut_ptr() as *mut libc::c_void,
        ((*tonal).inmem)
            .as_mut_ptr()
            .offset(720 as libc::c_int as isize)
            .offset(-(240 as libc::c_int as isize)) as *const libc::c_void,
        (240 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<opus_val32>() as libc::c_ulong)
            .wrapping_add(
                (0 as libc::c_int as libc::c_long
                    * ((*tonal).inmem)
                        .as_mut_ptr()
                        .offset_from(
                            ((*tonal).inmem)
                                .as_mut_ptr()
                                .offset(720 as libc::c_int as isize)
                                .offset(-(240 as libc::c_int as isize)),
                        ) as libc::c_long) as libc::c_ulong,
            ),
    );
    remaining = len - (ANALYSIS_BUF_SIZE - (*tonal).mem_fill);
    (*tonal)
        .hp_ener_accum = downmix_and_resample(
        downmix,
        x,
        &mut *((*tonal).inmem).as_mut_ptr().offset(240 as libc::c_int as isize),
        ((*tonal).downmix_state).as_mut_ptr(),
        remaining,
        offset + ANALYSIS_BUF_SIZE - (*tonal).mem_fill,
        c1,
        c2,
        C,
        (*tonal).Fs,
    );
    (*tonal).mem_fill = 240 as libc::c_int + remaining;
    if is_silence != 0 {
        let mut prev_pos: libc::c_int = (*tonal).write_pos - 2 as libc::c_int;
        if prev_pos < 0 as libc::c_int {
            prev_pos += DETECT_SIZE;
        }
        memcpy(
            info as *mut libc::c_void,
            &mut *((*tonal).info).as_mut_ptr().offset(prev_pos as isize)
                as *mut AnalysisInfo as *const libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<AnalysisInfo>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * info
                            .offset_from(
                                &mut *((*tonal).info).as_mut_ptr().offset(prev_pos as isize),
                            ) as libc::c_long) as libc::c_ulong,
                ),
        );
        return;
    }
    opus_fft_c(kfft, in_0.as_mut_ptr(), out.as_mut_ptr());
    if out[0 as libc::c_int as usize].r != out[0 as libc::c_int as usize].r {
        (*info).valid = 0 as libc::c_int;
        return;
    }
    i = 1 as libc::c_int;
    while i < N2 {
        let mut X1r: libc::c_float = 0.;
        let mut X2r: libc::c_float = 0.;
        let mut X1i: libc::c_float = 0.;
        let mut X2i: libc::c_float = 0.;
        let mut angle: libc::c_float = 0.;
        let mut d_angle: libc::c_float = 0.;
        let mut d2_angle: libc::c_float = 0.;
        let mut angle2: libc::c_float = 0.;
        let mut d_angle2: libc::c_float = 0.;
        let mut d2_angle2: libc::c_float = 0.;
        let mut mod1: libc::c_float = 0.;
        let mut mod2: libc::c_float = 0.;
        let mut avg_mod: libc::c_float = 0.;
        X1r = out[i as usize].r + out[(N - i) as usize].r;
        X1i = out[i as usize].i - out[(N - i) as usize].i;
        X2r = out[i as usize].i + out[(N - i) as usize].i;
        X2i = out[(N - i) as usize].r - out[i as usize].r;
        angle = (0.5f32 as libc::c_double / M_PI) as libc::c_float
            * fast_atan2f(X1i, X1r);
        d_angle = angle - *A.offset(i as isize);
        d2_angle = d_angle - *dA.offset(i as isize);
        angle2 = (0.5f32 as libc::c_double / M_PI) as libc::c_float
            * fast_atan2f(X2i, X2r);
        d_angle2 = angle2 - angle;
        d2_angle2 = d_angle2 - d_angle;
        mod1 = d2_angle - float2int(d2_angle) as libc::c_float;
        noisiness[i as usize] = fabs(mod1 as libc::c_double) as libc::c_float;
        mod1 *= mod1;
        mod1 *= mod1;
        mod2 = d2_angle2 - float2int(d2_angle2) as libc::c_float;
        noisiness[i as usize] += fabs(mod2 as libc::c_double) as libc::c_float;
        mod2 *= mod2;
        mod2 *= mod2;
        avg_mod = 0.25f32
            * (*d2A.offset(i as isize) + mod1
                + 2 as libc::c_int as libc::c_float * mod2);
        tonality[i
            as usize] = 1.0f32 / (1.0f32 + 40.0f32 * 16.0f32 * pi4 * avg_mod) - 0.015f32;
        tonality2[i
            as usize] = 1.0f32 / (1.0f32 + 40.0f32 * 16.0f32 * pi4 * mod2) - 0.015f32;
        *A.offset(i as isize) = angle2;
        *dA.offset(i as isize) = d_angle2;
        *d2A.offset(i as isize) = mod2;
        i += 1;
    }
    i = 2 as libc::c_int;
    while i < N2 - 1 as libc::c_int {
        let tt: libc::c_float = if tonality2[i as usize]
            < (if tonality2[(i - 1 as libc::c_int) as usize]
                > tonality2[(i + 1 as libc::c_int) as usize]
            {
                tonality2[(i - 1 as libc::c_int) as usize]
            } else {
                tonality2[(i + 1 as libc::c_int) as usize]
            })
        {
            tonality2[i as usize]
        } else if tonality2[(i - 1 as libc::c_int) as usize]
            > tonality2[(i + 1 as libc::c_int) as usize]
        {
            tonality2[(i - 1 as libc::c_int) as usize]
        } else {
            tonality2[(i + 1 as libc::c_int) as usize]
        };
        tonality[i
            as usize] = 0.9f32
            * (if tonality[i as usize] > tt - 0.1f32 {
                tonality[i as usize]
            } else {
                tt - 0.1f32
            });
        i += 1;
    }
    frame_tonality = 0 as libc::c_int as libc::c_float;
    max_frame_tonality = 0 as libc::c_int as libc::c_float;
    (*info).activity = 0 as libc::c_int as libc::c_float;
    frame_noisiness = 0 as libc::c_int as libc::c_float;
    frame_stationarity = 0 as libc::c_int as libc::c_float;
    if (*tonal).count == 0 {
        b = 0 as libc::c_int;
        while b < NB_TBANDS {
            (*tonal).lowE[b as usize] = 1e10f64 as libc::c_float;
            (*tonal).highE[b as usize] = -1e10f64 as libc::c_float;
            b += 1;
        }
    }
    relativeE = 0 as libc::c_int as libc::c_float;
    frame_loudness = 0 as libc::c_int as libc::c_float;
    let mut E: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut X1r_0: libc::c_float = 0.;
    let mut X2r_0: libc::c_float = 0.;
    X1r_0 = 2 as libc::c_int as libc::c_float * out[0 as libc::c_int as usize].r;
    X2r_0 = 2 as libc::c_int as libc::c_float * out[0 as libc::c_int as usize].i;
    E = X1r_0 * X1r_0 + X2r_0 * X2r_0;
    i = 1 as libc::c_int;
    while i < 4 as libc::c_int {
        let binE: libc::c_float = out[i as usize].r * out[i as usize].r
            + out[(N - i) as usize].r * out[(N - i) as usize].r
            + out[i as usize].i * out[i as usize].i
            + out[(N - i) as usize].i * out[(N - i) as usize].i;
        E += binE;
        i += 1;
    }
    E = E;
    band_log2[0 as libc::c_int
        as usize] = 0.5f32 * 1.442695f32
        * log((E + 1e-10f32) as libc::c_double) as libc::c_float;
    b = 0 as libc::c_int;
    while b < NB_TBANDS {
        let mut E_0: libc::c_float = 0 as libc::c_int as libc::c_float;
        let mut tE: libc::c_float = 0 as libc::c_int as libc::c_float;
        let mut nE: libc::c_float = 0 as libc::c_int as libc::c_float;
        let mut L1: libc::c_float = 0.;
        let mut L2: libc::c_float = 0.;
        let mut stationarity: libc::c_float = 0.;
        i = tbands[b as usize];
        while i < tbands[(b + 1 as libc::c_int) as usize] {
            let mut binE_0: libc::c_float = out[i as usize].r * out[i as usize].r
                + out[(N - i) as usize].r * out[(N - i) as usize].r
                + out[i as usize].i * out[i as usize].i
                + out[(N - i) as usize].i * out[(N - i) as usize].i;
            binE_0 = binE_0;
            E_0 += binE_0;
            tE
                += binE_0
                    * (if 0 as libc::c_int as libc::c_float > tonality[i as usize] {
                        0 as libc::c_int as libc::c_float
                    } else {
                        tonality[i as usize]
                    });
            nE += binE_0 * 2.0f32 * (0.5f32 - noisiness[i as usize]);
            i += 1;
        }
        if !(E_0 < 1e9f32) || E_0 != E_0 {
            (*info).valid = 0 as libc::c_int;
            return;
        }
        (*tonal).E[(*tonal).E_count as usize][b as usize] = E_0;
        frame_noisiness += nE / (1e-15f32 + E_0);
        frame_loudness += sqrt((E_0 + 1e-10f32) as libc::c_double) as libc::c_float;
        logE[b as usize] = log((E_0 + 1e-10f32) as libc::c_double) as libc::c_float;
        band_log2[(b + 1 as libc::c_int)
            as usize] = 0.5f32 * 1.442695f32
            * log((E_0 + 1e-10f32) as libc::c_double) as libc::c_float;
        (*tonal).logE[(*tonal).E_count as usize][b as usize] = logE[b as usize];
        if (*tonal).count == 0 as libc::c_int {
            (*tonal).lowE[b as usize] = logE[b as usize];
            (*tonal).highE[b as usize] = (*tonal).lowE[b as usize];
        }
        if (*tonal).highE[b as usize] as libc::c_double
            > (*tonal).lowE[b as usize] as libc::c_double + 7.5f64
        {
            if (*tonal).highE[b as usize] - logE[b as usize]
                > logE[b as usize] - (*tonal).lowE[b as usize]
            {
                (*tonal).highE[b as usize] -= 0.01f32;
            } else {
                (*tonal).lowE[b as usize] += 0.01f32;
            }
        }
        if logE[b as usize] > (*tonal).highE[b as usize] {
            (*tonal).highE[b as usize] = logE[b as usize];
            (*tonal)
                .lowE[b
                as usize] = if (*tonal).highE[b as usize]
                - 15 as libc::c_int as libc::c_float > (*tonal).lowE[b as usize]
            {
                (*tonal).highE[b as usize] - 15 as libc::c_int as libc::c_float
            } else {
                (*tonal).lowE[b as usize]
            };
        } else if logE[b as usize] < (*tonal).lowE[b as usize] {
            (*tonal).lowE[b as usize] = logE[b as usize];
            (*tonal)
                .highE[b
                as usize] = if ((*tonal).lowE[b as usize]
                + 15 as libc::c_int as libc::c_float) < (*tonal).highE[b as usize]
            {
                (*tonal).lowE[b as usize] + 15 as libc::c_int as libc::c_float
            } else {
                (*tonal).highE[b as usize]
            };
        }
        relativeE
            += (logE[b as usize] - (*tonal).lowE[b as usize])
                / (1e-5f32 + ((*tonal).highE[b as usize] - (*tonal).lowE[b as usize]));
        L2 = 0 as libc::c_int as libc::c_float;
        L1 = L2;
        i = 0 as libc::c_int;
        while i < NB_FRAMES {
            L1
                += sqrt((*tonal).E[i as usize][b as usize] as libc::c_double)
                    as libc::c_float;
            L2 += (*tonal).E[i as usize][b as usize];
            i += 1;
        }
        stationarity = if 0.99f32
            < L1
                / sqrt(
                    1e-15f64 + (8 as libc::c_int as libc::c_float * L2) as libc::c_double,
                ) as libc::c_float
        {
            0.99f32
        } else {
            L1
                / sqrt(
                    1e-15f64 + (8 as libc::c_int as libc::c_float * L2) as libc::c_double,
                ) as libc::c_float
        };
        stationarity *= stationarity;
        stationarity *= stationarity;
        frame_stationarity += stationarity;
        band_tonality[b
            as usize] = if tE / (1e-15f32 + E_0)
            > stationarity * (*tonal).prev_band_tonality[b as usize]
        {
            tE / (1e-15f32 + E_0)
        } else {
            stationarity * (*tonal).prev_band_tonality[b as usize]
        };
        frame_tonality += band_tonality[b as usize];
        if b >= NB_TBANDS - NB_TONAL_SKIP_BANDS {
            frame_tonality
                -= band_tonality[(b - NB_TBANDS + NB_TONAL_SKIP_BANDS) as usize];
        }
        max_frame_tonality = if max_frame_tonality
            > (1.0f32 + 0.03f32 * (b - 18 as libc::c_int) as libc::c_float)
                * frame_tonality
        {
            max_frame_tonality
        } else {
            (1.0f32 + 0.03f32 * (b - 18 as libc::c_int) as libc::c_float)
                * frame_tonality
        };
        slope += band_tonality[b as usize] * (b - 8 as libc::c_int) as libc::c_float;
        (*tonal).prev_band_tonality[b as usize] = band_tonality[b as usize];
        b += 1;
    }
    leakage_from[0 as libc::c_int as usize] = band_log2[0 as libc::c_int as usize];
    leakage_to[0 as libc::c_int
        as usize] = band_log2[0 as libc::c_int as usize] - LEAKAGE_OFFSET;
    b = 1 as libc::c_int;
    while b < NB_TBANDS + 1 as libc::c_int {
        let leak_slope: libc::c_float = LEAKAGE_SLOPE
            * (tbands[b as usize] - tbands[(b - 1 as libc::c_int) as usize])
                as libc::c_float / 4 as libc::c_int as libc::c_float;
        leakage_from[b
            as usize] = if leakage_from[(b - 1 as libc::c_int) as usize] + leak_slope
            < band_log2[b as usize]
        {
            leakage_from[(b - 1 as libc::c_int) as usize] + leak_slope
        } else {
            band_log2[b as usize]
        };
        leakage_to[b
            as usize] = if leakage_to[(b - 1 as libc::c_int) as usize] - leak_slope
            > band_log2[b as usize] - 2.5f32
        {
            leakage_to[(b - 1 as libc::c_int) as usize] - leak_slope
        } else {
            band_log2[b as usize] - 2.5f32
        };
        b += 1;
    }
    b = NB_TBANDS - 2 as libc::c_int;
    while b >= 0 as libc::c_int {
        let leak_slope_0: libc::c_float = LEAKAGE_SLOPE
            * (tbands[(b + 1 as libc::c_int) as usize] - tbands[b as usize])
                as libc::c_float / 4 as libc::c_int as libc::c_float;
        leakage_from[b
            as usize] = if leakage_from[(b + 1 as libc::c_int) as usize] + leak_slope_0
            < leakage_from[b as usize]
        {
            leakage_from[(b + 1 as libc::c_int) as usize] + leak_slope_0
        } else {
            leakage_from[b as usize]
        };
        leakage_to[b
            as usize] = if leakage_to[(b + 1 as libc::c_int) as usize] - leak_slope_0
            > leakage_to[b as usize]
        {
            leakage_to[(b + 1 as libc::c_int) as usize] - leak_slope_0
        } else {
            leakage_to[b as usize]
        };
        b -= 1;
    }
    if !(18 as libc::c_int + 1 as libc::c_int <= 19 as libc::c_int) {
        celt_fatal(
            b"assertion failed: NB_TBANDS+1 <= LEAK_BANDS\0" as *const u8
                as *const libc::c_char,
            b"src/analysis.c\0" as *const u8 as *const libc::c_char,
            740 as libc::c_int,
        );
    }
    b = 0 as libc::c_int;
    while b < NB_TBANDS + 1 as libc::c_int {
        let boost: libc::c_float = (if 0 as libc::c_int as libc::c_float
            > leakage_to[b as usize] - band_log2[b as usize]
        {
            0 as libc::c_int as libc::c_float
        } else {
            leakage_to[b as usize] - band_log2[b as usize]
        })
            + (if 0 as libc::c_int as libc::c_float
                > band_log2[b as usize] - (leakage_from[b as usize] + 2.5f32)
            {
                0 as libc::c_int as libc::c_float
            } else {
                band_log2[b as usize] - (leakage_from[b as usize] + 2.5f32)
            });
        (*info)
            .leak_boost[b
            as usize] = (if (255 as libc::c_int)
            < floor(0.5f64 + (64.0f32 * boost) as libc::c_double) as libc::c_int
        {
            255 as libc::c_int
        } else {
            floor(0.5f64 + (64.0f32 * boost) as libc::c_double) as libc::c_int
        }) as libc::c_uchar;
        b += 1;
    }
    while b < LEAK_BANDS {
        (*info).leak_boost[b as usize] = 0 as libc::c_int as libc::c_uchar;
        b += 1;
    }
    i = 0 as libc::c_int;
    while i < NB_FRAMES {
        let mut j: libc::c_int = 0;
        let mut mindist: libc::c_float = 1e15f32;
        j = 0 as libc::c_int;
        while j < NB_FRAMES {
            let mut k: libc::c_int = 0;
            let mut dist: libc::c_float = 0 as libc::c_int as libc::c_float;
            k = 0 as libc::c_int;
            while k < NB_TBANDS {
                let mut tmp: libc::c_float = 0.;
                tmp = (*tonal).logE[i as usize][k as usize]
                    - (*tonal).logE[j as usize][k as usize];
                dist += tmp * tmp;
                k += 1;
            }
            if j != i {
                mindist = if mindist < dist { mindist } else { dist };
            }
            j += 1;
        }
        spec_variability += mindist;
        i += 1;
    }
    spec_variability = sqrt(
        (spec_variability / NB_FRAMES as libc::c_float / NB_TBANDS as libc::c_float)
            as libc::c_double,
    ) as libc::c_float;
    bandwidth_mask = 0 as libc::c_int as libc::c_float;
    bandwidth = 0 as libc::c_int;
    maxE = 0 as libc::c_int as libc::c_float;
    noise_floor = 5.7e-4f32
        / ((1 as libc::c_int)
            << (if 0 as libc::c_int > lsb_depth - 8 as libc::c_int {
                0 as libc::c_int
            } else {
                lsb_depth - 8 as libc::c_int
            })) as libc::c_float;
    noise_floor *= noise_floor;
    below_max_pitch = 0 as libc::c_int as libc::c_float;
    above_max_pitch = 0 as libc::c_int as libc::c_float;
    b = 0 as libc::c_int;
    while b < NB_TBANDS {
        let mut E_1: libc::c_float = 0 as libc::c_int as libc::c_float;
        let mut Em: libc::c_float = 0.;
        let mut band_start: libc::c_int = 0;
        let mut band_end: libc::c_int = 0;
        band_start = tbands[b as usize];
        band_end = tbands[(b + 1 as libc::c_int) as usize];
        i = band_start;
        while i < band_end {
            let binE_1: libc::c_float = out[i as usize].r * out[i as usize].r
                + out[(N - i) as usize].r * out[(N - i) as usize].r
                + out[i as usize].i * out[i as usize].i
                + out[(N - i) as usize].i * out[(N - i) as usize].i;
            E_1 += binE_1;
            i += 1;
        }
        E_1 = E_1;
        maxE = if maxE > E_1 { maxE } else { E_1 };
        if band_start < 64 as libc::c_int {
            below_max_pitch += E_1;
        } else {
            above_max_pitch += E_1;
        }
        (*tonal)
            .meanE[b
            as usize] = if (1 as libc::c_int as libc::c_float - alphaE2)
            * (*tonal).meanE[b as usize] > E_1
        {
            (1 as libc::c_int as libc::c_float - alphaE2) * (*tonal).meanE[b as usize]
        } else {
            E_1
        };
        Em = if E_1 > (*tonal).meanE[b as usize] {
            E_1
        } else {
            (*tonal).meanE[b as usize]
        };
        if E_1 * 1e9f32 > maxE
            && (Em
                > 3 as libc::c_int as libc::c_float * noise_floor
                    * (band_end - band_start) as libc::c_float
                || E_1 > noise_floor * (band_end - band_start) as libc::c_float)
        {
            bandwidth = b + 1 as libc::c_int;
        }
        is_masked[b
            as usize] = (E_1
            < (if (*tonal).prev_bandwidth >= b + 1 as libc::c_int {
                0.01f32
            } else {
                0.05f32
            }) * bandwidth_mask) as libc::c_int;
        bandwidth_mask = if 0.05f32 * bandwidth_mask > E_1 {
            0.05f32 * bandwidth_mask
        } else {
            E_1
        };
        b += 1;
    }
    if (*tonal).Fs == 48000 as libc::c_int {
        let mut noise_ratio: libc::c_float = 0.;
        let mut Em_0: libc::c_float = 0.;
        let E_2: libc::c_float = hp_ener
            * (1.0f32 / (60 as libc::c_int * 60 as libc::c_int) as libc::c_float);
        noise_ratio = if (*tonal).prev_bandwidth == 20 as libc::c_int {
            10.0f32
        } else {
            30.0f32
        };
        above_max_pitch += E_2;
        (*tonal)
            .meanE[b
            as usize] = if (1 as libc::c_int as libc::c_float - alphaE2)
            * (*tonal).meanE[b as usize] > E_2
        {
            (1 as libc::c_int as libc::c_float - alphaE2) * (*tonal).meanE[b as usize]
        } else {
            E_2
        };
        Em_0 = if E_2 > (*tonal).meanE[b as usize] {
            E_2
        } else {
            (*tonal).meanE[b as usize]
        };
        if Em_0
            > 3 as libc::c_int as libc::c_float * noise_ratio * noise_floor
                * 160 as libc::c_int as libc::c_float
            || E_2 > noise_ratio * noise_floor * 160 as libc::c_int as libc::c_float
        {
            bandwidth = 20 as libc::c_int;
        }
        is_masked[b
            as usize] = (E_2
            < (if (*tonal).prev_bandwidth == 20 as libc::c_int {
                0.01f32
            } else {
                0.05f32
            }) * bandwidth_mask) as libc::c_int;
    }
    if above_max_pitch > below_max_pitch {
        (*info).max_pitch_ratio = below_max_pitch / above_max_pitch;
    } else {
        (*info).max_pitch_ratio = 1 as libc::c_int as libc::c_float;
    }
    if bandwidth == 20 as libc::c_int && is_masked[NB_TBANDS as usize] != 0 {
        bandwidth -= 2 as libc::c_int;
    } else if bandwidth > 0 as libc::c_int && bandwidth <= NB_TBANDS
        && is_masked[(bandwidth - 1 as libc::c_int) as usize] != 0
    {
        bandwidth -= 1;
    }
    if (*tonal).count <= 2 as libc::c_int {
        bandwidth = 20 as libc::c_int;
    }
    frame_loudness = 20 as libc::c_int as libc::c_float
        * log10(frame_loudness as libc::c_double) as libc::c_float;
    (*tonal)
        .Etracker = if (*tonal).Etracker - 0.003f32 > frame_loudness {
        (*tonal).Etracker - 0.003f32
    } else {
        frame_loudness
    };
    (*tonal).lowECount *= 1 as libc::c_int as libc::c_float - alphaE;
    if frame_loudness < (*tonal).Etracker - 30 as libc::c_int as libc::c_float {
        (*tonal).lowECount += alphaE;
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        let mut sum: libc::c_float = 0 as libc::c_int as libc::c_float;
        b = 0 as libc::c_int;
        while b < 16 as libc::c_int {
            sum += dct_table[(i * 16 as libc::c_int + b) as usize] * logE[b as usize];
            b += 1;
        }
        BFCC[i as usize] = sum;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        let mut sum_0: libc::c_float = 0 as libc::c_int as libc::c_float;
        b = 0 as libc::c_int;
        while b < 16 as libc::c_int {
            sum_0
                += dct_table[(i * 16 as libc::c_int + b) as usize] * 0.5f32
                    * ((*tonal).highE[b as usize] + (*tonal).lowE[b as usize]);
            b += 1;
        }
        midE[i as usize] = sum_0;
        i += 1;
    }
    frame_stationarity /= NB_TBANDS as libc::c_float;
    relativeE /= NB_TBANDS as libc::c_float;
    if (*tonal).count < 10 as libc::c_int {
        relativeE = 0.5f32;
    }
    frame_noisiness /= NB_TBANDS as libc::c_float;
    (*info)
        .activity = frame_noisiness
        + (1 as libc::c_int as libc::c_float - frame_noisiness) * relativeE;
    frame_tonality = max_frame_tonality
        / (NB_TBANDS - NB_TONAL_SKIP_BANDS) as libc::c_float;
    frame_tonality = if frame_tonality > (*tonal).prev_tonality * 0.8f32 {
        frame_tonality
    } else {
        (*tonal).prev_tonality * 0.8f32
    };
    (*tonal).prev_tonality = frame_tonality;
    slope /= (8 as libc::c_int * 8 as libc::c_int) as libc::c_float;
    (*info).tonality_slope = slope;
    (*tonal).E_count = ((*tonal).E_count + 1 as libc::c_int) % NB_FRAMES;
    (*tonal)
        .count = if ((*tonal).count + 1 as libc::c_int) < 10000 as libc::c_int {
        (*tonal).count + 1 as libc::c_int
    } else {
        10000 as libc::c_int
    };
    (*info).tonality = frame_tonality;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        features[i
            as usize] = -0.12299f32
            * (BFCC[i as usize] + (*tonal).mem[(i + 24 as libc::c_int) as usize])
            + 0.49195f32
                * ((*tonal).mem[i as usize]
                    + (*tonal).mem[(i + 16 as libc::c_int) as usize])
            + 0.69693f32 * (*tonal).mem[(i + 8 as libc::c_int) as usize]
            - 1.4349f32 * (*tonal).cmean[i as usize];
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        (*tonal)
            .cmean[i
            as usize] = (1 as libc::c_int as libc::c_float - alpha)
            * (*tonal).cmean[i as usize] + alpha * BFCC[i as usize];
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        features[(4 as libc::c_int + i)
            as usize] = 0.63246f32
            * (BFCC[i as usize] - (*tonal).mem[(i + 24 as libc::c_int) as usize])
            + 0.31623f32
                * ((*tonal).mem[i as usize]
                    - (*tonal).mem[(i + 16 as libc::c_int) as usize]);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        features[(8 as libc::c_int + i)
            as usize] = 0.53452f32
            * (BFCC[i as usize] + (*tonal).mem[(i + 24 as libc::c_int) as usize])
            - 0.26726f32
                * ((*tonal).mem[i as usize]
                    + (*tonal).mem[(i + 16 as libc::c_int) as usize])
            - 0.53452f32 * (*tonal).mem[(i + 8 as libc::c_int) as usize];
        i += 1;
    }
    if (*tonal).count > 5 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 9 as libc::c_int {
            (*tonal)
                .std[i
                as usize] = (1 as libc::c_int as libc::c_float - alpha)
                * (*tonal).std[i as usize]
                + alpha * features[i as usize] * features[i as usize];
            i += 1;
        }
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        features[i as usize] = BFCC[i as usize] - midE[i as usize];
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        (*tonal)
            .mem[(i + 24 as libc::c_int)
            as usize] = (*tonal).mem[(i + 16 as libc::c_int) as usize];
        (*tonal)
            .mem[(i + 16 as libc::c_int)
            as usize] = (*tonal).mem[(i + 8 as libc::c_int) as usize];
        (*tonal).mem[(i + 8 as libc::c_int) as usize] = (*tonal).mem[i as usize];
        (*tonal).mem[i as usize] = BFCC[i as usize];
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 9 as libc::c_int {
        features[(11 as libc::c_int + i)
            as usize] = sqrt((*tonal).std[i as usize] as libc::c_double) as libc::c_float
            - std_feature_bias[i as usize];
        i += 1;
    }
    features[18 as libc::c_int as usize] = spec_variability - 0.78f32;
    features[20 as libc::c_int as usize] = (*info).tonality - 0.154723f32;
    features[21 as libc::c_int as usize] = (*info).activity - 0.724643f32;
    features[22 as libc::c_int as usize] = frame_stationarity - 0.743717f32;
    features[23 as libc::c_int as usize] = (*info).tonality_slope + 0.069216f32;
    features[24 as libc::c_int as usize] = (*tonal).lowECount - 0.067930f32;
    compute_dense(&layer0, layer_out.as_mut_ptr(), features.as_mut_ptr());
    compute_gru(&layer1, ((*tonal).rnn_state).as_mut_ptr(), layer_out.as_mut_ptr());
    compute_dense(&layer2, frame_probs.as_mut_ptr(), ((*tonal).rnn_state).as_mut_ptr());
    (*info).activity_probability = frame_probs[1 as libc::c_int as usize];
    (*info).music_prob = frame_probs[0 as libc::c_int as usize];
    (*info).bandwidth = bandwidth;
    (*tonal).prev_bandwidth = bandwidth;
    (*info).noisiness = frame_noisiness;
    (*info).valid = 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "953:1"]
pub unsafe extern "C" fn run_analysis(
    mut analysis: *mut TonalityAnalysisState,
    celt_mode: *const OpusCustomMode,
    analysis_pcm: *const libc::c_void,
    mut analysis_frame_size: libc::c_int,
    frame_size: libc::c_int,
    c1: libc::c_int,
    c2: libc::c_int,
    C: libc::c_int,
    Fs: opus_int32,
    lsb_depth: libc::c_int,
    downmix: downmix_func,
    analysis_info: *mut AnalysisInfo,
) {
    let mut offset: libc::c_int = 0;
    let mut pcm_len: libc::c_int = 0;
    analysis_frame_size -= analysis_frame_size & 1 as libc::c_int;
    if !analysis_pcm.is_null() {
        analysis_frame_size = if ((100 as libc::c_int - 5 as libc::c_int) * Fs
            / 50 as libc::c_int) < analysis_frame_size
        {
            (100 as libc::c_int - 5 as libc::c_int) * Fs / 50 as libc::c_int
        } else {
            analysis_frame_size
        };
        pcm_len = analysis_frame_size - (*analysis).analysis_offset;
        offset = (*analysis).analysis_offset;
        while pcm_len > 0 as libc::c_int {
            tonality_analysis(
                analysis,
                celt_mode,
                analysis_pcm,
                if (Fs / 50 as libc::c_int) < pcm_len {
                    Fs / 50 as libc::c_int
                } else {
                    pcm_len
                },
                offset,
                c1,
                c2,
                C,
                lsb_depth,
                downmix,
            );
            offset += Fs / 50 as libc::c_int;
            pcm_len -= Fs / 50 as libc::c_int;
        }
        (*analysis).analysis_offset = analysis_frame_size;
        (*analysis).analysis_offset -= frame_size;
    }
    tonality_get_info(analysis, analysis_info, frame_size);
}
