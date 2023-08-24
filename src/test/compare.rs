use std::f32::consts::PI;

fn read_pcm16(raw_data: &[u8], nchannels: usize) -> (Vec<f32>, usize) {
    let mut samples = Vec::new();

    for chunk in raw_data.chunks_exact(2) {
        let chunk = chunk.try_into().unwrap();
        let sample = i16::from_le_bytes(chunk) as i32;
        let sample = (sample & 0xffff ^ 0x8000) - 0x8000;
        samples.push(sample as f32);
    }

    let len = samples.len();
    assert_eq!(len % nchannels, 0);

    (samples, len / nchannels)
}

fn band_energy(
    mut out: Option<&mut [f32]>,
    ps: &mut [f32],
    bands: &[usize],
    input: &[f32],
    channels: usize,
    frames: usize,
    window_size: usize,
    step: usize,
    downsample: usize,
) {
    let mut window = vec![0f32; window_size];
    let mut c = vec![0f32; window_size];
    let mut s = vec![0f32; window_size];
    let mut x = vec![0f32; channels * window_size];

    let ps_sz = window_size / 2;

    for xj in 0..window_size {
        window[xj] = 0.5 - 0.5 * ((2.0 * PI / (window_size - 1) as f32) * xj as f32).cos();
    }

    for xj in 0..window_size {
        c[xj] = (2.0 * PI / window_size as f32 * xj as f32).cos();
    }

    for xj in 0..window_size {
        s[xj] = (2.0 * PI / window_size as f32 * xj as f32).sin();
    }

    for xi in 0..frames {
        for ci in 0..channels {
            for xk in 0..window_size {
                x[ci * window_size + xk] = window[xk] * input[(xi * step + xk) * channels + ci]
            }
        }

        let mut xj = 0;
        for bi in 0..bands.len() - 1 {
            let mut p: [f32; 2] = [0 as f32, 0.];
            while xj < bands[bi + 1] {
                for ci in 0..channels {
                    let mut ti = 0;
                    let mut im = 0f32;
                    let mut re = 0f32;
                    for xk in 0..window_size {
                        re += c[ti] * x[ci * window_size + xk];
                        im -= s[ti] * x[ci * window_size + xk];
                        ti += xj;
                        if ti >= window_size {
                            ti -= window_size;
                        }
                    }
                    re *= downsample as f32;
                    im *= downsample as f32;
                    ps[(xi * ps_sz + xj) * channels + ci] = re * re + im * im + 100000.0;
                    p[ci] += ps[(xi * ps_sz + xj) * channels + ci];
                }
                xj += 1;
            }

            if let Some(_out) = out.as_mut() {
                _out[(xi * (bands.len() - 1) + bi) * channels] =
                    p[0] / (bands[bi + 1] - bands[bi]) as f32;
                if channels == 2 {
                    _out[(xi * (bands.len() - 1) + bi) * channels + 1] =
                        p[1] / (bands[bi + 1] - bands[bi]) as f32;
                }
            }
        }
    }
}

const NBANDS: usize = 21;
const NFREQS: usize = 240;

/// Bands on which we compute the pseudo-NMR (Bark-derived CELT bands).
static BANDS: [usize; NBANDS + 1] = [
    0, 2, 4, 6, 8, 10, 12, 14, 16, 20, 24, 28, 32, 40, 48, 56, 68, 80, 96, 120, 156, 200,
];

const TEST_WIN_SIZE: usize = 480;
const TEST_WIN_STEP: usize = 120;

#[derive(Debug, Copy, Clone)]
pub struct CompareParams {
    pub sample_rate: usize,
    pub stereo: bool,
}

impl Default for CompareParams {
    fn default() -> Self {
        Self {
            sample_rate: 48000,
            stereo: false,
        }
    }
}

pub struct CompareResult {
    pub error: f64,
    pub quality: f64,
}

pub fn opus_compare(params: CompareParams, true_file: &[u8], tested_file: &[u8]) -> CompareResult {
    let nchannels = if params.stereo { 2 } else { 1 };

    assert!(
        matches!(params.sample_rate, 8000 | 12000 | 16000 | 24000 | 48000),
        "Sampling rate must be 8000, 12000, 16000, 24000 or 48000"
    );

    let downsample = 48000 / params.sample_rate;
    let ybands = match params.sample_rate {
        8000 => 13,
        12000 => 15,
        16000 => 17,
        24000 => 19,
        _ => 21,
    };
    let yfreqs = NFREQS / downsample;

    let (mut x, xlength) = read_pcm16(&true_file, 2);

    if nchannels == 1 {
        for xi in 0..xlength {
            x[xi] = (x[2 * xi] + x[2 * xi + 1]) / 2.0;
        }
    }

    let (y, ylength) = read_pcm16(&tested_file, nchannels);

    assert_eq!(xlength, ylength * downsample, "Sample counts do not match");
    assert!(
        xlength >= TEST_WIN_SIZE,
        "Insufficient sample data ({}<{})",
        xlength,
        TEST_WIN_SIZE
    );

    let nframes = (xlength - TEST_WIN_SIZE + TEST_WIN_STEP) / TEST_WIN_STEP;

    let mut xb = vec![0f32; nframes * NBANDS * nchannels];
    let mut x_bands = vec![0f32; nframes * NFREQS * nchannels];
    let mut y_bands = vec![0f32; nframes * yfreqs * nchannels];
    // Compute the per-band spectral energy of the original signal and the error
    band_energy(
        Some(&mut xb),
        &mut x_bands,
        &BANDS[..NBANDS + 1],
        &x,
        nchannels,
        nframes,
        TEST_WIN_SIZE,
        TEST_WIN_STEP,
        1,
    );
    drop(x);

    band_energy(
        None,
        &mut y_bands,
        &BANDS[..ybands + 1],
        &y,
        nchannels,
        nframes,
        TEST_WIN_SIZE / downsample,
        TEST_WIN_STEP / downsample,
        downsample,
    );
    drop(y);

    for xi in 0..nframes {
        // Frequency masking (low to high): 10 dB/Bark slope
        for bi in 1..NBANDS {
            for ci in 0..nchannels {
                xb[(xi * NBANDS + bi) * nchannels + ci] +=
                    0.1f32 * xb[(xi * NBANDS + bi - 1) * nchannels + ci];
            }
        }

        // Frequency masking (high to low): 15 dB/Bark slope
        for bi in (0..NBANDS - 1).rev() {
            for ci in 0..nchannels {
                xb[(xi * NBANDS + bi) * nchannels + ci] +=
                    0.03f32 * xb[(xi * NBANDS + bi + 1) * nchannels + ci];
            }
        }

        if xi > 0 {
            // Temporal masking: -3 dB/2.5ms slope
            for bi in 0..NBANDS {
                for ci in 0..nchannels {
                    xb[(xi * NBANDS + bi) * nchannels + ci] +=
                        0.5f32 * xb[((xi - 1) * NBANDS + bi) * nchannels + ci]
                }
            }
        }
        // Allowing some cross-talk
        if nchannels == 2 {
            for bi in 0..NBANDS {
                let l = xb[(xi * NBANDS + bi) * nchannels + 0];
                let r = xb[(xi * NBANDS + bi) * nchannels + 1];
                xb[(xi * NBANDS + bi) * nchannels + 0] += 0.01f32 * r;
                xb[(xi * NBANDS + bi) * nchannels + 1] += 0.01f32 * l;
            }
        }

        // Apply masking
        for bi in 0..ybands {
            for xj in BANDS[bi]..BANDS[bi + 1] {
                for ci in 0..nchannels {
                    x_bands[(xi * NFREQS + xj) * nchannels + ci] +=
                        0.1f32 * xb[(xi * NBANDS + bi) * nchannels + ci];
                    y_bands[(xi * yfreqs + xj) * nchannels + ci] +=
                        0.1f32 * xb[(xi * NBANDS + bi) * nchannels + ci];
                }
            }
        }
    }

    // Average of consecutive frames to make comparison slightly less sensitive
    for bi in 0..ybands {
        for xj in BANDS[bi]..BANDS[bi + 1] {
            for ci in 0..nchannels {
                let mut xtmp = x_bands[xj * nchannels + ci];
                let mut ytmp = y_bands[xj * nchannels + ci];
                for xi in 1..nframes {
                    let xtmp2 = x_bands[(xi * NFREQS + xj) * nchannels + ci];
                    let ytmp2 = y_bands[(xi * yfreqs + xj) * nchannels + ci];
                    x_bands[(xi * NFREQS + xj) * nchannels + ci] += xtmp;
                    y_bands[(xi * yfreqs + xj) * nchannels + ci] += ytmp;
                    xtmp = xtmp2;
                    ytmp = ytmp2;
                }
            }
        }
    }

    // If working at a lower sampling rate, don't take into account the last
    //      300 Hz to allow for different transition bands.
    //     For 12 kHz, we don't skip anything, because the last band already skips
    //      400 Hz.
    let max_compare = match params.sample_rate {
        48000 => BANDS[NBANDS],
        12000 => BANDS[ybands],
        _ => BANDS[ybands] - 3,
    };

    let mut error = 0 as f64;
    for xi in 0..nframes {
        let mut err_frame = 0.0f64;
        for bi in 0..ybands {
            let mut err_band = 0.0f64;

            for xj in BANDS[bi]..std::cmp::min(BANDS[bi + 1], max_compare) {
                for ci in 0..nchannels {
                    let re = y_bands[(xi * yfreqs + xj) * nchannels + ci]
                        / x_bands[(xi * NFREQS + xj) * nchannels + ci];
                    let mut im = re - re.ln() - 1.0;

                    // Make comparison less sensitive around the SILK/CELT cross-over to
                    //             allow for mode freedom in the filters
                    if xj >= 79 && xj <= 81 {
                        im *= 0.1f32;
                    }
                    if xj == 80 {
                        im *= 0.1f32;
                    }
                    err_band += im as f64;
                }
            }

            err_band /= ((BANDS[bi + 1] - BANDS[bi]) * nchannels) as f64;
            err_frame += err_band * err_band;
        }

        // Using a fixed normalization value means we're willing to accept slightly
        // lower quality for lower sampling rates.
        err_frame /= NBANDS as f64;
        err_frame *= err_frame;
        error += err_frame * err_frame;
    }

    let error = (error / nframes as f64).powf(1.0 / 16.0);
    let quality = 100.0 * (1.0 - 0.5 * (1.0 + error).ln() / 1.13f64.ln());

    CompareResult { error, quality }
}
