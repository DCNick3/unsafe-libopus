use std::path::PathBuf;
use std::{env, path::Path};

const OPUS_SOURCES: &[&str] = &[
    // OPUS_SOURCES
    "src/opus.c",
    "src/opus_decoder.c",
    "src/opus_encoder.c",
    "src/opus_multistream.c",
    "src/opus_multistream_encoder.c",
    "src/opus_multistream_decoder.c",
    "src/repacketizer.c",
    "src/opus_projection_encoder.c",
    "src/opus_projection_decoder.c",
    "src/mapping_matrix.c",
    // OPUS_SOURCES_FLOAT
    "src/analysis.c",
    "src/mlp.c",
    "src/mlp_data.c",
    // CELT_SOURCES
    "celt/bands.c",
    "celt/celt.c",
    "celt/celt_encoder.c",
    "celt/celt_decoder.c",
    "celt/cwrs.c",
    "celt/entcode.c",
    "celt/entdec.c",
    "celt/entenc.c",
    "celt/kiss_fft.c",
    "celt/laplace.c",
    "celt/mathops.c",
    "celt/mdct.c",
    "celt/modes.c",
    "celt/pitch.c",
    "celt/celt_lpc.c",
    "celt/quant_bands.c",
    "celt/rate.c",
    "celt/vq.c",
    // SILK_SOURCES
    "silk/CNG.c",
    "silk/code_signs.c",
    "silk/init_decoder.c",
    "silk/decode_core.c",
    "silk/decode_frame.c",
    "silk/decode_parameters.c",
    "silk/decode_indices.c",
    "silk/decode_pulses.c",
    "silk/decoder_set_fs.c",
    "silk/dec_API.c",
    "silk/enc_API.c",
    "silk/encode_indices.c",
    "silk/encode_pulses.c",
    "silk/gain_quant.c",
    "silk/interpolate.c",
    "silk/LP_variable_cutoff.c",
    "silk/NLSF_decode.c",
    "silk/NSQ.c",
    "silk/NSQ_del_dec.c",
    "silk/PLC.c",
    "silk/shell_coder.c",
    "silk/tables_gain.c",
    "silk/tables_LTP.c",
    "silk/tables_NLSF_CB_NB_MB.c",
    "silk/tables_NLSF_CB_WB.c",
    "silk/tables_other.c",
    "silk/tables_pitch_lag.c",
    "silk/tables_pulses_per_block.c",
    "silk/VAD.c",
    "silk/control_audio_bandwidth.c",
    "silk/quant_LTP_gains.c",
    "silk/VQ_WMat_EC.c",
    "silk/HP_variable_cutoff.c",
    "silk/NLSF_encode.c",
    "silk/NLSF_VQ.c",
    "silk/NLSF_unpack.c",
    "silk/NLSF_del_dec_quant.c",
    "silk/process_NLSFs.c",
    "silk/stereo_LR_to_MS.c",
    "silk/stereo_MS_to_LR.c",
    "silk/check_control_input.c",
    "silk/control_SNR.c",
    "silk/init_encoder.c",
    "silk/control_codec.c",
    "silk/A2NLSF.c",
    "silk/ana_filt_bank_1.c",
    "silk/biquad_alt.c",
    "silk/bwexpander_32.c",
    "silk/bwexpander.c",
    "silk/debug.c",
    "silk/decode_pitch.c",
    "silk/inner_prod_aligned.c",
    "silk/lin2log.c",
    "silk/log2lin.c",
    "silk/LPC_analysis_filter.c",
    "silk/LPC_inv_pred_gain.c",
    "silk/table_LSF_cos.c",
    "silk/NLSF2A.c",
    "silk/NLSF_stabilize.c",
    "silk/NLSF_VQ_weights_laroia.c",
    "silk/pitch_est_tables.c",
    "silk/resampler.c",
    "silk/resampler_down2_3.c",
    "silk/resampler_down2.c",
    "silk/resampler_private_AR2.c",
    "silk/resampler_private_down_FIR.c",
    "silk/resampler_private_IIR_FIR.c",
    "silk/resampler_private_up2_HQ.c",
    "silk/resampler_rom.c",
    "silk/sigm_Q15.c",
    "silk/sort.c",
    "silk/sum_sqr_shift.c",
    "silk/stereo_decode_pred.c",
    "silk/stereo_encode_pred.c",
    "silk/stereo_find_predictor.c",
    "silk/stereo_quant_pred.c",
    "silk/LPC_fit.c",
    // SILK_SOURCES_FLOAT
    "silk/float/apply_sine_window_FLP.c",
    "silk/float/corrMatrix_FLP.c",
    "silk/float/encode_frame_FLP.c",
    "silk/float/find_LPC_FLP.c",
    "silk/float/find_LTP_FLP.c",
    "silk/float/find_pitch_lags_FLP.c",
    "silk/float/find_pred_coefs_FLP.c",
    "silk/float/LPC_analysis_filter_FLP.c",
    "silk/float/LTP_analysis_filter_FLP.c",
    "silk/float/LTP_scale_ctrl_FLP.c",
    "silk/float/noise_shape_analysis_FLP.c",
    "silk/float/process_gains_FLP.c",
    "silk/float/regularize_correlations_FLP.c",
    "silk/float/residual_energy_FLP.c",
    "silk/float/warped_autocorrelation_FLP.c",
    "silk/float/wrappers_FLP.c",
    "silk/float/autocorrelation_FLP.c",
    "silk/float/burg_modified_FLP.c",
    "silk/float/bwexpander_FLP.c",
    "silk/float/energy_FLP.c",
    "silk/float/inner_product_FLP.c",
    "silk/float/k2a_FLP.c",
    "silk/float/LPC_inv_pred_gain_FLP.c",
    "silk/float/pitch_analysis_core_FLP.c",
    "silk/float/scale_copy_vector_FLP.c",
    "silk/float/scale_vector_FLP.c",
    "silk/float/schur_FLP.c",
    "silk/float/sort_FLP.c",
];

const OPUS_HEADERS: &[&str] = &[
    // OPUS_HEAD
    "include/opus.h",
    "include/opus_multistream.h",
    "include/opus_projection.h",
    "src/opus_private.h",
    "src/analysis.h",
    "src/mapping_matrix.h",
    "src/mlp.h",
    "src/tansig_table.h",
    // CELT_HEAD
    "celt/arch.h",
    "celt/bands.h",
    "celt/celt.h",
    "celt/cpu_support.h",
    "include/opus_types.h",
    "include/opus_defines.h",
    "include/opus_custom.h",
    "celt/cwrs.h",
    "celt/ecintrin.h",
    "celt/entcode.h",
    "celt/entdec.h",
    "celt/entenc.h",
    "celt/fixed_debug.h",
    "celt/fixed_generic.h",
    "celt/float_cast.h",
    "celt/_kiss_fft_guts.h",
    "celt/kiss_fft.h",
    "celt/laplace.h",
    "celt/mathops.h",
    "celt/mdct.h",
    "celt/mfrngcod.h",
    "celt/modes.h",
    "celt/os_support.h",
    "celt/pitch.h",
    "celt/celt_lpc.h",
    "celt/x86/celt_lpc_sse.h",
    "celt/quant_bands.h",
    "celt/rate.h",
    "celt/stack_alloc.h",
    "celt/vq.h",
    "celt/static_modes_float.h",
    "celt/static_modes_fixed.h",
    "celt/static_modes_float_arm_ne10.h",
    "celt/static_modes_fixed_arm_ne10.h",
    // we do not want the architecture-specific intrinsics to be used!
    // "celt/arm/armcpu.h",
    // "celt/arm/fixed_armv4.h",
    // "celt/arm/fixed_armv5e.h",
    // "celt/arm/fixed_arm64.h",
    // "celt/arm/kiss_fft_armv4.h",
    // "celt/arm/kiss_fft_armv5e.h",
    // "celt/arm/pitch_arm.h",
    // "celt/arm/fft_arm.h",
    // "celt/arm/mdct_arm.h",
    // "celt/mips/celt_mipsr1.h",
    // "celt/mips/fixed_generic_mipsr1.h",
    // "celt/mips/kiss_fft_mipsr1.h",
    // "celt/mips/mdct_mipsr1.h",
    // "celt/mips/pitch_mipsr1.h",
    // "celt/mips/vq_mipsr1.h",
    // "celt/x86/pitch_sse.h",
    // "celt/x86/vq_sse.h",
    // "celt/x86/x86cpu.h",
    // SILK_HEAD
    "silk/debug.h",
    "silk/control.h",
    "silk/errors.h",
    "silk/API.h",
    "silk/typedef.h",
    "silk/define.h",
    "silk/main.h",
    // "silk/x86/main_sse.h",
    "silk/PLC.h",
    "silk/structs.h",
    "silk/tables.h",
    "silk/tuning_parameters.h",
    "silk/Inlines.h",
    "silk/MacroCount.h",
    "silk/MacroDebug.h",
    "silk/macros.h",
    "silk/NSQ.h",
    "silk/pitch_est_defines.h",
    "silk/resampler_private.h",
    "silk/resampler_rom.h",
    "silk/resampler_structs.h",
    "silk/SigProc_FIX.h",
    // we do not want the architecture-specific intrinsics to be used!
    // "silk/x86/SigProc_FIX_sse.h",
    // "silk/arm/biquad_alt_arm.h",
    // "silk/arm/LPC_inv_pred_gain_arm.h",
    // "silk/arm/macros_armv4.h",
    // "silk/arm/macros_armv5e.h",
    // "silk/arm/macros_arm64.h",
    // "silk/arm/SigProc_FIX_armv4.h",
    // "silk/arm/SigProc_FIX_armv5e.h",
    // "silk/arm/NSQ_del_dec_arm.h",
    // "silk/arm/NSQ_neon.h",
    // we do not want the fixed-point impls to be used
    // "silk/fixed/main_FIX.h",
    // "silk/fixed/structs_FIX.h",
    // "silk/fixed/arm/warped_autocorrelation_FIX_arm.h",
    // "silk/fixed/mips/noise_shape_analysis_FIX_mipsr1.h",
    // "silk/fixed/mips/warped_autocorrelation_FIX_mipsr1.h",
    "silk/float/main_FLP.h",
    "silk/float/structs_FLP.h",
    "silk/float/SigProc_FLP.h",
    "silk/mips/macros_mipsr1.h",
    "silk/mips/NSQ_del_dec_mipsr1.h",
    "silk/mips/sigproc_fix_mipsr1.h",
];

const OPUS_INCLUDES: &[&str] = &[".", "include", "celt", "silk", "silk/float"];

const CONFIG_H: &str = r##"
/* config.h.  Generated from config.h.in by configure.  */
/* config.h.in.  Generated from configure.ac by autoheader.  */

/* Get CPU Info by asm method */
/* #undef CPU_INFO_BY_ASM */

/* Get CPU Info by c method */
/* #undef CPU_INFO_BY_C */

/* Custom modes */
/* #undef CUSTOM_MODES */

/* Do not build the float API */
/* #undef DISABLE_FLOAT_API */

/* Disable bitstream fixes from RFC 8251 */
/* #undef DISABLE_UPDATE_DRAFT */

/* Assertions */
/* #undef ENABLE_ASSERTIONS */

/* Hardening */
#define ENABLE_HARDENING 1

/* Debug fixed-point implementation */
/* #undef FIXED_DEBUG */

/* Compile as fixed-point (for machines without a fast enough FPU) */
/* #undef FIXED_POINT */

/* Float approximations */
/* #undef FLOAT_APPROX */

/* Fuzzing */
/* #undef FUZZING */

/* Define to 1 if you have the <alloca.h> header file. */
/* #undef HAVE_ALLOCA_H */

/* NE10 library is installed on host. Make sure it is on target! */
/* #undef HAVE_ARM_NE10 */

/* Define to 1 if you have the <dlfcn.h> header file. */
#define HAVE_DLFCN_H 1

/* Define to 1 if you have the <inttypes.h> header file. */
#define HAVE_INTTYPES_H 1

/* Define to 1 if you have the 'lrint' function. */
#define HAVE_LRINT 1

/* Define to 1 if you have the 'lrintf' function. */
#define HAVE_LRINTF 1

/* Define to 1 if you have the <stdint.h> header file. */
#define HAVE_STDINT_H 1

/* Define to 1 if you have the <stdio.h> header file. */
#define HAVE_STDIO_H 1

/* Define to 1 if you have the <stdlib.h> header file. */
#define HAVE_STDLIB_H 1

/* Define to 1 if you have the <strings.h> header file. */
#define HAVE_STRINGS_H 1

/* Define to 1 if you have the <string.h> header file. */
#define HAVE_STRING_H 1

/* Define to 1 if you have the <sys/stat.h> header file. */
#define HAVE_SYS_STAT_H 1

/* Define to 1 if you have the <sys/types.h> header file. */
#define HAVE_SYS_TYPES_H 1

/* Define to 1 if you have the <unistd.h> header file. */
#define HAVE_UNISTD_H 1

/* Define to 1 if you have the '__malloc_hook' function. */
/* #undef HAVE___MALLOC_HOOK */

/* Define to the sub-directory where libtool stores uninstalled libraries. */
#define LT_OBJDIR ".libs/"

/* Make use of ARM asm optimization */
/* #undef OPUS_ARM_ASM */

/* Use generic ARMv4 inline asm optimizations */
/* #undef OPUS_ARM_INLINE_ASM */

/* Use ARMv5E inline asm optimizations */
/* #undef OPUS_ARM_INLINE_EDSP */

/* Use ARMv6 inline asm optimizations */
/* #undef OPUS_ARM_INLINE_MEDIA */

/* Use ARM NEON inline asm optimizations */
/* #undef OPUS_ARM_INLINE_NEON */

/* Define if assembler supports EDSP instructions */
/* #undef OPUS_ARM_MAY_HAVE_EDSP */

/* Define if assembler supports ARMv6 media instructions */
/* #undef OPUS_ARM_MAY_HAVE_MEDIA */

/* Define if compiler supports NEON instructions */
/* #undef OPUS_ARM_MAY_HAVE_NEON */

/* Compiler supports ARMv7/Aarch64 Neon Intrinsics */
/* #undef OPUS_ARM_MAY_HAVE_NEON_INTR */

/* Define if binary requires Aarch64 Neon Intrinsics */
/* #undef OPUS_ARM_PRESUME_AARCH64_NEON_INTR */

/* Define if binary requires EDSP instruction support */
/* #undef OPUS_ARM_PRESUME_EDSP */

/* Define if binary requires ARMv6 media instruction support */
/* #undef OPUS_ARM_PRESUME_MEDIA */

/* Define if binary requires NEON instruction support */
/* #undef OPUS_ARM_PRESUME_NEON */

/* Define if binary requires NEON intrinsics support */
/* #undef OPUS_ARM_PRESUME_NEON_INTR */

/* This is a build of OPUS */
#define OPUS_BUILD /**/

/* Run bit-exactness checks between optimized and c implementations */
/* #undef OPUS_CHECK_ASM */

/* Use run-time CPU capabilities detection */
/* #undef OPUS_HAVE_RTCD */

/* Compiler supports X86 AVX Intrinsics */
/* #undef OPUS_X86_MAY_HAVE_AVX */

/* Compiler supports X86 SSE Intrinsics */
/* #undef OPUS_X86_MAY_HAVE_SSE */

/* Compiler supports X86 SSE2 Intrinsics */
/* #undef OPUS_X86_MAY_HAVE_SSE2 */

/* Compiler supports X86 SSE4.1 Intrinsics */
/* #undef OPUS_X86_MAY_HAVE_SSE4_1 */

/* Define if binary requires AVX intrinsics support */
/* #undef OPUS_X86_PRESUME_AVX */

/* Define if binary requires SSE intrinsics support */
/* #undef OPUS_X86_PRESUME_SSE */

/* Define if binary requires SSE2 intrinsics support */
/* #undef OPUS_X86_PRESUME_SSE2 */

/* Define if binary requires SSE4.1 intrinsics support */
/* #undef OPUS_X86_PRESUME_SSE4_1 */

/* Define to the address where bug reports for this package should be sent. */
#define PACKAGE_BUGREPORT "opus@xiph.org"

/* Define to the full name of this package. */
#define PACKAGE_NAME "opus"

/* Define to the full name and version of this package. */
#define PACKAGE_STRING "opus 1.3.1"

/* Define to the one symbol short name of this package. */
#define PACKAGE_TARNAME "opus"

/* Define to the home page for this package. */
#define PACKAGE_URL ""

/* Define to the version of this package. */
#define PACKAGE_VERSION "1.3.1"

/* Define to 1 if all of the C89 standard headers exist (not just the ones
   required in a freestanding environment). This macro is provided for
   backward compatibility; new code need not use it. */
#define STDC_HEADERS 1

/* Make use of alloca */
/* #undef USE_ALLOCA */

/* Use C99 variable-size arrays */
#define VAR_ARRAYS 1

/* Define to empty if 'const' does not conform to ANSI C. */
/* #undef const */

/* Define to '__inline__' or '__inline' if that's what the C compiler
   calls it, or to nothing if 'inline' is not supported under any name.  */
#ifndef __cplusplus
/* #undef inline */
#endif

/* Define to the equivalent of the C99 'restrict' keyword, or to
   nothing if this is not supported.  Do not define if restrict is
   supported directly.  */
#define restrict __restrict
/* Work around a bug in Sun C++: it does not support _Restrict or
   __restrict__, even though the corresponding Sun C compiler ends up with
   "#define restrict _Restrict" or "#define restrict __restrict__" in the
   previous line.  Perhaps some future version of Sun C++ will work with
   restrict; if so, hopefully it defines __RESTRICT like Sun C does.  */
#if defined __SUNPRO_CC && !defined __RESTRICT
# define _Restrict
# define __restrict__
#endif
"##;

/// Generates a new binding at `src/lib.rs` using `src/wrapper.h`.
#[cfg(feature = "generate_binding")]
fn generate_binding() {
    const ALLOW_UNCONVENTIONALS: &'static str = "#![allow(non_upper_case_globals)]\n\
                                                 #![allow(non_camel_case_types)]\n\
                                                 #![allow(non_snake_case)]\n";

    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .raw_line(ALLOW_UNCONVENTIONALS)
        .generate()
        .expect("Unable to generate binding");

    let binding_target_path = PathBuf::new().join("src").join("lib.rs");

    bindings
        .write_to_file(binding_target_path)
        .expect("Could not write binding to the file at `src/lib.rs`");
}

fn build_opus() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let opus_build_dir = out_dir.join("opus_build");
    let opus_build_src_dir = out_dir.join("opus_src");

    let opus_source_path = Path::new("opus");

    // delete the build directories if they exist
    if opus_build_src_dir.exists() {
        std::fs::remove_dir_all(&opus_build_src_dir).expect("Could not remove build directory");
    }
    if opus_build_dir.exists() {
        std::fs::remove_dir_all(&opus_build_dir).expect("Could not remove build directory");
    }

    // copy the opus sources to the build directory
    for source in OPUS_SOURCES.iter().chain(OPUS_HEADERS) {
        let source_path = opus_source_path.join(source);
        let target_path = opus_build_src_dir.join(source);

        std::fs::create_dir_all(
            target_path
                .parent()
                .expect("Could not get parent directory"),
        )
        .expect("Could not create target directory");
        std::fs::copy(&source_path, &target_path).expect("Could not copy source file");
    }
    // put the config.h in there
    std::fs::write(opus_build_src_dir.join("config.h"), CONFIG_H)
        .expect("Could not write config.h");

    cc::Build::new()
        .std("c11")
        .compiler("clang")
        .includes(
            OPUS_INCLUDES
                .iter()
                .map(|path| opus_build_src_dir.join(path)),
        )
        .files(
            OPUS_SOURCES
                .iter()
                .map(|source| opus_build_src_dir.join(source)),
        )
        .define("HAVE_CONFIG_H", "1")
        // this disables FMA fusion and other trickery that rust doesn't do
        // it's important to get reproducible bitcode on FMA-capable CPUs (like Arm64)
        .flag("-ffp-model=strict")
        .out_dir(&opus_build_dir)
        .compile("opus");

    link_opus(&opus_build_dir);

    // re-run on any changes to the sources
    for source in OPUS_SOURCES.iter().chain(OPUS_HEADERS) {
        println!(
            "cargo:rerun-if-changed={}",
            opus_source_path.join(source).display()
        );
    }
}

fn link_opus(opus_build_dir: &Path) {
    println!("cargo:rustc-link-lib=static=opus");
    println!(
        "cargo:rustc-link-search=native={}",
        opus_build_dir.display()
    );
}

fn main() {
    #[cfg(feature = "generate_binding")]
    generate_binding();

    build_opus();
}
