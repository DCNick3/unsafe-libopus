#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use libc::{atoi, fclose, fopen, fprintf, fread, FILE};
use libc_stdhandle::stderr;
use unsafe_libopus::externs::{free, malloc, realloc, strcmp};

type size_t = u64;

unsafe fn read_pcm16(
    mut _samples: *mut *mut f32,
    mut _fin: *mut FILE,
    mut _nchannels: i32,
) -> size_t {
    let mut buf: [u8; 1024] = [0; 1024];
    let mut samples: *mut f32 = 0 as *mut f32;
    let mut nsamples: size_t = 0;
    let mut csamples: size_t = 0;
    let mut xi: size_t = 0;
    let mut nread: size_t = 0;
    samples = 0 as *mut f32;
    csamples = 0 as i32 as size_t;
    nsamples = csamples;
    loop {
        nread = fread(
            buf.as_mut_ptr() as *mut core::ffi::c_void,
            (2 as i32 * _nchannels) as _,
            (1024 as i32 / (2 as i32 * _nchannels)) as _,
            _fin,
        ) as _;
        if nread <= 0 as i32 as u64 {
            break;
        }
        if nsamples.wrapping_add(nread) > csamples {
            loop {
                csamples = csamples << 1 as i32 | 1 as i32 as u64;
                if !(nsamples.wrapping_add(nread) > csamples) {
                    break;
                }
            }
            samples = realloc(
                samples as *mut core::ffi::c_void,
                (_nchannels as u64)
                    .wrapping_mul(csamples)
                    .wrapping_mul(::core::mem::size_of::<f32>() as u64),
            ) as *mut f32;
        }
        xi = 0 as i32 as size_t;
        while xi < nread {
            let mut ci: i32 = 0;
            ci = 0 as i32;
            while ci < _nchannels {
                let mut s: i32 = 0;
                s = (buf[(2 as i32 as u64)
                    .wrapping_mul(xi.wrapping_mul(_nchannels as u64).wrapping_add(ci as u64))
                    .wrapping_add(1 as i32 as u64) as usize] as i32)
                    << 8 as i32
                    | buf[(2 as i32 as u64)
                        .wrapping_mul(xi.wrapping_mul(_nchannels as u64).wrapping_add(ci as u64))
                        as usize] as i32;
                s = (s & 0xffff as i32 ^ 0x8000 as i32) - 0x8000 as i32;
                *samples.offset(
                    nsamples
                        .wrapping_add(xi)
                        .wrapping_mul(_nchannels as u64)
                        .wrapping_add(ci as u64) as isize,
                ) = s as f32;
                ci += 1;
            }
            xi = xi.wrapping_add(1);
        }
        nsamples = (nsamples as u64).wrapping_add(nread) as size_t as size_t;
    }
    *_samples = realloc(
        samples as *mut core::ffi::c_void,
        (_nchannels as u64)
            .wrapping_mul(nsamples)
            .wrapping_mul(::core::mem::size_of::<f32>() as u64),
    ) as *mut f32;
    return nsamples;
}
unsafe fn band_energy(
    mut _out: *mut f32,
    mut _ps: *mut f32,
    mut _bands: *const i32,
    mut _nbands: i32,
    mut _in: *const f32,
    mut _nchannels: i32,
    mut _nframes: size_t,
    mut _window_sz: i32,
    mut _step: i32,
    mut _downsample: i32,
) {
    let mut window: *mut f32 = 0 as *mut f32;
    let mut x: *mut f32 = 0 as *mut f32;
    let mut c: *mut f32 = 0 as *mut f32;
    let mut s: *mut f32 = 0 as *mut f32;
    let mut xi: size_t = 0;
    let mut xj: i32 = 0;
    let mut ps_sz: i32 = 0;
    window = malloc(
        (((3 as i32 + _nchannels) * _window_sz) as u64)
            .wrapping_mul(::core::mem::size_of::<f32>() as u64),
    ) as *mut f32;
    c = window.offset(_window_sz as isize);
    s = c.offset(_window_sz as isize);
    x = s.offset(_window_sz as isize);
    ps_sz = _window_sz / 2 as i32;
    xj = 0 as i32;
    while xj < _window_sz {
        *window.offset(xj as isize) = 0.5f32
            - 0.5f32 * (2.0 * std::f32::consts::PI / (_window_sz - 1) as f32 * xj as f32).cos();
        xj += 1;
    }
    xj = 0 as i32;
    while xj < _window_sz {
        *c.offset(xj as isize) = (2.0 * std::f32::consts::PI / _window_sz as f32 * xj as f32).cos();
        xj += 1;
    }
    xj = 0 as i32;
    while xj < _window_sz {
        *s.offset(xj as isize) = (2.0 * std::f32::consts::PI / _window_sz as f32 * xj as f32).sin();
        xj += 1;
    }
    xi = 0 as i32 as size_t;
    while xi < _nframes {
        let mut ci: i32 = 0;
        let mut xk: i32 = 0;
        let mut bi: i32 = 0;
        ci = 0 as i32;
        while ci < _nchannels {
            xk = 0 as i32;
            while xk < _window_sz {
                *x.offset((ci * _window_sz + xk) as isize) = *window.offset(xk as isize)
                    * *_in.offset(
                        xi.wrapping_mul(_step as u64)
                            .wrapping_add(xk as u64)
                            .wrapping_mul(_nchannels as u64)
                            .wrapping_add(ci as u64) as isize,
                    );
                xk += 1;
            }
            ci += 1;
        }
        xj = 0 as i32;
        bi = xj;
        while bi < _nbands {
            let mut p: [f32; 2] = [0 as i32 as f32, 0.];
            while xj < *_bands.offset((bi + 1 as i32) as isize) {
                ci = 0 as i32;
                while ci < _nchannels {
                    let mut re: f32 = 0.;
                    let mut im: f32 = 0.;
                    let mut ti: i32 = 0;
                    ti = 0 as i32;
                    im = 0 as i32 as f32;
                    re = im;
                    xk = 0 as i32;
                    while xk < _window_sz {
                        re += *c.offset(ti as isize) * *x.offset((ci * _window_sz + xk) as isize);
                        im -= *s.offset(ti as isize) * *x.offset((ci * _window_sz + xk) as isize);
                        ti += xj;
                        if ti >= _window_sz {
                            ti -= _window_sz;
                        }
                        xk += 1;
                    }
                    re *= _downsample as f32;
                    im *= _downsample as f32;
                    *_ps.offset(
                        xi.wrapping_mul(ps_sz as u64)
                            .wrapping_add(xj as u64)
                            .wrapping_mul(_nchannels as u64)
                            .wrapping_add(ci as u64) as isize,
                    ) = re * re + im * im + 100000 as i32 as f32;
                    p[ci as usize] += *_ps.offset(
                        xi.wrapping_mul(ps_sz as u64)
                            .wrapping_add(xj as u64)
                            .wrapping_mul(_nchannels as u64)
                            .wrapping_add(ci as u64) as isize,
                    );
                    ci += 1;
                }
                xj += 1;
            }
            if !_out.is_null() {
                *_out.offset(
                    xi.wrapping_mul(_nbands as u64)
                        .wrapping_add(bi as u64)
                        .wrapping_mul(_nchannels as u64) as isize,
                ) = p[0 as i32 as usize]
                    / (*_bands.offset((bi + 1 as i32) as isize) - *_bands.offset(bi as isize))
                        as f32;
                if _nchannels == 2 as i32 {
                    *_out.offset(
                        xi.wrapping_mul(_nbands as u64)
                            .wrapping_add(bi as u64)
                            .wrapping_mul(_nchannels as u64)
                            .wrapping_add(1 as i32 as u64) as isize,
                    ) = p[1 as i32 as usize]
                        / (*_bands.offset((bi + 1 as i32) as isize) - *_bands.offset(bi as isize))
                            as f32;
                }
            }
            bi += 1;
        }
        xi = xi.wrapping_add(1);
    }
    free(window as *mut core::ffi::c_void);
}
static mut BANDS: [i32; 22] = [
    0 as i32, 2 as i32, 4 as i32, 6 as i32, 8 as i32, 10 as i32, 12 as i32, 14 as i32, 16 as i32,
    20 as i32, 24 as i32, 28 as i32, 32 as i32, 40 as i32, 48 as i32, 56 as i32, 68 as i32,
    80 as i32, 96 as i32, 120 as i32, 156 as i32, 200 as i32,
];
unsafe fn main_0(mut _argc: i32, mut _argv: *mut *const i8) -> i32 {
    let mut fin1: *mut FILE = 0 as *mut FILE;
    let mut fin2: *mut FILE = 0 as *mut FILE;
    let mut x: *mut f32 = 0 as *mut f32;
    let mut y: *mut f32 = 0 as *mut f32;
    let mut xb: *mut f32 = 0 as *mut f32;
    let mut X: *mut f32 = 0 as *mut f32;
    let mut Y: *mut f32 = 0 as *mut f32;
    let mut err: f64 = 0.;
    let mut Q: f32 = 0.;
    let mut xlength: size_t = 0;
    let mut ylength: size_t = 0;
    let mut nframes: size_t = 0;
    let mut xi: size_t = 0;
    let mut ci: i32 = 0;
    let mut xj: i32 = 0;
    let mut bi: i32 = 0;
    let mut nchannels: i32 = 0;
    let mut rate: u32 = 0;
    let mut downsample: i32 = 0;
    let mut ybands: i32 = 0;
    let mut yfreqs: i32 = 0;
    let mut max_compare: i32 = 0;
    if _argc < 3 as i32 || _argc > 6 as i32 {
        fprintf(
            stderr(),
            b"Usage: %s [-s] [-r rate2] <file1.sw> <file2.sw>\n\0" as *const u8 as *const i8,
            *_argv.offset(0 as i32 as isize),
        );
        return 1 as i32;
    }
    nchannels = 1 as i32;
    if strcmp(
        *_argv.offset(1 as i32 as isize),
        b"-s\0" as *const u8 as *const i8,
    ) == 0 as i32
    {
        nchannels = 2 as i32;
        _argv = _argv.offset(1);
    }
    rate = 48000 as i32 as u32;
    ybands = 21 as i32;
    yfreqs = 240 as i32;
    downsample = 1 as i32;
    if strcmp(
        *_argv.offset(1 as i32 as isize),
        b"-r\0" as *const u8 as *const i8,
    ) == 0 as i32
    {
        rate = atoi(*_argv.offset(2 as i32 as isize)) as u32;
        if rate != 8000 as i32 as u32
            && rate != 12000 as i32 as u32
            && rate != 16000 as i32 as u32
            && rate != 24000 as i32 as u32
            && rate != 48000 as i32 as u32
        {
            fprintf(
                stderr(),
                b"Sampling rate must be 8000, 12000, 16000, 24000, or 48000\n\0" as *const u8
                    as *const i8,
            );
            return 1 as i32;
        }
        downsample = (48000 as i32 as u32).wrapping_div(rate) as i32;
        match rate {
            8000 => {
                ybands = 13 as i32;
            }
            12000 => {
                ybands = 15 as i32;
            }
            16000 => {
                ybands = 17 as i32;
            }
            24000 => {
                ybands = 19 as i32;
            }
            _ => {}
        }
        yfreqs = 240 as i32 / downsample;
        _argv = _argv.offset(2 as i32 as isize);
    }
    fin1 = fopen(
        *_argv.offset(1 as i32 as isize),
        b"rb\0" as *const u8 as *const i8,
    );
    if fin1.is_null() {
        fprintf(
            stderr(),
            b"Error opening '%s'.\n\0" as *const u8 as *const i8,
            *_argv.offset(1 as i32 as isize),
        );
        return 1 as i32;
    }
    fin2 = fopen(
        *_argv.offset(2 as i32 as isize),
        b"rb\0" as *const u8 as *const i8,
    );
    if fin2.is_null() {
        fprintf(
            stderr(),
            b"Error opening '%s'.\n\0" as *const u8 as *const i8,
            *_argv.offset(2 as i32 as isize),
        );
        fclose(fin1);
        return 1 as i32;
    }
    xlength = read_pcm16(&mut x, fin1, 2 as i32);
    if nchannels == 1 as i32 {
        xi = 0 as i32 as size_t;
        while xi < xlength {
            *x.offset(xi as isize) = (0.5f64
                * (*x.offset((2 as i32 as u64).wrapping_mul(xi) as isize)
                    + *x.offset(
                        (2 as i32 as u64)
                            .wrapping_mul(xi)
                            .wrapping_add(1 as i32 as u64) as isize,
                    )) as f64) as f32;
            xi = xi.wrapping_add(1);
        }
    }
    fclose(fin1);
    ylength = read_pcm16(&mut y, fin2, nchannels);
    fclose(fin2);
    if xlength != ylength.wrapping_mul(downsample as u64) {
        fprintf(
            stderr(),
            b"Sample counts do not match (%lu!=%lu).\n\0" as *const u8 as *const i8,
            xlength,
            ylength.wrapping_mul(downsample as u64),
        );
        return 1 as i32;
    }
    if xlength < 480 as i32 as u64 {
        fprintf(
            stderr(),
            b"Insufficient sample data (%lu<%i).\n\0" as *const u8 as *const i8,
            xlength,
            480 as i32,
        );
        return 1 as i32;
    }
    nframes = xlength
        .wrapping_sub(480 as i32 as u64)
        .wrapping_add(120 as i32 as u64)
        .wrapping_div(120 as i32 as u64);
    xb = malloc(
        nframes
            .wrapping_mul(21 as i32 as u64)
            .wrapping_mul(nchannels as u64)
            .wrapping_mul(::core::mem::size_of::<f32>() as u64),
    ) as *mut f32;
    X = malloc(
        nframes
            .wrapping_mul(240 as i32 as u64)
            .wrapping_mul(nchannels as u64)
            .wrapping_mul(::core::mem::size_of::<f32>() as u64),
    ) as *mut f32;
    Y = malloc(
        nframes
            .wrapping_mul(yfreqs as u64)
            .wrapping_mul(nchannels as u64)
            .wrapping_mul(::core::mem::size_of::<f32>() as u64),
    ) as *mut f32;
    band_energy(
        xb,
        X,
        BANDS.as_ptr(),
        21 as i32,
        x,
        nchannels,
        nframes,
        480 as i32,
        120 as i32,
        1 as i32,
    );
    free(x as *mut core::ffi::c_void);
    band_energy(
        0 as *mut f32,
        Y,
        BANDS.as_ptr(),
        ybands,
        y,
        nchannels,
        nframes,
        480 as i32 / downsample,
        120 as i32 / downsample,
        downsample,
    );
    free(y as *mut core::ffi::c_void);
    xi = 0 as i32 as size_t;
    while xi < nframes {
        bi = 1 as i32;
        while bi < 21 as i32 {
            ci = 0 as i32;
            while ci < nchannels {
                *xb.offset(
                    xi.wrapping_mul(21 as i32 as u64)
                        .wrapping_add(bi as u64)
                        .wrapping_mul(nchannels as u64)
                        .wrapping_add(ci as u64) as isize,
                ) += 0.1f32
                    * *xb.offset(
                        xi.wrapping_mul(21 as i32 as u64)
                            .wrapping_add(bi as u64)
                            .wrapping_sub(1 as i32 as u64)
                            .wrapping_mul(nchannels as u64)
                            .wrapping_add(ci as u64) as isize,
                    );
                ci += 1;
            }
            bi += 1;
        }
        bi = 21 as i32 - 1 as i32;
        loop {
            let fresh0 = bi;
            bi = bi - 1;
            if !(fresh0 > 0 as i32) {
                break;
            }
            ci = 0 as i32;
            while ci < nchannels {
                *xb.offset(
                    xi.wrapping_mul(21 as i32 as u64)
                        .wrapping_add(bi as u64)
                        .wrapping_mul(nchannels as u64)
                        .wrapping_add(ci as u64) as isize,
                ) += 0.03f32
                    * *xb.offset(
                        xi.wrapping_mul(21 as i32 as u64)
                            .wrapping_add(bi as u64)
                            .wrapping_add(1 as i32 as u64)
                            .wrapping_mul(nchannels as u64)
                            .wrapping_add(ci as u64) as isize,
                    );
                ci += 1;
            }
        }
        if xi > 0 as i32 as u64 {
            bi = 0 as i32;
            while bi < 21 as i32 {
                ci = 0 as i32;
                while ci < nchannels {
                    *xb.offset(
                        xi.wrapping_mul(21 as i32 as u64)
                            .wrapping_add(bi as u64)
                            .wrapping_mul(nchannels as u64)
                            .wrapping_add(ci as u64) as isize,
                    ) += 0.5f32
                        * *xb.offset(
                            xi.wrapping_sub(1 as i32 as u64)
                                .wrapping_mul(21 as i32 as u64)
                                .wrapping_add(bi as u64)
                                .wrapping_mul(nchannels as u64)
                                .wrapping_add(ci as u64) as isize,
                        );
                    ci += 1;
                }
                bi += 1;
            }
        }
        if nchannels == 2 as i32 {
            bi = 0 as i32;
            while bi < 21 as i32 {
                let mut l: f32 = 0.;
                let mut r: f32 = 0.;
                l = *xb.offset(
                    xi.wrapping_mul(21 as i32 as u64)
                        .wrapping_add(bi as u64)
                        .wrapping_mul(nchannels as u64)
                        .wrapping_add(0 as i32 as u64) as isize,
                );
                r = *xb.offset(
                    xi.wrapping_mul(21 as i32 as u64)
                        .wrapping_add(bi as u64)
                        .wrapping_mul(nchannels as u64)
                        .wrapping_add(1 as i32 as u64) as isize,
                );
                *xb.offset(
                    xi.wrapping_mul(21 as i32 as u64)
                        .wrapping_add(bi as u64)
                        .wrapping_mul(nchannels as u64)
                        .wrapping_add(0 as i32 as u64) as isize,
                ) += 0.01f32 * r;
                *xb.offset(
                    xi.wrapping_mul(21 as i32 as u64)
                        .wrapping_add(bi as u64)
                        .wrapping_mul(nchannels as u64)
                        .wrapping_add(1 as i32 as u64) as isize,
                ) += 0.01f32 * l;
                bi += 1;
            }
        }
        bi = 0 as i32;
        while bi < ybands {
            xj = BANDS[bi as usize];
            while xj < BANDS[(bi + 1 as i32) as usize] {
                ci = 0 as i32;
                while ci < nchannels {
                    *X.offset(
                        xi.wrapping_mul(240 as i32 as u64)
                            .wrapping_add(xj as u64)
                            .wrapping_mul(nchannels as u64)
                            .wrapping_add(ci as u64) as isize,
                    ) += 0.1f32
                        * *xb.offset(
                            xi.wrapping_mul(21 as i32 as u64)
                                .wrapping_add(bi as u64)
                                .wrapping_mul(nchannels as u64)
                                .wrapping_add(ci as u64) as isize,
                        );
                    *Y.offset(
                        xi.wrapping_mul(yfreqs as u64)
                            .wrapping_add(xj as u64)
                            .wrapping_mul(nchannels as u64)
                            .wrapping_add(ci as u64) as isize,
                    ) += 0.1f32
                        * *xb.offset(
                            xi.wrapping_mul(21 as i32 as u64)
                                .wrapping_add(bi as u64)
                                .wrapping_mul(nchannels as u64)
                                .wrapping_add(ci as u64) as isize,
                        );
                    ci += 1;
                }
                xj += 1;
            }
            bi += 1;
        }
        xi = xi.wrapping_add(1);
    }
    bi = 0 as i32;
    while bi < ybands {
        xj = BANDS[bi as usize];
        while xj < BANDS[(bi + 1 as i32) as usize] {
            ci = 0 as i32;
            while ci < nchannels {
                let mut xtmp: f32 = 0.;
                let mut ytmp: f32 = 0.;
                xtmp = *X.offset((xj * nchannels + ci) as isize);
                ytmp = *Y.offset((xj * nchannels + ci) as isize);
                xi = 1 as i32 as size_t;
                while xi < nframes {
                    let mut xtmp2: f32 = 0.;
                    let mut ytmp2: f32 = 0.;
                    xtmp2 = *X.offset(
                        xi.wrapping_mul(240 as i32 as u64)
                            .wrapping_add(xj as u64)
                            .wrapping_mul(nchannels as u64)
                            .wrapping_add(ci as u64) as isize,
                    );
                    ytmp2 = *Y.offset(
                        xi.wrapping_mul(yfreqs as u64)
                            .wrapping_add(xj as u64)
                            .wrapping_mul(nchannels as u64)
                            .wrapping_add(ci as u64) as isize,
                    );
                    *X.offset(
                        xi.wrapping_mul(240 as i32 as u64)
                            .wrapping_add(xj as u64)
                            .wrapping_mul(nchannels as u64)
                            .wrapping_add(ci as u64) as isize,
                    ) += xtmp;
                    *Y.offset(
                        xi.wrapping_mul(yfreqs as u64)
                            .wrapping_add(xj as u64)
                            .wrapping_mul(nchannels as u64)
                            .wrapping_add(ci as u64) as isize,
                    ) += ytmp;
                    xtmp = xtmp2;
                    ytmp = ytmp2;
                    xi = xi.wrapping_add(1);
                }
                ci += 1;
            }
            xj += 1;
        }
        bi += 1;
    }
    if rate == 48000 as i32 as u32 {
        max_compare = BANDS[21 as i32 as usize];
    } else if rate == 12000 as i32 as u32 {
        max_compare = BANDS[ybands as usize];
    } else {
        max_compare = BANDS[ybands as usize] - 3 as i32;
    }
    err = 0 as i32 as f64;
    xi = 0 as i32 as size_t;
    while xi < nframes {
        let mut Ef: f64 = 0.;
        Ef = 0 as i32 as f64;
        bi = 0 as i32;
        while bi < ybands {
            let mut Eb: f64 = 0.;
            Eb = 0 as i32 as f64;
            xj = BANDS[bi as usize];
            while xj < BANDS[(bi + 1 as i32) as usize] && xj < max_compare {
                ci = 0 as i32;
                while ci < nchannels {
                    let mut re: f32 = 0.;
                    let mut im: f32 = 0.;
                    re = *Y.offset(
                        xi.wrapping_mul(yfreqs as u64)
                            .wrapping_add(xj as u64)
                            .wrapping_mul(nchannels as u64)
                            .wrapping_add(ci as u64) as isize,
                    ) / *X.offset(
                        xi.wrapping_mul(240 as i32 as u64)
                            .wrapping_add(xj as u64)
                            .wrapping_mul(nchannels as u64)
                            .wrapping_add(ci as u64) as isize,
                    );
                    im = re - re.ln() - 1.0;
                    if xj >= 79 as i32 && xj <= 81 as i32 {
                        im *= 0.1f32;
                    }
                    if xj == 80 as i32 {
                        im *= 0.1f32;
                    }
                    Eb += im as f64;
                    ci += 1;
                }
                xj += 1;
            }
            Eb /= ((BANDS[(bi + 1 as i32) as usize] - BANDS[bi as usize]) * nchannels) as f64;
            Ef += Eb * Eb;
            bi += 1;
        }
        Ef /= 21 as i32 as f64;
        Ef *= Ef;
        err += Ef * Ef;
        xi = xi.wrapping_add(1);
    }
    free(xb as *mut core::ffi::c_void);
    free(X as *mut core::ffi::c_void);
    free(Y as *mut core::ffi::c_void);
    err = (err / nframes as f64).powf(1.0 / 16.0);
    Q = (100.0 * (1.0 - 0.5 * (1.0 + err).ln() / 1.13f64.ln())) as f32;
    if Q < 0 as i32 as f32 {
        fprintf(stderr(), b"Test vector FAILS\n\0" as *const u8 as *const i8);
        fprintf(
            stderr(),
            b"Internal weighted error is %f\n\0" as *const u8 as *const i8,
            err,
        );
        return 1 as i32;
    } else {
        fprintf(
            stderr(),
            b"Test vector PASSES\n\0" as *const u8 as *const i8,
        );
        fprintf(
            stderr(),
            b"Opus quality metric: %.1f %% (internal weighted error is %f)\n\0" as *const u8
                as *const i8,
            Q as f64,
            err,
        );
        return 0 as i32;
    };
}
pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *const i8) as i32,
        )
    }
}
