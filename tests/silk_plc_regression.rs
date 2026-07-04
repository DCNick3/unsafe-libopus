//! Regression test for the SILK PLC saturation bug.
//!
//! `silk_SAT16`'s LOWER bound in the C source is `(opus_int16)0x8000`
//! (= -32768); the c2rust expansion in `silk/PLC.rs` rendered it as the
//! bare literal `0x8000` (= +32768 as i32), so every non-positively-
//! saturated sample took the "clamped low" arm and every concealed frame
//! on a SILK/hybrid stream came out as full-scale garbage (RMS ~ 1.0,
//! where the reference libopus conceals at a decaying fraction of the
//! signal level).
//!
//! The test decodes a short SILK-only stream (VOIP application at a
//! bitrate that forces SILK mode), then requests packet-loss concealment
//! (NULL packet) and asserts the concealed audio is a plausible decaying
//! continuation instead of saturated garbage.

use unsafe_libopus::{
    opus_decode_float, opus_decoder_create, opus_decoder_destroy, opus_encode_float,
    opus_encoder_create, opus_encoder_ctl, opus_encoder_destroy, OPUS_APPLICATION_VOIP,
    OPUS_SET_BITRATE_REQUEST,
};

fn rms(samples: &[f32]) -> f32 {
    (samples.iter().map(|s| s * s).sum::<f32>() / samples.len() as f32).sqrt()
}

#[test]
fn silk_plc_conceals_instead_of_saturating() {
    const FRAME: usize = 960; // 20 ms @ 48 kHz mono
    unsafe {
        let mut err = 0i32;
        let enc = opus_encoder_create(48_000, 1, OPUS_APPLICATION_VOIP, &mut err);
        assert!(err == 0 && !enc.is_null(), "encoder create failed: {err}");
        // Force SILK mode via a speech-range bitrate.
        let ret = opus_encoder_ctl!(enc, OPUS_SET_BITRATE_REQUEST, 12000);
        assert_eq!(ret, 0, "OPUS_SET_BITRATE failed: {ret}");

        let dec = opus_decoder_create(48_000, 1, &mut err);
        assert!(err == 0 && !dec.is_null(), "decoder create failed: {err}");

        // 25 frames of a continuous-phase tone at a speech-like level.
        let mut phase = 0f64;
        let mut pkt = vec![0u8; 4000];
        let mut out = vec![0f32; FRAME];
        let mut last_real_rms = 0f32;
        for _ in 0..25 {
            let frame: Vec<f32> = (0..FRAME)
                .map(|_| {
                    phase += 2.0 * std::f64::consts::PI * 440.0 / 48_000.0;
                    (phase.sin() * 0.4) as f32
                })
                .collect();
            let n = opus_encode_float(enc, frame.as_ptr(), FRAME as i32, pkt.as_mut_ptr(), 4000);
            assert!(n > 0, "encode failed: {n}");
            let d = opus_decode_float(&mut *dec, pkt.as_ptr(), n, out.as_mut_ptr(), FRAME as i32, 0);
            assert_eq!(d as usize, FRAME, "decode failed: {d}");
            last_real_rms = rms(&out);
        }
        assert!(
            last_real_rms > 0.05,
            "precondition: the decoded stream must carry energy (got rms {last_real_rms})"
        );

        // Three consecutive PLC calls (NULL packet = concealment).
        let mut plc_rms = [0f32; 3];
        for (step, slot) in plc_rms.iter_mut().enumerate() {
            let d = opus_decode_float(
                &mut *dec,
                core::ptr::null(),
                0,
                out.as_mut_ptr(),
                FRAME as i32,
                0,
            );
            assert_eq!(d as usize, FRAME, "plc decode failed at step {step}: {d}");
            *slot = rms(&out);
        }
        opus_decoder_destroy(dec);
        opus_encoder_destroy(enc);

        // Broken SAT16 lower bound: every concealed frame saturates to
        // ~full scale (rms ~= 1.0 regardless of signal level). Correct
        // concealment continues the signal at (at most) a comparable level
        // and decays. Reference libopus on this stream conceals at
        // rms ~= 0.1-0.2 decaying; 0.6 is far above any legitimate
        // concealment of a 0.4-amplitude tone and far below saturation.
        for (step, &r) in plc_rms.iter().enumerate() {
            assert!(
                r < 0.6,
                "PLC step {step} saturated: rms {r} (expected a decaying \
                 continuation, not full-scale garbage)"
            );
        }
        assert!(
            plc_rms[2] <= plc_rms[0] + 0.05,
            "PLC energy must not grow across consecutive concealed frames: {plc_rms:?}"
        );
    }
}
