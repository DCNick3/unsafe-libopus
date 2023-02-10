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

use ::libc;
use libc::{fclose, fopen, fprintf, fread, FILE};
use libc_stdhandle::stderr;
use libopus_unsafe::externs::{free, malloc, realloc, strcmp};

#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:28"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/stdlib.h:29"]
pub mod stdlib_h {
    #[inline]
    #[c2rust::src_loc = "361:1"]
    pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
        return strtol(
            __nptr,
            0 as *mut libc::c_void as *mut *mut libc::c_char,
            10 as libc::c_int,
        ) as libc::c_int;
    }
    extern "C" {
        #[c2rust::src_loc = "177:17"]
        pub fn strtol(
            _: *const libc::c_char,
            _: *mut *mut libc::c_char,
            _: libc::c_int,
        ) -> libc::c_long;
        #[c2rust::src_loc = "637:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
pub use self::stddef_h::size_t;
pub use self::stdlib_h::{atoi, exit};

#[c2rust::src_loc = "38:1"]
unsafe extern "C" fn check_alloc(mut _ptr: *mut libc::c_void) -> *mut libc::c_void {
    if _ptr.is_null() {
        fprintf(
            stderr(),
            b"Out of memory.\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    return _ptr;
}
#[c2rust::src_loc = "46:1"]
unsafe extern "C" fn opus_malloc(mut _size: size_t) -> *mut libc::c_void {
    return check_alloc(malloc(_size));
}
#[c2rust::src_loc = "50:1"]
unsafe extern "C" fn opus_realloc(
    mut _ptr: *mut libc::c_void,
    mut _size: size_t,
) -> *mut libc::c_void {
    return check_alloc(realloc(_ptr, _size));
}
#[c2rust::src_loc = "54:1"]
unsafe extern "C" fn read_pcm16(
    mut _samples: *mut *mut libc::c_float,
    mut _fin: *mut FILE,
    mut _nchannels: libc::c_int,
) -> size_t {
    let mut buf: [libc::c_uchar; 1024] = [0; 1024];
    let mut samples: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut nsamples: size_t = 0;
    let mut csamples: size_t = 0;
    let mut xi: size_t = 0;
    let mut nread: size_t = 0;
    samples = 0 as *mut libc::c_float;
    csamples = 0 as libc::c_int as size_t;
    nsamples = csamples;
    loop {
        nread = fread(
            buf.as_mut_ptr() as *mut libc::c_void,
            (2 as libc::c_int * _nchannels) as _,
            (1024 as libc::c_int / (2 as libc::c_int * _nchannels)) as _,
            _fin,
        ) as _;
        if nread <= 0 as libc::c_int as libc::c_ulong {
            break;
        }
        if nsamples.wrapping_add(nread) > csamples {
            loop {
                csamples = csamples << 1 as libc::c_int | 1 as libc::c_int as libc::c_ulong;
                if !(nsamples.wrapping_add(nread) > csamples) {
                    break;
                }
            }
            samples = opus_realloc(
                samples as *mut libc::c_void,
                (_nchannels as libc::c_ulong)
                    .wrapping_mul(csamples)
                    .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong),
            ) as *mut libc::c_float;
        }
        xi = 0 as libc::c_int as size_t;
        while xi < nread {
            let mut ci: libc::c_int = 0;
            ci = 0 as libc::c_int;
            while ci < _nchannels {
                let mut s: libc::c_int = 0;
                s = (buf[(2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        xi.wrapping_mul(_nchannels as libc::c_ulong)
                            .wrapping_add(ci as libc::c_ulong),
                    )
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    as usize] as libc::c_int)
                    << 8 as libc::c_int
                    | buf[(2 as libc::c_int as libc::c_ulong).wrapping_mul(
                        xi.wrapping_mul(_nchannels as libc::c_ulong)
                            .wrapping_add(ci as libc::c_ulong),
                    ) as usize] as libc::c_int;
                s = (s & 0xffff as libc::c_int ^ 0x8000 as libc::c_int) - 0x8000 as libc::c_int;
                *samples.offset(
                    nsamples
                        .wrapping_add(xi)
                        .wrapping_mul(_nchannels as libc::c_ulong)
                        .wrapping_add(ci as libc::c_ulong) as isize,
                ) = s as libc::c_float;
                ci += 1;
            }
            xi = xi.wrapping_add(1);
        }
        nsamples = (nsamples as libc::c_ulong).wrapping_add(nread) as size_t as size_t;
    }
    *_samples = opus_realloc(
        samples as *mut libc::c_void,
        (_nchannels as libc::c_ulong)
            .wrapping_mul(nsamples)
            .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong),
    ) as *mut libc::c_float;
    return nsamples;
}
#[c2rust::src_loc = "88:1"]
unsafe extern "C" fn band_energy(
    mut _out: *mut libc::c_float,
    mut _ps: *mut libc::c_float,
    mut _bands: *const libc::c_int,
    mut _nbands: libc::c_int,
    mut _in: *const libc::c_float,
    mut _nchannels: libc::c_int,
    mut _nframes: size_t,
    mut _window_sz: libc::c_int,
    mut _step: libc::c_int,
    mut _downsample: libc::c_int,
) {
    let mut window: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut x: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut c: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut s: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut xi: size_t = 0;
    let mut xj: libc::c_int = 0;
    let mut ps_sz: libc::c_int = 0;
    window = opus_malloc(
        (((3 as libc::c_int + _nchannels) * _window_sz) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong),
    ) as *mut libc::c_float;
    c = window.offset(_window_sz as isize);
    s = c.offset(_window_sz as isize);
    x = s.offset(_window_sz as isize);
    ps_sz = _window_sz / 2 as libc::c_int;
    xj = 0 as libc::c_int;
    while xj < _window_sz {
        *window.offset(xj as isize) = 0.5f32
            - 0.5f32 * (2.0 * std::f32::consts::PI / (_window_sz - 1) as f32 * xj as f32).cos();
        xj += 1;
    }
    xj = 0 as libc::c_int;
    while xj < _window_sz {
        *c.offset(xj as isize) = (2.0 * std::f32::consts::PI / _window_sz as f32 * xj as f32).cos();
        xj += 1;
    }
    xj = 0 as libc::c_int;
    while xj < _window_sz {
        *s.offset(xj as isize) = (2.0 * std::f32::consts::PI / _window_sz as f32 * xj as f32).sin();
        xj += 1;
    }
    xi = 0 as libc::c_int as size_t;
    while xi < _nframes {
        let mut ci: libc::c_int = 0;
        let mut xk: libc::c_int = 0;
        let mut bi: libc::c_int = 0;
        ci = 0 as libc::c_int;
        while ci < _nchannels {
            xk = 0 as libc::c_int;
            while xk < _window_sz {
                *x.offset((ci * _window_sz + xk) as isize) = *window.offset(xk as isize)
                    * *_in.offset(
                        xi.wrapping_mul(_step as libc::c_ulong)
                            .wrapping_add(xk as libc::c_ulong)
                            .wrapping_mul(_nchannels as libc::c_ulong)
                            .wrapping_add(ci as libc::c_ulong) as isize,
                    );
                xk += 1;
            }
            ci += 1;
        }
        xj = 0 as libc::c_int;
        bi = xj;
        while bi < _nbands {
            let mut p: [libc::c_float; 2] = [0 as libc::c_int as libc::c_float, 0.];
            while xj < *_bands.offset((bi + 1 as libc::c_int) as isize) {
                ci = 0 as libc::c_int;
                while ci < _nchannels {
                    let mut re: libc::c_float = 0.;
                    let mut im: libc::c_float = 0.;
                    let mut ti: libc::c_int = 0;
                    ti = 0 as libc::c_int;
                    im = 0 as libc::c_int as libc::c_float;
                    re = im;
                    xk = 0 as libc::c_int;
                    while xk < _window_sz {
                        re += *c.offset(ti as isize) * *x.offset((ci * _window_sz + xk) as isize);
                        im -= *s.offset(ti as isize) * *x.offset((ci * _window_sz + xk) as isize);
                        ti += xj;
                        if ti >= _window_sz {
                            ti -= _window_sz;
                        }
                        xk += 1;
                    }
                    re *= _downsample as libc::c_float;
                    im *= _downsample as libc::c_float;
                    *_ps.offset(
                        xi.wrapping_mul(ps_sz as libc::c_ulong)
                            .wrapping_add(xj as libc::c_ulong)
                            .wrapping_mul(_nchannels as libc::c_ulong)
                            .wrapping_add(ci as libc::c_ulong) as isize,
                    ) = re * re + im * im + 100000 as libc::c_int as libc::c_float;
                    p[ci as usize] += *_ps.offset(
                        xi.wrapping_mul(ps_sz as libc::c_ulong)
                            .wrapping_add(xj as libc::c_ulong)
                            .wrapping_mul(_nchannels as libc::c_ulong)
                            .wrapping_add(ci as libc::c_ulong) as isize,
                    );
                    ci += 1;
                }
                xj += 1;
            }
            if !_out.is_null() {
                *_out.offset(
                    xi.wrapping_mul(_nbands as libc::c_ulong)
                        .wrapping_add(bi as libc::c_ulong)
                        .wrapping_mul(_nchannels as libc::c_ulong) as isize,
                ) = p[0 as libc::c_int as usize]
                    / (*_bands.offset((bi + 1 as libc::c_int) as isize)
                        - *_bands.offset(bi as isize)) as libc::c_float;
                if _nchannels == 2 as libc::c_int {
                    *_out.offset(
                        xi.wrapping_mul(_nbands as libc::c_ulong)
                            .wrapping_add(bi as libc::c_ulong)
                            .wrapping_mul(_nchannels as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) = p[1 as libc::c_int as usize]
                        / (*_bands.offset((bi + 1 as libc::c_int) as isize)
                            - *_bands.offset(bi as isize))
                            as libc::c_float;
                }
            }
            bi += 1;
        }
        xi = xi.wrapping_add(1);
    }
    free(window as *mut libc::c_void);
}
#[c2rust::src_loc = "158:18"]
static mut BANDS: [libc::c_int; 22] = [
    0 as libc::c_int,
    2 as libc::c_int,
    4 as libc::c_int,
    6 as libc::c_int,
    8 as libc::c_int,
    10 as libc::c_int,
    12 as libc::c_int,
    14 as libc::c_int,
    16 as libc::c_int,
    20 as libc::c_int,
    24 as libc::c_int,
    28 as libc::c_int,
    32 as libc::c_int,
    40 as libc::c_int,
    48 as libc::c_int,
    56 as libc::c_int,
    68 as libc::c_int,
    80 as libc::c_int,
    96 as libc::c_int,
    120 as libc::c_int,
    156 as libc::c_int,
    200 as libc::c_int,
];
#[c2rust::src_loc = "165:1"]
unsafe fn main_0(mut _argc: libc::c_int, mut _argv: *mut *const libc::c_char) -> libc::c_int {
    let mut fin1: *mut FILE = 0 as *mut FILE;
    let mut fin2: *mut FILE = 0 as *mut FILE;
    let mut x: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut y: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut xb: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut X: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut Y: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut err: libc::c_double = 0.;
    let mut Q: libc::c_float = 0.;
    let mut xlength: size_t = 0;
    let mut ylength: size_t = 0;
    let mut nframes: size_t = 0;
    let mut xi: size_t = 0;
    let mut ci: libc::c_int = 0;
    let mut xj: libc::c_int = 0;
    let mut bi: libc::c_int = 0;
    let mut nchannels: libc::c_int = 0;
    let mut rate: libc::c_uint = 0;
    let mut downsample: libc::c_int = 0;
    let mut ybands: libc::c_int = 0;
    let mut yfreqs: libc::c_int = 0;
    let mut max_compare: libc::c_int = 0;
    if _argc < 3 as libc::c_int || _argc > 6 as libc::c_int {
        fprintf(
            stderr(),
            b"Usage: %s [-s] [-r rate2] <file1.sw> <file2.sw>\n\0" as *const u8
                as *const libc::c_char,
            *_argv.offset(0 as libc::c_int as isize),
        );
        return 1 as libc::c_int;
    }
    nchannels = 1 as libc::c_int;
    if strcmp(
        *_argv.offset(1 as libc::c_int as isize),
        b"-s\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        nchannels = 2 as libc::c_int;
        _argv = _argv.offset(1);
    }
    rate = 48000 as libc::c_int as libc::c_uint;
    ybands = 21 as libc::c_int;
    yfreqs = 240 as libc::c_int;
    downsample = 1 as libc::c_int;
    if strcmp(
        *_argv.offset(1 as libc::c_int as isize),
        b"-r\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        rate = atoi(*_argv.offset(2 as libc::c_int as isize)) as libc::c_uint;
        if rate != 8000 as libc::c_int as libc::c_uint
            && rate != 12000 as libc::c_int as libc::c_uint
            && rate != 16000 as libc::c_int as libc::c_uint
            && rate != 24000 as libc::c_int as libc::c_uint
            && rate != 48000 as libc::c_int as libc::c_uint
        {
            fprintf(
                stderr(),
                b"Sampling rate must be 8000, 12000, 16000, 24000, or 48000\n\0" as *const u8
                    as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
        downsample = (48000 as libc::c_int as libc::c_uint).wrapping_div(rate) as libc::c_int;
        match rate {
            8000 => {
                ybands = 13 as libc::c_int;
            }
            12000 => {
                ybands = 15 as libc::c_int;
            }
            16000 => {
                ybands = 17 as libc::c_int;
            }
            24000 => {
                ybands = 19 as libc::c_int;
            }
            _ => {}
        }
        yfreqs = 240 as libc::c_int / downsample;
        _argv = _argv.offset(2 as libc::c_int as isize);
    }
    fin1 = fopen(
        *_argv.offset(1 as libc::c_int as isize),
        b"rb\0" as *const u8 as *const libc::c_char,
    );
    if fin1.is_null() {
        fprintf(
            stderr(),
            b"Error opening '%s'.\n\0" as *const u8 as *const libc::c_char,
            *_argv.offset(1 as libc::c_int as isize),
        );
        return 1 as libc::c_int;
    }
    fin2 = fopen(
        *_argv.offset(2 as libc::c_int as isize),
        b"rb\0" as *const u8 as *const libc::c_char,
    );
    if fin2.is_null() {
        fprintf(
            stderr(),
            b"Error opening '%s'.\n\0" as *const u8 as *const libc::c_char,
            *_argv.offset(2 as libc::c_int as isize),
        );
        fclose(fin1);
        return 1 as libc::c_int;
    }
    xlength = read_pcm16(&mut x, fin1, 2 as libc::c_int);
    if nchannels == 1 as libc::c_int {
        xi = 0 as libc::c_int as size_t;
        while xi < xlength {
            *x.offset(xi as isize) = (0.5f64
                * (*x.offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(xi) as isize)
                    + *x.offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(xi)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    )) as libc::c_double) as libc::c_float;
            xi = xi.wrapping_add(1);
        }
    }
    fclose(fin1);
    ylength = read_pcm16(&mut y, fin2, nchannels);
    fclose(fin2);
    if xlength != ylength.wrapping_mul(downsample as libc::c_ulong) {
        fprintf(
            stderr(),
            b"Sample counts do not match (%lu!=%lu).\n\0" as *const u8 as *const libc::c_char,
            xlength,
            ylength.wrapping_mul(downsample as libc::c_ulong),
        );
        return 1 as libc::c_int;
    }
    if xlength < 480 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr(),
            b"Insufficient sample data (%lu<%i).\n\0" as *const u8 as *const libc::c_char,
            xlength,
            480 as libc::c_int,
        );
        return 1 as libc::c_int;
    }
    nframes = xlength
        .wrapping_sub(480 as libc::c_int as libc::c_ulong)
        .wrapping_add(120 as libc::c_int as libc::c_ulong)
        .wrapping_div(120 as libc::c_int as libc::c_ulong);
    xb = opus_malloc(
        nframes
            .wrapping_mul(21 as libc::c_int as libc::c_ulong)
            .wrapping_mul(nchannels as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong),
    ) as *mut libc::c_float;
    X = opus_malloc(
        nframes
            .wrapping_mul(240 as libc::c_int as libc::c_ulong)
            .wrapping_mul(nchannels as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong),
    ) as *mut libc::c_float;
    Y = opus_malloc(
        nframes
            .wrapping_mul(yfreqs as libc::c_ulong)
            .wrapping_mul(nchannels as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong),
    ) as *mut libc::c_float;
    band_energy(
        xb,
        X,
        BANDS.as_ptr(),
        21 as libc::c_int,
        x,
        nchannels,
        nframes,
        480 as libc::c_int,
        120 as libc::c_int,
        1 as libc::c_int,
    );
    free(x as *mut libc::c_void);
    band_energy(
        0 as *mut libc::c_float,
        Y,
        BANDS.as_ptr(),
        ybands,
        y,
        nchannels,
        nframes,
        480 as libc::c_int / downsample,
        120 as libc::c_int / downsample,
        downsample,
    );
    free(y as *mut libc::c_void);
    xi = 0 as libc::c_int as size_t;
    while xi < nframes {
        bi = 1 as libc::c_int;
        while bi < 21 as libc::c_int {
            ci = 0 as libc::c_int;
            while ci < nchannels {
                *xb.offset(
                    xi.wrapping_mul(21 as libc::c_int as libc::c_ulong)
                        .wrapping_add(bi as libc::c_ulong)
                        .wrapping_mul(nchannels as libc::c_ulong)
                        .wrapping_add(ci as libc::c_ulong) as isize,
                ) += 0.1f32
                    * *xb.offset(
                        xi.wrapping_mul(21 as libc::c_int as libc::c_ulong)
                            .wrapping_add(bi as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(nchannels as libc::c_ulong)
                            .wrapping_add(ci as libc::c_ulong) as isize,
                    );
                ci += 1;
            }
            bi += 1;
        }
        bi = 21 as libc::c_int - 1 as libc::c_int;
        loop {
            let fresh0 = bi;
            bi = bi - 1;
            if !(fresh0 > 0 as libc::c_int) {
                break;
            }
            ci = 0 as libc::c_int;
            while ci < nchannels {
                *xb.offset(
                    xi.wrapping_mul(21 as libc::c_int as libc::c_ulong)
                        .wrapping_add(bi as libc::c_ulong)
                        .wrapping_mul(nchannels as libc::c_ulong)
                        .wrapping_add(ci as libc::c_ulong) as isize,
                ) += 0.03f32
                    * *xb.offset(
                        xi.wrapping_mul(21 as libc::c_int as libc::c_ulong)
                            .wrapping_add(bi as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(nchannels as libc::c_ulong)
                            .wrapping_add(ci as libc::c_ulong) as isize,
                    );
                ci += 1;
            }
        }
        if xi > 0 as libc::c_int as libc::c_ulong {
            bi = 0 as libc::c_int;
            while bi < 21 as libc::c_int {
                ci = 0 as libc::c_int;
                while ci < nchannels {
                    *xb.offset(
                        xi.wrapping_mul(21 as libc::c_int as libc::c_ulong)
                            .wrapping_add(bi as libc::c_ulong)
                            .wrapping_mul(nchannels as libc::c_ulong)
                            .wrapping_add(ci as libc::c_ulong) as isize,
                    ) += 0.5f32
                        * *xb.offset(
                            xi.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(21 as libc::c_int as libc::c_ulong)
                                .wrapping_add(bi as libc::c_ulong)
                                .wrapping_mul(nchannels as libc::c_ulong)
                                .wrapping_add(ci as libc::c_ulong)
                                as isize,
                        );
                    ci += 1;
                }
                bi += 1;
            }
        }
        if nchannels == 2 as libc::c_int {
            bi = 0 as libc::c_int;
            while bi < 21 as libc::c_int {
                let mut l: libc::c_float = 0.;
                let mut r: libc::c_float = 0.;
                l = *xb.offset(
                    xi.wrapping_mul(21 as libc::c_int as libc::c_ulong)
                        .wrapping_add(bi as libc::c_ulong)
                        .wrapping_mul(nchannels as libc::c_ulong)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        as isize,
                );
                r = *xb.offset(
                    xi.wrapping_mul(21 as libc::c_int as libc::c_ulong)
                        .wrapping_add(bi as libc::c_ulong)
                        .wrapping_mul(nchannels as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        as isize,
                );
                *xb.offset(
                    xi.wrapping_mul(21 as libc::c_int as libc::c_ulong)
                        .wrapping_add(bi as libc::c_ulong)
                        .wrapping_mul(nchannels as libc::c_ulong)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        as isize,
                ) += 0.01f32 * r;
                *xb.offset(
                    xi.wrapping_mul(21 as libc::c_int as libc::c_ulong)
                        .wrapping_add(bi as libc::c_ulong)
                        .wrapping_mul(nchannels as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        as isize,
                ) += 0.01f32 * l;
                bi += 1;
            }
        }
        bi = 0 as libc::c_int;
        while bi < ybands {
            xj = BANDS[bi as usize];
            while xj < BANDS[(bi + 1 as libc::c_int) as usize] {
                ci = 0 as libc::c_int;
                while ci < nchannels {
                    *X.offset(
                        xi.wrapping_mul(240 as libc::c_int as libc::c_ulong)
                            .wrapping_add(xj as libc::c_ulong)
                            .wrapping_mul(nchannels as libc::c_ulong)
                            .wrapping_add(ci as libc::c_ulong) as isize,
                    ) += 0.1f32
                        * *xb.offset(
                            xi.wrapping_mul(21 as libc::c_int as libc::c_ulong)
                                .wrapping_add(bi as libc::c_ulong)
                                .wrapping_mul(nchannels as libc::c_ulong)
                                .wrapping_add(ci as libc::c_ulong)
                                as isize,
                        );
                    *Y.offset(
                        xi.wrapping_mul(yfreqs as libc::c_ulong)
                            .wrapping_add(xj as libc::c_ulong)
                            .wrapping_mul(nchannels as libc::c_ulong)
                            .wrapping_add(ci as libc::c_ulong) as isize,
                    ) += 0.1f32
                        * *xb.offset(
                            xi.wrapping_mul(21 as libc::c_int as libc::c_ulong)
                                .wrapping_add(bi as libc::c_ulong)
                                .wrapping_mul(nchannels as libc::c_ulong)
                                .wrapping_add(ci as libc::c_ulong)
                                as isize,
                        );
                    ci += 1;
                }
                xj += 1;
            }
            bi += 1;
        }
        xi = xi.wrapping_add(1);
    }
    bi = 0 as libc::c_int;
    while bi < ybands {
        xj = BANDS[bi as usize];
        while xj < BANDS[(bi + 1 as libc::c_int) as usize] {
            ci = 0 as libc::c_int;
            while ci < nchannels {
                let mut xtmp: libc::c_float = 0.;
                let mut ytmp: libc::c_float = 0.;
                xtmp = *X.offset((xj * nchannels + ci) as isize);
                ytmp = *Y.offset((xj * nchannels + ci) as isize);
                xi = 1 as libc::c_int as size_t;
                while xi < nframes {
                    let mut xtmp2: libc::c_float = 0.;
                    let mut ytmp2: libc::c_float = 0.;
                    xtmp2 = *X.offset(
                        xi.wrapping_mul(240 as libc::c_int as libc::c_ulong)
                            .wrapping_add(xj as libc::c_ulong)
                            .wrapping_mul(nchannels as libc::c_ulong)
                            .wrapping_add(ci as libc::c_ulong) as isize,
                    );
                    ytmp2 = *Y.offset(
                        xi.wrapping_mul(yfreqs as libc::c_ulong)
                            .wrapping_add(xj as libc::c_ulong)
                            .wrapping_mul(nchannels as libc::c_ulong)
                            .wrapping_add(ci as libc::c_ulong) as isize,
                    );
                    *X.offset(
                        xi.wrapping_mul(240 as libc::c_int as libc::c_ulong)
                            .wrapping_add(xj as libc::c_ulong)
                            .wrapping_mul(nchannels as libc::c_ulong)
                            .wrapping_add(ci as libc::c_ulong) as isize,
                    ) += xtmp;
                    *Y.offset(
                        xi.wrapping_mul(yfreqs as libc::c_ulong)
                            .wrapping_add(xj as libc::c_ulong)
                            .wrapping_mul(nchannels as libc::c_ulong)
                            .wrapping_add(ci as libc::c_ulong) as isize,
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
    if rate == 48000 as libc::c_int as libc::c_uint {
        max_compare = BANDS[21 as libc::c_int as usize];
    } else if rate == 12000 as libc::c_int as libc::c_uint {
        max_compare = BANDS[ybands as usize];
    } else {
        max_compare = BANDS[ybands as usize] - 3 as libc::c_int;
    }
    err = 0 as libc::c_int as libc::c_double;
    xi = 0 as libc::c_int as size_t;
    while xi < nframes {
        let mut Ef: libc::c_double = 0.;
        Ef = 0 as libc::c_int as libc::c_double;
        bi = 0 as libc::c_int;
        while bi < ybands {
            let mut Eb: libc::c_double = 0.;
            Eb = 0 as libc::c_int as libc::c_double;
            xj = BANDS[bi as usize];
            while xj < BANDS[(bi + 1 as libc::c_int) as usize] && xj < max_compare {
                ci = 0 as libc::c_int;
                while ci < nchannels {
                    let mut re: libc::c_float = 0.;
                    let mut im: libc::c_float = 0.;
                    re = *Y.offset(
                        xi.wrapping_mul(yfreqs as libc::c_ulong)
                            .wrapping_add(xj as libc::c_ulong)
                            .wrapping_mul(nchannels as libc::c_ulong)
                            .wrapping_add(ci as libc::c_ulong) as isize,
                    ) / *X.offset(
                        xi.wrapping_mul(240 as libc::c_int as libc::c_ulong)
                            .wrapping_add(xj as libc::c_ulong)
                            .wrapping_mul(nchannels as libc::c_ulong)
                            .wrapping_add(ci as libc::c_ulong) as isize,
                    );
                    im = re - re.ln() - 1.0;
                    if xj >= 79 as libc::c_int && xj <= 81 as libc::c_int {
                        im *= 0.1f32;
                    }
                    if xj == 80 as libc::c_int {
                        im *= 0.1f32;
                    }
                    Eb += im as libc::c_double;
                    ci += 1;
                }
                xj += 1;
            }
            Eb /= ((BANDS[(bi + 1 as libc::c_int) as usize] - BANDS[bi as usize]) * nchannels)
                as libc::c_double;
            Ef += Eb * Eb;
            bi += 1;
        }
        Ef /= 21 as libc::c_int as libc::c_double;
        Ef *= Ef;
        err += Ef * Ef;
        xi = xi.wrapping_add(1);
    }
    free(xb as *mut libc::c_void);
    free(X as *mut libc::c_void);
    free(Y as *mut libc::c_void);
    err = (err / nframes as f64).powf(1.0 / 16.0);
    Q = (100.0 * (1.0 - 0.5 * (1.0 + err).ln() / 1.13f64.ln())) as f32;
    if Q < 0 as libc::c_int as libc::c_float {
        fprintf(
            stderr(),
            b"Test vector FAILS\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr(),
            b"Internal weighted error is %f\n\0" as *const u8 as *const libc::c_char,
            err,
        );
        return 1 as libc::c_int;
    } else {
        fprintf(
            stderr(),
            b"Test vector PASSES\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr(),
            b"Opus quality metric: %.1f %% (internal weighted error is %f)\n\0" as *const u8
                as *const libc::c_char,
            Q as libc::c_double,
            err,
        );
        return 0 as libc::c_int;
    };
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *const libc::c_char,
        ) as i32)
    }
}
