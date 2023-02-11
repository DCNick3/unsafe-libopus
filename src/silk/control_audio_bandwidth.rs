use ::libc;

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:32"]
pub mod define_h {
    #[c2rust::src_loc = "90:9"]
    pub const MAX_NB_SUBFR: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "94:9"]
    pub const SUB_FRAME_LENGTH_MS: libc::c_int = 5 as libc::c_int;
    #[c2rust::src_loc = "96:9"]
    pub const MAX_FRAME_LENGTH_MS: libc::c_int = SUB_FRAME_LENGTH_MS * MAX_NB_SUBFR;
    #[c2rust::src_loc = "215:9"]
    pub const TRANSITION_TIME_MS: libc::c_int = 5120 as libc::c_int;
    #[c2rust::src_loc = "219:9"]
    pub const TRANSITION_FRAMES: libc::c_int = TRANSITION_TIME_MS / MAX_FRAME_LENGTH_MS;
}
pub use self::define_h::{
    MAX_FRAME_LENGTH_MS, MAX_NB_SUBFR, SUB_FRAME_LENGTH_MS, TRANSITION_FRAMES, TRANSITION_TIME_MS,
};
use crate::externs::memset;
use crate::silk::enc_API::silk_EncControlStruct;
use crate::silk::structs::silk_encoder_state;
#[no_mangle]
#[c2rust::src_loc = "36:1"]
pub unsafe extern "C" fn silk_control_audio_bandwidth(
    mut psEncC: *mut silk_encoder_state,
    mut encControl: *mut silk_EncControlStruct,
) -> libc::c_int {
    let mut fs_kHz: libc::c_int = 0;
    let mut orig_kHz: libc::c_int = 0;
    let mut fs_Hz: i32 = 0;
    orig_kHz = (*psEncC).fs_kHz;
    if orig_kHz == 0 as libc::c_int {
        orig_kHz = (*psEncC).sLP.saved_fs_kHz;
    }
    fs_kHz = orig_kHz;
    fs_Hz = fs_kHz as i16 as i32 * 1000 as libc::c_int as i16 as i32;
    if fs_Hz == 0 as libc::c_int {
        fs_Hz = if (*psEncC).desiredInternal_fs_Hz < (*psEncC).API_fs_Hz {
            (*psEncC).desiredInternal_fs_Hz
        } else {
            (*psEncC).API_fs_Hz
        };
        fs_kHz = fs_Hz / 1000 as libc::c_int;
    } else if fs_Hz > (*psEncC).API_fs_Hz
        || fs_Hz > (*psEncC).maxInternal_fs_Hz
        || fs_Hz < (*psEncC).minInternal_fs_Hz
    {
        fs_Hz = (*psEncC).API_fs_Hz;
        fs_Hz = if fs_Hz < (*psEncC).maxInternal_fs_Hz {
            fs_Hz
        } else {
            (*psEncC).maxInternal_fs_Hz
        };
        fs_Hz = if fs_Hz > (*psEncC).minInternal_fs_Hz {
            fs_Hz
        } else {
            (*psEncC).minInternal_fs_Hz
        };
        fs_kHz = fs_Hz / 1000 as libc::c_int;
    } else {
        if (*psEncC).sLP.transition_frame_no >= TRANSITION_FRAMES {
            (*psEncC).sLP.mode = 0 as libc::c_int;
        }
        if (*psEncC).allow_bandwidth_switch != 0 || (*encControl).opusCanSwitch != 0 {
            if orig_kHz as i16 as i32 * 1000 as libc::c_int as i16 as i32
                > (*psEncC).desiredInternal_fs_Hz
            {
                if (*psEncC).sLP.mode == 0 as libc::c_int {
                    (*psEncC).sLP.transition_frame_no = TRANSITION_FRAMES;
                    memset(
                        ((*psEncC).sLP.In_LP_State).as_mut_ptr() as *mut libc::c_void,
                        0 as libc::c_int,
                        ::core::mem::size_of::<[i32; 2]>() as libc::c_ulong,
                    );
                }
                if (*encControl).opusCanSwitch != 0 {
                    (*psEncC).sLP.mode = 0 as libc::c_int;
                    fs_kHz = if orig_kHz == 16 as libc::c_int {
                        12 as libc::c_int
                    } else {
                        8 as libc::c_int
                    };
                } else if (*psEncC).sLP.transition_frame_no <= 0 as libc::c_int {
                    (*encControl).switchReady = 1 as libc::c_int;
                    (*encControl).maxBits -= (*encControl).maxBits * 5 as libc::c_int
                        / ((*encControl).payloadSize_ms + 5 as libc::c_int);
                } else {
                    (*psEncC).sLP.mode = -(2 as libc::c_int);
                }
            } else if (orig_kHz as i16 as i32 * 1000 as libc::c_int as i16 as i32)
                < (*psEncC).desiredInternal_fs_Hz
            {
                if (*encControl).opusCanSwitch != 0 {
                    fs_kHz = if orig_kHz == 8 as libc::c_int {
                        12 as libc::c_int
                    } else {
                        16 as libc::c_int
                    };
                    (*psEncC).sLP.transition_frame_no = 0 as libc::c_int;
                    memset(
                        ((*psEncC).sLP.In_LP_State).as_mut_ptr() as *mut libc::c_void,
                        0 as libc::c_int,
                        ::core::mem::size_of::<[i32; 2]>() as libc::c_ulong,
                    );
                    (*psEncC).sLP.mode = 1 as libc::c_int;
                } else if (*psEncC).sLP.mode == 0 as libc::c_int {
                    (*encControl).switchReady = 1 as libc::c_int;
                    (*encControl).maxBits -= (*encControl).maxBits * 5 as libc::c_int
                        / ((*encControl).payloadSize_ms + 5 as libc::c_int);
                } else {
                    (*psEncC).sLP.mode = 1 as libc::c_int;
                }
            } else if (*psEncC).sLP.mode < 0 as libc::c_int {
                (*psEncC).sLP.mode = 1 as libc::c_int;
            }
        }
    }
    return fs_kHz;
}
