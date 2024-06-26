pub const TYPE_NO_VOICE_ACTIVITY: i32 = 0;
pub const MAX_LPC_ORDER: i32 = 16;
pub const MIN_LPC_ORDER: i32 = 10;
pub const MAX_NB_SUBFR: i32 = 4;
pub const SHELL_CODEC_FRAME_LENGTH: i32 = 16;
pub const N_RATE_LEVELS: i32 = 10;
pub const SILK_MAX_PULSES: i32 = 16;
pub const TYPE_VOICED: i32 = 2;
pub const MAX_SHAPE_LPC_ORDER: i32 = 24;
pub const HARM_SHAPE_FIR_TAPS: i32 = 3;
pub const LTP_ORDER: i32 = 5;
pub const NSQ_LPC_BUF_LENGTH: i32 = MAX_LPC_ORDER;
pub const STEREO_INTERP_LEN_MS: i32 = 8;
pub const LA_SHAPE_MS: i32 = 5;
pub const STEREO_QUANT_SUB_STEPS: i32 = 5;
pub const STEREO_QUANT_TAB_SIZE: i32 = 16;
pub const MAX_DEL_DEC_STATES: i32 = 4;
pub const SUB_FRAME_LENGTH_MS: i32 = 5;
pub const CODE_INDEPENDENTLY: i32 = 0;
pub const CODE_CONDITIONALLY: i32 = 2;
pub const NLSF_QUANT_MAX_AMPLITUDE: i32 = 4;
pub const CODE_INDEPENDENTLY_NO_LTP_SCALING: i32 = 1;
pub const MAX_API_FS_KHZ: i32 = 48;
pub const ENCODER_NUM_CHANNELS: i32 = 2;
pub const QUANT_LEVEL_ADJUST_Q10: i32 = 80;
pub const MIN_QGAIN_DB: i32 = 2;
pub const MIN_DELTA_GAIN_QUANT: i32 = -(4);
pub const N_LEVELS_QGAIN: i32 = 64;
pub const MAX_DELTA_GAIN_QUANT: i32 = 36;
pub const TRANSITION_NA: usize = 2;
pub const TRANSITION_NB: usize = 3;
pub const TRANSITION_INT_NUM: usize = 5;
pub const OFFSET_UVL_Q10: i32 = 100;
pub const OFFSET_UVH_Q10: i32 = 240;
pub const OFFSET_VL_Q10: i32 = 32;
pub const OFFSET_VH_Q10: i32 = 100;
pub const NLSF_QUANT_DEL_DEC_STATES_LOG2: i32 = 2;
pub const NLSF_QUANT_DEL_DEC_STATES: i32 = (1) << NLSF_QUANT_DEL_DEC_STATES_LOG2;
pub const NLSF_QUANT_MAX_AMPLITUDE_EXT: i32 = 10;
pub const DECISION_DELAY: i32 = 40;
pub const MAX_FRAME_LENGTH_MS: i32 = SUB_FRAME_LENGTH_MS * MAX_NB_SUBFR;
pub const TRANSITION_TIME_MS: i32 = 5120;
pub const TRANSITION_FRAMES: i32 = TRANSITION_TIME_MS / MAX_FRAME_LENGTH_MS;
pub const LSF_COS_TAB_SZ_FIX: i32 = 128;
pub const MAX_LPC_STABILIZE_ITERATIONS: i32 = 16;
pub const MAX_PREDICTION_POWER_GAIN: f32 = 1e4f32;
pub const CNG_BUF_MASK_MAX: i32 = 255;
pub const TYPE_UNVOICED: i32 = 1;
pub const NB_SPEECH_FRAMES_BEFORE_DTX: i32 = 10;
pub const MAX_CONSECUTIVE_DTX: i32 = 20;
pub const VAD_NO_ACTIVITY: i32 = 0;
pub const BWE_AFTER_LOSS_Q16: i32 = 63570;
pub const MAX_PREDICTION_POWER_GAIN_AFTER_RESET: f32 = 1e2f32;
pub const VAD_N_BANDS: i32 = 4;
pub const VAD_NEGATIVE_OFFSET_Q5: i32 = 128;
pub const VAD_NOISE_LEVEL_SMOOTH_COEF_Q16: i32 = 1024;
pub const VAD_INTERNAL_SUBFRAMES_LOG2: i32 = 2;
pub const VAD_INTERNAL_SUBFRAMES: i32 = (1) << VAD_INTERNAL_SUBFRAMES_LOG2;
pub const USE_HARM_SHAPING: i32 = 1;
pub const VAD_NO_DECISION: i32 = -1;
pub const DTX_ACTIVITY_THRESHOLD: f32 = 0.1f32;
