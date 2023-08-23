use crate::externs::memcpy;
use crate::silk::resampler_private_IIR_FIR::silk_resampler_private_IIR_FIR;
use crate::silk::resampler_private_down_FIR::silk_resampler_private_down_FIR;
use crate::silk::resampler_private_up2_HQ::silk_resampler_private_up2_HQ_wrapper;
use crate::silk::resampler_rom::{
    silk_Resampler_1_2_COEFS, silk_Resampler_1_3_COEFS, silk_Resampler_1_4_COEFS,
    silk_Resampler_1_6_COEFS, silk_Resampler_2_3_COEFS, silk_Resampler_3_4_COEFS,
    RESAMPLER_DOWN_ORDER_FIR0, RESAMPLER_DOWN_ORDER_FIR1, RESAMPLER_DOWN_ORDER_FIR2,
};
use crate::silk::resampler_structs::{
    sFIR_union, silk_resampler_state_struct, SILK_RESAMPLER_MAX_FIR_ORDER,
    SILK_RESAMPLER_MAX_IIR_ORDER,
};

pub const RESAMPLER_MAX_BATCH_SIZE_MS: i32 = 10;

/*
 * Matrix of resampling methods used:
 *                                 Fs_out (kHz)
 *                        8      12     16     24     48
 *
 *               8        C      UF     U      UF     UF
 *              12        AF     C      UF     U      UF
 * Fs_in (kHz)  16        D      AF     C      UF     UF
 *              24        AF     D      AF     C      U
 *              48        AF     AF     AF     D      C
 *
 * C   -> Copy (no resampling)
 * D   -> Allpass-based 2x downsampling
 * U   -> Allpass-based 2x upsampling
 * UF  -> Allpass-based 2x upsampling followed by FIR interpolation
 * AF  -> AR2 filter followed by FIR interpolation
 */

#[rustfmt::skip]
static delay_matrix_enc: [[i8; 3]; 5] = [
    /* in  \ out  8  12  16 */
    /*  8 */   [  6,  0,  3 ],
    /* 12 */   [  0,  7,  3 ],
    /* 16 */   [  0,  1, 10 ],
    /* 24 */   [  0,  2,  6 ],
    /* 48 */   [ 18, 10, 12 ],
];
#[rustfmt::skip]
static delay_matrix_dec: [[i8; 5]; 3] = [
    /* in  \ out  8  12  16  24  48 */
    /*  8 */   [  4,  0,  2,  0,  0 ],
    /* 12 */   [  0,  9,  4,  7,  4 ],
    /* 16 */   [  0,  3, 12,  7,  7 ],
];

/* Simple way to make [8000, 12000, 16000, 24000, 48000] to [0, 1, 2, 3, 4] */
fn rate_id(r: i32) -> usize {
    match r {
        8000 => 0,
        12000 => 1,
        16000 => 2,
        24000 => 3,
        48000 => 4,
        _ => unreachable!("unsupported sampling rate"),
    }
}

const USE_silk_resampler_copy: i32 = 0;
const USE_silk_resampler_private_up2_HQ_wrapper: i32 = 1;
const USE_silk_resampler_private_IIR_FIR: i32 = 2;
const USE_silk_resampler_private_down_FIR: i32 = 3;

pub fn silk_resampler_init(
    Fs_Hz_in: i32,
    Fs_Hz_out: i32,
    forEnc: i32,
) -> silk_resampler_state_struct {
    let inputDelay = if forEnc != 0 {
        if !matches!(Fs_Hz_in, 8000 | 12000 | 16000 | 24000 | 48000)
            || !matches!(Fs_Hz_out, 8000 | 12000 | 16000)
        {
            // see comments in `[unsafe_libopus::silk::check_control_input]`
            // TODO: we should probably make this function return a Result..
            panic!("libopus: assert(0) called");
            // return -1;
        }

        delay_matrix_enc[rate_id(Fs_Hz_in)][rate_id(Fs_Hz_out)] as i32
    } else {
        if !matches!(Fs_Hz_in, 8000 | 12000 | 16000)
            || !matches!(Fs_Hz_out, 8000 | 12000 | 16000 | 24000 | 48000)
        {
            // see comments in `[unsafe_libopus::silk::check_control_input]`
            // TODO: we should probably make this function return a Result..
            panic!("libopus: assert(0) called");
            // return -1;
        }

        delay_matrix_dec[rate_id(Fs_Hz_in)][rate_id(Fs_Hz_out)] as i32
    };

    let Fs_in_kHz = Fs_Hz_in / 1000;
    let Fs_out_kHz = Fs_Hz_out / 1000;
    let batchSize = Fs_in_kHz * RESAMPLER_MAX_BATCH_SIZE_MS;

    let mut FIR_Fracs = 0;
    let mut FIR_Order = 0;
    let mut Coefs = [].as_ref();

    let mut up2x = 0;
    let resampler_function = if Fs_Hz_out > Fs_Hz_in {
        if Fs_Hz_out == Fs_Hz_in * 2 {
            USE_silk_resampler_private_up2_HQ_wrapper
        } else {
            up2x = 1;
            USE_silk_resampler_private_IIR_FIR
        }
    } else if Fs_Hz_out < Fs_Hz_in {
        if Fs_Hz_out * 4 == Fs_Hz_in * 3 {
            FIR_Fracs = 3;
            FIR_Order = RESAMPLER_DOWN_ORDER_FIR0;
            Coefs = &silk_Resampler_3_4_COEFS;
        } else if Fs_Hz_out * 3 == Fs_Hz_in * 2 {
            FIR_Fracs = 2;
            FIR_Order = RESAMPLER_DOWN_ORDER_FIR0;
            Coefs = &silk_Resampler_2_3_COEFS;
        } else if Fs_Hz_out * 2 == Fs_Hz_in {
            FIR_Fracs = 1;
            FIR_Order = RESAMPLER_DOWN_ORDER_FIR1;
            Coefs = &silk_Resampler_1_2_COEFS;
        } else if Fs_Hz_out * 3 == Fs_Hz_in {
            FIR_Fracs = 1;
            FIR_Order = RESAMPLER_DOWN_ORDER_FIR2;
            Coefs = &silk_Resampler_1_3_COEFS;
        } else if Fs_Hz_out * 4 == Fs_Hz_in {
            FIR_Fracs = 1;
            FIR_Order = RESAMPLER_DOWN_ORDER_FIR2;
            Coefs = &silk_Resampler_1_4_COEFS;
        } else if Fs_Hz_out * 6 == Fs_Hz_in {
            FIR_Fracs = 1;
            FIR_Order = RESAMPLER_DOWN_ORDER_FIR2;
            Coefs = &silk_Resampler_1_6_COEFS;
        } else {
            // see comments in `[unsafe_libopus::silk::check_control_input]`
            // TODO: we should probably make this function return a Result..
            panic!("libopus: assert(0) called");
            // return -1;
        }

        USE_silk_resampler_private_down_FIR
    } else {
        /* Input and output sampling rates are equal: copy */
        USE_silk_resampler_copy
    };
    /* Ratio of input/output samples */
    let mut invRatio_Q16 =
        (((((Fs_Hz_in as u32) << (14 + up2x)) as i32 / Fs_Hz_out) as u32) << 2) as i32;
    /* Make sure the ratio is rounded up */
    while (((invRatio_Q16 as i64 * Fs_Hz_out as i64) >> 16) as i32)
        < ((Fs_Hz_in as u32) << up2x) as i32
    {
        invRatio_Q16 += 1;
    }

    silk_resampler_state_struct {
        sIIR: [0; SILK_RESAMPLER_MAX_IIR_ORDER],
        sFIR: sFIR_union {
            i32_0: [0; SILK_RESAMPLER_MAX_FIR_ORDER],
        },
        delayBuf: [0; 48],
        resampler_function,
        batchSize,
        invRatio_Q16,
        FIR_Order,
        FIR_Fracs,
        Fs_in_kHz,
        Fs_out_kHz,
        inputDelay,
        Coefs,
    }
}

pub unsafe fn silk_resampler(
    S: *mut silk_resampler_state_struct,
    out: *mut i16,
    in_0: *const i16,
    inLen: i32,
) -> i32 {
    let mut nSamples: i32 = 0;
    assert!(inLen >= (*S).Fs_in_kHz);
    assert!((*S).inputDelay <= (*S).Fs_in_kHz);
    nSamples = (*S).Fs_in_kHz - (*S).inputDelay;
    memcpy(
        &mut *((*S).delayBuf)
            .as_mut_ptr()
            .offset((*S).inputDelay as isize) as *mut i16 as *mut core::ffi::c_void,
        in_0 as *const core::ffi::c_void,
        (nSamples as u64).wrapping_mul(::core::mem::size_of::<i16>() as u64),
    );
    match (*S).resampler_function {
        USE_silk_resampler_private_up2_HQ_wrapper => {
            silk_resampler_private_up2_HQ_wrapper(
                S as *mut core::ffi::c_void,
                out,
                ((*S).delayBuf).as_mut_ptr(),
                (*S).Fs_in_kHz,
            );
            silk_resampler_private_up2_HQ_wrapper(
                S as *mut core::ffi::c_void,
                &mut *out.offset((*S).Fs_out_kHz as isize),
                &*in_0.offset(nSamples as isize),
                inLen - (*S).Fs_in_kHz,
            );
        }
        USE_silk_resampler_private_IIR_FIR => {
            silk_resampler_private_IIR_FIR(
                S as *mut core::ffi::c_void,
                out,
                ((*S).delayBuf).as_mut_ptr() as *const i16,
                (*S).Fs_in_kHz,
            );
            silk_resampler_private_IIR_FIR(
                S as *mut core::ffi::c_void,
                &mut *out.offset((*S).Fs_out_kHz as isize),
                &*in_0.offset(nSamples as isize),
                inLen - (*S).Fs_in_kHz,
            );
        }
        USE_silk_resampler_private_down_FIR => {
            silk_resampler_private_down_FIR(
                S as *mut core::ffi::c_void,
                out,
                ((*S).delayBuf).as_mut_ptr() as *const i16,
                (*S).Fs_in_kHz,
            );
            silk_resampler_private_down_FIR(
                S as *mut core::ffi::c_void,
                &mut *out.offset((*S).Fs_out_kHz as isize),
                &*in_0.offset(nSamples as isize),
                inLen - (*S).Fs_in_kHz,
            );
        }
        _ => {
            memcpy(
                out as *mut core::ffi::c_void,
                ((*S).delayBuf).as_mut_ptr() as *const core::ffi::c_void,
                ((*S).Fs_in_kHz as u64).wrapping_mul(::core::mem::size_of::<i16>() as u64),
            );
            memcpy(
                &mut *out.offset((*S).Fs_out_kHz as isize) as *mut i16 as *mut core::ffi::c_void,
                &*in_0.offset(nSamples as isize) as *const i16 as *const core::ffi::c_void,
                ((inLen - (*S).Fs_in_kHz) as u64)
                    .wrapping_mul(::core::mem::size_of::<i16>() as u64),
            );
        }
    }
    memcpy(
        ((*S).delayBuf).as_mut_ptr() as *mut core::ffi::c_void,
        &*in_0.offset((inLen - (*S).inputDelay) as isize) as *const i16 as *const core::ffi::c_void,
        ((*S).inputDelay as u64).wrapping_mul(::core::mem::size_of::<i16>() as u64),
    );
    return 0;
}
