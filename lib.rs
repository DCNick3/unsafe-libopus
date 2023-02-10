#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(register_tool)]
#![feature(stdsimd)]
#![register_tool(c2rust)]

extern crate libc;
pub mod src {
    pub mod celt {
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
    pub mod silk {
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
        pub mod tests {
            pub mod test_unit_LPC_inv_pred_gain;
        } // mod tests
    } // mod silk
    pub mod src {
        pub mod analysis;
        pub mod mapping_matrix;
        pub mod mlp;
        pub mod mlp_data;
        pub mod opus;
        pub mod opus_compare;
        pub mod opus_decoder;
        pub mod opus_demo;
        pub mod opus_encoder;
        pub mod opus_multistream;
        pub mod opus_multistream_decoder;
        pub mod opus_multistream_encoder;
        pub mod opus_projection_decoder;
        pub mod opus_projection_encoder;
        pub mod repacketizer;
        pub mod repacketizer_demo;
    } // mod src
      // pub mod tests {
      //     pub mod opus_encode_regressions;
      //     pub mod test_opus_api;
      //     pub mod test_opus_decode;
      //     pub mod test_opus_encode;
      //     pub mod test_opus_padding;
      //     pub mod test_opus_projection;
      // } // mod tests
} // mod src
