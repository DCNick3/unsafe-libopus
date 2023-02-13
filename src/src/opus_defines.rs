pub const OPUS_OK: i32 = 0 as i32;
pub const OPUS_BAD_ARG: i32 = -1;
pub const OPUS_BUFFER_TOO_SMALL: i32 = -2;
pub const OPUS_INTERNAL_ERROR: i32 = -3;
pub const OPUS_INVALID_PACKET: i32 = -4;
pub const OPUS_UNIMPLEMENTED: i32 = -5;
pub const OPUS_INVALID_STATE: i32 = -6;
pub const OPUS_ALLOC_FAIL: i32 = -7;

pub const OPUS_SET_APPLICATION_REQUEST: i32 = 4000;
pub const OPUS_GET_APPLICATION_REQUEST: i32 = 4001;
pub const OPUS_SET_BITRATE_REQUEST: i32 = 4002;
pub const OPUS_GET_BITRATE_REQUEST: i32 = 4003;
pub const OPUS_SET_MAX_BANDWIDTH_REQUEST: i32 = 4004;
pub const OPUS_GET_MAX_BANDWIDTH_REQUEST: i32 = 4005;
pub const OPUS_SET_VBR_REQUEST: i32 = 4006;
pub const OPUS_GET_VBR_REQUEST: i32 = 4007;
pub const OPUS_SET_BANDWIDTH_REQUEST: i32 = 4008;
pub const OPUS_GET_BANDWIDTH_REQUEST: i32 = 4009;
pub const OPUS_SET_COMPLEXITY_REQUEST: i32 = 4010;
pub const OPUS_GET_COMPLEXITY_REQUEST: i32 = 4011;
pub const OPUS_SET_INBAND_FEC_REQUEST: i32 = 4012;
pub const OPUS_GET_INBAND_FEC_REQUEST: i32 = 4013;
pub const OPUS_SET_PACKET_LOSS_PERC_REQUEST: i32 = 4014;
pub const OPUS_GET_PACKET_LOSS_PERC_REQUEST: i32 = 4015;
pub const OPUS_SET_DTX_REQUEST: i32 = 4016;
pub const OPUS_GET_DTX_REQUEST: i32 = 4017;
pub const OPUS_SET_VBR_CONSTRAINT_REQUEST: i32 = 4020;
pub const OPUS_GET_VBR_CONSTRAINT_REQUEST: i32 = 4021;
pub const OPUS_SET_FORCE_CHANNELS_REQUEST: i32 = 4022;
pub const OPUS_GET_FORCE_CHANNELS_REQUEST: i32 = 4023;
pub const OPUS_SET_SIGNAL_REQUEST: i32 = 4024;
pub const OPUS_GET_SIGNAL_REQUEST: i32 = 4025;
pub const OPUS_GET_LOOKAHEAD_REQUEST: i32 = 4027;
pub const OPUS_RESET_STATE: i32 = 4028;
pub const OPUS_GET_SAMPLE_RATE_REQUEST: i32 = 4029;
pub const OPUS_GET_FINAL_RANGE_REQUEST: i32 = 4031;
pub const OPUS_GET_PITCH_REQUEST: i32 = 4033;
pub const OPUS_SET_GAIN_REQUEST: i32 = 4034;
pub const OPUS_GET_GAIN_REQUEST: i32 = 4045; /* Should have been 4035 */
pub const OPUS_SET_LSB_DEPTH_REQUEST: i32 = 4036;
pub const OPUS_GET_LSB_DEPTH_REQUEST: i32 = 4037;
pub const OPUS_GET_LAST_PACKET_DURATION_REQUEST: i32 = 4039;
pub const OPUS_SET_EXPERT_FRAME_DURATION_REQUEST: i32 = 4040;
pub const OPUS_GET_EXPERT_FRAME_DURATION_REQUEST: i32 = 4041;
pub const OPUS_SET_PREDICTION_DISABLED_REQUEST: i32 = 4042;
pub const OPUS_GET_PREDICTION_DISABLED_REQUEST: i32 = 4043;
pub const OPUS_SET_PHASE_INVERSION_DISABLED_REQUEST: i32 = 4046;
pub const OPUS_GET_PHASE_INVERSION_DISABLED_REQUEST: i32 = 4047;
pub const OPUS_GET_IN_DTX_REQUEST: i32 = 4049;

pub const OPUS_AUTO: i32 = -1000;
pub const OPUS_BITRATE_MAX: i32 = -1;

pub const OPUS_APPLICATION_VOIP: i32 = 2048;
pub const OPUS_APPLICATION_AUDIO: i32 = 2049;
pub const OPUS_APPLICATION_RESTRICTED_LOWDELAY: i32 = 2051;

pub const OPUS_SIGNAL_VOICE: i32 = 3001;
pub const OPUS_SIGNAL_MUSIC: i32 = 3002;
pub const OPUS_BANDWIDTH_NARROWBAND: i32 = 1101;
pub const OPUS_BANDWIDTH_MEDIUMBAND: i32 = 1102;
pub const OPUS_BANDWIDTH_WIDEBAND: i32 = 1103;
pub const OPUS_BANDWIDTH_SUPERWIDEBAND: i32 = 1104;
pub const OPUS_BANDWIDTH_FULLBAND: i32 = 1105;

pub const OPUS_FRAMESIZE_ARG: i32 = 5000;
pub const OPUS_FRAMESIZE_2_5_MS: i32 = 5001;
pub const OPUS_FRAMESIZE_5_MS: i32 = 5002;
pub const OPUS_FRAMESIZE_10_MS: i32 = 5003;
pub const OPUS_FRAMESIZE_20_MS: i32 = 5004;
pub const OPUS_FRAMESIZE_40_MS: i32 = 5005;
pub const OPUS_FRAMESIZE_60_MS: i32 = 5006;
pub const OPUS_FRAMESIZE_80_MS: i32 = 5007;
pub const OPUS_FRAMESIZE_100_MS: i32 = 5008;
pub const OPUS_FRAMESIZE_120_MS: i32 = 5009;
