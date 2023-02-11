use ::libc;
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_defines.h:32"]
pub mod opus_defines_h {
    #[c2rust::src_loc = "54:9"]
    pub const OPUS_INVALID_PACKET: libc::c_int = -(4 as libc::c_int);
    #[c2rust::src_loc = "48:9"]
    pub const OPUS_BAD_ARG: libc::c_int = -(1 as libc::c_int);
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "89:11"]
    pub const NULL: libc::c_int = 0 as libc::c_int;
}
pub use self::opus_defines_h::{OPUS_BAD_ARG, OPUS_INVALID_PACKET};
pub use self::stddef_h::NULL;

#[c2rust::src_loc = "36:1"]
pub unsafe extern "C" fn opus_pcm_soft_clip(
    mut _x: *mut libc::c_float,
    N: libc::c_int,
    C: libc::c_int,
    declip_mem: *mut libc::c_float,
) {
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut x: *mut libc::c_float = 0 as *mut libc::c_float;
    if C < 1 as libc::c_int || N < 1 as libc::c_int || _x.is_null() || declip_mem.is_null() {
        return;
    }
    i = 0 as libc::c_int;
    while i < N * C {
        *_x.offset(i as isize) = if -2.0f32
            > (if 2.0f32 < *_x.offset(i as isize) {
                2.0f32
            } else {
                *_x.offset(i as isize)
            }) {
            -2.0f32
        } else if 2.0f32 < *_x.offset(i as isize) {
            2.0f32
        } else {
            *_x.offset(i as isize)
        };
        i += 1;
    }
    c = 0 as libc::c_int;
    while c < C {
        let mut a: libc::c_float = 0.;
        let mut x0: libc::c_float = 0.;
        let mut curr: libc::c_int = 0;
        x = _x.offset(c as isize);
        a = *declip_mem.offset(c as isize);
        i = 0 as libc::c_int;
        while i < N {
            if *x.offset((i * C) as isize) * a >= 0 as libc::c_int as libc::c_float {
                break;
            }
            *x.offset((i * C) as isize) = *x.offset((i * C) as isize)
                + a * *x.offset((i * C) as isize) * *x.offset((i * C) as isize);
            i += 1;
        }
        curr = 0 as libc::c_int;
        x0 = *x.offset(0 as libc::c_int as isize);
        loop {
            let mut start: libc::c_int = 0;
            let mut end: libc::c_int = 0;
            let mut maxval: libc::c_float = 0.;
            let mut special: libc::c_int = 0 as libc::c_int;
            let mut peak_pos: libc::c_int = 0;
            i = curr;
            while i < N {
                if *x.offset((i * C) as isize) > 1 as libc::c_int as libc::c_float
                    || *x.offset((i * C) as isize) < -(1 as libc::c_int) as libc::c_float
                {
                    break;
                }
                i += 1;
            }
            if i == N {
                a = 0 as libc::c_int as libc::c_float;
                break;
            } else {
                peak_pos = i;
                end = i;
                start = end;
                maxval = (*x.offset((i * C) as isize)).abs();
                while start > 0 as libc::c_int
                    && *x.offset((i * C) as isize)
                        * *x.offset(((start - 1 as libc::c_int) * C) as isize)
                        >= 0 as libc::c_int as libc::c_float
                {
                    start -= 1;
                }
                while end < N
                    && *x.offset((i * C) as isize) * *x.offset((end * C) as isize)
                        >= 0 as libc::c_int as libc::c_float
                {
                    if (*x.offset((end * C) as isize)).abs() > maxval {
                        maxval = (*x.offset((end * C) as isize)).abs();
                        peak_pos = end;
                    }
                    end += 1;
                }
                special = (start == 0 as libc::c_int
                    && *x.offset((i * C) as isize) * *x.offset(0 as libc::c_int as isize)
                        >= 0 as libc::c_int as libc::c_float)
                    as libc::c_int;
                a = (maxval - 1 as libc::c_int as libc::c_float) / (maxval * maxval);
                a += a * 2.4e-7f32;
                if *x.offset((i * C) as isize) > 0 as libc::c_int as libc::c_float {
                    a = -a;
                }
                i = start;
                while i < end {
                    *x.offset((i * C) as isize) = *x.offset((i * C) as isize)
                        + a * *x.offset((i * C) as isize) * *x.offset((i * C) as isize);
                    i += 1;
                }
                if special != 0 && peak_pos >= 2 as libc::c_int {
                    let mut delta: libc::c_float = 0.;
                    let mut offset: libc::c_float = x0 - *x.offset(0 as libc::c_int as isize);
                    delta = offset / peak_pos as libc::c_float;
                    i = curr;
                    while i < peak_pos {
                        offset -= delta;
                        *x.offset((i * C) as isize) += offset;
                        *x.offset((i * C) as isize) = if -1.0f32
                            > (if 1.0f32 < *x.offset((i * C) as isize) {
                                1.0f32
                            } else {
                                *x.offset((i * C) as isize)
                            }) {
                            -1.0f32
                        } else if 1.0f32 < *x.offset((i * C) as isize) {
                            1.0f32
                        } else {
                            *x.offset((i * C) as isize)
                        };
                        i += 1;
                    }
                }
                curr = end;
                if curr == N {
                    break;
                }
            }
        }
        *declip_mem.offset(c as isize) = a;
        c += 1;
    }
}
#[c2rust::src_loc = "140:1"]
pub unsafe extern "C" fn encode_size(size: libc::c_int, data: *mut libc::c_uchar) -> libc::c_int {
    if size < 252 as libc::c_int {
        *data.offset(0 as libc::c_int as isize) = size as libc::c_uchar;
        return 1 as libc::c_int;
    } else {
        *data.offset(0 as libc::c_int as isize) =
            (252 as libc::c_int + (size & 0x3 as libc::c_int)) as libc::c_uchar;
        *data.offset(1 as libc::c_int as isize) = (size
            - *data.offset(0 as libc::c_int as isize) as libc::c_int
            >> 2 as libc::c_int) as libc::c_uchar;
        return 2 as libc::c_int;
    };
}
#[c2rust::src_loc = "153:1"]
unsafe extern "C" fn parse_size(
    data: *const libc::c_uchar,
    len: i32,
    size: *mut i16,
) -> libc::c_int {
    if len < 1 as libc::c_int {
        *size = -(1 as libc::c_int) as i16;
        return -(1 as libc::c_int);
    } else if (*data.offset(0 as libc::c_int as isize) as libc::c_int) < 252 as libc::c_int {
        *size = *data.offset(0 as libc::c_int as isize) as i16;
        return 1 as libc::c_int;
    } else if len < 2 as libc::c_int {
        *size = -(1 as libc::c_int) as i16;
        return -(1 as libc::c_int);
    } else {
        *size = (4 as libc::c_int * *data.offset(1 as libc::c_int as isize) as libc::c_int
            + *data.offset(0 as libc::c_int as isize) as libc::c_int) as i16;
        return 2 as libc::c_int;
    };
}
#[c2rust::src_loc = "173:1"]
pub unsafe extern "C" fn opus_packet_get_samples_per_frame(
    data: *const libc::c_uchar,
    Fs: i32,
) -> libc::c_int {
    let mut audiosize: libc::c_int = 0;
    if *data.offset(0 as libc::c_int as isize) as libc::c_int & 0x80 as libc::c_int != 0 {
        audiosize = *data.offset(0 as libc::c_int as isize) as libc::c_int >> 3 as libc::c_int
            & 0x3 as libc::c_int;
        audiosize = (Fs << audiosize) / 400 as libc::c_int;
    } else if *data.offset(0 as libc::c_int as isize) as libc::c_int & 0x60 as libc::c_int
        == 0x60 as libc::c_int
    {
        audiosize =
            if *data.offset(0 as libc::c_int as isize) as libc::c_int & 0x8 as libc::c_int != 0 {
                Fs / 50 as libc::c_int
            } else {
                Fs / 100 as libc::c_int
            };
    } else {
        audiosize = *data.offset(0 as libc::c_int as isize) as libc::c_int >> 3 as libc::c_int
            & 0x3 as libc::c_int;
        if audiosize == 3 as libc::c_int {
            audiosize = Fs * 60 as libc::c_int / 1000 as libc::c_int;
        } else {
            audiosize = (Fs << audiosize) / 100 as libc::c_int;
        }
    }
    return audiosize;
}
#[c2rust::src_loc = "194:1"]
pub unsafe extern "C" fn opus_packet_parse_impl(
    mut data: *const libc::c_uchar,
    mut len: i32,
    self_delimited: libc::c_int,
    out_toc: *mut libc::c_uchar,
    frames: *mut *const libc::c_uchar,
    size: *mut i16,
    payload_offset: *mut libc::c_int,
    packet_offset: *mut i32,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut bytes: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut cbr: libc::c_int = 0;
    let mut ch: libc::c_uchar = 0;
    let mut toc: libc::c_uchar = 0;
    let mut framesize: libc::c_int = 0;
    let mut last_size: i32 = 0;
    let mut pad: i32 = 0 as libc::c_int;
    let data0: *const libc::c_uchar = data;
    if size.is_null() || len < 0 as libc::c_int {
        return OPUS_BAD_ARG;
    }
    if len == 0 as libc::c_int {
        return OPUS_INVALID_PACKET;
    }
    framesize = opus_packet_get_samples_per_frame(data, 48000 as libc::c_int);
    cbr = 0 as libc::c_int;
    let fresh0 = data;
    data = data.offset(1);
    toc = *fresh0;
    len -= 1;
    last_size = len;
    match toc as libc::c_int & 0x3 as libc::c_int {
        0 => {
            count = 1 as libc::c_int;
        }
        1 => {
            count = 2 as libc::c_int;
            cbr = 1 as libc::c_int;
            if self_delimited == 0 {
                if len & 0x1 as libc::c_int != 0 {
                    return OPUS_INVALID_PACKET;
                }
                last_size = len / 2 as libc::c_int;
                *size.offset(0 as libc::c_int as isize) = last_size as i16;
            }
        }
        2 => {
            count = 2 as libc::c_int;
            bytes = parse_size(data, len, size);
            len -= bytes;
            if (*size.offset(0 as libc::c_int as isize) as libc::c_int) < 0 as libc::c_int
                || *size.offset(0 as libc::c_int as isize) as libc::c_int > len
            {
                return OPUS_INVALID_PACKET;
            }
            data = data.offset(bytes as isize);
            last_size = len - *size.offset(0 as libc::c_int as isize) as libc::c_int;
        }
        _ => {
            if len < 1 as libc::c_int {
                return OPUS_INVALID_PACKET;
            }
            let fresh1 = data;
            data = data.offset(1);
            ch = *fresh1;
            count = ch as libc::c_int & 0x3f as libc::c_int;
            if count <= 0 as libc::c_int || framesize * count > 5760 as libc::c_int {
                return OPUS_INVALID_PACKET;
            }
            len -= 1;
            if ch as libc::c_int & 0x40 as libc::c_int != 0 {
                let mut p: libc::c_int = 0;
                loop {
                    let mut tmp: libc::c_int = 0;
                    if len <= 0 as libc::c_int {
                        return OPUS_INVALID_PACKET;
                    }
                    let fresh2 = data;
                    data = data.offset(1);
                    p = *fresh2 as libc::c_int;
                    len -= 1;
                    tmp = if p == 255 as libc::c_int {
                        254 as libc::c_int
                    } else {
                        p
                    };
                    len -= tmp;
                    pad += tmp;
                    if !(p == 255 as libc::c_int) {
                        break;
                    }
                }
            }
            if len < 0 as libc::c_int {
                return OPUS_INVALID_PACKET;
            }
            cbr = (ch as libc::c_int & 0x80 as libc::c_int == 0) as libc::c_int;
            if cbr == 0 {
                last_size = len;
                i = 0 as libc::c_int;
                while i < count - 1 as libc::c_int {
                    bytes = parse_size(data, len, size.offset(i as isize));
                    len -= bytes;
                    if (*size.offset(i as isize) as libc::c_int) < 0 as libc::c_int
                        || *size.offset(i as isize) as libc::c_int > len
                    {
                        return OPUS_INVALID_PACKET;
                    }
                    data = data.offset(bytes as isize);
                    last_size -= bytes + *size.offset(i as isize) as libc::c_int;
                    i += 1;
                }
                if last_size < 0 as libc::c_int {
                    return OPUS_INVALID_PACKET;
                }
            } else if self_delimited == 0 {
                last_size = len / count;
                if last_size * count != len {
                    return OPUS_INVALID_PACKET;
                }
                i = 0 as libc::c_int;
                while i < count - 1 as libc::c_int {
                    *size.offset(i as isize) = last_size as i16;
                    i += 1;
                }
            }
        }
    }
    if self_delimited != 0 {
        bytes = parse_size(
            data,
            len,
            size.offset(count as isize)
                .offset(-(1 as libc::c_int as isize)),
        );
        len -= bytes;
        if (*size.offset((count - 1 as libc::c_int) as isize) as libc::c_int) < 0 as libc::c_int
            || *size.offset((count - 1 as libc::c_int) as isize) as libc::c_int > len
        {
            return OPUS_INVALID_PACKET;
        }
        data = data.offset(bytes as isize);
        if cbr != 0 {
            if *size.offset((count - 1 as libc::c_int) as isize) as libc::c_int * count > len {
                return OPUS_INVALID_PACKET;
            }
            i = 0 as libc::c_int;
            while i < count - 1 as libc::c_int {
                *size.offset(i as isize) = *size.offset((count - 1 as libc::c_int) as isize);
                i += 1;
            }
        } else if bytes + *size.offset((count - 1 as libc::c_int) as isize) as libc::c_int
            > last_size
        {
            return OPUS_INVALID_PACKET;
        }
    } else {
        if last_size > 1275 as libc::c_int {
            return OPUS_INVALID_PACKET;
        }
        *size.offset((count - 1 as libc::c_int) as isize) = last_size as i16;
    }
    if !payload_offset.is_null() {
        *payload_offset = data.offset_from(data0) as libc::c_long as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < count {
        if !frames.is_null() {
            let ref mut fresh3 = *frames.offset(i as isize);
            *fresh3 = data;
        }
        data = data.offset(*size.offset(i as isize) as libc::c_int as isize);
        i += 1;
    }
    if !packet_offset.is_null() {
        *packet_offset = pad + data.offset_from(data0) as libc::c_long as i32;
    }
    if !out_toc.is_null() {
        *out_toc = toc;
    }
    return count;
}
#[c2rust::src_loc = "349:1"]
pub unsafe extern "C" fn opus_packet_parse(
    data: *const libc::c_uchar,
    len: i32,
    out_toc: *mut libc::c_uchar,
    frames: *mut *const libc::c_uchar,
    size: *mut i16,
    payload_offset: *mut libc::c_int,
) -> libc::c_int {
    return opus_packet_parse_impl(
        data,
        len,
        0 as libc::c_int,
        out_toc,
        frames,
        size,
        payload_offset,
        NULL as *mut i32,
    );
}
