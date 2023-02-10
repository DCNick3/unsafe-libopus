use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "37:1"]
    pub type __int8_t = libc::c_schar;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:33"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "24:1"]
    pub type int8_t = __int8_t;
    use super::types_h::__int8_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:33"]
pub mod opus_types_h {
    #[c2rust::src_loc = "51:4"]
    pub type opus_int8 = int8_t;
    use super::stdint_intn_h::int8_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/src/mlp.h:37"]
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
    #[c2rust::src_loc = "32:9"]
    pub const WEIGHTS_SCALE: libc::c_float = 1.0f32
        / 128 as libc::c_int as libc::c_float;
    use super::opus_types_h::opus_int8;
}
#[c2rust::header_src = "/usr/include/bits/mathcalls.h:32"]
pub mod mathcalls_h {
    extern "C" {
        #[c2rust::src_loc = "165:14"]
        pub fn floor(_: libc::c_double) -> libc::c_double;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/src/tansig_table.h:36"]
pub mod tansig_table_h {
    #[c2rust::src_loc = "3:20"]
    pub static mut tansig_table: [libc::c_float; 201] = [
        0.000000f32,
        0.039979f32,
        0.079830f32,
        0.119427f32,
        0.158649f32,
        0.197375f32,
        0.235496f32,
        0.272905f32,
        0.309507f32,
        0.345214f32,
        0.379949f32,
        0.413644f32,
        0.446244f32,
        0.477700f32,
        0.507977f32,
        0.537050f32,
        0.564900f32,
        0.591519f32,
        0.616909f32,
        0.641077f32,
        0.664037f32,
        0.685809f32,
        0.706419f32,
        0.725897f32,
        0.744277f32,
        0.761594f32,
        0.777888f32,
        0.793199f32,
        0.807569f32,
        0.821040f32,
        0.833655f32,
        0.845456f32,
        0.856485f32,
        0.866784f32,
        0.876393f32,
        0.885352f32,
        0.893698f32,
        0.901468f32,
        0.908698f32,
        0.915420f32,
        0.921669f32,
        0.927473f32,
        0.932862f32,
        0.937863f32,
        0.942503f32,
        0.946806f32,
        0.950795f32,
        0.954492f32,
        0.957917f32,
        0.961090f32,
        0.964028f32,
        0.966747f32,
        0.969265f32,
        0.971594f32,
        0.973749f32,
        0.975743f32,
        0.977587f32,
        0.979293f32,
        0.980869f32,
        0.982327f32,
        0.983675f32,
        0.984921f32,
        0.986072f32,
        0.987136f32,
        0.988119f32,
        0.989027f32,
        0.989867f32,
        0.990642f32,
        0.991359f32,
        0.992020f32,
        0.992631f32,
        0.993196f32,
        0.993718f32,
        0.994199f32,
        0.994644f32,
        0.995055f32,
        0.995434f32,
        0.995784f32,
        0.996108f32,
        0.996407f32,
        0.996682f32,
        0.996937f32,
        0.997172f32,
        0.997389f32,
        0.997590f32,
        0.997775f32,
        0.997946f32,
        0.998104f32,
        0.998249f32,
        0.998384f32,
        0.998508f32,
        0.998623f32,
        0.998728f32,
        0.998826f32,
        0.998916f32,
        0.999000f32,
        0.999076f32,
        0.999147f32,
        0.999213f32,
        0.999273f32,
        0.999329f32,
        0.999381f32,
        0.999428f32,
        0.999472f32,
        0.999513f32,
        0.999550f32,
        0.999585f32,
        0.999617f32,
        0.999646f32,
        0.999673f32,
        0.999699f32,
        0.999722f32,
        0.999743f32,
        0.999763f32,
        0.999781f32,
        0.999798f32,
        0.999813f32,
        0.999828f32,
        0.999841f32,
        0.999853f32,
        0.999865f32,
        0.999875f32,
        0.999885f32,
        0.999893f32,
        0.999902f32,
        0.999909f32,
        0.999916f32,
        0.999923f32,
        0.999929f32,
        0.999934f32,
        0.999939f32,
        0.999944f32,
        0.999948f32,
        0.999952f32,
        0.999956f32,
        0.999959f32,
        0.999962f32,
        0.999965f32,
        0.999968f32,
        0.999970f32,
        0.999973f32,
        0.999975f32,
        0.999977f32,
        0.999978f32,
        0.999980f32,
        0.999982f32,
        0.999983f32,
        0.999984f32,
        0.999986f32,
        0.999987f32,
        0.999988f32,
        0.999989f32,
        0.999990f32,
        0.999990f32,
        0.999991f32,
        0.999992f32,
        0.999992f32,
        0.999993f32,
        0.999994f32,
        0.999994f32,
        0.999994f32,
        0.999995f32,
        0.999995f32,
        0.999996f32,
        0.999996f32,
        0.999996f32,
        0.999997f32,
        0.999997f32,
        0.999997f32,
        0.999997f32,
        0.999997f32,
        0.999998f32,
        0.999998f32,
        0.999998f32,
        0.999998f32,
        0.999998f32,
        0.999998f32,
        0.999999f32,
        0.999999f32,
        0.999999f32,
        0.999999f32,
        0.999999f32,
        0.999999f32,
        0.999999f32,
        0.999999f32,
        0.999999f32,
        0.999999f32,
        0.999999f32,
        0.999999f32,
        0.999999f32,
        1.000000f32,
        1.000000f32,
        1.000000f32,
        1.000000f32,
        1.000000f32,
        1.000000f32,
        1.000000f32,
        1.000000f32,
        1.000000f32,
        1.000000f32,
        1.000000f32,
    ];
}
pub use self::types_h::__int8_t;
pub use self::stdint_intn_h::int8_t;
pub use self::opus_types_h::opus_int8;
pub use self::mlp_h::{DenseLayer, GRULayer, WEIGHTS_SCALE};
use self::mathcalls_h::floor;
pub use self::tansig_table_h::tansig_table;
#[inline]
#[c2rust::src_loc = "39:1"]
unsafe extern "C" fn tansig_approx(mut x: libc::c_float) -> libc::c_float {
    let mut i: libc::c_int = 0;
    let mut y: libc::c_float = 0.;
    let mut dy: libc::c_float = 0.;
    let mut sign: libc::c_float = 1 as libc::c_int as libc::c_float;
    if !(x < 8 as libc::c_int as libc::c_float) {
        return 1 as libc::c_int as libc::c_float;
    }
    if !(x > -(8 as libc::c_int) as libc::c_float) {
        return -(1 as libc::c_int) as libc::c_float;
    }
    if x != x {
        return 0 as libc::c_int as libc::c_float;
    }
    if x < 0 as libc::c_int as libc::c_float {
        x = -x;
        sign = -(1 as libc::c_int) as libc::c_float;
    }
    i = floor((0.5f32 + 25 as libc::c_int as libc::c_float * x) as libc::c_double)
        as libc::c_int;
    x -= 0.04f32 * i as libc::c_float;
    y = tansig_table[i as usize];
    dy = 1 as libc::c_int as libc::c_float - y * y;
    y = y + x * dy * (1 as libc::c_int as libc::c_float - y * x);
    return sign * y;
}
#[inline]
#[c2rust::src_loc = "67:1"]
unsafe extern "C" fn sigmoid_approx(mut x: libc::c_float) -> libc::c_float {
    return 0.5f32 + 0.5f32 * tansig_approx(0.5f32 * x);
}
#[c2rust::src_loc = "72:1"]
unsafe extern "C" fn gemm_accum(
    mut out: *mut libc::c_float,
    mut weights: *const opus_int8,
    mut rows: libc::c_int,
    mut cols: libc::c_int,
    mut col_stride: libc::c_int,
    mut x: *const libc::c_float,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < rows {
        j = 0 as libc::c_int;
        while j < cols {
            *out.offset(i as isize)
                += *weights.offset((j * col_stride + i) as isize) as libc::c_int
                    as libc::c_float * *x.offset(j as isize);
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "82:1"]
pub unsafe extern "C" fn compute_dense(
    mut layer: *const DenseLayer,
    mut output: *mut libc::c_float,
    mut input: *const libc::c_float,
) {
    let mut i: libc::c_int = 0;
    let mut N: libc::c_int = 0;
    let mut M: libc::c_int = 0;
    let mut stride: libc::c_int = 0;
    M = (*layer).nb_inputs;
    N = (*layer).nb_neurons;
    stride = N;
    i = 0 as libc::c_int;
    while i < N {
        *output
            .offset(i as isize) = *((*layer).bias).offset(i as isize) as libc::c_float;
        i += 1;
    }
    gemm_accum(output, (*layer).input_weights, N, M, stride, input);
    i = 0 as libc::c_int;
    while i < N {
        *output.offset(i as isize) *= WEIGHTS_SCALE;
        i += 1;
    }
    if (*layer).sigmoid != 0 {
        i = 0 as libc::c_int;
        while i < N {
            *output.offset(i as isize) = sigmoid_approx(*output.offset(i as isize));
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < N {
            *output.offset(i as isize) = tansig_approx(*output.offset(i as isize));
            i += 1;
        }
    };
}
#[no_mangle]
#[c2rust::src_loc = "104:1"]
pub unsafe extern "C" fn compute_gru(
    mut gru: *const GRULayer,
    mut state: *mut libc::c_float,
    mut input: *const libc::c_float,
) {
    let mut i: libc::c_int = 0;
    let mut N: libc::c_int = 0;
    let mut M: libc::c_int = 0;
    let mut stride: libc::c_int = 0;
    let mut tmp: [libc::c_float; 32] = [0.; 32];
    let mut z: [libc::c_float; 32] = [0.; 32];
    let mut r: [libc::c_float; 32] = [0.; 32];
    let mut h: [libc::c_float; 32] = [0.; 32];
    M = (*gru).nb_inputs;
    N = (*gru).nb_neurons;
    stride = 3 as libc::c_int * N;
    i = 0 as libc::c_int;
    while i < N {
        z[i as usize] = *((*gru).bias).offset(i as isize) as libc::c_float;
        i += 1;
    }
    gemm_accum(z.as_mut_ptr(), (*gru).input_weights, N, M, stride, input);
    gemm_accum(z.as_mut_ptr(), (*gru).recurrent_weights, N, N, stride, state);
    i = 0 as libc::c_int;
    while i < N {
        z[i as usize] = sigmoid_approx(WEIGHTS_SCALE * z[i as usize]);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < N {
        r[i as usize] = *((*gru).bias).offset((N + i) as isize) as libc::c_float;
        i += 1;
    }
    gemm_accum(
        r.as_mut_ptr(),
        &*((*gru).input_weights).offset(N as isize),
        N,
        M,
        stride,
        input,
    );
    gemm_accum(
        r.as_mut_ptr(),
        &*((*gru).recurrent_weights).offset(N as isize),
        N,
        N,
        stride,
        state,
    );
    i = 0 as libc::c_int;
    while i < N {
        r[i as usize] = sigmoid_approx(WEIGHTS_SCALE * r[i as usize]);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < N {
        h[i
            as usize] = *((*gru).bias).offset((2 as libc::c_int * N + i) as isize)
            as libc::c_float;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < N {
        tmp[i as usize] = *state.offset(i as isize) * r[i as usize];
        i += 1;
    }
    gemm_accum(
        h.as_mut_ptr(),
        &*((*gru).input_weights).offset((2 as libc::c_int * N) as isize),
        N,
        M,
        stride,
        input,
    );
    gemm_accum(
        h.as_mut_ptr(),
        &*((*gru).recurrent_weights).offset((2 as libc::c_int * N) as isize),
        N,
        N,
        stride,
        tmp.as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i < N {
        h[i
            as usize] = z[i as usize] * *state.offset(i as isize)
            + (1 as libc::c_int as libc::c_float - z[i as usize])
                * tansig_approx(WEIGHTS_SCALE * h[i as usize]);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < N {
        *state.offset(i as isize) = h[i as usize];
        i += 1;
    }
}
