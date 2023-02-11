#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(register_tool)]
#![feature(stdsimd)]
#![register_tool(c2rust)]

pub mod externs;

extern crate libc;
mod celt {
    pub mod bands;
    pub mod celt;
    pub mod celt_decoder;
    pub mod celt_encoder;
    pub mod celt_lpc;
    pub mod cwrs;
    pub mod entcode;
    pub mod entdec;
    pub mod entenc;
    pub mod kiss_fft;
    pub mod laplace;
    pub mod mathops;
    pub mod mdct;
    pub mod modes;
    pub mod pitch;
    pub mod quant_bands;
    pub mod rate;
    // pub mod tests {
    //     pub mod test_unit_cwrs32;
    //     pub mod test_unit_dft;
    //     pub mod test_unit_entropy;
    //     pub mod test_unit_laplace;
    //     pub mod test_unit_mathops;
    //     pub mod test_unit_mdct;
    //     pub mod test_unit_rotation;
    //     pub mod test_unit_types;
    // } // mod tests
    pub mod vq;
} // mod celt
mod silk {
    pub mod A2NLSF;
    pub mod CNG;
    pub mod HP_variable_cutoff;
    pub mod LPC_analysis_filter;
    pub mod LPC_fit;
    pub mod LPC_inv_pred_gain;
    pub mod LP_variable_cutoff;
    pub mod NLSF2A;
    pub mod NLSF_VQ;
    pub mod NLSF_VQ_weights_laroia;
    pub mod NLSF_decode;
    pub mod NLSF_del_dec_quant;
    pub mod NLSF_encode;
    pub mod NLSF_stabilize;
    pub mod NLSF_unpack;
    pub mod NSQ;
    pub mod NSQ_del_dec;
    pub mod PLC;
    pub mod VAD;
    pub mod VQ_WMat_EC;
    pub mod ana_filt_bank_1;
    pub mod biquad_alt;
    pub mod bwexpander;
    pub mod bwexpander_32;
    pub mod check_control_input;
    pub mod code_signs;
    pub mod control_SNR;
    pub mod control_audio_bandwidth;
    pub mod control_codec;
    pub mod debug;
    pub mod dec_API;
    pub mod decode_core;
    pub mod decode_frame;
    pub mod decode_indices;
    pub mod decode_parameters;
    pub mod decode_pitch;
    pub mod decode_pulses;
    pub mod decoder_set_fs;
    pub mod enc_API;
    pub mod encode_indices;
    pub mod encode_pulses;
    pub mod float {
        pub mod LPC_analysis_filter_FLP;
        pub mod LPC_inv_pred_gain_FLP;
        pub mod LTP_analysis_filter_FLP;
        pub mod LTP_scale_ctrl_FLP;
        pub mod apply_sine_window_FLP;
        pub mod autocorrelation_FLP;
        pub mod burg_modified_FLP;
        pub mod bwexpander_FLP;
        pub mod corrMatrix_FLP;
        pub mod encode_frame_FLP;
        pub mod energy_FLP;
        pub mod find_LPC_FLP;
        pub mod find_LTP_FLP;
        pub mod find_pitch_lags_FLP;
        pub mod find_pred_coefs_FLP;
        pub mod inner_product_FLP;
        pub mod k2a_FLP;
        pub mod noise_shape_analysis_FLP;
        pub mod pitch_analysis_core_FLP;
        pub mod process_gains_FLP;
        pub mod regularize_correlations_FLP;
        pub mod residual_energy_FLP;
        pub mod scale_copy_vector_FLP;
        pub mod scale_vector_FLP;
        pub mod schur_FLP;
        pub mod sort_FLP;
        pub mod warped_autocorrelation_FLP;
        pub mod wrappers_FLP;
    } // mod float
    pub mod gain_quant;
    pub mod init_decoder;
    pub mod init_encoder;
    pub mod inner_prod_aligned;
    pub mod interpolate;
    pub mod lin2log;
    pub mod log2lin;
    pub mod pitch_est_tables;
    pub mod process_NLSFs;
    pub mod quant_LTP_gains;
    pub mod resampler;
    pub mod resampler_down2;
    pub mod resampler_down2_3;
    pub mod resampler_private_AR2;
    pub mod resampler_private_IIR_FIR;
    pub mod resampler_private_down_FIR;
    pub mod resampler_private_up2_HQ;
    pub mod resampler_rom;
    pub mod shell_coder;
    pub mod sigm_Q15;
    pub mod sort;
    pub mod stereo_LR_to_MS;
    pub mod stereo_MS_to_LR;
    pub mod stereo_decode_pred;
    pub mod stereo_encode_pred;
    pub mod stereo_find_predictor;
    pub mod stereo_quant_pred;
    pub mod sum_sqr_shift;
    pub mod table_LSF_cos;
    pub mod tables_LTP;
    pub mod tables_NLSF_CB_NB_MB;
    pub mod tables_NLSF_CB_WB;
    pub mod tables_gain;
    pub mod tables_other;
    pub mod tables_pitch_lag;
    pub mod tables_pulses_per_block;
    // stuff for structs that do not have a clear home, named after the header files
    pub mod Inlines;
    pub mod SigProc_FIX;
    pub mod macros;
    pub mod resampler_structs;
    pub mod structs;
} // mod silk
mod src {
    pub mod analysis;
    pub mod mapping_matrix;
    pub mod mlp;
    pub mod mlp_data;
    pub mod opus;
    pub mod opus_decoder;
    pub mod opus_encoder;
    pub mod opus_multistream;
    pub mod opus_multistream_decoder;
    pub mod opus_multistream_encoder;
    pub mod opus_projection_decoder;
    pub mod opus_projection_encoder;
    pub mod repacketizer;
    // stuff for structs that do not have a clear home, named after the header files
    pub mod opus_private;
} // mod src

// TODO: copy over the docs
// =====
// opus.h
// =====

// opus_encoder
pub use crate::src::opus_encoder::{
    opus_encode, opus_encode_float, opus_encoder_create, opus_encoder_ctl, opus_encoder_destroy,
    opus_encoder_get_size, opus_encoder_init, OpusEncoder,
};
// opus_decoder
pub use crate::src::opus_decoder::{
    opus_decode, opus_decode_float, opus_decoder_create, opus_decoder_ctl, opus_decoder_destroy,
    opus_decoder_get_nb_samples, opus_decoder_get_size, opus_decoder_init,
    opus_packet_get_bandwidth, opus_packet_get_nb_channels, opus_packet_get_nb_frames,
    opus_packet_get_nb_samples, OpusDecoder,
};

pub use crate::src::opus::{
    opus_packet_get_samples_per_frame, opus_packet_parse, opus_pcm_soft_clip,
};
// opus_repacketizer
pub use crate::src::repacketizer::{
    opus_multistream_packet_pad, opus_multistream_packet_unpad, opus_packet_pad, opus_packet_unpad,
    opus_repacketizer_cat, opus_repacketizer_create, opus_repacketizer_destroy,
    opus_repacketizer_get_nb_frames, opus_repacketizer_get_size, opus_repacketizer_init,
    opus_repacketizer_out, opus_repacketizer_out_range, opus_repacketizer_out_range_impl,
    OpusRepacketizer,
};

// =====
// opus_defines.h
// =====
// TODO: error codes (the defines were eaten by c2rust)
// opus_libinfo
pub use crate::celt::celt::{opus_get_version_string, opus_strerror};

// =====
// opus_multistream.h
// =====
pub use crate::src::opus_multistream_decoder::{
    opus_multistream_decode, opus_multistream_decode_float, opus_multistream_decoder_create,
    opus_multistream_decoder_ctl, opus_multistream_decoder_destroy,
    opus_multistream_decoder_get_size, opus_multistream_decoder_init, OpusMSDecoder,
};
pub use crate::src::opus_multistream_encoder::{
    opus_multistream_encode, opus_multistream_encode_float, opus_multistream_encoder_create,
    opus_multistream_encoder_ctl, opus_multistream_encoder_destroy,
    opus_multistream_encoder_get_size, opus_multistream_encoder_init,
    opus_multistream_surround_encoder_create, opus_multistream_surround_encoder_get_size,
    opus_multistream_surround_encoder_init, OpusMSEncoder,
};

// =====
// opus_projection.h
// =====
pub use crate::src::opus_projection_decoder::{
    opus_projection_decode, opus_projection_decode_float, opus_projection_decoder_create,
    opus_projection_decoder_ctl, opus_projection_decoder_destroy, opus_projection_decoder_get_size,
    opus_projection_decoder_init, OpusProjectionDecoder,
};
pub use crate::src::opus_projection_encoder::{
    opus_projection_ambisonics_encoder_create, opus_projection_ambisonics_encoder_get_size,
    opus_projection_ambisonics_encoder_init, opus_projection_encode, opus_projection_encode_float,
    opus_projection_encoder_ctl, opus_projection_encoder_destroy, OpusProjectionEncoder,
};
