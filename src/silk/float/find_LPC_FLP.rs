pub mod float_h {
    pub const FLT_MAX: f32 = __FLT_MAX__;
    use super::internal::__FLT_MAX__;
}
pub mod typedef_h {
    pub const silk_float_MAX: f32 = FLT_MAX;
    use super::float_h::FLT_MAX;
}
pub mod internal {
    pub const __FLT_MAX__: f32 = 3.40282347e+38f32;
}
pub use self::float_h::FLT_MAX;
pub use self::internal::__FLT_MAX__;
pub use self::typedef_h::silk_float_MAX;
use crate::celt::celt::celt_fatal;
use crate::silk::define::MAX_NB_SUBFR;
use crate::silk::float::burg_modified_FLP::silk_burg_modified_FLP;
use crate::silk::float::energy_FLP::silk_energy_FLP;
use crate::silk::float::wrappers_FLP::{silk_A2NLSF_FLP, silk_NLSF2A_FLP};
use crate::silk::float::LPC_analysis_filter_FLP::silk_LPC_analysis_filter_FLP;
use crate::silk::interpolate::silk_interpolate;
use crate::silk::structs::silk_encoder_state;

pub unsafe fn silk_find_LPC_FLP(
    psEncC: *mut silk_encoder_state,
    NLSF_Q15: *mut i16,
    x: *const f32,
    minInvGain: f32,
) {
    let mut k: i32 = 0;
    let mut subfr_length: i32 = 0;
    let mut a: [f32; 16] = [0.; 16];
    let mut res_nrg: f32 = 0.;
    let mut res_nrg_2nd: f32 = 0.;
    let mut res_nrg_interp: f32 = 0.;
    let mut NLSF0_Q15: [i16; 16] = [0; 16];
    let mut a_tmp: [f32; 16] = [0.; 16];
    let mut LPC_res: [f32; 384] = [0.; 384];
    subfr_length = (*psEncC).subfr_length + (*psEncC).predictLPCOrder;
    (*psEncC).indices.NLSFInterpCoef_Q2 = 4 as i32 as i8;
    res_nrg = silk_burg_modified_FLP(
        a.as_mut_ptr(),
        x,
        minInvGain,
        subfr_length,
        (*psEncC).nb_subfr,
        (*psEncC).predictLPCOrder,
    );
    if (*psEncC).useInterpolatedNLSFs != 0
        && (*psEncC).first_frame_after_reset == 0
        && (*psEncC).nb_subfr == MAX_NB_SUBFR
    {
        res_nrg -= silk_burg_modified_FLP(
            a_tmp.as_mut_ptr(),
            x.offset((MAX_NB_SUBFR / 2 as i32 * subfr_length) as isize),
            minInvGain,
            subfr_length,
            MAX_NB_SUBFR / 2 as i32,
            (*psEncC).predictLPCOrder,
        );
        silk_A2NLSF_FLP(NLSF_Q15, a_tmp.as_mut_ptr(), (*psEncC).predictLPCOrder);
        res_nrg_2nd = silk_float_MAX;
        k = 3 as i32;
        while k >= 0 as i32 {
            silk_interpolate(
                NLSF0_Q15.as_mut_ptr(),
                ((*psEncC).prev_NLSFq_Q15).as_mut_ptr() as *const i16,
                NLSF_Q15 as *const i16,
                k,
                (*psEncC).predictLPCOrder,
            );
            silk_NLSF2A_FLP(
                a_tmp.as_mut_ptr(),
                NLSF0_Q15.as_mut_ptr(),
                (*psEncC).predictLPCOrder,
                (*psEncC).arch,
            );
            silk_LPC_analysis_filter_FLP(
                LPC_res.as_mut_ptr(),
                a_tmp.as_mut_ptr() as *const f32,
                x,
                2 as i32 * subfr_length,
                (*psEncC).predictLPCOrder,
            );
            res_nrg_interp = (silk_energy_FLP(
                LPC_res
                    .as_mut_ptr()
                    .offset((*psEncC).predictLPCOrder as isize),
                subfr_length - (*psEncC).predictLPCOrder,
            ) + silk_energy_FLP(
                LPC_res
                    .as_mut_ptr()
                    .offset((*psEncC).predictLPCOrder as isize)
                    .offset(subfr_length as isize),
                subfr_length - (*psEncC).predictLPCOrder,
            )) as f32;
            if res_nrg_interp < res_nrg {
                res_nrg = res_nrg_interp;
                (*psEncC).indices.NLSFInterpCoef_Q2 = k as i8;
            } else if res_nrg_interp > res_nrg_2nd {
                break;
            }
            res_nrg_2nd = res_nrg_interp;
            k -= 1;
        }
    }
    if (*psEncC).indices.NLSFInterpCoef_Q2 as i32 == 4 as i32 {
        silk_A2NLSF_FLP(NLSF_Q15, a.as_mut_ptr(), (*psEncC).predictLPCOrder);
    }
    if !((*psEncC).indices.NLSFInterpCoef_Q2 as i32 == 4 as i32
        || (*psEncC).useInterpolatedNLSFs != 0
            && (*psEncC).first_frame_after_reset == 0
            && (*psEncC).nb_subfr == 4 as i32)
    {
        celt_fatal(
            b"assertion failed: psEncC->indices.NLSFInterpCoef_Q2 == 4 || ( psEncC->useInterpolatedNLSFs && !psEncC->first_frame_after_reset && psEncC->nb_subfr == MAX_NB_SUBFR )\0"
                as *const u8 as *const i8,
            b"silk/float/find_LPC_FLP.c\0" as *const u8 as *const i8,
            103 as i32,
        );
    }
}
